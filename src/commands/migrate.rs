use crate::{commands::Run, globals::TERM_ERR, utils};
use anyhow::{Context, Result};
use dialoguer::Input;
use manifest::{Manifest, OldManifest};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use structopt::StructOpt;

/// Migrate command options
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
    fn run(self, verbose: bool) -> Result<()> {
        if verbose {
            TERM_ERR.write_line("Reading old manifest...")?;
        }
        let f = File::open(&self.file).context("Can't open specified file")?;
        let old_manifest = OldManifest::from_reader(f).context("Invalid manifest")?;
        let license = self.license.unwrap_or(
            Input::new()
                .with_prompt("SPDX identifier of the license for this mod")
                .interact_on(&*TERM_ERR)?,
        );
        let mut new_manifest = Manifest::from((old_manifest, license));
        utils::edit_until_valid(&mut new_manifest)?;

        if verbose {
            TERM_ERR.write_line("Backing up old manifest")?;
        }
        let old_manifest_path = format!("{}.old", &self.file.display());
        fs::rename(&self.file, old_manifest_path)?;
        if verbose {
            TERM_ERR.write_line("Writing new manifest...")?;
        }
        let f = File::create(&self.file)?;
        new_manifest.to_writer(f)?;
        Ok(())
    }
}
