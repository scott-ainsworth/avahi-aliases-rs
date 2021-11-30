//! Avahi D-Bus names (constants)

/// D-Bus name of the Avahi service
pub const DBUS_NAME: &str = "org.freedesktop.Avahi"; // cov(skip)

/// D-Bus Avahi service path
pub const DBUS_PATH_SERVER: &str = "/"; // cov(skip)

/// D- Bus name of the Avahi Entry Group service
pub const DBUS_INTERFACE_ENTRY_GROUP: &str = "org.freedesktop.Avahi.EntryGroup";

mod tests {

    #[test]
    fn dbus_constants_are_correct() {
        assert_eq!(super::DBUS_NAME, "org.freedesktop.Avahi");
        assert_eq!(super::DBUS_PATH_SERVER, "/");
        assert_eq!(super::DBUS_INTERFACE_ENTRY_GROUP, "org.freedesktop.Avahi.EntryGroup");
    }
}
