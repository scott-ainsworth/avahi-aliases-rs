#![warn(clippy::all)]

//use std::io;

mod options;
//use avahi_aliases::cnames::CNamesFile;

#[paw::main]
fn main(args: options::Args) {
    println!("Hello, World!");
    println!("{:?}", args);
}
