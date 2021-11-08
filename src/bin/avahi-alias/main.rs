#![warn(clippy::all)]

use std::collections::HashSet;
use std::io;

mod messaging;
use messaging::msg;
mod options;
use options::{Args, Command};
use avahi_aliases::aliases::AliasesFile;

#[paw::main]
fn main(args: Args) {
    match args.cmd {
        Command::Add { common_opts, aliases } => {
            messaging::init(common_opts.verbose, common_opts.debug);
            add(common_opts.file, &aliases)
        },
        Command::List { common_opts } => {
            messaging::init(common_opts.verbose, common_opts.debug);
            list(common_opts.file)
        },
        Command::Remove { common_opts, aliases } => {
            messaging::init(common_opts.verbose, common_opts.debug);
            remove(common_opts.file, &aliases)
        },
    }
    .err()
    .iter()
    .for_each(|error| eprintln!("{:?}", error));
}

fn add(filename: String, arg_aliases: &[String]) -> Result<(), io::Error> {
    let file = AliasesFile::from_file(&filename)?;
    modify(
        &file,
        arg_aliases,
        &|alias| msg::info!("{:?} is already in {}", alias, filename),
        &|alias| msg::info!("Adding {:?} to {}", alias, filename),
        &|_, new_aliases| file.append(new_aliases),
    )
}

fn list(filename: String) -> Result<(), io::Error> {
    let file = AliasesFile::from_file(&filename)?;
    for alias in file.aliases() {
        println!("{}", alias);
    }
    Ok(())
}

fn remove(filename: String, arg_aliases: &[String]) -> Result<(), io::Error> {
    let file = AliasesFile::from_file(&filename)?;
    modify(
        &file,
        arg_aliases,
        &|alias| msg::info!("Removing alias {:?} from {}", alias, filename),
        &|alias| msg::info!("{:?} is not in {}", alias, filename),
        &|extant_aliases, _| file.remove(extant_aliases),
    )
}

fn modify(
    file: &AliasesFile, arg_aliases: &[String], extant_msg: &dyn Fn(&str),
    new_msg: &dyn Fn(&str), action: &dyn Fn(Vec<&str>, Vec<&str>) -> Result<(), io::Error>,
) -> Result<(), io::Error> {
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
