#![warn(clippy::all)]

use std::collections::HashSet;

use avahi_aliases::{alias, logging, AliasesFile, Command, CommandOpts, ErrorWrapper};

#[paw::main]
fn main(opts: CommandOpts) {
    logging::init_console(opts.common.verbose, opts.common.debug);
    let result = match opts.cmd {
        Command::Add { aliases } => add(&opts.common.file, &aliases),
        Command::List {} => list(&opts.common.file),
        Command::Remove { aliases } => remove(&opts.common.file, &aliases),
    };
    if let Err(error) = result {
        log::error!("Error: {}", error);
    }
}

fn add(filename: &str, arg_aliases: &[String]) -> Result<(), ErrorWrapper> {
    alias::validate_aliases(arg_aliases)?;
    let aliases_file = AliasesFile::from_file(filename)?;
    aliases_file.is_valid()?;
    let file_aliases: HashSet<&str> = aliases_file.aliases().into_iter().collect();
    let (_, new_aliases) = split_aliases(&file_aliases, arg_aliases);
    for alias in new_aliases.iter() {
        log::info!("Adding {:?} to {}", alias, filename);
    }
    aliases_file.append(&new_aliases)
}

fn list(filename: &str) -> Result<(), ErrorWrapper> {
    let aliases_file = AliasesFile::from_file(filename)?;
    for alias in aliases_file.all_aliases() {
        match alias {
            Ok(alias) => println!("{}", alias),
            Err(invalid_alias) => {
                println!("ERROR: {}", ErrorWrapper::new_invalid_alias_error(invalid_alias))
            },
        }
    }
    Ok(())
}

fn remove(filename: &str, arg_aliases: &[String]) -> Result<(), ErrorWrapper> {
    alias::validate_aliases(arg_aliases)?;
    let aliases_file = AliasesFile::from_file(filename)?;
    aliases_file.is_valid()?;
    let file_aliases: HashSet<&str> = aliases_file.aliases().into_iter().collect();
    let (extant_aliases, _) = split_aliases(&file_aliases, arg_aliases);
    for alias in extant_aliases.iter() {
        log::info!("Removing alias {:?} from {}", alias, filename);
    }
    aliases_file.remove(&extant_aliases)
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