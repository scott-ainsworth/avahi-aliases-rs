//! This crate implements avahi-aliases

#![warn(clippy::all, rust_2018_idioms)]

use std::time;

mod alias;
pub use alias::{is_valid_alias, new_alias, validate_aliases, Alias};
mod aliases_file;
pub use aliases_file::AliasesFile;
pub mod avahi_dbus;
pub mod encoding;
mod error;
pub use error::ErrorWrapper;
mod line;
pub use line::Line;
mod logging;
pub use logging::{init_console_logging, init_syslog_logging};
mod options;
pub use options::{Command, CommandOpts, DaemonOpts};

/// Default server timeout
pub const DEFAULT_TIMEOUT: time::Duration = time::Duration::from_secs(60);

// end
