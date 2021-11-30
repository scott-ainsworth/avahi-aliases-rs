//! Error consolidation
//!
//! Consolidate errors so that fatal, but handlable errors can be pushed up the stack
//! to user-facing and/or logging code.

#![warn(clippy::all)]

use std::io;

use thiserror::Error;

use crate::avahi_client::avahi;

/// An enum to consolidate all errors passed to user-aware and logging code.
#[derive(Error, Debug)]
pub enum ErrorWrapper {
    /// Attempted to convert an unrecognized Avahi `ClientState` value.
    #[error(transparent)]
    AvahiClientStateOutOfRangeError(#[from] avahi::ClientStateOutOfRangeError),

    /// Attempted to convert an unrecognized Avahi `EntryGroupState` value.
    #[error(transparent)]
    AvahiEntryGroupStateOutOfRangeError(#[from] avahi::EntryGroupStateOutOfRangeError),

    /// Attempted to convert an unrecognized Avahi `Interface` value.
    #[error(transparent)]
    AvahiInterfaceOutOfRangeError(#[from] avahi::InterfaceOutOfRangeError),

    /// Attempted to convert an unrecognized Avahi `Protocol` value.
    #[error(transparent)]
    AvahiProtocolOutOfRangeError(#[from] avahi::ProtocolOutOfRangeError),

    /// Attempted to convert an unrecognized Avahi `RecordClass` value.
    #[error(transparent)]
    AvahiRecordClassOutOfRangeError(#[from] avahi::RecordClassOutOfRangeError),

    /// Attempted to convert an unrecognized Avahi `RecordType` value.
    #[error(transparent)]
    AvahiRecordTypeOutOfRangeError(#[from] avahi::RecordTypeOutOfRangeError),

    /// Attempted to convert an unrecognized Avahi `ServerState` value.
    #[error(transparent)]
    AvahiServerStateOutOfRangeError(#[from] avahi::ServerStateOutOfRangeError),

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

    // /// Invalid value converting an int to an enum
    // #[error(r#"Invalid {} value: {}."#, enum_name, value)]
    // EnumOutOfRangeError {
    //     /// Name of the enum
    //     enum_name: String,
    //     /// The out-of-range value
    //     value: i32,
    // },

    // /// An I/O error
    // #[error("could not publish aliases")]
    // PublishError { source: io::Error },
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

    use crate::avahi_client::avahi;
    use crate::avahi::{
        ClientState, EntryGroupState, Interface, Protocol, RecordClass, RecordType, ServerState,
    };
    use super::*;

    type R = Result<(), ErrorWrapper>;

    static ALIAS: &str = "a0.local";
    static FILENAME: &str = "avahi-aliases";
    static MESSAGE: &str = "message";

    //***************************************************************************************
    // ErrorWrapper::Avahi...OutOfRangeError

    #[test]
    fn avahi_client_state_enum_out_of_range_error_creation_works() {
        let result = ClientState::try_from(-99);
        let error_wrapper = ErrorWrapper::AvahiClientStateOutOfRangeError(result.unwrap_err());
        assert!(matches!(error_wrapper, ErrorWrapper::AvahiClientStateOutOfRangeError(..)));
    }

    #[test]
    fn from_avahi_client_state_enum_out_of_range_error_produces_correct_error_wrapper() {
        fn convert(value: i32) -> Result<avahi::ClientState, ErrorWrapper> {
            Ok(avahi::ClientState::try_from(value)?)
        }
        let result = convert(-99);
        assert!(matches!(result, Err(ErrorWrapper::AvahiClientStateOutOfRangeError(..))));
    }

    #[test]
    fn avahi_entry_group_state_enum_out_of_range_error_creation_works() {
        let result = EntryGroupState::try_from(-99);
        let error_wrapper =
            ErrorWrapper::AvahiEntryGroupStateOutOfRangeError(result.unwrap_err());
        assert!(matches!(error_wrapper, ErrorWrapper::AvahiEntryGroupStateOutOfRangeError(..)));
    }

    #[test]
    fn from_avahi_entry_group_enum_out_of_range_error_produces_correct_error_wrapper() {
        fn convert(value: i32) -> Result<avahi::EntryGroupState, ErrorWrapper> {
            Ok(avahi::EntryGroupState::try_from(value)?)
        }
        let result = convert(-99);
        assert!(matches!(result, Err(ErrorWrapper::AvahiEntryGroupStateOutOfRangeError(..))));
    }

    #[test]
    fn avahi_interface_enum_out_of_range_error_creation_works() {
        let result = Interface::try_from(-99);
        let error_wrapper = ErrorWrapper::AvahiInterfaceOutOfRangeError(result.unwrap_err());
        assert!(matches!(error_wrapper, ErrorWrapper::AvahiInterfaceOutOfRangeError(..)));
    }

    #[test]
    fn from_avahi_interface_enum_out_of_range_error_produces_correct_error_wrapper() {
        fn convert(value: i32) -> Result<avahi::Interface, ErrorWrapper> {
            Ok(avahi::Interface::try_from(value)?)
        }
        let result = convert(-99);
        assert!(matches!(result, Err(ErrorWrapper::AvahiInterfaceOutOfRangeError(..))));
    }

    #[test]
    fn avahi_protocol_enum_out_of_range_error_creation_works() {
        let result = Protocol::try_from(-99);
        let error_wrapper = ErrorWrapper::AvahiProtocolOutOfRangeError(result.unwrap_err());
        assert!(matches!(error_wrapper, ErrorWrapper::AvahiProtocolOutOfRangeError(..)));
    }

    #[test]
    fn from_avahi_protocol_enum_out_of_range_error_produces_correct_error_wrapper() {
        fn convert(value: i32) -> Result<avahi::Protocol, ErrorWrapper> {
            Ok(avahi::Protocol::try_from(value)?)
        }
        let result = convert(-99);
        assert!(matches!(result, Err(ErrorWrapper::AvahiProtocolOutOfRangeError(..))));
    }

    #[test]
    fn avahi_record_class_enum_out_of_range_error_creation_works() {
        let result = RecordClass::try_from(99);
        let error_wrapper = ErrorWrapper::AvahiRecordClassOutOfRangeError(result.unwrap_err());
        assert!(matches!(error_wrapper, ErrorWrapper::AvahiRecordClassOutOfRangeError(..)));
    }

    #[test]
    fn from_avahi_record_class_enum_out_of_range_error_produces_correct_error_wrapper() {
        fn convert(value: u16) -> Result<avahi::RecordClass, ErrorWrapper> {
            Ok(avahi::RecordClass::try_from(value)?)
        }
        let result = convert(99);
        assert!(matches!(result, Err(ErrorWrapper::AvahiRecordClassOutOfRangeError(..))));
    }

    #[test]
    fn avahi_record_type_enum_out_of_range_error_creation_works() {
        let result = RecordType::try_from(99);
        let error_wrapper = ErrorWrapper::AvahiRecordTypeOutOfRangeError(result.unwrap_err());
        assert!(matches!(error_wrapper, ErrorWrapper::AvahiRecordTypeOutOfRangeError(..)));
    }

    #[test]
    fn from_avahi_record_type_enum_out_of_range_error_produces_correct_error_wrapper() {
        fn convert(value: u16) -> Result<avahi::RecordType, ErrorWrapper> {
            Ok(avahi::RecordType::try_from(value)?)
        }
        let result = convert(99);
        assert!(matches!(result, Err(ErrorWrapper::AvahiRecordTypeOutOfRangeError(..))));
    }

    #[test]
    fn avahi_server_state_enum_out_of_range_error_creation_works() {
        let result = ServerState::try_from(-99);
        let error_wrapper = ErrorWrapper::AvahiServerStateOutOfRangeError(result.unwrap_err());
        assert!(matches!(error_wrapper, ErrorWrapper::AvahiServerStateOutOfRangeError(..)));
    }

    #[test]
    fn from_avahi_server_state_enum_out_of_range_error_produces_correct_error_wrapper() {
        fn convert(value: i32) -> Result<avahi::ServerState, ErrorWrapper> {
            Ok(avahi::ServerState::try_from(value)?)
        }
        let result = convert(-99);
        assert!(matches!(result, Err(ErrorWrapper::AvahiServerStateOutOfRangeError(..))));
    }

    //******************************************************************************************
    // ErrorWrapper::DBusError

    #[test]
    fn dbus_error_creation_works() {
        if let Err(error) = Err(DBusError { message: MESSAGE.to_owned() }) as R {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn from_dbus_error_creates_error_wrapper() {
        let dbus_error = dbus::Error::new_failed(MESSAGE);
        let _: ErrorWrapper = dbus_error.into();
    }

    #[test]
    fn dbus_error_produces_correct_message() {
        if let Err(error) = Err(DBusError { message: MESSAGE.to_owned() }) as R {
            assert_eq!(format!("{}", error), format!(r#"D-Bus failure: {}"#, MESSAGE))
        }
    }

    //******************************************************************************************
    // ErrorWrapper::InvalidAliasError

    #[test]
    fn invalid_alias_error_creation_works() {
        if let Err(error) = Err(InvalidAliasError { alias: ALIAS.to_owned() }) as R {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_invalid_alias_error_creation_works() {
        let _ = ErrorWrapper::new_invalid_alias_error(ALIAS);
    }

    #[test]
    fn invalid_alias_error_produces_correct_message() {
        if let Err(error) = Err(InvalidAliasError { alias: ALIAS.to_owned() }) as R {
            assert_eq!(format!("{}", error), format!(r#"invalid alias: "{}""#, ALIAS))
        }
    }

    //******************************************************************************************
    // ErrorWrapper::InvalidAliasFileError

    #[test]
    fn invalid_alias_file_error_creation_works() {
        if let Err(error) = Err(InvalidAliasFileError {
            file_name: FILENAME.to_owned(),
            alias: ALIAS.to_owned(),
        }) as R
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_invalid_alias_file_error_creation_works() {
        let _ = ErrorWrapper::new_invalid_alias_file_error(FILENAME, ALIAS);
    }

    #[test]
    fn invalid_alias_file_error_produces_correct_message() {
        if let Err(error) = Err(InvalidAliasFileError {
            file_name: FILENAME.to_owned(),
            alias: ALIAS.to_owned(),
        }) as R
        {
            assert_eq!(
                format!("{}", error),
                format!(r#"invalid alias "{}" found in "{}""#, ALIAS, FILENAME)
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::LoggingError

    #[test]
    fn logging_error_creation_works() {
        if let Err(error) = Err(LoggingError) as R {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn logging_error_produces_correct_message() {
        if let Err(error) = Err(LoggingError) as R {
            assert_eq!(format!("{}", error), r#"logging error"#)
        }
    }

    //***************************************************************************************
    // ErrorWrapper::MetadataError

    #[test]
    fn metadata_error_creation_works() {
        if let Err(error) = Err(MetadataError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_metadata_error_creation_works() {
        let _ = ErrorWrapper::new_metadata_error(
            FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        );
    }

    #[test]
    fn metadata_error_produces_correct_message() {
        if let Err(error) = Err(MetadataError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not get metadata for "{}": {}."#,
                    FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, MESSAGE)
                )
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::OpenError

    #[test]
    fn open_error_creation_works() {
        if let Err(error) = Err(OpenError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_open_error_creation_works() {
        let _ = ErrorWrapper::new_open_error(
            FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        );
    }

    #[test]
    fn open_error_produces_correct_message() {
        if let Err(error) = Err(OpenError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not open "{}": {}."#,
                    FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, MESSAGE)
                )
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::ReadError

    #[test]
    fn read_error_creation_works() {
        if let Err(error) = Err(ReadError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_read_error_creation_works() {
        let _ = ErrorWrapper::new_read_error(
            FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        );
    }

    #[test]
    fn read_error_produces_correct_message() {
        if let Err(error) = Err(ReadError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not read "{}": {}."#,
                    FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, MESSAGE)
                )
            )
        }
    }

    //***************************************************************************************
    // ErrorWrapper::WriteError

    #[test]
    fn write_error_creation_works() {
        if let Err(error) = Err(WriteError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            let _ = format!("{}", error);
        }
    }

    #[test]
    fn new_write_error_creation_works() {
        let _ = ErrorWrapper::new_write_error(
            FILENAME,
            io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        );
    }

    #[test]
    fn write_error_produces_correct_message() {
        if let Err(error) = Err(WriteError {
            file_name: FILENAME.to_owned(),
            source: io::Error::new(io::ErrorKind::AddrInUse, MESSAGE),
        }) as R
        {
            assert_eq!(
                format!("{}", error),
                format!(
                    r#"could not write "{}": {}."#,
                    FILENAME,
                    io::Error::new(io::ErrorKind::AddrInUse, MESSAGE)
                )
            )
        }
    }
}
