//! Enumerations representing D-Bus constants

mod macros;
use macros::dbus_arg_iter_append;

mod client_state;
pub use client_state::{ClientState, Error as ClientStateOutOfRangeError};
mod entry_group_state;
pub use entry_group_state::{EntryGroupState, Error as EntryGroupStateOutOfRangeError};
mod interface;
pub use interface::{Error as InterfaceOutOfRangeError, Interface};
mod protocol;
pub use protocol::{Error as ProtocolOutOfRangeError, Protocol};
mod record_class;
pub use record_class::{Error as RecordClassOutOfRangeError, RecordClass};
mod record_type;
pub use record_type::{Error as RecordTypeOutOfRangeError, RecordType};
mod server_state;
pub use server_state::{Error as ServerStateOutOfRangeError, ServerState};
