mod publish;

use crate::commands::publish::Publish;
use structopt::StructOpt;

/// Available commands
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Publishes this mod to BeatMods2
    Publish(Publish),
}

pub trait Run {
    fn run(self, verbose: bool);
}

impl Run for Command {
    fn run(self, verbose: bool) {
        match self {
            Command::Publish(p) => p.run(verbose),
        }
    }
}
