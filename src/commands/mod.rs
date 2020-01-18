mod config;
mod init;
mod migrate;
mod publish;
mod update;

use crate::commands::{
    config::Config, init::Init, migrate::Migrate, publish::Publish, update::Update,
};
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
    Config: "Edits the application config",
    Init: "Initialises a new manifest",
    Migrate: "Migrates a manifest from the old to the new format",
    Publish: "Publishes this mod to BeatMods",
    Update: "Checks for updates and install them",
);
