use crate::{commands::Run, utils};
use anyhow::{bail, Context, Result};
use manifest::Manifest;
use reqwest::{
    multipart::{Form, Part},
    ClientBuilder, Response,
};
use std::{
    fs::{self, File},
    io::Cursor,
    path::PathBuf,
};
use structopt::StructOpt;

/// BeatMods1 categories (legacy)
static BM1_CATEGORIES: &[&str] = &[
    "Other",
    "Core",
    "Cosmetic",
    "Practice / Training",
    "Gameplay",
    "Stream Tools",
    "Libraries",
    "UI Enhancements",
    "Lighting",
    "Tweaks / Tools",
    "Multiplayer",
    "Text Changes",
];

/// Publish command options
#[derive(StructOpt, Debug)]
pub struct Publish {
    /// File to publish
    #[structopt(name = "FILE")]
    file: Option<String>,

    /// BeatMods1 category (legacy)
    #[structopt(short, long, name = "CATEGORY", default_value = "Other")]
    category: String,

    /// Lists BeatMods1 categories (legacy)
    #[structopt(short, long)]
    list_categories: bool,

    /// BeatMods1 user (legacy)
    #[structopt(short, long, name = "USER")]
    user: String,

    /// BeatMods1 password (legacy)
    #[structopt(short, long, name = "PASSWORD")]
    password: String,
}

impl Run for Publish {
    fn run(self, verbose: bool) -> Result<()> {
        if self.list_categories {
            println!("{}", BM1_CATEGORIES.join("\n"));
            return Ok(());
        }

        let manifest = read_manifest()?;
        if !&manifest.is_valid() {
            bail!("Invalid manifest");
        }
        run_commands(&manifest, verbose).context("Failed to run script specified in manifest")?;
        let resource = if let Some(file) = self.file {
            fs::read(file).context("Failed to read specified file")?
        } else if let Some(resource) = &manifest.publish.resource {
            read_resource(resource, verbose)
                .context("Failed to read resource specified in manifest")?
        } else {
            bail!("No resource to publish specified");
        };
        let mut response =
            publish_bm1(manifest, resource, self.category, self.user, self.password)?;
        println!("{:#?}", response.text());
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
fn run_commands(manifest: &Manifest, verbose: bool) -> Result<()> {
    print!("Running commands... ");

    let script = &manifest.publish.script;
    if script.is_empty() {
        print!("No commands.");
    };
    println!();
    for command in script {
        println!("$ {}", &command);

        let o = utils::shell_exec(&command, !verbose).context("Failed to run command")?;
        if !o.success() {
            bail!("Command did not exit successfully");
        }
    }
    Ok(())
}

/// Obtains a byte buffer containing the resource to upload to BeatMods2
fn read_resource(resource_path: &PathBuf, verbose: bool) -> Result<Vec<u8>> {
    println!("Getting resource ready...");

    if !resource_path.exists() {
        bail!("Can't find specified resource");
    }

    if resource_path.is_dir() {
        println!("Resource is a directory. Zipping...");

        let buffer = Cursor::new(Vec::new());
        Ok(utils::zip_dir(resource_path, buffer, verbose)
            .context("Failed to zip directory")?
            .into_inner())
    } else {
        Ok(fs::read(resource_path)?)
    }
}

/// Publishes the mod to BeatMods1 (legacy)
fn publish_bm1(
    manifest: Manifest,
    resource: Vec<u8>,
    category: String,
    user: String,
    password: String,
) -> Result<Response> {
    println!("Publishing to BeatMods1...");

    let version_string = manifest.version.to_string();
    let link_string = if let Some(l) = manifest.links.project_home {
        l.into_string()
    } else if let Some(l) = manifest.links.project_source {
        l.into_string()
    } else {
        unreachable!();
    };
    let description_string = manifest.description.join("\n");

    if !BM1_CATEGORIES.iter().any(|c| c == &category) {
        bail!("Invalid category");
    }

    let resource_len = resource.len();
    let mut resource_name = manifest.id.clone();
    resource_name.push('.');
    resource_name.push_str(&version_string);
    resource_name.push_str(".zip");
    let file = Part::reader_with_length(Cursor::new(resource), resource_len as u64)
        .file_name(resource_name)
        .mime_str("application/zip")
        .unwrap();

    let mut form = Form::new()
        .part("file", file)
        .text("name", manifest.name)
        .text("version", manifest.version.to_string())
        .text("gameVersion", manifest.game_version)
        .text("link", link_string)
        .text("description", description_string)
        .text("category", category);
    if let Some(d) = &manifest.depends_on {
        let dependencies_string = d
            .iter()
            .map(|d| {
                let mut s = d.0.clone();
                s.push('@');
                s.push_str(&d.1.minimum().to_string());
                s
            })
            .collect::<Vec<String>>()
            .join(",");
        form = form.text("dependencies", dependencies_string);
    }

    let client = ClientBuilder::new().cookie_store(true).build()?;

    let login_form = [("username", user), ("password", password)];
    let login_response = client
        .post("https://beatmods.com/api/v1/signIn")
        .form(&login_form)
        .send()?;
    let token = login_response
        .headers()
        .get("x-access-token")
        .context("Invalid credentials")?
        .to_str()?;

    Ok(client
        .post("https://beatmods.com/api/v1/mod/create/")
        .multipart(form)
        .bearer_auth(token)
        .send()?)
}
