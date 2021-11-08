#![warn(clippy::all)]

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "avahi-alias-daemon",
    about = "Publish /etc/avahi/avahi-aliases")]
pub struct Args {
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
