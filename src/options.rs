#![warn(clippy::all)]

use structopt::{self, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "avahi-alias", about = "Maintain /etc/avahi/avahi-aliases")]
pub struct CommandOpts {
    /// The subcommand to execute
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub struct CommonOpts {
    /// Print detailed progress messages
    // log level Info
    #[structopt(short, long)]
    pub verbose: bool,

    /// Print debugging messages
    /// Note: debug has presidence over verbose
    // log level Debug
    #[structopt(short, long)]
    pub debug: bool,

    /// Path of the avahi-aliases file
    #[structopt(short, long, default_value = "/etc/avahi/avahi-aliases")]
    pub file: String,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Add Aliases")]
    Add {
        #[structopt(flatten)]
        common: CommonOpts,

        /// Aliases to add
        #[structopt(name = "ALIAS", required = true)]
        aliases: Vec<String>,
    },

    #[structopt(about = "Remove Aliases from Avahi")]
    Remove {
        #[structopt(flatten)]
        common: CommonOpts,

        /// Aliases to remove
        #[structopt(name = "ALIAS", required = true)]
        aliases: Vec<String>,
    },

    #[structopt(about = "List existing Aliases")]
    List {
        #[structopt(flatten)]
        common: CommonOpts,
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
    assert_eq!(CommonOpts::from_iter(["avahi-daemon"]).debug, false);
    assert_eq!(CommonOpts::from_iter(["avahi-daemon", "-d"]).debug, true);
    assert_eq!(CommonOpts::from_iter(["avahi-daemon", "--debug"]).debug, true);
}

#[test]
fn file_option_works() {
    assert_eq!(CommonOpts::from_iter([
        "avahi-daemon"]).file, "/etc/avahi/avahi-aliases");
    assert_eq!(CommonOpts::from_iter([
        "avahi-daemon", "-f", "avahi-aliases"]).file, "avahi-aliases");
    assert_eq!(CommonOpts::from_iter([
        "avahi-daemon", "--file", "avahi-aliases"]).file, "avahi-aliases");
}

#[test]
fn verbose_flag_works() {
    assert_eq!(CommonOpts::from_iter(["avahi-daemon"]).verbose, false);
    assert_eq!(CommonOpts::from_iter(["avahi-daemon", "-v"]).verbose, true);
    assert_eq!(CommonOpts::from_iter(["avahi-daemon", "--verbose"]).verbose, true);
}

//**********************************************************************************************
// Add Command

#[test]
fn add_command_works() {
    assert!(matches!(
        CommandOpts::from_iter(["avahi-daemon", "add", "a1.local"]).cmd,
        Command::Add { .. }));
}

#[test]
fn add_flags_work() {
    match CommandOpts::from_iter([
            "avahi-daemon", "add", "-v", "-d", "-f", "avahi-aliases", "a1.local"]).cmd {
        Command::Add { common, .. } => {
            assert!(common.debug);
            assert_eq!(common.file, "avahi-aliases");
            assert!(common.verbose);
        },
        _ => ()
    }
}

#[test]
fn add_command_aliases_work() {
    match CommandOpts::from_iter(["avahi-daemon", "add", "a1.local"]).cmd {
        Command::Add { aliases, .. } => {
            assert_eq!(aliases.len(), 1);
            assert_eq!(aliases[0], "a1.local")
        },
        _ => ()
    };
    match CommandOpts::from_iter(["avahi-daemon", "add", "a1.local", "a2.local"]).cmd {
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
    let opts = CommandOpts::from_iter_safe(["avahi-daemon", "add"]);
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
        CommandOpts::from_iter(["avahi-daemon", "list"]).cmd,
        Command::List { .. }));
}

#[test]
fn list_flags_work() {
    match CommandOpts::from_iter([
            "avahi-daemon", "list", "-v", "-d", "-f", "avahi-aliases"]).cmd {
        Command::Add { common, .. } => {
            assert!(common.debug);
            assert_eq!(common.file, "avahi-aliases");
            assert!(common.verbose);
        },
        _ => ()
    }
}

//**********************************************************************************************
// Remove Command

#[test]
fn remove_command_works() {
    assert!(matches!(
        CommandOpts::from_iter(["avahi-daemon", "remove", "a1.local"]).cmd,
        Command::Remove { .. }));
}

#[test]
fn remove_flags_work() {
    match CommandOpts::from_iter([
            "avahi-daemon", "remove", "-v", "-d", "-f", "avahi-aliases", "a1.local"]).cmd {
        Command::Remove { common, .. } => {
            assert!(common.debug);
            assert_eq!(common.file, "avahi-aliases");
            assert!(common.verbose);
        },
        _ => ()
    }
}

#[test]
fn remove_command_requires_at_least_one_alias() {
    let opts = CommandOpts::from_iter_safe(["avahi-daemon", "remove"]);
    assert!(matches!(
        &opts.as_ref().unwrap_err(),
        clap::Error { kind: clap::ErrorKind::MissingRequiredArgument, .. }));
    assert!(opts.unwrap_err().message.contains("<ALIAS>"));
}

#[test]
fn remove_command_aliases_work() {
    match CommandOpts::from_iter(["avahi-daemon", "remove", "a1.local"]).cmd {
        Command::Remove { aliases, .. } => {
            assert_eq!(aliases.len(), 1);
            assert_eq!(aliases[0], "a1.local")
        },
        _ => ()
    };
    match CommandOpts::from_iter(["avahi-daemon", "remove", "a1.local", "a2.local"]).cmd {
        Command::Add { aliases, .. } => {
            assert_eq!(aliases.len(), 2);
            assert_eq!(aliases[0], "a1.local");
            assert_eq!(aliases[1], "a2.local")
        },
        _ => ()
    }
}

// end