//! Generated code and symbolic constants for Avahi DBus client interface

pub mod constants;
mod entry_group;
mod server;
pub use constants as avahi;
pub use entry_group::OrgFreedesktopAvahiEntryGroup;
pub use server::OrgFreedesktopAvahiServer;

// names for types created by dbus-codegen

/// Name the D-BUS proxy type (to avoid repetition)
pub type DBusProxy<'p, 'c> = dbus::blocking::Proxy<'p, &'c dbus::blocking::Connection>;
