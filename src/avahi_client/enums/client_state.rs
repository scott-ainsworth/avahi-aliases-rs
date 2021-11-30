//! Avahi Client State

#![warn(clippy::all)]

use crate::avahi_client::avahi;

/// States of a client object. A superset of ServerState.
///
/// **Ref**: Avahi source `avahi-common/client.h`
#[derive(
    Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Debug,
)]
#[repr(i32)]
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

impl PartialEq<avahi::ServerState> for ClientState {
    fn eq(&self, other: &avahi::ServerState) -> bool { *self as i32 == *other as i32 }
}

//**********************************************************************************************
// ERROR
//**********************************************************************************************

pub type Error = num_enum::TryFromPrimitiveError<ClientState>;

//**********************************************************************************************
// UNIT TESTS
//**********************************************************************************************

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &[(ClientState, i32)] = &[
        (ClientState::Registering, 1),
        (ClientState::Running, 2),
        (ClientState::Collision, 3),
        (ClientState::Failure, 100),
        (ClientState::Connecting, 101),
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
            let ok = ClientState::try_from(datum.1);
            eprintln!("try_converts_valid_value_to_correct_variant :: err={:?}", ok);
            assert_eq!(datum.0, ClientState::try_from(datum.1).unwrap())
        }
    }

    #[test]
    fn try_from_returns_error_for_invalid_values() {
        for value in [-2, 0] {
            assert!(ClientState::try_from(value).is_err());
        }
    }

    #[test]
    fn error_has_correct_message() {
        eprintln!(":: {}", ClientState::try_from(99).unwrap_err());
        assert_eq!(
            format!("{}", ClientState::try_from(99).unwrap_err()),
            "No discriminant in enum `ClientState` matches the value `99`"
        );
    }

    #[test]
    fn client_states_match_server_states() {
        assert_eq!(ClientState::Registering, avahi::ServerState::Registering);
        assert_eq!(ClientState::Running, avahi::ServerState::Running);
        assert_eq!(ClientState::Collision, avahi::ServerState::Collision);
    }
}
