mod commands;

use crate::commands::Command;
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
    println!("{:#?}", opt);
}
