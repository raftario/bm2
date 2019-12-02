#![cfg_attr(feature = "nightly", feature(backtrace))]

/// CLI subcommands
mod commands;
/// Various utilities and helpers
mod utils;

use crate::commands::{Command, Run};
use std::process;
use structopt::StructOpt;

/// CLI for the Beat Saber mod repository BeatMods2
#[derive(StructOpt, Debug)]
struct Opt {
    /// Prints more detailed information
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

fn main() {
    let opt = Opt::from_args();
    let result = opt.cmd.run(opt.verbose);
    match result {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{:#}", e);
            process::exit(1)
        }
    }
}
