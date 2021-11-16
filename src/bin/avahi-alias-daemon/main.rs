#![warn(clippy::all)]

use std::{fs, thread, time};

use ::time::format_description::well_known::Rfc3339;
use ::time::OffsetDateTime;
use structopt::StructOpt;
use anyhow::Result;
use avahi_aliases as lib;
use lib::{logging, AliasesFile, DaemonOpts, ErrorWrapper};

#[derive(PartialEq)]
struct ModifiedSize {
    last_modified: time::SystemTime,
    len: u64,
}

#[paw::main]
fn main(opts: DaemonOpts) {
    logging::init_console(opts.common.verbose, opts.common.debug);
    signon();

    let file_name = opts.common.file.as_str();

    let result = load_publish_loop(file_name, time::Duration::from_secs(5));
    if let Err(error) = result {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn load_publish_loop(file_name: &str, sleep_duration: time::Duration) -> Result<()> {
    let mut modified_size = ModifiedSize { last_modified: time::UNIX_EPOCH, len: 0 };

    loop {
        let new_modified_size = get_metadata(file_name)?;
        //.with_context(|| format!("could not load aliases from {:?}", file_name))?;
        if new_modified_size != modified_size {
            let aliases_file = load_aliases(file_name, &new_modified_size)?;
            //.with_context(|| format!("could not load aliases from {:?}", file_name))?;
            publish_aliases(&aliases_file, file_name, &new_modified_size)?;
            //.with_context(|| "could not publish aliases")?;
            modified_size = new_modified_size;
        } else {
            log::debug!("Alias file {:?} has not changed", file_name);
        }
        thread::sleep(sleep_duration);
    }
}

fn get_metadata(file_name: &str) -> Result<ModifiedSize, ErrorWrapper> {
    log::debug!("Retrieving metadata for {:?}", file_name);
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
    AliasesFile::from_file(file_name)
}

fn publish_aliases<'a>(
    aliases_file: &AliasesFile, file_name: &'a str, modified_size: &ModifiedSize,
) -> Result<(), ErrorWrapper> {
    let last_modified: OffsetDateTime = modified_size.last_modified.into();
    log::debug!(
        "Publishing aliases from {:?} (modified {})",
        file_name,
        last_modified.format(&Rfc3339).unwrap()
    );
    for alias in aliases_file.aliases() {
        log::info!("Publishing alias {}", alias);
    }
    log::info!(
        "Published aliases from {:?} (modified {})",
        file_name,
        last_modified.format(&Rfc3339).unwrap()
    );
    Ok(())
}

fn signon() {
    let app = DaemonOpts::clap();
    log::info!("{} {} {}", app.get_name(), clap::crate_version!(), clap::crate_authors!());
}


// end
