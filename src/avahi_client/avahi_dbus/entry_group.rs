#![allow(clippy::all)]
// This code was autogenerated with `dbus-codegen-rust --system-bus --client blocking`.
// See https://github.com/diwic/dbus-rs
use dbus::{arg, blocking};

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
{
    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgFreedesktopAvahiEntryGroup {
    fn free(&self) -> Result<(), dbus::Error>;
    fn commit(&self) -> Result<(), dbus::Error>;
    fn reset(&self) -> Result<(), dbus::Error>;
    fn get_state(&self) -> Result<i32, dbus::Error>;
    fn is_empty(&self) -> Result<bool, dbus::Error>;
    fn add_service(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, type_: &str,
        domain: &str, host: &str, port: u16, txt: Vec<Vec<u8>>,
    ) -> Result<(), dbus::Error>;
    fn add_service_subtype(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, type_: &str,
        domain: &str, subtype: &str,
    ) -> Result<(), dbus::Error>;
    fn update_service_txt(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, type_: &str,
        domain: &str, txt: Vec<Vec<u8>>,
    ) -> Result<(), dbus::Error>;
    fn add_address(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, address: &str,
    ) -> Result<(), dbus::Error>;
    fn add_record(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, clazz: u16, type_: u16,
        ttl: u32, rdata: Vec<u8>,
    ) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopAvahiEntryGroupStateChanged {
    pub state: i32,
    pub error: String,
}

impl arg::AppendAll for OrgFreedesktopAvahiEntryGroupStateChanged {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.state, i);
        arg::RefArg::append(&self.error, i);
    }
}

impl arg::ReadAll for OrgFreedesktopAvahiEntryGroupStateChanged {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopAvahiEntryGroupStateChanged { state: i.read()?, error: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopAvahiEntryGroupStateChanged {
    const NAME: &'static str = "StateChanged";
    const INTERFACE: &'static str = "org.freedesktop.Avahi.EntryGroup";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopAvahiEntryGroup for blocking::Proxy<'a, C>
{
    fn free(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Avahi.EntryGroup", "Free", ())
    }

    fn commit(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Avahi.EntryGroup", "Commit", ())
    }

    fn reset(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Avahi.EntryGroup", "Reset", ())
    }

    fn get_state(&self) -> Result<i32, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.EntryGroup", "GetState", ())
            .and_then(|r: (i32,)| Ok(r.0))
    }

    fn is_empty(&self) -> Result<bool, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.EntryGroup", "IsEmpty", ())
            .and_then(|r: (bool,)| Ok(r.0))
    }

    fn add_service(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, type_: &str,
        domain: &str, host: &str, port: u16, txt: Vec<Vec<u8>>,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.EntryGroup",
            "AddService",
            (interface, protocol, flags, name, type_, domain, host, port, txt),
        )
    }

    fn add_service_subtype(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, type_: &str,
        domain: &str, subtype: &str,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.EntryGroup",
            "AddServiceSubtype",
            (interface, protocol, flags, name, type_, domain, subtype),
        )
    }

    fn update_service_txt(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, type_: &str,
        domain: &str, txt: Vec<Vec<u8>>,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.EntryGroup",
            "UpdateServiceTxt",
            (interface, protocol, flags, name, type_, domain, txt),
        )
    }

    fn add_address(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, address: &str,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.EntryGroup",
            "AddAddress",
            (interface, protocol, flags, name, address),
        )
    }

    fn add_record(
        &self, interface: i32, protocol: i32, flags: u32, name: &str, clazz: u16, type_: u16,
        ttl: u32, rdata: Vec<u8>,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.EntryGroup",
            "AddRecord",
            (interface, protocol, flags, name, clazz, type_, ttl, rdata),
        )
    }
}
