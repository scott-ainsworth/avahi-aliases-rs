//! Error consolidation
//!
//! Consolidate errors so that fatal, but handlable errors can be pushed up the stack
//! to user-facing and/or logging code.

#![warn(clippy::all)]

use std::io;

use thiserror::Error;

/// An enum to consolidate all errors passed to user-aware and logging code.
#[derive(Error, Debug)]
pub enum ErrorWrapper {
    /// An error setting up or using debug
    #[error("D-Bus failure: {message}")]
    DBusError { message: String },

    /// An invalid alias
    #[error(r#"invalid alias: "{alias}""#)]
    InvalidAliasError {
        /// The invalid alias
        alias: String,
    },

    /// An invalid alias file (invalid alias found in the alias file)
    #[error(r#"invalid alias "{alias}" found in "{file_name}""#)]
    InvalidAliasFileError {
        /// Name of the aliases file containing the error
        file_name: String,
        /// The invalid alias the caused the error
        alias: String,
    },

    /// A logging error has occured
    #[error(r#"logging error"#)]
    LoggingError,

    /// An I/O error retrieving the aliases file metadata
    #[error(r#"could not get metadata for "{file_name}": {source}."#)]
    MetadataError {
        /// Name of the aliases file
        file_name: String,
        /// The source `io::Error`
        source: io::Error,
    },

    /// An I/O error while opeing the aliases file
    #[error(r#"could not open "{}": {source}."#, file_name)]
    OpenError {
        /// Name of the aliases file
        file_name: String,
        /// The source `io::Error`
        source: io::Error,
    },

    /// An I/O error while reading the aliases file
    #[error(r#"could not read "{}": {source}."#, file_name)]
    ReadError {
        /// Name of the aliases file
        file_name: String,
        /// The source `io::Error`
        source: io::Error,
    },

    /// An I/O error writing to the aliases file
    #[error(r#"could not write "{}": {source}."#, .file_name)]
    WriteError {
        /// Name of the aliases file
        file_name: String,
        /// The source `io::Error`
        source: io::Error,
    },
}

/// Convert a `SetLoggerError` into an `ErrorWrapper`.
impl From<log::SetLoggerError> for ErrorWrapper {
    fn from(_: log::SetLoggerError) -> ErrorWrapper { ErrorWrapper::LoggingError }
}

/// Convert a `SetLoggerError` into an `ErrorWrapper`.
impl From<syslog::Error> for ErrorWrapper {
    fn from(_: syslog::Error) -> ErrorWrapper { ErrorWrapper::LoggingError }
}

/// Convert a `dbus::Error` into an `ErrorWrapper`.
impl From<dbus::Error> for ErrorWrapper {
    fn from(error: dbus::Error) -> ErrorWrapper {
        ErrorWrapper::DBusError {
            message: error.message().unwrap_or("Unknown error").to_owned(),
        }
    }
}

impl ErrorWrapper {
    /// Initialize a new InvalidAliasError
    ///
    /// A helper function that copies `alias` to a `String` owned by
    /// the `InvalidAliasError`.
    pub fn new_invalid_alias_error<A>(alias: A) -> ErrorWrapper
    where
        A: Into<String>, {
        ErrorWrapper::InvalidAliasError { alias: alias.into() }
    }

    /// Initialize a new InvalidAliasFileError
    ///
    /// A helper function that copies `file_name` and `alias` to `String`s owned by
    /// the `InvalidAliasFileError`.
    pub fn new_invalid_alias_file_error<F, A>(file_name: F, alias: A) -> ErrorWrapper
    where
        F: Into<String>,
        A: Into<String>, {
        ErrorWrapper::InvalidAliasFileError { file_name: file_name.into(), alias: alias.into() }
    }

    /// Initialize a new MetadataError
    ///
    /// A helper function that copies `file_name` to a `String`s owned by
    /// the `MetadataError`.
    pub fn new_metadata_error<F>(file_name: F, source: io::Error) -> ErrorWrapper
    where
        F: Into<String>, {
        ErrorWrapper::MetadataError { file_name: file_name.into(), source }
    }

    /// Initialize a new OpenError
    ///
    /// A helper function that copies `file_name` to a `String`s owned by
    /// the `OpenError`.
    pub fn new_open_error<F>(file_name: F, source: io::Error) -> ErrorWrapper
    where
        F: Into<String>, {
        ErrorWrapper::OpenError { file_name: file_name.into(), source }
    }

    // /// Initialize a new EnumOutOfRangeError
    // ///
    // /// A helper function that copies `enum_name` to a `String`s owned by
    // /// the `EnumOutOfRangeError`.
    // pub fn new_enum_out_of_range_error<N>(enum_name: N, value: i32) -> ErrorWrapper
    // where
    //     N: Into<String>, {
    //     ErrorWrapper::EnumOutOfRangeError { enum_name: enum_name.into(), value }
    // }

    /// Initialize a new ReadError
    ///
    /// A helper function that copies `file_name` to a `String`s owned by
    /// the `ReadError`.
    pub fn new_read_error<F>(file_name: F, source: io::Error) -> ErrorWrapper
    where
        F: Into<String>, {
        ErrorWrapper::ReadError { file_name: file_name.into(), source }
    }

    /// Initialize a new WriteError
    ///
    /// A helper function that copies `file_name` to a `String`s owned by
    /// the `WriteError`.
    pub fn new_write_error<F>(file_name: F, source: io::Error) -> ErrorWrapper
    where
        F: Into<String>, {
        ErrorWrapper::WriteError { file_name: file_name.into(), source }
    }
}

//**********************************************************************************************
// Unit tests
//**********************************************************************************************

#[cfg(test)]
mod tests {
    use ErrorWrapper::*;

    use super::*;

    type TestResult = Result<(), ErrorWrapper>;

    static TEST_ALIAS: &str = "a0.local";
    static TEST_FILENAME: &str = "avahi-aliases";
    static TEST_MESSAGE: &str = "message";

    //******************************************************************************************
    // ErrorWrapper::DBusError

    #[test]
    fn dbus_error_creation_works() {
        if let Err(error) = Err(DBusError { message: TEST_MESSAGE.to_owned() }) as TestResult {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn from_dbus_error_creates_error_wrapper() {
        let dbus_error = dbus::Error::new_failed(TEST_MESSAGE);
        let _: ErrorWrapper = dbus_error.into();
    }

    #[test]
    fn dbus_error_produces_correct_message() {
        if let Err(error) = Err(DBusError { message: TEST_MESSAGE.to_owned() }) as TestResult {
            assert_eq!(format!("{}", error), format!(r#"D-Bus failure: {}"#, TEST_MESSAGE))
        }
    }

    //******************************************************************************************
    // ErrorWrapper::InvalidAliasError

    #[test]
    fn invalid_alias_error_creation_works() {
        if let Err(error) =
            Err(InvalidAliasError { alias: TEST_ALIAS.to_owned() }) as TestResult
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_invalid_alias_error_creation_works() {
        let _ = ErrorWrapper::new_invalid_alias_error(TEST_ALIAS);
    }

    #[test]
    fn invalid_alias_error_produces_correct_message() {
        if let Err(error) =
            Err(InvalidAliasError { alias: TEST_ALIAS.to_owned() }) as TestResult
        {
            assert_eq!(format!("{}", error), format!(r#"invalid alias: "{}""#, TEST_ALIAS))
        }
    }

    //******************************************************************************************
    // ErrorWrapper::InvalidAliasFileError

    #[test]
    fn invalid_alias_file_error_creation_works() {
        if let Err(error) = Err(InvalidAliasFileError {
            file_name: TEST_FILENAME.to_owned(),
            alias: TEST_ALIAS.to_owned(),
        }) as TestResult
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_invalid_alias_file_error_creation_works() {
        let _ = ErrorWrapper::new_invalid_alias_file_error(TEST_FILENAME, TEST_ALIAS);
    }

    #[test]
    fn invalid_alias_file_error_produces_correct_message() {
        if let Err(error) = Err(InvalidAliasFileError {
            file_name: TEST_FILENAME.to_owned(),
            alias: TEST_ALIAS.to_owned(),
        }) as TestResult
        {
            assert_eq!(
                format!("{}", error),
                format!(r#"invalid alias "{}" found in "{}""#, TEST_ALIAS, TEST_FILENAME)
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::LoggingError

    #[test]
    fn logging_error_creation_works() {
        if let Err(error) = Err(LoggingError) as TestResult {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn logging_error_produces_correct_message() {
        if let Err(error) = Err(LoggingError) as TestResult {
            assert_eq!(format!("{}", error), r#"logging error"#)
        }
    }

    //***************************************************************************************
    // ErrorWrapper::MetadataError

    #[test]
    fn metadata_error_creation_works() {
        if let Err(error) = Err(MetadataError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_metadata_error_creation_works() {
        let _ = ErrorWrapper::new_metadata_error(
            TEST_FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        );
    }

    #[test]
    fn metadata_error_produces_correct_message() {
        if let Err(error) = Err(MetadataError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not get metadata for "{}": {}."#,
                    TEST_FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE)
                )
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::OpenError

    #[test]
    fn open_error_creation_works() {
        if let Err(error) = Err(OpenError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_open_error_creation_works() {
        let _ = ErrorWrapper::new_open_error(
            TEST_FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        );
    }

    #[test]
    fn open_error_produces_correct_message() {
        if let Err(error) = Err(OpenError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not open "{}": {}."#,
                    TEST_FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE)
                )
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::ReadError

    #[test]
    fn read_error_creation_works() {
        if let Err(error) = Err(ReadError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_read_error_creation_works() {
        let _ = ErrorWrapper::new_read_error(
            TEST_FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        );
    }

    #[test]
    fn read_error_produces_correct_message() {
        if let Err(error) = Err(ReadError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not read "{}": {}."#,
                    TEST_FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE)
                )
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::WriteError

    #[test]
    fn write_error_creation_works() {
        if let Err(error) = Err(WriteError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_write_error_creation_works() {
        let _ = ErrorWrapper::new_write_error(
            TEST_FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        );
    }

    #[test]
    fn write_error_produces_correct_message() {
        if let Err(error) = Err(WriteError {
            file_name: TEST_FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE),
        }) as TestResult
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not write "{}": {}."#,
                    TEST_FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, TEST_MESSAGE)
                )
            )
        }
    }
}
