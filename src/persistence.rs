use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Creates a new file and makes its contents durable before returning.
pub(crate) fn write_new_file(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(content)?;
    file.sync_all()?;
    Ok(())
}

/// Atomically replaces the destination with a prepared file, then syncs its directory.
pub(crate) fn replace_file(temporary: &Path, destination: &Path) -> io::Result<()> {
    fs::rename(temporary, destination)?;
    sync_parent(destination)
}

/// Installs a file only if the destination does not already exist.
pub(crate) fn install_new_file(temporary: &Path, destination: &Path) -> io::Result<()> {
    fs::hard_link(temporary, destination)?;
    fs::remove_file(temporary)?;
    sync_parent(destination)
}

/// Persists a file through a temporary path and an atomic rename.
pub(crate) fn atomic_replace(
    temporary: &Path,
    destination: &Path,
    content: &[u8],
) -> io::Result<()> {
    write_new_file(temporary, content)?;
    replace_file(temporary, destination)
}

fn sync_parent(path: &Path) -> io::Result<()> {
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "destination has no parent directory")
    })?;
    OpenOptions::new().read(true).open(parent)?.sync_all()
}
