#![warn(clippy::all)]

pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "avahi-alias", about = "Maintain /etc/avahi/avahi-aliases")]
pub struct CommandOpts {
    /// The subcommand to execute
    #[structopt(subcommand)]
    pub cmd: Command, // cov(skip)

    #[structopt(flatten)]
    pub common: CommonOpts, // cov(skip)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "avahi-alias-daemon", about = "Publish Avahi aliases")]
pub struct DaemonOpts {
    /// Common options (verbose, debug, & filename)
    #[structopt(flatten)]
    pub common: CommonOpts, // cov(skip)

    /// Change detection polling interval
    #[structopt(short = "p", long = "poll", default_value = "30")]
    pub polling_interval: u64,

    /// Log to syslog (vice console)
    #[structopt(long = "syslog")]
    pub syslog: bool, // cov(skip)
}

#[derive(Debug, StructOpt)]
pub struct CommonOpts {
    /// Prints detailed messages
    #[structopt(short = "v", long = "verbose", global = true)]
    pub verbose: bool, // cov(skip)

    /// Prints detailed (verbose) and debug messages
    #[structopt(short = "d", long = "debug", global = true)]
    pub debug: bool, // cov(skip)

    /// Sets the avahi-aliases file
    #[structopt(
        short = "f",
        long = "file",
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
        #[structopt(name = "ALIAS", required_unless = "force")]
        aliases: Vec<String>,

        /// Force removal of invalid aliases
        #[structopt(long = "force", global = true)]
        force: bool, // cov(skip)
    },

    #[structopt(about = "List existing Aliases")]
    List {},
}

//**********************************************************************************************
// Unit tests
//**********************************************************************************************

#[cfg(test)]
mod tests {
    use structopt::{self, StructOpt};

    use super::*;

    //******************************************************************************************
    // Common Options

    #[test]
    fn debug_flag_works() {
        assert!(!CommonOpts::from_iter([""]).debug);
        assert!(CommonOpts::from_iter(["", "-d"]).debug);
        assert!(CommonOpts::from_iter(["", "--debug"]).debug);
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
        assert!(!CommonOpts::from_iter([""]).verbose);
        assert!(CommonOpts::from_iter(["", "-v"]).verbose);
        assert!(CommonOpts::from_iter(["", "--verbose"]).verbose);
    }

    #[test]
    fn empty_common_options_work_for_command() {
        for cmd in ["add", "list", "remove"] {
            let opts = CommandOpts::from_iter(match cmd {
                "list" => vec!["", cmd],
                _ => vec!["", cmd, "a0.local"],
            });
            assert!(!opts.common.debug);
            assert_eq!(opts.common.file, "/etc/avahi/avahi-aliases");
            assert!(!opts.common.verbose);
        }
    }

    #[test]
    fn long_common_options_work_for_command() {
        for cmd in ["add", "list", "remove"] {
            let opts = CommandOpts::from_iter(match cmd {
                "list" => vec!["", cmd, "--verbose", "--debug", "--file", "avahi-aliases"],
                _ => {
                    vec!["", cmd, "--verbose", "--debug", "--file", "avahi-aliases", "a1.local"]
                },
            });
            assert!(opts.common.debug);
            assert_eq!(opts.common.file, "avahi-aliases");
            assert!(opts.common.verbose);
        }
    }

    #[test]
    fn short_common_options_work_for_command() {
        for cmd in ["add", "list", "remove"] {
            let opts = CommandOpts::from_iter(match cmd {
                "list" => vec!["", cmd, "-v", "-f", "avahi-aliases"],
                _ => vec!["", cmd, "-v", "-f", "avahi-aliases", "a1.local"],
            });
            assert!(!opts.common.debug);
            assert_eq!(opts.common.file, "avahi-aliases");
            assert!(opts.common.verbose);
        }
    }

    #[test]
    fn empty_common_options_work_for_daemon() {
        let opts = DaemonOpts::from_iter([""]);
        assert!(!opts.common.debug);
        assert_eq!(opts.common.file, "/etc/avahi/avahi-aliases");
        assert!(!opts.common.verbose);
    }

    #[test]
    fn long_common_options_work_for_daemon() {
        let opts =
            DaemonOpts::from_iter(["", "--verbose", "--debug", "--file", "avahi-aliases"]);
        assert!(opts.common.debug);
        assert_eq!(opts.common.file, "avahi-aliases");
        assert!(opts.common.verbose);
    }

    #[test]
    fn short_common_options_work_for_daemon() {
        let opts = DaemonOpts::from_iter(["", "-v", "-f", "avahi-aliases"]);
        assert!(!opts.common.debug);
        assert_eq!(opts.common.file, "avahi-aliases");
        assert!(opts.common.verbose);
    }


    //******************************************************************************************
    // Add Command

    #[test]
    fn add_command_yields_add_command_opts() {
        assert!(matches!(
            CommandOpts::from_iter(["", "add", "a1.local"]).cmd,
            Command::Add { .. }
        ));
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
    fn list_command_yields_list_command_opts() {
        assert!(matches!(CommandOpts::from_iter(["", "list"]).cmd, Command::List { .. }));
    }

    //******************************************************************************************
    // Remove Command

    #[test]
    fn remove_command_yields_remove_command_opts() {
        assert!(matches!(
            CommandOpts::from_iter(["", "remove", "a1.local"]).cmd,
            Command::Remove { .. }
        ));
    }

    //******************************************************************************************
    // Command line aliases for add and remove subcommands

    #[test]
    fn command_line_aliases_are_available() {
        for cmd in ["add", "remove"] {
            match CommandOpts::from_iter(["", cmd, "a1.local"]).cmd {
                Command::Remove { aliases, .. } => {
                    assert_eq!(aliases.len(), 1);
                    assert_eq!(aliases[0], "a1.local")
                },
                _ => (),
            };
            match CommandOpts::from_iter(["", cmd, "a1.local", "a2.local"]).cmd {
                Command::Add { aliases, .. } => {
                    assert_eq!(aliases.len(), 2);
                    assert_eq!(aliases[0], "a1.local");
                    assert_eq!(aliases[1], "a2.local")
                },
                _ => (),
            }
        }
    }

    #[test]
    fn at_least_one_command_line_aliases_is_required() {
        for cmd in ["add", "remove"] {
            let opts = CommandOpts::from_iter_safe(["", cmd]);
            assert!(matches!(&opts.as_ref().unwrap_err(), clap::Error {
                kind: clap::ErrorKind::MissingRequiredArgument,
                ..
            }));
            assert!(opts.unwrap_err().message.contains("<ALIAS>"));
        }
    }
}

//******************************************************************************************
// Command line aliases for add and remove subcommands

#[test]
fn daemon_empty_options_work() {
    let opts = DaemonOpts::from_iter([""]);
    assert_eq!(opts.polling_interval, 30);
    assert!(!opts.syslog);
}

#[test]
fn daemon_long_options_work() {
    let opts = DaemonOpts::from_iter(["", "--poll", "10", "--syslog", "--file"]);
    assert_eq!(opts.polling_interval, 10);
    assert!(opts.syslog);
}

// end
