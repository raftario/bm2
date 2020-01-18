use crate::{commands::Run, config::Config, globals::TERM_ERR, utils};
use anyhow::Result;
use dialoguer::Input;
use manifest::{Manifest, SCHEMA};
use semver::Version;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

/// Init command options
#[derive(StructOpt, Debug)]
pub struct Init {
    /// File to write
    #[structopt(name = "FILE", default_value = "manifest.json")]
    file: PathBuf,

    /// ID
    #[structopt(long, name = "ID")]
    id: Option<String>,

    /// Name
    #[structopt(long, name = "NAME")]
    name: Option<String>,

    /// Game version
    #[structopt(long, name = "VERSION")]
    game_version: Option<String>,

    /// Description
    #[structopt(long, name = "DESCRIPTION")]
    description: Option<String>,

    /// Author
    #[structopt(long, name = "AUTHOR")]
    author: Option<String>,

    /// License
    #[structopt(long, name = "LICENSE")]
    license: Option<String>,
}

impl Run for Init {
    fn run(self, verbose: bool) -> Result<()> {
        let config = Config::read()?;

        let id = self
            .id
            .unwrap_or(Input::new().with_prompt("ID").interact_on(&*TERM_ERR)?);
        let name = self
            .name
            .unwrap_or(Input::new().with_prompt("Name").interact_on(&*TERM_ERR)?);
        let game_version = Input::new()
            .with_prompt("Game version")
            .interact_on(&*TERM_ERR)?;
        let description = self.description.unwrap_or(
            Input::new()
                .with_prompt("Description")
                .interact_on(&*TERM_ERR)?,
        );

        let author = self.author.unwrap_or(
            config
                .defaults
                .author
                .unwrap_or(Input::new().with_prompt("Author").interact_on(&*TERM_ERR)?),
        );
        let license = self.license.unwrap_or(
            config.defaults.license.unwrap_or(
                Input::new()
                    .with_prompt("License")
                    .interact_on(&*TERM_ERR)?,
            ),
        );

        let mut manifest = Manifest {
            schema: SCHEMA.to_owned(),
            id,
            name,
            version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: vec![],
                build: vec![],
            },
            game_version,
            description: vec![description],
            author,
            license,
            depends_on: None,
            conflicts_with: None,
            load_after: None,
            load_before: None,
            features: None,
            icon: None,
            links: Default::default(),
            publish: Default::default(),
            readme: None,
        };
        utils::edit_until_valid(&mut manifest)?;

        if verbose {
            TERM_ERR.write_line("Writing manifest...")?;
        }
        let f = File::create(&self.file)?;
        manifest.to_writer(f)?;
        Ok(())
    }
}
