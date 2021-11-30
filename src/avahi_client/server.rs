#![warn(clippy::all)]
use std::time::Duration;

use crate::{avahi_client, ErrorWrapper};
use crate::avahi_client::avahi_dbus::{self, OrgFreedesktopAvahiServer};
use crate::avahi_client::entry_group::{self, EntryGroup};

/// A type-safe Avahi D-Bus Server client
///
/// This struct presents a type-safe adapter/façade fot the Avahi D-Bus generated code.
pub struct Server(dbus::blocking::Connection);

impl<'a> Server {
    pub fn new() -> Result<Self, ErrorWrapper> {
        Ok(Self(dbus::blocking::Connection::new_system()?))
    }

    pub fn new_entry_group(&self) -> Result<EntryGroup<'_>, ErrorWrapper> {
        Ok(entry_group::new(
            self,
            self.with_proxy(avahi_dbus::DBUS_PATH_SERVER).entry_group_new()?,
            avahi_client::DEFAULT_TTL,
        ))
    }

    pub fn get_host_name_fqdn(&self) -> Result<String, ErrorWrapper> {
        Ok(self.with_proxy(avahi_dbus::DBUS_PATH_SERVER).get_host_name_fqdn()?)
    }

    pub fn get_version(&self) -> Result<String, ErrorWrapper> {
        Ok(self.with_proxy(avahi_dbus::DBUS_PATH_SERVER).get_version_string()?)
    }

    pub(crate) fn with_proxy<P>(
        &'a self, path: P,
    ) -> dbus::blocking::Proxy<'_, &dbus::blocking::Connection>
    where
        P: Into<dbus::Path<'a>>, {
        self.0.with_proxy(avahi_dbus::DBUS_NAME, path, Duration::from_secs(5))
    }
}

//**********************************************************************************************
// unit tests
//**********************************************************************************************

#[cfg(test)]
mod tests {

    #[cfg(target_os = "linux")]
    use super::*;

    // TODO: All Linux-specific tests should be mocked

    #[test]
    #[cfg(target_os = "linux")]
    fn dbus_creation_succeeds() -> Result<(), ErrorWrapper> {
        Server::new()?;
        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn new_entry_group_succeeds() -> Result<(), ErrorWrapper> {
        use crate::avahi::EntryGroupState;

        let avahi_client = Server::new()?;
        let entry_group = avahi_client.new_entry_group()?;
        eprintln!("**** entry_group.get_state() = {:?}", entry_group.get_state()?);
        eprintln!("**** entry_group.is_empty() = {}", entry_group.is_empty()?);
        assert!(entry_group.is_empty()?);
        assert_eq!(entry_group.get_state()?, EntryGroupState::Uncommited);
        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn get_host_name_fqdn_succeeds() -> Result<(), ErrorWrapper> {
        eprintln!(
            "**** avahi_client.get_host_name_fqdn() = {}",
            Server::new()?.get_host_name_fqdn()?
        );
        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn get_proxy_succeeds() -> Result<(), ErrorWrapper> {
        Server::new()?.get_proxy();
        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn get_version_succeeds() -> Result<(), ErrorWrapper> {
        eprintln!("**** avahi_client.get_version() = {}", Server::new()?.get_version()?);
        Ok(())
    }
}
