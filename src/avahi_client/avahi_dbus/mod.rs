//! Generated code for Avahi DBus client interface

mod constants;
pub use constants::{DBUS_INTERFACE_ENTRY_GROUP, DBUS_NAME, DBUS_PATH_SERVER};
mod entry_group;
mod server;
pub use server::OrgFreedesktopAvahiServer;

// /// Name the D-BUS proxy type (to avoid repetition)
// pub(crate) type DBusProxy<'a> // cov(skip)
//     = dbus::blocking::Proxy<'a, &'a dbus::blocking::Connection>;
