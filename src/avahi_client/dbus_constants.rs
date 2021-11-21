#![warn(clippy::all)]
//! Avahi D-Bus contants

/// D-Bus name of the Avahi service
pub const DBUS_NAME: &str = "org.freedesktop.Avahi";

/// D-Bus Avahi service path
pub const DBUS_PATH_SERVER: &str = "/";

/// D-Bus name of the Avahi Entry Group service
pub const DBUS_INTERFACE_ENTRY_GROUP: &str = "org.freedesktop.Avahi.EntryGroup";

//**********************************************************************************************
// UNIT TESTS
//**********************************************************************************************

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn dbus_constants_are_correct() {
        assert_eq!(DBUS_NAME, "org.freedesktop.Avahi");
        assert_eq!(DBUS_PATH_SERVER, "/");
        assert_eq!(DBUS_INTERFACE_ENTRY_GROUP, "org.freedesktop.Avahi.EntryGroup");
    }
}
