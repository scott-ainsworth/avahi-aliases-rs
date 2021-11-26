#![warn(clippy::all)]

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(u16)]
pub enum RecordType {
    /// IPv4 Address record
    A     = 1,

    /// Name Server record
    Ns    = 2,

    /// Canonical Name record
    Cname = 5,

    /// Start of Authority record
    Soa   = 6,
    /// Pointer (reverse lookup) record
    Ptr   = 12,

    /// Host Informattion record
    Hinfo = 13,

    /// Mail Exchanger record
    Mx    = 15,

    /// Text record
    Txt   = 16,

    /// IPv6 address record
    Aaa   = 28,

    /// Service record
    Srv   = 33,
}

super::enums::dbus_arg_iter_append!(RecordType);

//**********************************************************************************************
// Unit Tests
//**********************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[(RecordType, u16)] = &[
        (RecordType::A, 1),
        (RecordType::Ns, 2),
        (RecordType::Cname, 5),
        (RecordType::Soa, 6),
        (RecordType::Ptr, 12),
        (RecordType::Hinfo, 13),
        (RecordType::Mx, 15),
        (RecordType::Txt, 16),
        (RecordType::Aaa, 28),
        (RecordType::Srv, 33),
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
            let ok = RecordType::try_from(datum.1);
            eprintln!("try_converts_valid_value_to_correct_variant :: err={:?}", ok);
            assert_eq!(datum.0, RecordType::try_from(datum.1).unwrap())
        }
    }

    #[test]
    fn try_from_returns_error_for_invalid_values() {
        for value in [0, 3, 4, 7, 8, 11, 14, 17, 27, 34] {
            let err = RecordType::try_from(value);
            eprintln!("try_from_returns_error_for_invalid_values :: err={:?}", err);
            assert!(RecordType::try_from(value).is_err());
        }
    }

    #[test]
    fn error_has_correct_message() {
        eprintln!(":: {}", RecordType::try_from(99).unwrap_err());
        assert_eq!(
            format!("{}", RecordType::try_from(99).unwrap_err()),
            "No discriminant in enum `RecordType` matches the value `99`"
        );
    }
}
