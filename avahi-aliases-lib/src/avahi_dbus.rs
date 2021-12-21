//! Generated code and symbolic constants for Avahi DBus client interface

pub mod constants;
pub mod entry_group;
pub mod server;
pub use constants as avahi;

/// Name the D-BUS proxy type (to avoid repetition)
pub type DBusProxy<'p, 'c> = dbus::blocking::Proxy<'p, &'c dbus::blocking::Connection>;
