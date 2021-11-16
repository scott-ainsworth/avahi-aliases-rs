#![warn(clippy::all)]

pub use structopt::StructOpt;

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

    /// Log to syslog (vice console)
    #[structopt(short, long)]
    pub syslog: bool,
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
    #[structopt(
        short,
        long,
        global = true,
        name = "ALIASES-FILE",
        default_value = "/etc/avahi/avahi-aliases"
    )]
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
    List {},
}

//**********************************************************************************************
// Unit tests
//**********************************************************************************************

#[cfg(test)]
mod test {
    use structopt::{self, StructOpt};

    use super::{Command, CommandOpts, CommonOpts};

    //******************************************************************************************
    // Flags

    #[test]
    fn debug_flag_works() {
        assert_eq!(CommonOpts::from_iter([""]).debug, false);
        assert_eq!(CommonOpts::from_iter(["", "-d"]).debug, true);
        assert_eq!(CommonOpts::from_iter(["", "--debug"]).debug, true);
    }

    #[test]
    fn file_option_works() {
        assert_eq!(CommonOpts::from_iter([""]).file, "/etc/avahi/avahi-aliases");
        assert_eq!(CommonOpts::from_iter(["", "-f", "avahi-aliases"]).file, "avahi-aliases");
        assert_eq!(
            CommonOpts::from_iter(["", "--file", "avahi-aliases"]).file,
            "avahi-aliases"
        );
    }

    #[test]
    fn verbose_flag_works() {
        assert_eq!(CommonOpts::from_iter([""]).verbose, false);
        assert_eq!(CommonOpts::from_iter(["", "-v"]).verbose, true);
        assert_eq!(CommonOpts::from_iter(["", "--verbose"]).verbose, true);
    }

    //******************************************************************************************
    // Add Command

    #[test]
    fn add_command_works() {
        assert!(matches!(
            CommandOpts::from_iter(["", "add", "a1.local"]).cmd,
            Command::Add { .. }
        ));
    }

    #[test]
    fn add_flags_work() {
        let opts =
            CommandOpts::from_iter(["", "add", "-v", "-d", "-f", "avahi-aliases", "a1.local"]);
        assert!(opts.common.debug);
        assert_eq!(opts.common.file, "avahi-aliases");
        assert!(opts.common.verbose);
    }

    #[test]
    fn add_command_aliases_work() {
        match CommandOpts::from_iter(["", "add", "a1.local"]).cmd {
            Command::Add { aliases, .. } => {
                assert_eq!(aliases.len(), 1);
                assert_eq!(aliases[0], "a1.local")
            },
            _ => (),
        };
        match CommandOpts::from_iter(["", "add", "a1.local", "a2.local"]).cmd {
            Command::Add { aliases, .. } => {
                assert_eq!(aliases.len(), 2);
                assert_eq!(aliases[0], "a1.local");
                assert_eq!(aliases[1], "a2.local")
            },
            _ => (),
        }
    }

    #[test]
    fn add_command_requires_at_least_one_alias() {
        let opts = CommandOpts::from_iter_safe(["", "add"]);
        assert!(matches!(&opts.as_ref().unwrap_err(), clap::Error {
            kind: clap::ErrorKind::MissingRequiredArgument,
            ..
        }));
        assert!(opts.unwrap_err().message.contains("<ALIAS>"));
    }

    //******************************************************************************************
    // List Command

    #[test]
    fn list_command_works() {
        assert!(matches!(CommandOpts::from_iter(["", "list"]).cmd, Command::List { .. }));
    }

    #[test]
    fn list_flags_work() {
        let opts = CommandOpts::from_iter(["", "list", "-v", "-d", "-f", "avahi-aliases"]);
        assert!(opts.common.debug);
        assert_eq!(opts.common.file, "avahi-aliases");
        assert!(opts.common.verbose);
    }

    //******************************************************************************************
    // Remove Command

    #[test]
    fn remove_command_works() {
        assert!(matches!(
            CommandOpts::from_iter(["", "remove", "a1.local"]).cmd,
            Command::Remove { .. }
        ));
    }

    #[test]
    fn remove_flags_work() {
        let opts = CommandOpts::from_iter([
            "",
            "remove",
            "-v",
            "-d",
            "-f",
            "avahi-aliases",
            "a1.local",
        ]);
        assert!(opts.common.debug);
        assert_eq!(opts.common.file, "avahi-aliases");
        assert!(opts.common.verbose);
    }

    #[test]
    fn remove_command_requires_at_least_one_alias() {
        let opts = CommandOpts::from_iter_safe(["", "remove"]);
        assert!(matches!(&opts.as_ref().unwrap_err(), clap::Error {
            kind: clap::ErrorKind::MissingRequiredArgument,
            ..
        }));
        assert!(opts.unwrap_err().message.contains("<ALIAS>"));
    }

    #[test]
    fn remove_command_aliases_work() {
        match CommandOpts::from_iter(["", "remove", "a1.local"]).cmd {
            Command::Remove { aliases, .. } => {
                assert_eq!(aliases.len(), 1);
                assert_eq!(aliases[0], "a1.local")
            },
            _ => (),
        };
        match CommandOpts::from_iter(["", "remove", "a1.local", "a2.local"]).cmd {
            Command::Add { aliases, .. } => {
                assert_eq!(aliases.len(), 2);
                assert_eq!(aliases[0], "a1.local");
                assert_eq!(aliases[1], "a2.local")
            },
            _ => (),
        }
    }
}

// end
