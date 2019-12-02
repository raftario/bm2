use crate::{commands::Run, utils};
use anyhow::{bail, Context, Result};
use manifest::Manifest;
use std::{
    fs::{self, File},
    io::Cursor,
    path::PathBuf,
};
use structopt::StructOpt;

/// Publish command options
#[derive(StructOpt, Debug)]
pub struct Publish {
    /// File to publish
    file: Option<String>,
}

impl Run for Publish {
    fn run(self, verbose: bool) -> Result<()> {
        let manifest = read_manifest()?;
        println!("{:#?}", manifest);
        run_commands(&manifest).context("Failed to run script specified in manifest")?;
        let resource = if let Some(file) = self.file {
            fs::read(file).context("Failed to read specified file")?
        } else if let Some(resource) = &manifest.publish.resource {
            read_resource(resource).context("Failed to read resource specified in manifest")?
        } else {
            bail!("No resource to publish specified");
        };
        Ok(())
    }
}

/// Reads and parses the `manifest.json` file
fn read_manifest() -> Result<Manifest> {
    println!("Reading manifest...");

    let manifest_path = PathBuf::from("manifest.json");
    if !manifest_path.exists() {
        bail!("Can't find manifest file, make sure you are running from the same directory.");
    }

    let manifest_file = File::open(manifest_path).context("Failed to read manifest file")?;
    Ok(Manifest::from_reader(&manifest_file).context("Invalid manifest file")?)
}

/// Runs the publish script commands from the manifest
fn run_commands(manifest: &Manifest) -> Result<()> {
    print!("Running commands... ");

    let script = &manifest.publish.script;
    if script.is_empty() {
        print!("No commands.");
    };
    println!();
    for command in script {
        println!("$ {}", &command);

        let o = utils::shell_exec(&command).context("Failed to run command")?;
        if !o.success() {
            bail!("Command did not exit successfully");
        }
    }
    Ok(())
}

/// Obtains a byte buffer containing the resource to upload to BeatMods2
fn read_resource(resource_path: &PathBuf) -> Result<Vec<u8>> {
    println!("Getting resource ready...");

    if !resource_path.exists() {
        bail!("Can't find specified resource");
    }

    if resource_path.is_dir() {
        println!("Resource is a directory. Zipping...");

        let buffer = Cursor::new(Vec::new());
        Ok(utils::zip_dir(resource_path, buffer)
            .context("Failed to zip directory")?
            .into_inner())
    } else {
        Ok(fs::read(resource_path)?)
    }
}
