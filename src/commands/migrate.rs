use crate::{commands::Run, globals::TERM_ERR};
use anyhow::{Context, Result};
use dialoguer::Input;
use indicatif::ProgressBar;
use manifest::{Manifest, OldManifest};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use structopt::StructOpt;

/// Publish command options
#[derive(StructOpt, Debug)]
pub struct Migrate {
    /// File to migrate
    #[structopt(name = "FILE", default_value = "manifest.json")]
    file: PathBuf,

    /// License to use
    #[structopt(short, long, name = "LICENSE")]
    license: Option<String>,
}

impl Run for Migrate {
    fn run(self, _verbose: bool) -> Result<()> {
        let p = ProgressBar::new_spinner();
        p.set_message("Reading old manifest");
        p.enable_steady_tick(100);

        let f = File::open(&self.file).context("Can't open specified file")?;
        let old_manifest = OldManifest::from_reader(f).context("Invalid manifest")?;
        p.finish_and_clear();
        let license = self.license.unwrap_or(
            Input::new()
                .with_prompt("SPDX identifier of the license for this mod")
                .interact_on(&*TERM_ERR)?,
        );
        let new_manifest = Manifest::from((old_manifest, license));

        p.set_message("Writing new manifest");

        let old_manifest_path = format!("{}.old", &self.file.display());
        fs::rename(&self.file, old_manifest_path)?;
        let f = File::create(&self.file)?;
        new_manifest.to_writer(f)?;

        p.finish_and_clear();
        Ok(())
    }
}
