#![warn(clippy::all)]

use std::{fs, thread, time};

use ::time::format_description::well_known::Rfc3339;
use ::time::OffsetDateTime;
use structopt::StructOpt;
use anyhow::Result;
use avahi_aliases::{
    init_console_logging, init_syslog_logging, AliasesFile, Avahi, DaemonOpts, ErrorWrapper,
    LoggingError,
};

#[derive(PartialEq)]
struct ModifiedSize {
    last_modified: time::SystemTime,
    len: u64,
}

#[paw::main]
fn main(opts: DaemonOpts) {
    match exec(opts) {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            log::error!("Error: {}", error);
            std::process::exit(1)
        },
    }
}

fn exec(opts: DaemonOpts) -> Result<(), ErrorWrapper> {
    init_logging(opts.common.verbose, opts.common.debug, opts.syslog)?;
    signon_app();
    let file_name = opts.common.file.as_str();
    let avahi: Avahi = Avahi::new()?;
    signon_avahi(&avahi)?;
    load_publish_loop(&avahi, file_name, time::Duration::from_secs(5))?;
    Ok(())
}

fn init_logging(verbose: bool, debug: bool, syslog: bool) -> Result<(), LoggingError> {
    match syslog {
        true => init_syslog_logging(verbose, debug),
        false => {
            init_console_logging(verbose, debug);
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
    avahi: &Avahi, file_name: &str, sleep_duration: time::Duration,
) -> Result<(), ErrorWrapper> {
    let mut modified_size = ModifiedSize { last_modified: time::UNIX_EPOCH, len: 0 };

    loop {
        log::debug!("Retrieving metadata for {:?}", file_name);
        let new_modified_size = get_metadata(file_name)?;
        //.with_context(|| format!("could not load aliases from {:?}", file_name))?;
        if new_modified_size != modified_size {
            let aliases_file = load_aliases(file_name, &new_modified_size)?;
            //.with_context(|| format!("could not load aliases from {:?}", file_name))?;
            publish_aliases(avahi, &aliases_file, file_name, &new_modified_size)?;
            //.with_context(|| "could not publish aliases")?;
            modified_size = new_modified_size;
        } else {
            log::debug!("Alias file {:?} has not changed", file_name);
        }
        thread::sleep(sleep_duration);
    }
}

fn publish_aliases<'a>(
    avahi: &Avahi, aliases_file: &AliasesFile, file_name: &'a str, modified_size: &ModifiedSize,
) -> Result<(), ErrorWrapper> {
    let last_modified: OffsetDateTime = modified_size.last_modified.into();
    let fqdn = avahi.get_host_name_fqdn()?;
    log::debug!(
        r#"Publishing aliases from "{}" for "{}" (modified {})"#,
        file_name,
        fqdn,
        last_modified.format(&Rfc3339).unwrap()
    );
    let rdata = Avahi::encode_rdata(&fqdn);
    for alias in aliases_file.all_aliases() {
        let group = avahi.get_group()?;
        match alias {
            Ok(a) => {
                log::info!("Publishing alias {}", a);
                let cname = Avahi::encode_name(a);
                group.add_record(&cname, &rdata, 60)?;
                group.commit()?;
            },
            Err(a) => log::info!(r#"WARNING: invalid alias "{}" ignored"#, a),
        }
    }
    log::info!(
        "Published aliases from {:?} (modified {})",
        file_name,
        last_modified.format(&Rfc3339).unwrap()
    );
    Ok(())
}

fn signon_app() {
    let app = DaemonOpts::clap();
    log::info!("{} {} {}", app.get_name(), clap::crate_version!(), clap::crate_authors!());
}

fn signon_avahi(avahi: &Avahi) -> Result<(), ErrorWrapper> {
    let version = avahi.get_version()?;
    let host_fqdn = avahi.get_host_name_fqdn()?;
    log::info!("{}, host fqdn: {}", version, host_fqdn);
    Ok(())
}

// end
