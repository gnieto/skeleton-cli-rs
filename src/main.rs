extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

use clap::App;
use std::env;
use errors::*;

/// Representation of library errors
pub mod errors {
    error_chain!{}
}

fn main() {
    env_logger::init().unwrap();

    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    Ok(())
}