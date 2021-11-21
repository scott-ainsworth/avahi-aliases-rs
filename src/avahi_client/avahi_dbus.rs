#![allow(clippy::all)]
// This code was autogenerated with `dbus-codegen-rust --system-bus --client blocking
// --destination org.freedesktop.Avahi`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

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

pub trait OrgFreedesktopAvahiServer {
    fn get_version_string(&self) -> Result<String, dbus::Error>;
    fn get_apiversion(&self) -> Result<u32, dbus::Error>;
    fn get_host_name(&self) -> Result<String, dbus::Error>;
    fn set_host_name(&self, name: &str) -> Result<(), dbus::Error>;
    fn get_host_name_fqdn(&self) -> Result<String, dbus::Error>;
    fn get_domain_name(&self) -> Result<String, dbus::Error>;
    fn is_nsssupport_available(&self) -> Result<bool, dbus::Error>;
    fn get_state(&self) -> Result<i32, dbus::Error>;
    fn get_local_service_cookie(&self) -> Result<u32, dbus::Error>;
    fn get_alternative_host_name(&self, name: &str) -> Result<String, dbus::Error>;
    fn get_alternative_service_name(&self, name: &str) -> Result<String, dbus::Error>;
    fn get_network_interface_name_by_index(&self, index: i32) -> Result<String, dbus::Error>;
    fn get_network_interface_index_by_name(&self, name: &str) -> Result<i32, dbus::Error>;
    fn resolve_host_name(
        &self, interface: i32, protocol: i32, name: &str, aprotocol: i32, flags: u32,
    ) -> Result<(i32, i32, String, i32, String, u32), dbus::Error>;
    fn resolve_address(
        &self, interface: i32, protocol: i32, address: &str, flags: u32,
    ) -> Result<(i32, i32, i32, String, String, u32), dbus::Error>;
    fn resolve_service(
        &self, interface: i32, protocol: i32, name: &str, type_: &str, domain: &str,
        aprotocol: i32, flags: u32,
    ) -> Result<
        (i32, i32, String, String, String, String, i32, String, u16, Vec<Vec<u8>>, u32),
        dbus::Error,
    >;
    fn entry_group_new(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn domain_browser_new(
        &self, interface: i32, protocol: i32, domain: &str, btype: i32, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn service_type_browser_new(
        &self, interface: i32, protocol: i32, domain: &str, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn service_browser_new(
        &self, interface: i32, protocol: i32, type_: &str, domain: &str, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn service_resolver_new(
        &self, interface: i32, protocol: i32, name: &str, type_: &str, domain: &str,
        aprotocol: i32, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn host_name_resolver_new(
        &self, interface: i32, protocol: i32, name: &str, aprotocol: i32, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn address_resolver_new(
        &self, interface: i32, protocol: i32, address: &str, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn record_browser_new(
        &self, interface: i32, protocol: i32, name: &str, clazz: u16, type_: u16, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopAvahiServerStateChanged {
    pub state: i32,
    pub error: String,
}

impl arg::AppendAll for OrgFreedesktopAvahiServerStateChanged {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.state, i);
        arg::RefArg::append(&self.error, i);
    }
}

impl arg::ReadAll for OrgFreedesktopAvahiServerStateChanged {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopAvahiServerStateChanged { state: i.read()?, error: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopAvahiServerStateChanged {
    const NAME: &'static str = "StateChanged";
    const INTERFACE: &'static str = "org.freedesktop.Avahi.Server";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopAvahiServer for blocking::Proxy<'a, C>
{
    fn get_version_string(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetVersionString", ())
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn get_apiversion(&self) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetAPIVersion", ())
            .and_then(|r: (u32,)| Ok(r.0))
    }

    fn get_host_name(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetHostName", ())
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn set_host_name(&self, name: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "SetHostName", (name,))
    }

    fn get_host_name_fqdn(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetHostNameFqdn", ())
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn get_domain_name(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetDomainName", ())
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn is_nsssupport_available(&self) -> Result<bool, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "IsNSSSupportAvailable", ())
            .and_then(|r: (bool,)| Ok(r.0))
    }

    fn get_state(&self) -> Result<i32, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetState", ())
            .and_then(|r: (i32,)| Ok(r.0))
    }

    fn get_local_service_cookie(&self) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetLocalServiceCookie", ())
            .and_then(|r: (u32,)| Ok(r.0))
    }

    fn get_alternative_host_name(&self, name: &str) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetAlternativeHostName", (name,))
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn get_alternative_service_name(&self, name: &str) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "GetAlternativeServiceName", (name,))
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn get_network_interface_name_by_index(&self, index: i32) -> Result<String, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "GetNetworkInterfaceNameByIndex",
            (index,),
        )
        .and_then(|r: (String,)| Ok(r.0))
    }

    fn get_network_interface_index_by_name(&self, name: &str) -> Result<i32, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "GetNetworkInterfaceIndexByName",
            (name,),
        )
        .and_then(|r: (i32,)| Ok(r.0))
    }

    fn resolve_host_name(
        &self, interface: i32, protocol: i32, name: &str, aprotocol: i32, flags: u32,
    ) -> Result<(i32, i32, String, i32, String, u32), dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "ResolveHostName",
            (interface, protocol, name, aprotocol, flags),
        )
    }

    fn resolve_address(
        &self, interface: i32, protocol: i32, address: &str, flags: u32,
    ) -> Result<(i32, i32, i32, String, String, u32), dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "ResolveAddress",
            (interface, protocol, address, flags),
        )
    }

    fn resolve_service(
        &self, interface: i32, protocol: i32, name: &str, type_: &str, domain: &str,
        aprotocol: i32, flags: u32,
    ) -> Result<
        (i32, i32, String, String, String, String, i32, String, u16, Vec<Vec<u8>>, u32),
        dbus::Error,
    > {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "ResolveService",
            (interface, protocol, name, type_, domain, aprotocol, flags),
        )
    }

    fn entry_group_new(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.Avahi.Server", "EntryGroupNew", ())
            .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn domain_browser_new(
        &self, interface: i32, protocol: i32, domain: &str, btype: i32, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "DomainBrowserNew",
            (interface, protocol, domain, btype, flags),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn service_type_browser_new(
        &self, interface: i32, protocol: i32, domain: &str, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "ServiceTypeBrowserNew",
            (interface, protocol, domain, flags),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn service_browser_new(
        &self, interface: i32, protocol: i32, type_: &str, domain: &str, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "ServiceBrowserNew",
            (interface, protocol, type_, domain, flags),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn service_resolver_new(
        &self, interface: i32, protocol: i32, name: &str, type_: &str, domain: &str,
        aprotocol: i32, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "ServiceResolverNew",
            (interface, protocol, name, type_, domain, aprotocol, flags),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn host_name_resolver_new(
        &self, interface: i32, protocol: i32, name: &str, aprotocol: i32, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "HostNameResolverNew",
            (interface, protocol, name, aprotocol, flags),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn address_resolver_new(
        &self, interface: i32, protocol: i32, address: &str, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "AddressResolverNew",
            (interface, protocol, address, flags),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn record_browser_new(
        &self, interface: i32, protocol: i32, name: &str, clazz: u16, type_: u16, flags: u32,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.Avahi.Server",
            "RecordBrowserNew",
            (interface, protocol, name, clazz, type_, flags),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }
}
