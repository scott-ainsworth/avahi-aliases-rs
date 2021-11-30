//! Macros to assist with DRY

macro_rules! dbus_arg_iter_append {
    ($name:ident) => {
        /// Implement D-Bus argument append for $type
        impl dbus::arg::Append for $name {
            #[inline(always)]
            fn append(self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(self); }
            #[inline(always)]
            fn append_by_ref(&self, ia: &mut dbus::arg::IterAppend<'_>) { ia.append(*self); }
        }
    };
}

pub(super) use dbus_arg_iter_append;
