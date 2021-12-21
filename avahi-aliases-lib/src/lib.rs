//! Core avahi-aliases functions

mod alias;
pub use alias::{is_valid_alias, new_alias, validate_aliases, Alias};
mod aliases_file;
pub use aliases_file::AliasesFile;
pub mod avahi_dbus;
pub mod encoding;
mod line;
pub use line::Line;
mod logging;
pub use logging::{init_console_logging, init_syslog_logging};
mod options;
pub use options::{Command, CommandOpts, DaemonOpts};

// end
