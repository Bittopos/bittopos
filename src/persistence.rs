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
        io::Error::new(io::ErrorKind::InvalidInput, "ruta de destino sin directorio")
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
