mod publish;

use crate::commands::publish::Publish;
use structopt::StructOpt;

/// Available commands
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Publishes this mod to BeatMods2
    Publish(Publish),
}
