#![warn(clippy::all)]

//use std::io;

use avahi_aliases as lib;
use lib::CommonOpts;

#[paw::main]
fn main(opts: CommonOpts) {
    println!("Hello, World!");
    println!("{:?}", opts);
}
