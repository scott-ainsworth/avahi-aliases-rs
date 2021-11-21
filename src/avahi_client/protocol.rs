#![warn(clippy::all)]

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    /// Protocol not specified (or not needed)
    Unspecified = -1,

    /// Internet Protocol v4
    #[allow(dead_code)]
    IPv4        = 0,

    /// Internet Protocol v6
    #[allow(dead_code)]
    IPv6        = 1,
}

impl dbus::arg::Append for Protocol {
    #[inline(always)]
    fn append(self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(self as i32); }
    #[inline(always)]
    fn append_by_ref(&self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(*self as i32); }
}
