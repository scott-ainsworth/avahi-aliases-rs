#![warn(clippy::all)]

use std::collections::HashSet;
use std::io;

use avahi_aliases as lib;
use lib::{AliasesFile, CommandOpts, Command};

mod messaging;
use messaging::msg;

#[paw::main]
fn main(opts: CommandOpts) {
    match opts.cmd {
        Command::Add { aliases } => {
            messaging::init(opts.common.verbose, opts.common.debug);
            add(&opts.common.file, &aliases)
        },
        Command::List { } => {
            messaging::init(opts.common.verbose, opts.common.debug);
            list(&opts.common.file)
        },
        Command::Remove { aliases } => {
            messaging::init(opts.common.verbose, opts.common.debug);
            remove(&opts.common.file, &aliases)
        },
    }
    .err()
    .iter()
    .for_each(|error| {
        match error.kind() {
            io::ErrorKind::NotFound => eprintln!(
                "AVAHI-ALIASES file ({}) not found", &opts.common.file),
            _ => eprintln!("{:?}", error)
        };
        std::process::exit(1);
    });
}

fn add(filename: &str, arg_aliases: &[String]) -> Result<(), io::Error> {
    let file = AliasesFile::from_file(filename)?;
    modify(
        &file,
        arg_aliases,
        &|alias| msg::info!("{:?} is already in {}", alias, filename),
        &|alias| msg::info!("Adding {:?} to {}", alias, filename),
        &|_, new_aliases| file.append(new_aliases),
    )
}

fn list(filename: &str) -> Result<(), io::Error> {
    let file = AliasesFile::from_file(filename)?;
    for alias in file.aliases() {
        println!("{}", alias);
    }
    Ok(())
}

fn remove(filename: &str, arg_aliases: &[String]) -> Result<(), io::Error> {
    let file = AliasesFile::from_file(filename)?;
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
