use crate::{
    commands::Run,
    utils::{self, CWD},
};
use manifest::Manifest;
use std::{fs::File, process};
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

        // If a command execution fails, publishing is cancelled
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
