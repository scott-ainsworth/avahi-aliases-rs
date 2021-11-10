#![warn(clippy::all)]

use avahi_aliases as lib;
use lib::DaemonOpts;

#[paw::main]
fn main(opts: DaemonOpts) {
    println!("Hello, World!");
    println!("{:?}", opts);
}

// end