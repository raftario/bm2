mod publish;

use crate::commands::publish::Publish;
use failure::Fallible;
use structopt::StructOpt;

/// Available commands
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Publishes this mod to BeatMods2
    Publish(Publish),
}

/// Run function, the trait is not really needed but it's a nice convention
pub trait Run {
    /// Runs the command
    fn run(self, verbose: bool) -> Fallible<()>;
}

impl Run for Command {
    fn run(self, verbose: bool) -> Fallible<()> {
        match self {
            Command::Publish(p) => p.run(verbose),
        }
    }
}
