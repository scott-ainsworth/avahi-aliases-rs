//! This crate implements avahi-aliases

#![warn(clippy::all, rust_2018_idioms)]

mod alias;
pub use alias::{is_valid_alias, new_alias, validate_aliases, Alias};
mod aliases_file;
pub use aliases_file::AliasesFile;
mod error;
pub use error::ErrorWrapper;
mod line;
pub use line::Line;
mod logging;
pub use logging::{init_console_logging, init_syslog_logging, LoggingError};
mod options;
pub use options::{Command, CommandOpts, DaemonOpts};

// end
