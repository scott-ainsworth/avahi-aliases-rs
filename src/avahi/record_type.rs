#![warn(clippy::all)]
#[derive(Debug, Clone, Copy)]
pub enum RecordType {
    /// IPv4 Address record
    #[allow(dead_code)]
    A     = 1,

    /// Name Server record
    #[allow(dead_code)]
    Ns    = 2,

    /// Canonical Name record
    Cname = 5,

    /// Start of Authority record
    #[allow(dead_code)]
    Soa   = 6,
    /// Pointer (reverse lookup) record
    #[allow(dead_code)]
    Ptr   = 12,

    /// Host Informattion record
    #[allow(dead_code)]
    Hinfo = 13,

    /// Mail Exchanger record
    #[allow(dead_code)]
    Mx    = 15,

    /// Text record
    #[allow(dead_code)]
    Txt   = 16,

    /// IPv6 address record
    #[allow(dead_code)]
    Aaa   = 28,

    /// Service record
    #[allow(dead_code)]
    Srv   = 33,
}

impl dbus::arg::Append for RecordType {
    #[inline(always)]
    fn append(self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(self as u16); }
    #[inline(always)]
    fn append_by_ref(&self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(*self as u16); }
}
