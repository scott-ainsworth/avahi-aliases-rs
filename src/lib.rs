//! This crate implements avahi-aliases

#![warn(clippy::all, rust_2018_idioms)]

mod aliases_file;
pub use aliases_file::AliasesFile;
mod line;
pub use line::Line;
mod options;
pub use options::{Command, CommandOpts, CommonOpts, DaemonOpts};

// end
