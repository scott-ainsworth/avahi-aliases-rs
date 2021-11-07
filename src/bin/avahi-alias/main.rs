#![warn(clippy::all)]

use std::collections::HashSet;
use std::io;

mod messaging;
use messaging::msg;
mod options;
use options::{Args, Command};
use avahi_aliases::cnames::CNamesFile;

#[paw::main]
fn main(args: Args) {
    match args.cmd {
        Command::Add { common_opts, cnames } => {
            messaging::init(common_opts.verbose, common_opts.debug);
            add(common_opts.file, &cnames)
        },
        Command::List { common_opts } => {
            messaging::init(common_opts.verbose, common_opts.debug);
            list(common_opts.file)
        },
        Command::Remove { common_opts, cnames } => {
            messaging::init(common_opts.verbose, common_opts.debug);
            remove(common_opts.file, &cnames)
        },
    }
    .err()
    .iter()
    .for_each(|error| eprintln!("{:?}", error));
}

fn add(filename: String, arg_cnames: &[String]) -> Result<(), io::Error> {
    let file = CNamesFile::from_file(&filename)?;
    modify(
        &file,
        arg_cnames,
        &|cname| msg::info!("{:?} is already in {}", cname, filename),
        &|cname| msg::info!("Adding {:?} to {}", cname, filename),
        &|_, new_cnames| file.append(new_cnames),
    )
}

fn list(filename: String) -> Result<(), io::Error> {
    let file = CNamesFile::from_file(&filename)?;
    for cname in file.cnames() {
        println!("{}", cname);
    }
    Ok(())
}

fn remove(filename: String, arg_cnames: &[String]) -> Result<(), io::Error> {
    let file = CNamesFile::from_file(&filename)?;
    modify(
        &file,
        arg_cnames,
        &|cname| msg::info!("Removing CNAME {:?} from {}", cname, filename),
        &|cname| msg::info!("{:?} is not in {}", cname, filename),
        &|extant_cnames, _| file.remove(extant_cnames),
    )
}

fn modify(
    file: &CNamesFile, arg_cnames: &[String], extant_msg: &dyn Fn(&str),
    new_msg: &dyn Fn(&str), action: &dyn Fn(Vec<&str>, Vec<&str>) -> Result<(), io::Error>,
) -> Result<(), io::Error> {
    let file_cnames: HashSet<&str> = file.cnames().into_iter().collect();
    let (extant_cnames, new_cnames) = split_cnames(&file_cnames, arg_cnames);
    for cname in extant_cnames.iter() {
        extant_msg(cname);
    }
    for cname in new_cnames.iter() {
        new_msg(cname);
    }
    action(extant_cnames, new_cnames)
}

fn split_cnames<'a>(
    file_cnames: &HashSet<&str>, cnames_arg: &'a [String],
) -> (Vec<&'a str>, Vec<&'a str>) {
    cnames_arg
        .iter()
        .map(|c| c.as_ref())
        .into_iter()
        .partition(|cname| file_cnames.contains(cname))
}
