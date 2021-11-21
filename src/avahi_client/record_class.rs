#![warn(clippy::all)]

#[derive(Debug, Clone, Copy)]
pub enum RecordClass {
    In = 1,
}

impl dbus::arg::Append for RecordClass {
    #[inline(always)]
    fn append(self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(self as u16); }
    #[inline(always)]
    fn append_by_ref(&self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(*self as u16); }
}
