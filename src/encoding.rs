//! Avahi-specific encoding

#![warn(clippy::all)]

pub fn encode_rdata(name: &str) -> Vec<u8> {
    // TODO(#24) fix capacity to account for IDNA
    let mut rdata: Vec<u8> = Vec::<u8>::with_capacity(name.len() + 1);
    for part in name.split('.').filter(|p| !p.is_empty()) {
        let encoded_part = to_ascii(part);
        rdata.push(part.len() as u8);
        rdata.extend(encoded_part.as_bytes());
    }
    rdata.push(0u8);
    rdata
}

/// Convert IDNA domains to ASCII (currently a no-op/passthrough)
pub fn to_ascii(idna_name: &str) -> String { idna_name.to_owned() }

//**********************************************************************************************
// unit tests
//**********************************************************************************************

#[cfg(test)]
mod tests {

    static TEST_RDATA: &[(&str, &[u8])] = &[
        ("a.local", &[1, b'a', 5, b'l', b'o', b'c', b'a', b'l', 0]),
        ("a0.local", &[2, b'a', b'0', 5, b'l', b'o', b'c', b'a', b'l', 0]),
        ("xyzzy.local", &[5, b'x', b'y', b'z', b'z', b'y', 5, b'l', b'o', b'c', b'a', b'l', 0]),
        ("a.z.local", &[1, b'a', 1, b'z', 5, b'l', b'o', b'c', b'a', b'l', 0]),
        ("a..local", &[1, b'a', 5, b'l', b'o', b'c', b'a', b'l', 0]),
    ];

    static TEST_IDNA: &[(&str, &str)] = &[
        ("a.local", "a.local"),
        ("a0.local", "a0.local"),
        ("xyzzy.local", "xyzzy.local"),
        ("a.z.local", "a.z.local"),
        ("a..local", "a..local"),
    ];

    #[test]
    fn encore_rdata_encodes_correctly() {
        for (name, encoded) in TEST_RDATA {
            assert_eq!(super::encode_rdata(name).as_slice(), *encoded);
        }
    }

    #[test]
    fn to_ascii_encodes_correctly() {
        for (name, encoded) in TEST_IDNA {
            assert_eq!(super::to_ascii(name), *encoded);
        }
    }
}
