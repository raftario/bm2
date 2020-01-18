use crate::{commands::Run, config::Config, globals::TERM_ERR};
use anyhow::Result;
use dialoguer::Input;
use manifest::{Manifest, DESCRIPTION_REGEX, ID_REGEX, NAME_REGEX, SCHEMA};
use regex::Regex;
use semver::Version;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

/// Init command options
#[derive(StructOpt, Debug)]
pub struct Init {
    /// File to write
    #[structopt(name = "FILE", default_value = "manifest.json")]
    file: PathBuf,

    /// Mod ID
    #[structopt(long, name = "ID")]
    id: Option<String>,

    /// Mod name
    #[structopt(long, name = "NAME")]
    name: Option<String>,

    /// Mod game version
    #[structopt(long, name = "GAMEVERSION")]
    game_version: Option<String>,

    /// Mod description
    #[structopt(long, name = "DESCRIPTION")]
    description: Option<String>,

    /// Mod author
    #[structopt(long, name = "AUTHOR")]
    author: Option<String>,

    /// Mod license
    #[structopt(long, name = "LICENSE")]
    license: Option<String>,
}

impl Run for Init {
    fn run(self, _verbose: bool) -> Result<()> {
        let config = Config::read()?;

        let id = self.id.unwrap_or(ask_until_valid("ID", &*ID_REGEX)?);
        let name = self.name.unwrap_or(ask_until_valid("Name", &*NAME_REGEX)?);
        let game_version = Input::new()
            .with_prompt("Game version")
            .interact_on(&*TERM_ERR)?;
        let description = self
            .description
            .unwrap_or(ask_until_valid("Description", &*DESCRIPTION_REGEX)?);

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

        let manifest = Manifest {
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

        let f = File::create(&self.file)?;
        manifest.to_writer(f)?;
        Ok(())
    }
}

fn ask_until_valid(prompt: &str, check: &Regex) -> Result<String> {
    let mut answer: String;
    loop {
        answer = Input::new().with_prompt(prompt).interact_on(&*TERM_ERR)?;
        if check.is_match(&answer) {
            break;
        }
        TERM_ERR.write_line(&format!("{} doesn't match the manifest format", prompt))?;
    }
    Ok(answer)
}
