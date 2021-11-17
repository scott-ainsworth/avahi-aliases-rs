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
