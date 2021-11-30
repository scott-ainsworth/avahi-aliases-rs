//! Avahi argument and return value contants

#![warn(clippy::all)]

use crate::avahi_client::avahi;

/// States of a server object
///
/// **Ref**: Avahi source `avahi-common/defs.h`
#[derive(
    Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Debug,
)]
#[repr(i32)]
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

impl PartialEq<avahi::ClientState> for ServerState {
    fn eq(&self, other: &avahi::ClientState) -> bool { *self as i32 == *other as i32 }
}

//**********************************************************************************************
// ERROR
//**********************************************************************************************

pub type Error = num_enum::TryFromPrimitiveError<ServerState>;

//**********************************************************************************************
// UNIT TESTS
//**********************************************************************************************

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &[(ServerState, i32)] = &[
        (ServerState::Invalid, 0),
        (ServerState::Registering, 1),
        (ServerState::Running, 2),
        (ServerState::Collision, 3),
        (ServerState::Failure, 4),
    ];

    #[test]
    fn constants_are_correct() {
        for datum in TEST_DATA {
            assert_eq!(datum.1, datum.0.into())
        }
    }


    #[test]
    fn try_from_converts_valid_values_to_correct_variant() {
        for datum in TEST_DATA {
            assert_eq!(datum.0, ServerState::try_from(datum.1).unwrap())
        }
    }

    #[test]
    #[test]
    fn try_from_returns_error_for_invalid_values() {
        for value in [-1, 5] {
            let err = ServerState::try_from(value);
            eprintln!("try_from_returns_error_for_invalid_values :: err={:?}", err);
            assert!(ServerState::try_from(value).is_err());
        }
    }

    #[test]
    fn error_has_correct_message() {
        eprintln!(":: {}", ServerState::try_from(99).unwrap_err());
        assert_eq!(
            format!("{}", ServerState::try_from(99).unwrap_err()),
            "No discriminant in enum `ServerState` matches the value `99`"
        );
    }

    #[test]
    fn client_states_match_server_states() {
        assert_eq!(ServerState::Registering, avahi::ClientState::Registering);
        assert_eq!(ServerState::Running, avahi::ClientState::Running);
        assert_eq!(ServerState::Collision, avahi::ClientState::Collision);
    }
}
