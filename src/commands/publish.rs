use crate::{commands::Run, utils};
use manifest::Manifest;
use std::{
    fs::{self, File},
    io::Cursor,
    path::PathBuf,
    process,
};
use structopt::StructOpt;

/// Publish command options
#[derive(StructOpt, Debug)]
pub struct Publish {
    /// File to publish
    file: Option<String>,
}

impl Run for Publish {
    fn run(self, verbose: bool) {
        let manifest = read_manifest();
        println!("{:#?}", manifest);
        run_commands(&manifest);
        let resource = get_resource(&self.file, &manifest);
    }
}

/// Reads and parses the `manifest.json` file
fn read_manifest() -> Manifest {
    eprintln!("Reading manifest...");

    let manifest_path = PathBuf::from("manifest.json");
    if !manifest_path.exists() {
        eprintln!("Can't find manifest file, make sure you are running from the same directory.");
        process::exit(1)
    }

    let manifest_file = File::open(manifest_path).unwrap_or_else(|e| {
        eprintln!("Can't read manifest file: {}", e);
        process::exit(1)
    });
    Manifest::from_reader(&manifest_file).unwrap_or_else(|e| {
        eprintln!("Invalid manifest file: {}", e);
        process::exit(1)
    })
}

/// Runs the publish script commands from the manifest
fn run_commands(manifest: &Manifest) {
    print!("Running commands... ");

    let commands = match &manifest.publish.script {
        Some(s) => s,
        None => {
            println!("No commands.");
            return;
        }
    };
    println!();
    for command in commands {
        println!("$ {}", &command);

        let output = utils::shell_exec(&command);
        match output {
            Ok(o) if !o.success() => {
                eprintln!("Command did not exit successfully.");
                process::exit(1);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Can't run command: {}", e);
                process::exit(1)
            }
        }
    }
}

/// Obtains a byte buffer containing the resource to upload to BeatMods2
fn get_resource(file: &Option<String>, manifest: &Manifest) -> Vec<u8> {
    println!("Getting resource ready...");

    if let Some(f) = file {
        return fs::read(f).unwrap_or_else(|e| {
            eprintln!("Can't read specified file: {}", e);
            process::exit(1);
        });
    }

    let resource_path = match &manifest.publish.resource {
        Some(p) => p,
        None => {
            eprintln!("No resource specified.");
            process::exit(1);
        }
    };

    if !resource_path.exists() {
        eprintln!("Can't find specified resource.");
        process::exit(1);
    }

    if resource_path.is_dir() {
        println!("Resource is a directory. Zipping...");

        let buffer = Cursor::new(Vec::new());
        utils::zip_dir(resource_path, buffer)
            .unwrap_or_else(|_| {
                eprintln!("Error zipping directory.");
                process::exit(1);
            })
            .into_inner()
    } else {
        fs::read(resource_path).unwrap_or_else(|_| {
            eprintln!("Can't read specified resource.");
            process::exit(1);
        })
    }
}
