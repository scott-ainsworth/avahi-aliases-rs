#![warn(clippy::all)]

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "avahi-alias", about = "Maintain /etc/avahi/avahi-aliases")]
pub struct Args {
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
        common_opts: CommonOpts,

        /// Aliases to add
        #[structopt(required = true)]
        aliases: Vec<String>,
    },

    #[structopt(about = "Remove Aliases from Avahi")]
    Remove {
        #[structopt(flatten)]
        common_opts: CommonOpts,

        /// Aliases to remove
        #[structopt(required = true)]
        aliases: Vec<String>,
    },

    #[structopt(about = "List existing Aliases")]
    List {
        #[structopt(flatten)]
        common_opts: CommonOpts,
    },
}
