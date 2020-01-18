#![cfg_attr(feature = "nightly", feature(backtrace))]

/// CLI subcommands
mod commands;
/// Application configuration
mod config;
/// Global constants and static variables
mod globals;
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
    /// Prints more information
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

fn main() -> Result<()> {
    if env::args().any(|a| &a == "finish_update") {
        return updater::finish_update();
    }
    if config::Config::read()?.auto_update {
        updater::update(true)?;
    }

    let opt = Opt::from_args();
    opt.cmd.run(opt.verbose)?;
    Ok(())
}
