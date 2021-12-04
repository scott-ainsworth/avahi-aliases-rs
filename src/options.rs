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

    /// Change detection polling interval in seconds
    #[structopt(short = "p", long = "poll", default_value = "30")]
    pub polling_interval: u64,

    /// Log to syslog (vice console)
    #[structopt(long = "syslog")]
    pub syslog: bool, // cov(skip)

    /// Alias mDNS time-to-live (TTL) in seconds
    #[structopt(long = "ttl", default_value = "60")]
    pub ttl: u64,
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
    fn empty_common_opts_yields_defaults() {
        let common_opts = CommonOpts::from_iter([""]);
        assert!(!common_opts.debug);
        assert_eq!(common_opts.file, "/etc/avahi/avahi-aliases");
        assert!(!common_opts.verbose);
    }

    #[test]
    fn debug_flag_works() {
        assert!(CommonOpts::from_iter(["", "-d"]).debug);
        assert!(CommonOpts::from_iter(["", "--debug"]).debug);
    }

    #[test]
    fn file_option_works() {
        assert_eq!(CommonOpts::from_iter(["", "-f", "avahi-aliases"]).file, "avahi-aliases");
        assert_eq!(
            CommonOpts::from_iter(["", "--file", "avahi-aliases"]).file,
            "avahi-aliases"
        );
    }

    #[test]
    fn verbose_flag_works() {
        assert!(CommonOpts::from_iter(["", "-v"]).verbose);
        assert!(CommonOpts::from_iter(["", "--verbose"]).verbose);
    }

    //******************************************************************************************
    // Add Command

    #[test]
    fn add_command_yields_add_command_opts() {
        let opts = CommandOpts::from_iter_safe(["", "add", "a1.local"]);
        assert!(opts.is_ok());
        assert!(matches!(opts.unwrap().cmd, Command::Add { .. }));
    }

    #[test]
    fn empty_options_yields_defaults_for_add() {
        let opts = CommandOpts::from_iter(vec!["", "add", "a0.local"]);
        assert!(!opts.common.debug);
        assert_eq!(opts.common.file, "/etc/avahi/avahi-aliases");
        assert!(!opts.common.verbose);
    }

    #[test]
    fn debug_flag_works_for_add() {
        assert!(CommandOpts::from_iter(["", "add", "-d", "a0.local"]).common.debug);
        assert!(CommandOpts::from_iter(["", "add", "--debug", "a0.local"]).common.debug);
    }

    #[test]
    fn file_option_works_for_add() {
        assert_eq!(
            CommandOpts::from_iter(["", "add", "-f", "avahi-aliases", "a0.local"]).common.file,
            "avahi-aliases"
        );
        assert_eq!(
            CommandOpts::from_iter(["", "add", "--file", "avahi-aliases", "a0.local"])
                .common
                .file,
            "avahi-aliases"
        );
    }

    #[test]
    fn verbose_flag_works_for_add() {
        assert!(CommandOpts::from_iter(["", "add", "-v", "a0.local"]).common.verbose);
        assert!(CommandOpts::from_iter(["", "add", "--verbose", "a0.local"]).common.verbose);
    }

    #[test]
    fn add_command_aliases_work() {
        let opts = CommandOpts::from_iter_safe(["", "add", "a1.local"]);
        assert!(opts.is_ok());
        if let Command::Add { aliases, .. } = opts.unwrap().cmd {
            assert_eq!(aliases.len(), 1);
            assert_eq!(aliases[0], "a1.local")
        };
        let opts = CommandOpts::from_iter_safe(["", "add", "a1.local", "a2.local"]);
        assert!(opts.is_ok());
        if let Command::Add { aliases, .. } = opts.unwrap().cmd {
            assert_eq!(aliases.len(), 2);
            assert_eq!(aliases[0], "a1.local");
            assert_eq!(aliases[1], "a2.local")
        };
    }

    #[test]
    fn add_command_requires_at_least_one_alias() {
        let opts = CommandOpts::from_iter_safe(["", "add"]);
        assert!(opts.is_err());
        let error = opts.unwrap_err();
        assert_eq!(error.kind, clap::ErrorKind::MissingRequiredArgument);
        assert!(error.message.contains("<ALIAS>"));
    }

    //******************************************************************************************
    // List Command

    #[test]
    fn list_command_yields_list_command_opts() {
        let opts = CommandOpts::from_iter_safe(["", "list"]);
        assert!(opts.is_ok());
        assert!(matches!(opts.unwrap().cmd, Command::List { .. }));
    }

    #[test]
    fn empty_options_yields_defaults_for_list() {
        let opts = CommandOpts::from_iter(vec!["", "list"]);
        assert!(!opts.common.debug);
        assert_eq!(opts.common.file, "/etc/avahi/avahi-aliases");
        assert!(!opts.common.verbose);
    }

    #[test]
    fn debug_flag_works_for_list() {
        assert!(CommandOpts::from_iter(["", "list", "-d"]).common.debug);
        assert!(CommandOpts::from_iter(["", "list", "--debug"]).common.debug);
    }

    #[test]
    fn file_option_works_for_list() {
        assert_eq!(
            CommandOpts::from_iter(["", "list", "-f", "avahi-aliases"]).common.file,
            "avahi-aliases"
        );
        assert_eq!(
            CommandOpts::from_iter(["", "list", "--file", "avahi-aliases"]).common.file,
            "avahi-aliases"
        );
    }

    #[test]
    fn verbose_flag_works_for_list() {
        assert!(CommandOpts::from_iter(["", "list", "-v"]).common.verbose);
        assert!(CommandOpts::from_iter(["", "list", "--verbose"]).common.verbose);
    }


    //******************************************************************************************
    // Remove Command

    #[test]
    fn remove_command_yields_remove_command_opts() {
        let opts = CommandOpts::from_iter_safe(["", "remove", "a1.local"]);
        assert!(opts.is_ok());
        assert!(matches!(opts.unwrap().cmd, Command::Remove { .. }));
    }

    #[test]
    fn empty_options_yields_defaults_for_remove() {
        let opts = CommandOpts::from_iter(vec!["", "remove", "a0.local"]);
        assert!(!opts.common.debug);
        assert_eq!(opts.common.file, "/etc/avahi/avahi-aliases");
        assert!(!opts.common.verbose);
    }

    #[test]
    fn debug_flag_works_for_remove() {
        assert!(CommandOpts::from_iter(["", "remove", "-d", "a0.local"]).common.debug);
        assert!(CommandOpts::from_iter(["", "remove", "--debug", "a0.local"]).common.debug);
    }

    #[test]
    fn file_option_works_for_remove() {
        assert_eq!(
            CommandOpts::from_iter(["", "remove", "-f", "avahi-aliases", "a0.local"])
                .common
                .file,
            "avahi-aliases"
        );
        assert_eq!(
            CommandOpts::from_iter(["", "remove", "--file", "avahi-aliases", "a0.local"])
                .common
                .file,
            "avahi-aliases"
        );
    }

    #[test]
    fn verbose_flag_works_for_remove() {
        assert!(CommandOpts::from_iter(["", "remove", "-v", "a0.local"]).common.verbose);
        assert!(CommandOpts::from_iter(["", "remove", "--verbose", "a0.local"]).common.verbose);
    }

    #[test]
    fn remove_command_aliases_work() {
        let opts = CommandOpts::from_iter_safe(["", "remove", "a1.local"]);
        assert!(opts.is_ok());
        if let Command::Remove { aliases, .. } = opts.unwrap().cmd {
            assert_eq!(aliases.len(), 1);
            assert_eq!(aliases[0], "a1.local")
        };
        let opts = CommandOpts::from_iter_safe(["", "remove", "a1.local", "a2.local"]);
        assert!(opts.is_ok());
        if let Command::Remove { aliases, .. } = opts.unwrap().cmd {
            assert_eq!(aliases.len(), 2);
            assert_eq!(aliases[0], "a1.local");
            assert_eq!(aliases[1], "a2.local")
        };
    }

    #[test]
    fn remove_command_without_force_requires_at_least_one_alias() {
        let opts = CommandOpts::from_iter_safe(["", "remove"]);
        assert!(opts.as_ref().is_err());
        let error = opts.as_ref().unwrap_err();
        assert_eq!(error.kind, clap::ErrorKind::MissingRequiredArgument);
        assert!(error.message.contains("<ALIAS>"));
    }

    #[test]
    fn remove_command_force_requires_no_aliases() {
        let opts = CommandOpts::from_iter_safe(["", "remove", "--force"]);
        assert!(&opts.as_ref().is_ok());
        if let Command::Remove { aliases, force } = opts.unwrap().cmd {
            assert!(force);
            assert_eq!(aliases.len(), 0);
        }
    }

    #[test]
    fn remove_command_accepts_both_force_and_aliases() {
        let opts = CommandOpts::from_iter_safe(["", "remove", "--force", "a0.local"]);
        assert!(&opts.as_ref().is_ok());
        if let Command::Remove { aliases, force } = opts.unwrap().cmd {
            assert!(force);
            assert_eq!(aliases.len(), 1)
        }
    }

    //******************************************************************************************
    // Daemon

    #[test]
    fn empty_options_yields_defaults_for_daemon() {
        let opts = DaemonOpts::from_iter([""]);
        assert!(!opts.common.debug);
        assert_eq!(opts.common.file, "/etc/avahi/avahi-aliases");
        assert!(!opts.common.verbose);
        assert_eq!(opts.polling_interval, 30);
        assert_eq!(opts.ttl, 60);
        assert!(!opts.syslog);
    }

    #[test]
    fn debug_flag_works_for_daemon() {
        assert!(DaemonOpts::from_iter(["", "-d"]).common.debug);
        assert!(DaemonOpts::from_iter(["", "--debug"]).common.debug);
    }

    #[test]
    fn file_option_works_for_daemon() {
        assert_eq!(
            DaemonOpts::from_iter(["", "-f", "avahi-aliases"]).common.file,
            "avahi-aliases"
        );
        assert_eq!(
            DaemonOpts::from_iter(["", "--file", "avahi-aliases"]).common.file,
            "avahi-aliases"
        );
    }

    #[test]
    fn verbose_flag_works_for_daemon() {
        assert!(DaemonOpts::from_iter(["", "-v"]).common.verbose);
        assert!(DaemonOpts::from_iter(["", "--verbose"]).common.verbose);
    }

    #[test]
    fn daemon_poll_option_works() {
        let opts = DaemonOpts::from_iter(["", "--poll", "10"]);
        assert_eq!(opts.polling_interval, 10);
        let opts = DaemonOpts::from_iter(["", "-p", "10"]);
        assert_eq!(opts.polling_interval, 10);
    }

    #[test]
    fn daemon_ttl_option_works() {
        let opts = DaemonOpts::from_iter(["", "--ttl", "100"]);
        assert_eq!(opts.ttl, 100);
    }

    #[test]
    fn daemon_syslog_option_works() {
        let opts = DaemonOpts::from_iter(["", "--syslog"]);
        assert!(opts.syslog);
    }
}

// end
