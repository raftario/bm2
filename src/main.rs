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
    if let Err(err) = opt.cmd.run(opt.verbose) {
        eprintln!("Error: {}", err);
        for cause in err.iter_causes() {
            eprintln!("  caused by: {}", cause);
        }
        // Use RUST_BACKTRACE=1 to enable
        let backtrace = err.backtrace();
        if !backtrace.is_empty() {
            eprintln!("\n{}", backtrace);
        }
        std::process::exit(1);
    }
}
