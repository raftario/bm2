use crate::{
    commands::Run,
    utils::{self, CWD},
};
use manifest::Manifest;
use std::{
    fs::{self, File},
    io::Cursor,
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

    let manifest_path = CWD.join("manifest.json");
    if !manifest_path.exists() {
        eprintln!("Can't find manifest file, make sure you are running from the same directory.");
        process::exit(1)
    }

    let manifest_file = File::open(manifest_path).unwrap_or_else(|_| {
        eprintln!("Can't read manifest file.");
        process::exit(1)
    });
    Manifest::from_reader(&manifest_file).unwrap_or_else(|_| {
        eprintln!("Invalid manifest file.");
        process::exit(1)
    })
}

/// Runs the publish script commands from the manifest
fn run_commands(manifest: &Manifest) {
    print!("Running commands... ");

    if manifest.publish.is_none() || manifest.publish.as_ref().unwrap().script.is_none() {
        println!("No commands.");
        return;
    }

    print!("\n");
    let commands = manifest.publish.as_ref().unwrap().script.as_ref().unwrap();
    for command in commands {
        println!("$ {}", &command);

        let output = utils::shell_exec(&command);
        if let Ok(o) = output {
            if !o.status.success() {
                eprintln!("Command did not exit successfully.");
                process::exit(1);
            }
        } else {
            eprintln!("Can't run command.");
            process::exit(1)
        }
    }
}

/// Obtains a byte buffer containing the resource to upload to BeatMods2
fn get_resource(file: &Option<String>, manifest: &Manifest) -> Vec<u8> {
    println!("Getting resource ready...");

    if let Some(f) = file {
        return fs::read(CWD.join(f)).unwrap_or_else(|_| {
            eprintln!("Can't read specified file.");
            process::exit(1);
        });
    }

    if manifest.publish.is_none() || manifest.publish.as_ref().unwrap().resource.is_none() {
        eprintln!("No resource specified.");
        process::exit(1);
    }

    let resource_path = manifest
        .publish
        .as_ref()
        .unwrap()
        .resource
        .as_ref()
        .unwrap();

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
        fs::read(CWD.join(resource_path)).unwrap_or_else(|_| {
            eprintln!("Can't read specified resource.");
            process::exit(1);
        })
    }
}
