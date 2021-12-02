//! Constants used by Avahi clients

//**********************************************************************************************
// D-Bus Names and Paths
//**********************************************************************************************

/// D-Bus name of the Avahi service
pub const AVAHI_DBUS_NAME: &str = "org.freedesktop.Avahi";

/// D-Bus Avahi service path
pub const AVAHI_DBUS_PATH_SERVER: &str = "/";

/// D- Bus name of the Avahi Entry Group service
pub const AVAHI_DBUS_INTERFACE_ENTRY_GROUP: &str = "org.freedesktop.Avahi.EntryGroup";

//**********************************************************************************************
// Client State
//**********************************************************************************************

#[repr(i32)]
pub enum ClientState {
    /// Server state: REGISTERING.
    REGISTERING = 1,

    /// Server state: RUNNING.
    RUNNING     = 2,

    /// Server state: COLLISION.
    COLLISION   = 3,

    /// Some kind of error happened on the client side.
    FAILURE     = 100,

    /// We're still connecting. This state is only entered when AVAHI_CLIENT_NO_FAIL has been
    /// passed to avahi_client_new() and the daemon is not yet available.
    CONNECTING  = 101,
}

//**********************************************************************************************
// Entry Group
//**********************************************************************************************

#[repr(i32)]
pub enum EntryGroupState {
    /// The group has not yet been commited, the user must still call
    /// avahi_entry_group_commit().
    UNCOMMITTED = 0,

    /// The entries of the group are currently being registered.
    REGISTERING = 1,

    /// The entries have successfully been established.
    ESTABLISHED = 2,

    /// A name collision for one of the entries in the group has been detected,
    /// the entries have been withdrawn.
    COLLISION   = 3,

    /// Some kind of failure happened, the entries have been withdrawn.
    FAILURE     = 4,
}

//**********************************************************************************************
// Interface
//**********************************************************************************************

/// Special values for AvahiIfIndex
///
/// Reference: Avahi source: `avahi-common/address.h`
#[repr(i32)]
pub enum Interface {
    /// Unspecified/all interface(s)
    UNSPECIFIED = -1,
}

//**********************************************************************************************
// Protocol
//**********************************************************************************************

#[derive(Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum Protocol {
    /// Unspecified/all protocol(s)
    UNSPEC = -1,

    /// IPv4
    INET   = 0,

    /// IPv6
    INET6  = 1,
}

//**********************************************************************************************
// Record Class
//**********************************************************************************************

#[repr(u16)]
pub enum RecordClass {
    /// Internet record class
    IN = 1,
}

//**********************************************************************************************
// Record Type
//**********************************************************************************************

#[repr(u16)]
pub enum RecordType {
    /// IPv4 Address record
    A     = 1,

    /// Name Server record
    NS    = 2,

    /// Canonical Name record
    CNAME = 5,

    /// Start of Authority record
    SOA   = 6,

    /// Pointer (reverse lookup) record
    PTR   = 12,

    /// Host Informattion record
    HINFO = 13,

    /// Mail Exchanger record
    MX    = 15,

    /// Text record
    TXT   = 16,

    /// IPv6 address record
    AAA   = 28,

    /// Service record
    SRV   = 33,
}

//**********************************************************************************************
// Server State
//**********************************************************************************************

#[repr(i32)]
pub enum ServerState {
    /// Invalid state (initial).
    INVALID     = 0,
    /// Host RRs are being registered.
    REGISTERING = 1,
    /// All host RRs have been established.
    RUNNING     = 2,
    /// There is a collision with a host RR. All host RRs have been withdrawn, the user should
    /// set a new host name via avahi_server_set_host_name().
    COLLISION   = 3,
    /// Fatal failure occured, the server is unable to proceed.
    FAILURE     = 4,
}

//**********************************************************************************************
// UNIT TESTS
//**********************************************************************************************

#[cfg(test)]
mod tests {

    use crate::avahi_dbus::constants::*;

    #[test]
    fn dbus_constants_are_correct() {
        assert_eq!(AVAHI_DBUS_NAME, "org.freedesktop.Avahi");
        assert_eq!(AVAHI_DBUS_PATH_SERVER, "/");
        assert_eq!(AVAHI_DBUS_INTERFACE_ENTRY_GROUP, "org.freedesktop.Avahi.EntryGroup");
    }

    #[test]
    fn client_state_constants_are_correct() {
        assert_eq!(ClientState::REGISTERING as i32, 1);
        assert_eq!(ClientState::RUNNING as i32, 2);
        assert_eq!(ClientState::COLLISION as i32, 3);
        assert_eq!(ClientState::FAILURE as i32, 100);
        assert_eq!(ClientState::CONNECTING as i32, 101);
    }

    #[test]
    fn entry_group_state_constants_are_correct() {
        assert_eq!(EntryGroupState::UNCOMMITTED as i32, 0);
        assert_eq!(EntryGroupState::REGISTERING as i32, 1);
        assert_eq!(EntryGroupState::ESTABLISHED as i32, 2);
        assert_eq!(EntryGroupState::COLLISION as i32, 3);
        assert_eq!(EntryGroupState::FAILURE as i32, 4);
    }

    #[test]
    fn interface_constants_are_correct() { assert_eq!(Interface::UNSPECIFIED as i32, -1) }

    #[test]
    fn protocol_constants_are_correct() {
        assert_eq!(Protocol::UNSPEC as i32, -1);
        assert_eq!(Protocol::INET as i32, 0);
        assert_eq!(Protocol::INET6 as i32, 1)
    }

    #[test]
    fn record_class_constants_are_correct() { assert_eq!(RecordClass::IN as u16, 1) }

    #[test]
    fn record_type_constants_are_correct() {
        assert_eq!(RecordType::A as u16, 1);
        assert_eq!(RecordType::NS as u16, 2);
        assert_eq!(RecordType::CNAME as u16, 5);
        assert_eq!(RecordType::SOA as u16, 6);
        assert_eq!(RecordType::PTR as u16, 12);
        assert_eq!(RecordType::HINFO as u16, 13);
        assert_eq!(RecordType::MX as u16, 15);
        assert_eq!(RecordType::TXT as u16, 16);
        assert_eq!(RecordType::AAA as u16, 28);
        assert_eq!(RecordType::SRV as u16, 33);
    }

    #[test]
    fn server_state_constants_are_correct() {
        assert_eq!(ServerState::INVALID as i32, 0);
        assert_eq!(ServerState::REGISTERING as i32, 1);
        assert_eq!(ServerState::RUNNING as i32, 2);
        assert_eq!(ServerState::COLLISION as i32, 3);
        assert_eq!(ServerState::FAILURE as i32, 4);
    }
}
