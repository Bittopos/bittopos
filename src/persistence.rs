//! Primitivas de persistencia compartidas por los almacenes locales.
//!
//! La escritura atómica protege un único archivo. No proporciona por sí sola una
//! transacción entre varios archivos; esas operaciones siguen coordinándose mediante
//! el journal del ciclo de vida.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Escribe un archivo temporal, sincroniza sus bytes y lo sustituye atómicamente.
///
/// El temporal debe estar en el mismo sistema de archivos que el destino para que
/// el renombrado sea atómico. La sincronización del directorio hace durable el cambio
/// de nombre en los sistemas que admiten esta operación.
pub(crate) fn atomic_write(
    temporary_path: &Path,
    destination: &Path,
    content: &[u8],
) -> io::Result<()> {
    let parent = destination.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "ruta de destino sin directorio",
        )
    })?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temporary_path)?;
    file.write_all(content)?;
    file.sync_all()?;
    drop(file);

    fs::rename(temporary_path, destination)?;
    OpenOptions::new().read(true).open(parent)?.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn atomically_writes_content_and_replaces_destination() {
        let dir = tempfile::tempdir().unwrap();
        let destination = dir.path().join("state.json");
        let temporary = dir.path().join(".state.tmp");

        atomic_write(&temporary, &destination, b"first").unwrap();
        assert_eq!(fs::read(&destination).unwrap(), b"first");

        let temporary = dir.path().join(".state-next.tmp");
        atomic_write(&temporary, &destination, b"second").unwrap();
        assert_eq!(fs::read(&destination).unwrap(), b"second");
        assert!(!temporary.exists());
    }

    #[test]
    fn refuses_to_overwrite_an_existing_temporary_file() {
        let dir = tempfile::tempdir().unwrap();
        let destination = dir.path().join("state.json");
        let temporary = dir.path().join(".state.tmp");
        fs::write(&destination, b"confirmed").unwrap();
        fs::write(&temporary, b"unconfirmed").unwrap();

        let error = atomic_write(&temporary, &destination, b"replacement").unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read(&destination).unwrap(), b"confirmed");
        assert_eq!(fs::read(&temporary).unwrap(), b"unconfirmed");
    }
}
