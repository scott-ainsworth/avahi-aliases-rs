#![warn(clippy::all)]

use std::collections::HashSet;

use anyhow::{Result};
use avahi_aliases::{
    init_console_logging, validate_aliases, AliasesFile, Command, CommandOpts,
};

#[paw::main]
fn main(opts: CommandOpts) {
    if let Err(error) = inner_main(opts) {
        log::error!("Error: {}", error);
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn inner_main(opts: CommandOpts) -> Result<()> {
    init_console_logging(opts.common.verbose, opts.common.debug)?;
    match opts.cmd {
        Command::Add { aliases } => add(&opts.common.file, &aliases),
        Command::List {} => list(&opts.common.file),
        Command::Remove { aliases, force } => remove(&opts.common.file, &aliases, force),
    }
}

fn add(filename: &str, arg_aliases: &[String]) -> Result<()> {
    // Validate command line aliases
    validate_aliases(arg_aliases)?;
    // Load the avahi-aliases file. (fails if there are invalid aliases.)
    let aliases_file = AliasesFile::from_file(filename, false)?;
    // new_aliases are commane line aliases not already in the file (don't add dups!).
    let (_, new_aliases) =
        split_aliases(&aliases_file.aliases().into_iter().collect(), arg_aliases);
    for alias in new_aliases.iter() {
        log::info!("Adding {:?} to {}", alias, filename);
    }
    aliases_file.append(&new_aliases)
}

fn list(filename: &str) -> Result<()> {
    let aliases_file = AliasesFile::from_file(filename, true)?;
    if aliases_file.alias_count() == 0 {
        log::warn!(r#"No aliases in "{}""#, filename);
        return Ok(());
    }
    for alias in aliases_file.all_aliases() {
        match alias {
            Ok(alias) => println!("{}", alias),
            Err(invalid_alias) => {
                println!(r#"ERROR: invalid alias "{}""#, invalid_alias)
            },
        }
    }
    Ok(())
}

fn remove(filename: &str, arg_aliases: &[String], force: bool) -> Result<()> {
    // Validate command line aliases
    validate_aliases(arg_aliases)?;
    // Load the avahi-aliases file. (Fails if there are invalid aliases
    // unless --force is in play.)
    let aliases_file = AliasesFile::from_file(filename, force)?;
    // If --force and there are invalid aliases, delete them
    if !aliases_file.invalid_aliases().is_empty() {
        for alias in aliases_file.invalid_aliases().iter() {
            log::info!("Removing invalid alias {:?} from {}", alias, filename);
        }
        aliases_file.remove(&aliases_file.invalid_aliases(), true)?;
    }
    // extant_aliases is the list of aliases to be removed
    let (extant_aliases, _) =
        split_aliases(&aliases_file.aliases().into_iter().collect(), arg_aliases);
    for alias in extant_aliases.iter() {
        log::info!("Removing alias {:?} from {}", alias, filename);
    }
    aliases_file.remove(&extant_aliases, false)
}

fn split_aliases<'a>(
    file_aliases: &HashSet<&str>, arg_aliases: &'a [String],
) -> (Vec<&'a str>, Vec<&'a str>) {
    arg_aliases
        .iter()
        .map(|c| c.as_ref())
        .into_iter()
        .partition(|alias| file_aliases.contains(alias))
}
