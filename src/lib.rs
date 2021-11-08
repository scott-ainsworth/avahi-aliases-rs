//! This crate implements avahi-aliases

#![warn(clippy::all, rust_2018_idioms)]

mod aliases;
pub use aliases::AliasesFile;
mod options;
pub use options::{CommonOpts, CommandOpts, Command};

// end
