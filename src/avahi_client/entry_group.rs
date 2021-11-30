#![warn(clippy::all)]

use std::time::Duration;

use super::{avahi, AvahiRecord, ErrorWrapper, Server};
use super::avahi_dbus::DBUS_INTERFACE_ENTRY_GROUP;

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
        let value = avahi::EntryGroupState::try_from(result.0)?;
        Ok(value)
    }

    pub fn is_empty(&self) -> Result<bool, ErrorWrapper> {
        let result: (bool,) = self.0.method_call(DBUS_INTERFACE_ENTRY_GROUP, "IsEmpty", ())?;
        Ok(result.0)
    }
}

/// Initialize a new `EntryGroup`.
pub fn new<'a>(server: &'a Server, path: dbus::Path<'a>, _ttl: Duration) -> EntryGroup<'a> {
    EntryGroup(server.with_proxy(path))
}
