#![warn(clippy::all)]

use structopt::{self, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "avahi-alias", about = "Maintain /etc/avahi/avahi-aliases")]
pub struct CommandOpts {
    /// The subcommand to execute
    #[structopt(subcommand)]
    pub cmd: Command,

    #[structopt(flatten)]
    pub common: CommonOpts,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "avahi-alias-daemon", about = "Publish Avahi aliases")]
pub struct DaemonOpts {
    /// Common options (verbose, debug, & filename)
    #[structopt(flatten)]
    pub common: CommonOpts,
}

#[derive(Debug, StructOpt)]
pub struct CommonOpts {
    /// Prints detailed messages
    #[structopt(short, long, global = true)]
    pub verbose: bool,

    /// Prints detailed and debug messages
    /// Note: debug has presidence over verbose
    #[structopt(short, long, global = true)]
    pub debug: bool,

    /// Sets the avahi-aliases file path
    #[structopt(short, long, global = true,
        name = "ALIASES-FILE", default_value = "/etc/avahi/avahi-aliases")]
    pub file: String,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Add Aliases")]
    Add {
        /// Aliases to add
        #[structopt(name = "ALIAS", required = true)]
        aliases: Vec<String>,
    },

    #[structopt(about = "Remove Aliases")]
    Remove {
        /// Aliases to remove
        #[structopt(name = "ALIAS", required = true)]
        aliases: Vec<String>,
    },

    #[structopt(about = "List existing Aliases")]
    List {
    },
}

//**********************************************************************************************
// Unit tests
//**********************************************************************************************

#[cfg(test)]

//**********************************************************************************************
// Flags

#[test]
fn debug_flag_works() {
    assert_eq!(CommonOpts::from_iter(["avahi-aliases"]).debug, false);
    assert_eq!(CommonOpts::from_iter(["avahi-aliases", "-d"]).debug, true);
    assert_eq!(CommonOpts::from_iter(["avahi-aliases", "--debug"]).debug, true);
}

#[test]
fn file_option_works() {
    assert_eq!(CommonOpts::from_iter([
        "avahi-aliases"]).file, "/etc/avahi/avahi-aliases");
    assert_eq!(CommonOpts::from_iter([
        "avahi-aliases", "-f", "avahi-aliases"]).file, "avahi-aliases");
    assert_eq!(CommonOpts::from_iter([
        "avahi-aliases", "--file", "avahi-aliases"]).file, "avahi-aliases");
}

#[test]
fn verbose_flag_works() {
    assert_eq!(CommonOpts::from_iter(["avahi-aliases"]).verbose, false);
    assert_eq!(CommonOpts::from_iter(["avahi-aliases", "-v"]).verbose, true);
    assert_eq!(CommonOpts::from_iter(["avahi-aliases", "--verbose"]).verbose, true);
}

//**********************************************************************************************
// Add Command

#[test]
fn add_command_works() {
    assert!(matches!(
        CommandOpts::from_iter(["avahi-aliases", "add", "a1.local"]).cmd,
        Command::Add { .. }));
}

#[test]
fn add_flags_work() {
    let opts = CommandOpts::from_iter([
            "avahi-aliases", "add", "-v", "-d", "-f", "avahi-aliases", "a1.local"]);
    assert!(opts.common.debug);
    assert_eq!(opts.common.file, "avahi-aliases");
    assert!(opts.common.verbose);
}

#[test]
fn add_command_aliases_work() {
    match CommandOpts::from_iter(["avahi-aliases", "add", "a1.local"]).cmd {
        Command::Add { aliases, .. } => {
            assert_eq!(aliases.len(), 1);
            assert_eq!(aliases[0], "a1.local")
        },
        _ => ()
    };
    match CommandOpts::from_iter(["avahi-aliases", "add", "a1.local", "a2.local"]).cmd {
        Command::Add { aliases, .. } => {
            assert_eq!(aliases.len(), 2);
            assert_eq!(aliases[0], "a1.local");
            assert_eq!(aliases[1], "a2.local")
        },
        _ => ()
    }
}

#[test]
fn add_command_requires_at_least_one_alias() {
    let opts = CommandOpts::from_iter_safe(["avahi-aliases", "add"]);
    assert!(matches!(
        &opts.as_ref().unwrap_err(),
        clap::Error { kind: clap::ErrorKind::MissingRequiredArgument, .. }));
    assert!(opts.unwrap_err().message.contains("<ALIAS>"));
}

//**********************************************************************************************
// List Command

#[test]
fn list_command_works() {
    assert!(matches!(
        CommandOpts::from_iter(["avahi-aliases", "list"]).cmd,
        Command::List { .. }));
}

#[test]
fn list_flags_work() {
    let opts = CommandOpts::from_iter([
            "avahi-aliases", "list", "-v", "-d", "-f", "avahi-aliases"]);
    assert!(opts.common.debug);
    assert_eq!(opts.common.file, "avahi-aliases");
    assert!(opts.common.verbose);
}

//**********************************************************************************************
// Remove Command

#[test]
fn remove_command_works() {
    assert!(matches!(
        CommandOpts::from_iter(["avahi-aliases", "remove", "a1.local"]).cmd,
        Command::Remove { .. }));
}

#[test]
fn remove_flags_work() {
    let opts = CommandOpts::from_iter([
            "avahi-aliases", "remove", "-v", "-d", "-f", "avahi-aliases", "a1.local"]);
    assert!(opts.common.debug);
    assert_eq!(opts.common.file, "avahi-aliases");
    assert!(opts.common.verbose);
}

#[test]
fn remove_command_requires_at_least_one_alias() {
    let opts = CommandOpts::from_iter_safe(["avahi-aliases", "remove"]);
    assert!(matches!(
        &opts.as_ref().unwrap_err(),
        clap::Error { kind: clap::ErrorKind::MissingRequiredArgument, .. }));
    assert!(opts.unwrap_err().message.contains("<ALIAS>"));
}

#[test]
fn remove_command_aliases_work() {
    match CommandOpts::from_iter(["avahi-aliases", "remove", "a1.local"]).cmd {
        Command::Remove { aliases, .. } => {
            assert_eq!(aliases.len(), 1);
            assert_eq!(aliases[0], "a1.local")
        },
        _ => ()
    };
    match CommandOpts::from_iter(["avahi-aliases", "remove", "a1.local", "a2.local"]).cmd {
        Command::Add { aliases, .. } => {
            assert_eq!(aliases.len(), 2);
            assert_eq!(aliases[0], "a1.local");
            assert_eq!(aliases[1], "a2.local")
        },
        _ => ()
    }
}

// end