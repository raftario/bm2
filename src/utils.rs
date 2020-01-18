use crate::globals::TERM_ERR;
use anyhow::Result;
use cfg_if::cfg_if;
use dialoguer::Input;
use manifest::{Manifest, ValidityError, DESCRIPTION_REGEX, ID_REGEX, NAME_REGEX};
use regex::Regex;
use std::{
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
    process::{Command, ExitStatus, Stdio},
};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

/// Zips a folder into the passed writer and returns it
pub fn zip_dir<P, W>(path: P, writer: W, verbose: bool) -> Result<W>
where
    P: AsRef<Path>,
    W: Write + Seek,
{
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default().unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in WalkDir::new(&path).min_depth(1) {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry_path.strip_prefix(&path)?;

        if entry_path.is_file() {
            zip.start_file_from_path(entry_name, options)?;
            let mut f = File::open(entry_path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;

            if verbose {
                TERM_ERR.write_line(&format!("Added file {}", entry_path.display()))?;
            }

            buffer.clear();
        } else {
            zip.add_directory_from_path(entry_name, options)?;

            if verbose {
                TERM_ERR.write_line(&format!("Added file {}", entry_path.display()))?;
            }
        }
    }

    Ok(zip.finish()?)
}

/// Runs a command using the OS specific shell and current working directory
pub fn shell_exec(command_str: &str, output: bool) -> io::Result<ExitStatus> {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (shell, flag) = ("cmd", "/C");
        } else {
            let (shell, flag) = ("sh", "-c");
        }
    }
    let mut cmd = Command::new(shell);
    cmd.arg(flag);
    cmd.arg(&command_str);
    if !output {
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
    }
    cmd.status()
}

/// Ask for input until it validates against the provided regex
pub fn ask_until_valid(prompt: &str, check: &Regex) -> Result<String> {
    let mut answer: String;
    loop {
        answer = Input::new().with_prompt(prompt).interact_on(&*TERM_ERR)?;
        if check.is_match(&answer) {
            break;
        }
        TERM_ERR.clear_last_lines(1)?;
    }
    Ok(answer)
}

/// Ask for modifications until the manifest is valid
pub fn edit_until_valid(manifest: &mut Manifest) -> Result<()> {
    while let Err(e) = manifest.validate() {
        match e {
            ValidityError::InvalidId => {
                TERM_ERR.write_line("The current ID doesn't follow the new manifest format (should follow the C# namespace naming convention)")?;
                manifest.id = ask_until_valid("New ID", &*ID_REGEX)?;
            }
            ValidityError::InvalidName => {
                TERM_ERR.write_line("The current name doesn't follow the new manifest format (should not contain newlines or tabs)")?;
                manifest.name = ask_until_valid("New name", &*NAME_REGEX)?;
            }
            ValidityError::InvalidDescription => {
                TERM_ERR.write_line("The current description doesn't follow the new manifest format (should not contain newlines)")?;
                manifest.description =
                    vec![ask_until_valid("New description", &*DESCRIPTION_REGEX)?];
            }
        }
    }
    Ok(())
}
