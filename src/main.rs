#![cfg_attr(feature = "nightly", feature(backtrace))]

/// CLI subcommands
mod commands;
/// Global constants and static variables
pub mod globals;
/// Auto updater
mod updater;
/// Various utilities and helpers
mod utils;

use crate::commands::{Command, Run};
use anyhow::Result;
use std::env;
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

fn main() -> Result<()> {
    if env::args().any(|a| &a == "finish_update") {
        return updater::finish_update();
    }
    updater::update()?;

    let opt = Opt::from_args();
    opt.cmd.run(opt.verbose)?;
    Ok(())
}
