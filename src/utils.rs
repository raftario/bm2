use std::io::Read;
use std::{
    fs::File,
    io::{Seek, Write},
    path::Path,
};
use walkdir::WalkDir;
use zip::{
    result::{ZipError, ZipResult},
    write::FileOptions,
    ZipWriter,
};

/// Zips a folder into the passed writer and returns it
pub fn zip_dir<P, W>(path: P, writer: W) -> ZipResult<()>
where
    P: AsRef<Path>,
    W: Write + Seek,
{
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default().unix_permissions(0o755);

    let mut buffer = Vec::new();
    let walkdir = WalkDir::new(&path).into_iter().filter_map(|e| e.ok());
    for entry in walkdir {
        let entry_path = entry.path();
        let entry_name = entry_path
            .strip_prefix(&path)
            .map_err(|_| ZipError::FileNotFound)?;

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

    zip.finish()?;
    Ok(())
}
