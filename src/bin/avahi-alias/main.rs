#![warn(clippy::all)]

use std::collections::HashSet;

use avahi_aliases as lib;
use lib::{logging, AliasesFile, Command, CommandOpts, ErrorWrapper};

#[paw::main]
fn main(opts: CommandOpts) {
    logging::init_console(opts.common.verbose, opts.common.debug);
    let result = match opts.cmd {
        Command::Add { aliases } => add(&opts.common.file, &aliases),
        Command::List {} => list(&opts.common.file),
        Command::Remove { aliases } => remove(&opts.common.file, &aliases),
    };
    if let Err(error) = result {
        log::error!("{}", error);
    }
}

fn add(filename: &str, arg_aliases: &[String]) -> Result<(), ErrorWrapper> {
    let file = AliasesFile::from_file(filename)?;
    let file_aliases: HashSet<&str> = file.aliases().into_iter().collect();
    let (_, new_aliases) = split_aliases(&file_aliases, arg_aliases);
    for alias in new_aliases.iter() {
        log::info!("Adding {:?} to {}", alias, filename);
    }
    file.append(&new_aliases)
}

fn list(filename: &str) -> Result<(), ErrorWrapper> {
    let file = AliasesFile::from_file(filename)?;
    for alias in file.aliases() {
        println!("{}", alias);
    }
    Ok(())
}

fn remove(filename: &str, arg_aliases: &[String]) -> Result<(), ErrorWrapper> {
    let file = AliasesFile::from_file(filename)?;
    let file_aliases: HashSet<&str> = file.aliases().into_iter().collect();
    let (extant_aliases, _) = split_aliases(&file_aliases, arg_aliases);
    for alias in extant_aliases.iter() {
        log::info!("Removing alias {:?} from {}", alias, filename);
    }
    file.remove(extant_aliases)
}

fn split_aliases<'a>(
    file_aliases: &HashSet<&str>, aliases_arg: &'a [String],
) -> (Vec<&'a str>, Vec<&'a str>) {
    aliases_arg
        .iter()
        .map(|c| c.as_ref())
        .into_iter()
        .partition(|alias| file_aliases.contains(alias))
}
