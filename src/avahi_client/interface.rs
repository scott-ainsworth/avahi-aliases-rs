#![warn(clippy::all)]
#[derive(Debug, Clone, Copy)]
pub enum Interface {
    Unspecified = -1,
}

impl dbus::arg::Append for Interface {
    #[inline(always)]
    fn append(self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(self as i32); }
    #[inline(always)]
    fn append_by_ref(&self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(*self as i32); }
}
