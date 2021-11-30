//! A type-safe client interface to Avahi D-Bus services
//!
//! The `avahi_client` module implements a type-safe adapter/facade to the code generate
//! by `[dbus-codegen](https://github.com/diwic/dbus-rs/tree/HEAD/dbus-codegen)`.

#![warn(clippy::all)]
use std::time::Duration;

use crate::ErrorWrapper;

pub mod avahi_dbus;
mod avahi_record;
mod encoding;
mod entry_group;
mod enums;
mod server;
pub use avahi_record::AvahiRecord;
pub use entry_group::EntryGroup;
pub use server::Server;

/// Avahi constants and utilities
pub mod avahi {
    pub use super::enums::{
        ClientState, ClientStateOutOfRangeError, EntryGroupState,
        EntryGroupStateOutOfRangeError, Interface, InterfaceOutOfRangeError, Protocol,
        ProtocolOutOfRangeError, RecordClass, RecordClassOutOfRangeError, RecordType,
        RecordTypeOutOfRangeError, ServerState, ServerStateOutOfRangeError,
    };
    pub use super::encoding::encode_rdata;
}

/// IDNA utilities
pub mod idna {
    pub use super::encoding::to_ascii;
}

const DEFAULT_TTL: Duration = Duration::from_secs(60);

// //*******************************************************************************************
// *** // unit tests
// //*******************************************************************************************
// ***

// #[cfg(test)]
// mod tests {

//     #[cfg(target_os = "linux")]
//     use super::*;

//     // TODO: All Linux-specific tests should be mocked

//     #[test]
//     #[cfg(target_os = "linux")]
//     fn dbus_creation_succeeds() -> Result<(), ErrorWrapper> {
//         Server::new()?;
//         Ok(())
//     }

//     #[test]
//     #[cfg(target_os = "linux")]
//     fn new_entry_group_succeeds() -> Result<(), ErrorWrapper> {
//         use crate::avahi::EntryGroupState;

//         let avahi_client = Server::new()?;
//         let entry_group = avahi_client.new_entry_group()?;
//         eprintln!("**** entry_group.get_state() = {:?}", entry_group.get_state()?);
//         eprintln!("**** entry_group.is_empty() = {}", entry_group.is_empty()?);
//         assert!(entry_group.is_empty()?);
//         assert_eq!(entry_group.get_state()?, EntryGroupState::Uncommited);
//         Ok(())
//     }

//     #[test]
//     #[cfg(target_os = "linux")]
//     fn get_host_name_fqdn_succeeds() -> Result<(), ErrorWrapper> {
//         eprintln!(
//             "**** avahi_client.get_host_name_fqdn() = {}",
//             Server::new()?.get_host_name_fqdn()?
//         );
//         Ok(())
//     }

//     #[test]
//     #[cfg(target_os = "linux")]
//     fn get_proxy_succeeds() -> Result<(), ErrorWrapper> {
//         Server::new()?.get_proxy();
//         Ok(())
//     }

//     #[test]
//     #[cfg(target_os = "linux")]
//     fn get_version_succeeds() -> Result<(), ErrorWrapper> {
//         eprintln!("**** avahi_client.get_version() = {}",
// Server::new()?.get_version()?);         Ok(())
//     }
// }
