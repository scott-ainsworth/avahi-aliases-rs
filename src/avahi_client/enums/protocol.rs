#![warn(clippy::all)]

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(i32)]
pub enum Protocol {
    /// Protocol not specified (or not needed)
    Unspecified = -1,

    /// Internet Protocol v4
    IPv4        = 0,

    /// Internet Protocol v6
    IPv6        = 1,
}

super::dbus_arg_iter_append!(Protocol);

//**********************************************************************************************
// ERROR
//**********************************************************************************************

pub type Error = num_enum::TryFromPrimitiveError<Protocol>;

//**********************************************************************************************
// UNIT TESTS
//**********************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[(Protocol, i32)] =
        &[(Protocol::Unspecified, -1), (Protocol::IPv4, 0), (Protocol::IPv6, 1)];

    #[test]
    fn constants_are_correct() {
        for datum in TEST_DATA {
            assert_eq!(datum.1, datum.0.into())
        }
    }

    #[test]
    fn try_from_converts_valid_values_to_correct_variant() {
        for datum in TEST_DATA {
            let ok = Protocol::try_from(datum.1);
            eprintln!("try_converts_valid_value_to_correct_variant :: err={:?}", ok);
            assert_eq!(datum.0, Protocol::try_from(datum.1).unwrap())
        }
    }

    #[test]
    fn try_from_returns_error_for_invalid_values() {
        for value in [-2, 2] {
            let err = Protocol::try_from(value);
            eprintln!("try_from_returns_error_for_invalid_values :: err={:?}", err);
            assert!(Protocol::try_from(value).is_err());
        }
    }

    #[test]
    fn error_has_correct_message() {
        eprintln!(":: {}", Protocol::try_from(99).unwrap_err());
        assert_eq!(
            format!("{}", Protocol::try_from(99).unwrap_err()),
            "No discriminant in enum `Protocol` matches the value `99`"
        );
    }
}
