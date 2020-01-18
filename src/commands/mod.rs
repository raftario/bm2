mod config;
mod publish;

use crate::commands::{config::Config, publish::Publish};
use anyhow::Result;
use structopt::StructOpt;

/// Run function, the trait is not really needed but it's a nice convention
pub trait Run {
    /// Runs the command
    fn run(self, verbose: bool) -> Result<()>;
}

macro_rules! create_command {
    ($($name:ident : $doc:literal,)*) => {
        /// Available commands
        #[derive(StructOpt, Debug)]
        pub enum Command {
            $(
                #[doc=$doc]
                $name($name),
            )*
        }

        impl Run for Command {
            fn run(self, verbose: bool) -> Result<()> {
                match self {
                    $(
                        Self::$name(c) => c.run(verbose),
                    )*
                }
            }
        }
    }
}

create_command!(
    Publish: "Publishes this mod to BeatMods",
    Config: "Edit the application config",
);
