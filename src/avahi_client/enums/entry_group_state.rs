#![warn(clippy::all)]

/// States of an entry group object
///
/// **Ref**: Avahi source `avahi-common/defs.h`
#[derive(
    Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Debug,
)]
#[repr(i32)]
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

//**********************************************************************************************
// ERROR
//**********************************************************************************************

pub type Error = num_enum::TryFromPrimitiveError<EntryGroupState>;

//**********************************************************************************************
// UNIT TESTS
//**********************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[(EntryGroupState, i32)] = &[
        (EntryGroupState::Uncommited, 0),
        (EntryGroupState::Registering, 1),
        (EntryGroupState::Established, 2),
        (EntryGroupState::Collision, 3),
        (EntryGroupState::Failure, 4),
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
            assert_eq!(datum.0, EntryGroupState::try_from(datum.1).unwrap())
        }
    }

    #[test]
    fn try_from_returns_error_for_invalid_values() {
        for value in [-1, 5] {
            let err = EntryGroupState::try_from(value);
            eprintln!("try_from_returns_error_for_invalid_values :: err={:?}", err);
            assert!(EntryGroupState::try_from(value).is_err());
        }
    }

    #[test]
    fn error_has_correct_message() {
        eprintln!(":: {}", EntryGroupState::try_from(99).unwrap_err());
        assert_eq!(
            format!("{}", EntryGroupState::try_from(99).unwrap_err()),
            "No discriminant in enum `EntryGroupState` matches the value `99`"
        );
    }
}
