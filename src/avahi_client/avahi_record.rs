#![warn(clippy::all)]

use super::{Interface, Protocol, RecordClass, RecordType};

#[derive(Debug, Clone, Copy)]
pub struct AvahiRecord<'a> {
    interface: Interface,
    protocol: Protocol,
    flags: u32,
    name: &'a str,
    class: RecordClass,
    r#type: RecordType,
    ttl: u32,
    rdata: &'a [u8],
}

impl<'a> AvahiRecord<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        interface: super::Interface, protocol: super::Protocol, flags: u32, name: &'a str,
        class: super::RecordClass, r#type: super::RecordType, ttl: u32, rdata: &'a [u8],
    ) -> Self {
        Self { interface, protocol, flags, name, class, r#type, ttl, rdata }
    }

    pub fn new_cname(name: &'a str, ttl: u32, rdata: &'a [u8]) -> Self {
        Self {
            interface: Interface::Unspecified,
            protocol: Protocol::Unspecified,
            flags: 0,
            name,
            class: RecordClass::In,
            r#type: RecordType::Cname,
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
        ia.append(self.ttl);
        ia.append(self.rdata);
    }
}
