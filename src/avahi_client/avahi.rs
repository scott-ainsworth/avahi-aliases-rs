#![warn(clippy::all)]
//! Avahi argument and return value contants


use super::ErrorWrapper;

macro_rules! implement_eq {
    ( $enum_type: ty ) => {
        impl PartialEq<i32> for $enum_type {
            fn eq(&self, other: &i32) -> bool { *self as i32 == *other }
        }
    };
}

macro_rules! implement_into {
    ( $enum_type: ty ) => {
        #[allow(clippy::from_over_into)]
        impl Into<i32> for $enum_type {
            fn into(self) -> i32 { self as i32 }
        }
    };
}

/// States of a client object. A superset of ServerState.
///
/// **Ref**: Avahi source `avahi-common/client.h`
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord, PartialEq)]
pub enum ClientState {
    /// Server state: REGISTERING.
    Registering = 1,
    /// Server state: RUNNING.
    Running     = 2,
    /// Server state: COLLISION.
    Collision   = 3,
    /// Some kind of error happened on the client side.
    Failure     = 100,
    /// We're still connecting. This state is only entered when AVAHI_CLIENT_NO_FAIL has been
    /// passed to avahi_client_new() and the daemon is not yet available.
    Connecting  = 101,
}

impl ClientState {
    // fn into(self) -> i32 { self as i32 }
    pub fn from_u32(i: i32) -> Result<Self, ErrorWrapper> {
        match i {
            1 => Ok(ClientState::Registering),
            2 => Ok(ClientState::Running),
            3 => Ok(ClientState::Collision),
            100 => Ok(ClientState::Failure),
            101 => Ok(ClientState::Connecting),
            _ => Err(ErrorWrapper::new_enum_out_of_range_error("ClientState", i)),
        }
    }
}

implement_into!(ClientState);
implement_eq!(ClientState);

impl PartialEq<ServerState> for ClientState {
    fn eq(&self, other: &ServerState) -> bool { *self as i32 == *other as i32 }
}

/// States of an entry group object
///
/// **Ref**: Avahi source `avahi-common/defs.h`
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord, PartialEq)]
pub enum EntryGroupState {
    /// The group has not yet been commited, the user must still call
    /// avahi_entry_group_commit().
    Uncommited  = 0,
    /// The entries of the group are currently being registered.
    Registering = 1,
    /// The entries have successfully been established.
    Established = 2,
    /// A name collision for one of the entries in the group has been detected,
    /// the entries have been withdrawn.
    Collision   = 3,
    /// Some kind of failure happened, the entries have been withdrawn.
    Failure     = 4,
}

impl EntryGroupState {
    pub fn from_u32(i: i32) -> Result<Self, ErrorWrapper> {
        static TABLE: [EntryGroupState; 5] = [
            EntryGroupState::Uncommited,
            EntryGroupState::Registering,
            EntryGroupState::Established,
            EntryGroupState::Collision,
            EntryGroupState::Failure,
        ];
        match i {
            i if (0..=4).contains(&i) => Ok(TABLE[i as usize]),
            _ => Err(ErrorWrapper::new_enum_out_of_range_error("EntryGroupState", i)),
        }
    }
}

implement_into!(EntryGroupState);
implement_eq!(EntryGroupState);

/// States of a server object
///
/// **Ref**: Avahi source `avahi-common/defs.h`
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerState {
    /// Invalid state (initial).
    Invalid     = 0,
    /// Host RRs are being registered.
    Registering = 1,
    /// All host RRs have been established.
    Running     = 2,
    /// There is a collision with a host RR. All host RRs have been withdrawn, the user should
    /// set a new host name via avahi_server_set_host_name().
    Collision   = 3,
    /// fatal failure happened, the server is unable to proceed.
    Failure     = 4,
}

impl ServerState {
    pub fn from_u32(i: i32) -> Result<Self, ErrorWrapper> {
        static TABLE: [ServerState; 5] = [
            ServerState::Invalid,
            ServerState::Registering,
            ServerState::Running,
            ServerState::Collision,
            ServerState::Failure,
        ];
        match i {
            i if (0..=4).contains(&i) => Ok(TABLE[i as usize]),
            _ => Err(ErrorWrapper::new_enum_out_of_range_error("ServerState", i)),
        }
    }
}

implement_into!(ServerState);
implement_eq!(ServerState);

impl PartialEq<ClientState> for ServerState {
    fn eq(&self, other: &ClientState) -> bool { *self as i32 == *other as i32 }
}

//**********************************************************************************************
// UNIT TESTS
//**********************************************************************************************

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn client_states_have_correct_values() {
        assert_eq!(ClientState::Registering, 1);
        assert_eq!(ClientState::Running, 2);
        assert_eq!(ClientState::Collision, 3);
        assert_eq!(ClientState::Failure, 100);
        assert_eq!(ClientState::Connecting, 101);
    }

    #[test]
    fn client_states_conversion_yeilds_correct_value_for_valid_u32_values() {
        assert_eq!(ClientState::from_u32(1).unwrap(), ClientState::Registering);
        assert_eq!(ClientState::from_u32(2).unwrap(), ClientState::Running);
        assert_eq!(ClientState::from_u32(3).unwrap(), ClientState::Collision);
        assert_eq!(ClientState::from_u32(100).unwrap(), ClientState::Failure);
        assert_eq!(ClientState::from_u32(101).unwrap(), ClientState::Connecting);
    }

    #[test]
    fn client_states_conversion_yeilds_error_for_invalid_u32_values() {
        assert!(ClientState::from_u32(-1).is_err());
        assert!(ClientState::from_u32(0).is_err());
        assert!(ClientState::from_u32(4).is_err());
        assert!(ClientState::from_u32(99).is_err());
        assert!(ClientState::from_u32(102).is_err());
    }

    #[test]
    fn client_states_match_server_states() {
        assert_eq!(ClientState::Registering, ServerState::Registering);
        assert_eq!(ClientState::Running, ServerState::Running);
        assert_eq!(ClientState::Collision, ServerState::Collision);
    }

    #[test]
    fn entry_group_states_have_correct_values() {
        assert_eq!(EntryGroupState::Uncommited, 0);
        assert_eq!(EntryGroupState::Registering, 1);
        assert_eq!(EntryGroupState::Established, 2);
        assert_eq!(EntryGroupState::Collision, 3);
        assert_eq!(EntryGroupState::Failure, 4);
    }

    #[test]
    fn entry_group_states_conversion_yeilds_correct_value_for_valid_u32_values() {
        assert_eq!(EntryGroupState::from_u32(0).unwrap(), EntryGroupState::Uncommited);
        assert_eq!(EntryGroupState::from_u32(1).unwrap(), EntryGroupState::Registering);
        assert_eq!(EntryGroupState::from_u32(2).unwrap(), EntryGroupState::Established);
        assert_eq!(EntryGroupState::from_u32(3).unwrap(), EntryGroupState::Collision);
        assert_eq!(EntryGroupState::from_u32(4).unwrap(), EntryGroupState::Failure);
    }

    #[test]
    fn entry_group_states_conversion_yeilds_error_for_invalid_u32_values() {
        assert!(EntryGroupState::from_u32(-1).is_err());
        assert!(EntryGroupState::from_u32(5).is_err());
    }

    #[test]
    fn server_states_have_correct_values() {
        assert_eq!(ServerState::Invalid, 0);
        assert_eq!(ServerState::Registering, 1);
        assert_eq!(ServerState::Running, 2);
        assert_eq!(ServerState::Collision, 3);
        assert_eq!(ServerState::Failure, 4);
    }

    #[test]
    fn server_states_conversion_yeilds_correct_value_for_valid_u32_values() {
        assert_eq!(ServerState::from_u32(0).unwrap(), ServerState::Invalid);
        assert_eq!(ServerState::from_u32(1).unwrap(), ServerState::Registering);
        assert_eq!(ServerState::from_u32(2).unwrap(), ServerState::Running);
        assert_eq!(ServerState::from_u32(3).unwrap(), ServerState::Collision);
        assert_eq!(ServerState::from_u32(4).unwrap(), ServerState::Failure);
    }

    #[test]
    fn server_states_conversion_yeilds_error_for_invalid_u32_values() {
        assert!(ServerState::from_u32(-1).is_err());
        assert!(ServerState::from_u32(5).is_err());
    }
}
