use anyhow::Result;
use std::{
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
    process::{Command, ExitStatus},
};
use walkdir::WalkDir;
use zip::{
    result::{ZipError, ZipResult},
    write::FileOptions,
    ZipWriter,
};

/// Zips a folder into the passed writer and returns it
pub fn zip_dir<P, W>(path: P, writer: W) -> Result<W>
where
    P: AsRef<Path>,
    W: Write + Seek,
{
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default().unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in WalkDir::new(&path) {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry_path.strip_prefix(&path)?;

        if entry_path.is_file() {
            zip.start_file_from_path(entry_name, options)?;
            let mut f = File::open(entry_path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;

            buffer.clear();
        } else {
            zip.add_directory_from_path(entry_name, options)?;
        }
    }

    Ok(zip.finish()?)
}

/// Runs a command using the OS specific shell and current working directory
/// Output is not captured, so it's forwarded to our stdout/stderr
pub fn shell_exec(command: &str) -> io::Result<ExitStatus> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(&command).status()
    } else {
        Command::new("sh").arg("-c").arg(&command).status()
    }
}
