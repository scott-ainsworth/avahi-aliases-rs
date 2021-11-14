#![warn(clippy::all)]

use std::io;

use thiserror::Error;

/// ErrorWrapper enumerates all errors passed to main
#[derive(Error, Debug)]
pub enum ErrorWrapper {
    /// An invalid alias
    #[error("invalid alias: \"{alias}\".")]
    InvalidAliasError { alias: String },

    /// An I/O error retrieving the aliases file metadata
    #[error("could not get metadata for \"{file_name}\": {source}.")]
    MetadataError { file_name: String, source: io::Error },

    /// An I/O error while opeing the aliases file
    #[error("could not open \"{}\": {source}.", file_name.to_owned())]
    OpenError { file_name: String, source: io::Error },

    // /// An I/O error
    // #[error("could not publish aliases")]
    // PublishError { source: io::Error },
    /// An I/O error while reading the aliases file
    #[error("could not read \"{}\": {source}.", file_name.to_owned())]
    ReadError { file_name: String, source: io::Error },

    /// An I/O error writing to the aliases file
    #[error("could not write \"{}\": {source}.", .file_name)]
    WriteError { file_name: String, source: io::Error },
}

impl ErrorWrapper {
    /// initialize a new InvalidAliasError
    pub fn invalid_alias_error(alias: &str) -> ErrorWrapper {
        ErrorWrapper::InvalidAliasError { alias: alias.to_owned() }
    }

    /// initialize a new MetadataError
    pub fn metadata_error(file_name: &str, source: io::Error) -> ErrorWrapper {
        ErrorWrapper::MetadataError { file_name: file_name.to_owned(), source }
    }

    /// initialize a new OpenError
    pub fn open_error(file_name: &str, source: io::Error) -> ErrorWrapper {
        ErrorWrapper::OpenError { file_name: file_name.to_owned(), source }
    }

    /// initialize a new ReadError
    pub fn read_error(file_name: &str, source: io::Error) -> ErrorWrapper {
        ErrorWrapper::ReadError { file_name: file_name.to_owned(), source }
    }

    /// initialize a new WriteError
    pub fn write_error(file_name: &str, source: io::Error) -> ErrorWrapper {
        ErrorWrapper::WriteError { file_name: file_name.to_owned(), source }
    }
}
