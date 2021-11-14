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
    // modify(
    //     &file,
    //     arg_aliases,
    //     &|alias| log::info!("{:?} is already in {}", alias, filename),
    //     &|alias| log::info!("Adding {:?} to {}", alias, filename),
    //     &|_, new_aliases| file.append(new_aliases),
    // )
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
    modify(
        &file,
        arg_aliases,
        &|alias| log::info!("Removing alias {:?} from {}", alias, filename),
        &|alias| log::info!("{:?} is not in {}", alias, filename),
        &|extant_aliases, _| file.remove(extant_aliases),
    )
}

fn modify(
    file: &AliasesFile, arg_aliases: &[String], extant_msg: &dyn Fn(&str),
    new_msg: &dyn Fn(&str), action: &dyn Fn(Vec<&str>, Vec<&str>) -> Result<(), ErrorWrapper>,
) -> Result<(), ErrorWrapper> {
    let file_aliases: HashSet<&str> = file.aliases().into_iter().collect();
    let (extant_aliases, new_aliases) = split_aliases(&file_aliases, arg_aliases);
    for alias in extant_aliases.iter() {
        extant_msg(alias);
    }
    for alias in new_aliases.iter() {
        new_msg(alias);
    }
    action(extant_aliases, new_aliases)
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
