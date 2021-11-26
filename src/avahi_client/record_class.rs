#![warn(clippy::all)]

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(u16)]
pub enum RecordClass {
    In = 1,
}

super::enums::dbus_arg_iter_append!(RecordClass);

//**********************************************************************************************
// Unit Tests
//**********************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[(RecordClass, u16)] = &[(RecordClass::In, 1)];

    #[test]
    fn constants_are_correct() {
        for datum in TEST_DATA {
            assert_eq!(datum.1, datum.0.into())
        }
    }

    #[test]
    fn try_from_converts_valid_values_to_correct_variant() {
        for datum in TEST_DATA {
            let ok = RecordClass::try_from(datum.1);
            eprintln!("try_converts_valid_value_to_correct_variant :: err={:?}", ok);
            assert_eq!(datum.0, RecordClass::try_from(datum.1).unwrap())
        }
    }

    #[test]
    fn try_from_returns_error_for_invalid_values() {
        for value in [0, 2] {
            let err = RecordClass::try_from(value);
            eprintln!("try_from_returns_error_for_invalid_values :: err={:?}", err);
            assert!(RecordClass::try_from(value).is_err());
        }
    }

    #[test]
    fn error_has_correct_message() {
        eprintln!(":: {}", RecordClass::try_from(99).unwrap_err());
        assert_eq!(
            format!("{}", RecordClass::try_from(99).unwrap_err()),
            "No discriminant in enum `RecordClass` matches the value `99`"
        );
    }
}
