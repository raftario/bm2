use crate::commands::Run;
use manifest::Manifest;
use std::{env, fs::File, process};
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
    }
}

fn read_manifest() -> Manifest {
    let cwd = env::current_dir().unwrap_or_else(|_| {
        eprintln!("Can't determine current working directory.");
        process::exit(1)
    });

    let manifest_path = cwd.join("manifest.json");
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
