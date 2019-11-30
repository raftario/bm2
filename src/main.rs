/// CLI subcommands
mod commands;
/// Various utilities and helpers
mod utils;

use crate::commands::{Command, Run};
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
    opt.cmd.run(opt.verbose);
}
