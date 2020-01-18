use crate::{commands::Run, updater};
use anyhow::Result;
use structopt::StructOpt;

/// Update command options
#[derive(StructOpt, Debug)]
pub struct Update {}

impl Run for Update {
    fn run(self, _verbose: bool) -> Result<()> {
        updater::update(false)
    }
}
