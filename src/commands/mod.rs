mod publish;

use crate::commands::publish::Publish;
use enum_dispatch::enum_dispatch;
use anyhow::Result;
use structopt::StructOpt;

/// Available commands
#[enum_dispatch(Run)]
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Publishes this mod to BeatMods2
    Publish(Publish),
}

/// Run function, the trait is not really needed but it's a nice convention
#[enum_dispatch]
pub trait Run {
    /// Runs the command
    fn run(self, verbose: bool) -> Result<()>;
}
