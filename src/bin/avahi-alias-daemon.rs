#![warn(clippy::all)]

use std::{fs, thread, time};

use ::time::format_description::well_known::Rfc3339;
use ::time::OffsetDateTime;
use structopt::StructOpt;
use anyhow::Result;
use avahi_aliases::{
    avahi_dbus, encoding, init_console_logging, init_syslog_logging, AliasesFile, DaemonOpts,
    ErrorWrapper,
};
use avahi_dbus::{avahi, OrgFreedesktopAvahiEntryGroup, OrgFreedesktopAvahiServer};

#[derive(PartialEq)]
struct ModifiedSize {
    last_modified: time::SystemTime,
    len: u64,
}

#[paw::main]
fn main(opts: DaemonOpts) {
    match inner_main(opts) {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            log::error!("Error: {}", error);
            std::process::exit(1)
        },
    }
}

fn inner_main(opts: DaemonOpts) -> Result<(), ErrorWrapper> {
    init_logging(opts.common.verbose, opts.common.debug, opts.syslog)?;
    signon_app();
    let file_name = opts.common.file.as_str();
    let dbus_connection = dbus::blocking::Connection::new_system()?;
    let avahi_server_proxy = dbus_connection.with_proxy(
        avahi::AVAHI_DBUS_NAME,
        avahi::AVAHI_DBUS_PATH_SERVER,
        avahi_aliases::DEFAULT_TIMEOUT,
    );
    signon_avahi(&avahi_server_proxy)?;
    load_publish_loop(
        &avahi_server_proxy,
        file_name,
        time::Duration::new(opts.polling_interval, 0),
    )?;
    Ok(())
}

fn init_logging(verbose: bool, debug: bool, syslog: bool) -> Result<(), ErrorWrapper> {
    match syslog {
        true => init_syslog_logging(verbose, debug),
        false => {
            init_console_logging(verbose, debug)?;
            Ok(())
        },
    }
}

fn get_metadata(file_name: &str) -> Result<ModifiedSize, ErrorWrapper> {
    match fs::metadata(file_name) {
        Ok(metadata) => Ok(ModifiedSize {
            last_modified: metadata.modified().unwrap(),
            len: metadata.len(),
        }),
        Err(error) => {
            Err(ErrorWrapper::MetadataError { file_name: file_name.to_owned(), source: error })
        },
    }
}

fn load_aliases(
    file_name: &str, modified_size: &ModifiedSize,
) -> Result<AliasesFile, ErrorWrapper> {
    let last_modified: OffsetDateTime = modified_size.last_modified.into();
    log::debug!(
        "Loading aliases from {:?} (modified {})",
        file_name,
        last_modified.format(&Rfc3339).unwrap()
    );
    AliasesFile::from_file(file_name, true)
}

fn load_publish_loop(
    avahi_server_proxy: &avahi_dbus::DBusProxy, file_name: &str,
    polling_interval: time::Duration,
) -> Result<(), ErrorWrapper> {
    let mut modified_size = ModifiedSize { last_modified: time::UNIX_EPOCH, len: 0 };

    loop {
        log::debug!(r#"Retrieving metadata for "{}""#, file_name);
        let new_modified_size = get_metadata(file_name)?;
        if new_modified_size != modified_size {
            let aliases_file = load_aliases(file_name, &new_modified_size)?;
            log::info!(r#"Loaded {} aliases from "{}""#, aliases_file.alias_count(), file_name);
            publish_aliases(avahi_server_proxy, &aliases_file, file_name, &new_modified_size)?;
            modified_size = new_modified_size;
        } else {
            log::debug!(r#"Alias file "{}" has not changed"#, file_name);
        }
        thread::sleep(polling_interval);
    }
}

fn publish_aliases<'a>(
    avahi_server_proxy: &avahi_dbus::DBusProxy, aliases_file: &AliasesFile, file_name: &'a str,
    modified_size: &ModifiedSize,
) -> Result<(), ErrorWrapper> {
    let last_modified: OffsetDateTime = modified_size.last_modified.into();
    if aliases_file.alias_count() == 0 {
        log::warn!(
            r#"No aliases in "{}" (modified {})"#,
            file_name,
            last_modified.format(&Rfc3339).unwrap()
        );
        return Ok(());
    }

    let fqdn = avahi_server_proxy.get_host_name_fqdn()?;
    log::debug!(r#"Publishing aliases from "{}" for "{}""#, file_name, fqdn,);

    let rdata = encoding::encode_rdata(&fqdn);
    let entry_group_path = avahi_server_proxy.entry_group_new()?;
    let entry_group_proxy = avahi_server_proxy.connection.with_proxy(
        avahi::AVAHI_DBUS_NAME,
        entry_group_path,
        avahi_aliases::DEFAULT_TIMEOUT,
    );
    for alias in aliases_file.all_aliases() {
        match alias {
            Ok(alias) => {
                log::info!("Publishing alias {}", alias);
                entry_group_proxy.add_record(
                    avahi::Interface::UNSPECIFIED as i32,
                    avahi::Protocol::UNSPEC as i32,
                    0,
                    alias,
                    avahi::RecordClass::IN as u16,
                    avahi::RecordType::CNAME as u16,
                    60,
                    rdata.clone(),
                )?;
                // let cname_record =
                //     AvahiRecord::new_cname(alias, time::Duration::from_secs(60), &rdata);
            },
            Err(a) => log::info!(r#"WARNING: invalid alias "{}" ignored"#, a),
        }
    }
    entry_group_proxy.commit()?;
    log::info!(
        "Published {} aliases from {:?} (modified {})",
        aliases_file.alias_count(),
        file_name,
        last_modified.format(&Rfc3339).unwrap()
    );
    Ok(())
}

fn signon_app() {
    let app = DaemonOpts::clap();
    log::info!("{} {} {}", app.get_name(), clap::crate_version!(), clap::crate_authors!());
}

fn signon_avahi(avahi_server_proxy: &avahi_dbus::DBusProxy) -> Result<(), ErrorWrapper> {
    let version = avahi_server_proxy.get_version_string()?;
    let host_fqdn = avahi_server_proxy.get_host_name_fqdn()?;
    log::info!("{}, host fqdn: {}", version, host_fqdn);
    Ok(())
}

// end
