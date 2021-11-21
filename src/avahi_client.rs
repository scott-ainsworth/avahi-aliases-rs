#![warn(clippy::all)]
//! An interface to Avahi via D-Bus
use std::time::Duration;

use crate::ErrorWrapper;

pub mod avahi;
mod avahi_dbus;
mod dbus_constants;
use dbus_constants::*;
pub use avahi_dbus::OrgFreedesktopAvahiServer;
mod entry_group;
pub use entry_group::EntryGroup;
mod avahi_record;
pub use avahi_record::AvahiRecord;
mod interface;
pub use interface::Interface;
mod protocol;
pub use protocol::Protocol;
mod record_class;
pub use record_class::RecordClass;
mod record_type;
pub use record_type::RecordType;

const DEFAULT_TTL: Duration = Duration::from_secs(60);

/// An Avahi D-Bus client
pub struct AvahiClient(dbus::blocking::Connection);

impl<'a> AvahiClient {
    pub fn new() -> Result<Self, ErrorWrapper> {
        Ok(Self(dbus::blocking::Connection::new_system()?))
    }

    pub fn encode_rdata(name: &str) -> Vec<u8> {
        // TODO: convert encode_rdata to functional style
        let mut rdata: Vec<u8> = Vec::<u8>::new();
        for part in name.split('.').filter(|p| !p.is_empty()) {
            rdata.extend([part.len().to_be_bytes().last().unwrap()]);
            rdata.extend(to_ascii(part).as_bytes());
        }
        rdata.extend(&[0u8]);
        rdata
    }

    pub fn encode_name(name: &str) -> String {
        name.split('.')
            .filter(|p| !p.is_empty())
            .map(to_ascii)
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn new_entry_group(&self) -> Result<EntryGroup<'_>, ErrorWrapper> {
        Ok(entry_group::new(self, self.get_proxy().entry_group_new()?, DEFAULT_TTL))
    }

    pub fn get_host_name_fqdn(&self) -> Result<String, ErrorWrapper> {
        Ok(self.get_proxy().get_host_name_fqdn()?)
    }

    pub fn get_version(&self) -> Result<String, ErrorWrapper> {
        Ok(self.get_proxy().get_version_string()?)
    }

    fn get_proxy(&'a self) -> dbus::blocking::Proxy<'_, &dbus::blocking::Connection> {
        self.0.with_proxy(DBUS_NAME, DBUS_PATH_SERVER, Duration::from_secs(5))
    }
}

/// Convert IDNA domains to ASCII (currently a no-op/passthrough)
fn to_ascii(idna_name: &str) -> String { idna_name.to_owned() }

#[cfg(test)]
mod test {

    use super::*;

    static TEST_DATA: &[(&str, &[u8])] = &[
        ("a0.local", &[2, b'a', b'0', 5, b'l', b'o', b'c', b'a', b'l', 0]),
        ("xyzzy.local", &[5, b'x', b'y', b'z', b'z', b'y', 5, b'l', b'o', b'c', b'a', b'l', 0]),
    ];

    #[test]
    fn dbus_constants_are_correct() {
        assert_eq!(DBUS_NAME, "org.freedesktop.Avahi");
        assert_eq!(DBUS_PATH_SERVER, "/");
        assert_eq!(DBUS_INTERFACE_ENTRY_GROUP, "org.freedesktop.Avahi.EntryGroup");
        assert_eq!(Interface::Unspecified as i32, -1);
        assert_eq!(RecordClass::In as u32, 0x01);
        assert_eq!(RecordType::Cname as u32, 0x05);
        assert_eq!(Protocol::Unspecified as i32, -1);
        assert_eq!(DEFAULT_TTL, Duration::from_secs(60));
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn dbus_creation_succeeds() -> Result<(), ErrorWrapper> {
        let avahi_client = AvahiClient::new()?;
        eprintln!("**** avahi_client.get_version() = {}", avahi_client.get_version()?);
        eprintln!(
            "**** avahi_client.get_host_name_fqdn() = {}",
            avahi_client.get_host_name_fqdn()?
        );
        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn new_entry_group_succeeds() -> Result<(), ErrorWrapper> {
        use crate::avahi::EntryGroupState;

        let avahi_client = AvahiClient::new()?;
        let entry_group = avahi_client.new_entry_group()?;
        eprintln!("**** entry_group.get_state() = {:?}", entry_group.get_state()?);
        eprintln!("**** entry_group.is_empty() = {}", entry_group.is_empty()?);
        assert!(entry_group.is_empty()?);
        assert_eq!(entry_group.get_state()?, EntryGroupState::Uncommited);
        Ok(())
    }

    #[test]
    fn resource_records_are_created_correctly() {
        for (alias, resource_record) in TEST_DATA {
            assert_eq!(*resource_record, AvahiClient::encode_rdata(alias).as_slice());
        }
    }
}
