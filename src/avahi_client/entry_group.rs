#![warn(clippy::all)]

use std::time::Duration;

use crate::avahi_client::ErrorWrapper;
use super::{avahi, AvahiClient, AvahiRecord};
use super::dbus_constants::*;

/// An Avahi entry group.
pub struct EntryGroup<'a>(dbus::blocking::Proxy<'a, &'a dbus::blocking::Connection>);

impl<'a> EntryGroup<'a> {
    pub fn add_record(&self, record: AvahiRecord<'_>) -> Result<(), ErrorWrapper> {
        let result = self.0.method_call(DBUS_INTERFACE_ENTRY_GROUP, "AddRecord", record)?;
        Ok(result)
    }

    pub fn commit(&self) -> Result<(), ErrorWrapper> {
        let result = self.0.method_call(DBUS_INTERFACE_ENTRY_GROUP, "Commit", ())?;
        Ok(result)
    }

    pub fn get_state(&self) -> Result<avahi::EntryGroupState, ErrorWrapper> {
        let result: (i32,) = self.0.method_call(DBUS_INTERFACE_ENTRY_GROUP, "GetState", ())?;
        let value = avahi::EntryGroupState::from_u32(result.0)?;
        Ok(value)
    }

    pub fn is_empty(&self) -> Result<bool, ErrorWrapper> {
        let result: (bool,) = self.0.method_call(DBUS_INTERFACE_ENTRY_GROUP, "IsEmpty", ())?;
        Ok(result.0)
    }
}

/// Initialize a new `EntryGroup`.
pub fn new<'a>(
    avahi_client: &'a AvahiClient, path: dbus::Path<'a>, ttl: Duration,
) -> EntryGroup<'a> {
    EntryGroup(avahi_client.0.with_proxy(DBUS_NAME, path, ttl))
}
