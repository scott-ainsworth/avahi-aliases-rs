#![warn(clippy::all)]

#[derive(
    Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Debug,
)]
#[repr(i32)]
pub enum Interface {
    Unspecified = -1,
}

impl dbus::arg::Append for Interface {
    #[inline(always)]
    fn append(self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(self as i32); }
    #[inline(always)]
    fn append_by_ref(&self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(*self as i32); }
}
//**********************************************************************************************
// Unit Tests
//**********************************************************************************************

#[cfg(test)]
mod test {
    use super::*;

    static TEST_DATA: &[(Interface, i32)] = &[(Interface::Unspecified, -1)];

    #[test]
    fn constants_are_correct() {
        for datum in TEST_DATA {
            assert_eq!(datum.1, datum.0.into())
        }
    }

    #[test]
    fn try_from_converts_valid_values_to_correct_variant() {
        for datum in TEST_DATA {
            let ok = Interface::try_from(datum.1);
            eprintln!("try_converts_valid_value_to_correct_variant :: err={:?}", ok);
            assert_eq!(datum.0, Interface::try_from(datum.1).unwrap())
        }
    }

    #[test]
    fn try_from_returns_error_for_invalid_values() {
        for value in [-2, 0] {
            let err = Interface::try_from(value);
            eprintln!("try_from_returns_error_for_invalid_values :: err={:?}", err);
            assert!(Interface::try_from(value).is_err());
        }
    }

    #[test]
    fn error_has_correct_message() {
        eprintln!(":: {}", Interface::try_from(99).unwrap_err());
        assert_eq!(
            format!("{}", Interface::try_from(99).unwrap_err()),
            "No discriminant in enum `Interface` matches the value `99`"
        );
    }
}
