#![warn(clippy::all)]

use std::time;

use super::avahi;

#[derive(Debug, Clone, Copy)]
pub struct AvahiRecord<'a> {
    interface: avahi::Interface,
    protocol: avahi::Protocol,
    flags: Flags,
    name: &'a str,
    class: avahi::RecordClass,
    r#type: avahi::RecordType,
    ttl: time::Duration,
    rdata: &'a RData,
}

type Flags = u32;
type RData = [u8];

impl<'a> AvahiRecord<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        interface: avahi::Interface, protocol: avahi::Protocol, flags: Flags, name: &'a str,
        class: avahi::RecordClass, r#type: avahi::RecordType, ttl: time::Duration,
        rdata: &'a RData,
    ) -> Self {
        Self { interface, protocol, flags, name, class, r#type, ttl, rdata }
    }

    pub fn new_cname(name: &'a str, ttl: time::Duration, rdata: &'a RData) -> Self {
        Self {
            interface: avahi::Interface::Unspecified,
            protocol: avahi::Protocol::Unspecified,
            flags: 0,
            name,
            class: avahi::RecordClass::In,
            r#type: avahi::RecordType::Cname,
            ttl,
            rdata,
        }
    }
}

impl<'a> dbus::arg::AppendAll for AvahiRecord<'a> {
    fn append(&self, ia: &mut dbus::arg::IterAppend<'_>) {
        ia.append(self.interface);
        ia.append(self.protocol);
        ia.append(self.flags);
        ia.append(self.name);
        ia.append(self.class);
        ia.append(self.r#type);
        ia.append(self.ttl.as_secs() as u32);
        ia.append(self.rdata);
    }
}

//**********************************************************************************************
// unit tests
//**********************************************************************************************

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use crate::{avahi, AvahiRecord};
    use crate::avahi_client::avahi_record;

    #[test]
    fn avahi_record_works() {
        let rdata = avahi::encode_rdata("host.local");
        let avahi_record = AvahiRecord {
            interface: avahi::Interface::Unspecified,
            protocol: avahi::Protocol::Unspecified,
            flags: 0,
            name: "alias.local",
            class: avahi::RecordClass::In,
            r#type: avahi::RecordType::Cname,
            ttl: Duration::from_secs(60),
            rdata: rdata.as_slice(),
        };
        assert_eq!(avahi_record.interface, avahi::Interface::Unspecified);
        assert_eq!(avahi_record.protocol, avahi::Protocol::Unspecified);
        assert_eq!(avahi_record.flags, 0);
        assert_eq!(avahi_record.name, "alias.local");
        assert_eq!(avahi_record.class, avahi::RecordClass::In);
        assert_eq!(avahi_record.r#type, avahi::RecordType::Cname);
        assert_eq!(avahi_record.ttl, Duration::from_secs(60));
        assert_eq!(avahi_record.rdata, rdata.as_slice());
    }

    #[test]
    fn new_function_populates_fields_correctly() {
        let rdata = avahi::encode_rdata("host.local");
        let avahi_record = AvahiRecord::new(
            avahi::Interface::Unspecified,
            avahi::Protocol::Unspecified,
            0,
            "alias.local",
            avahi::RecordClass::In,
            avahi::RecordType::Cname,
            Duration::from_secs(60),
            rdata.as_slice(),
        );
        assert_eq!(avahi_record.interface, avahi::Interface::Unspecified);
        assert_eq!(avahi_record.protocol, avahi::Protocol::Unspecified);
        assert_eq!(avahi_record.flags, 0);
        assert_eq!(avahi_record.name, "alias.local");
        assert_eq!(avahi_record.class, avahi::RecordClass::In);
        assert_eq!(avahi_record.r#type, avahi::RecordType::Cname);
        assert_eq!(avahi_record.ttl, Duration::from_secs(60));
        assert_eq!(avahi_record.rdata, rdata.as_slice());
    }
}
