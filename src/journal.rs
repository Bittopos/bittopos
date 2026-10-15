//! Persistencia del registro de operaciones pendientes del ciclo de vida.
//!
//! Este módulo conoce el formato y las rutas del journal; `lifecycle` decide cómo
//! aplicar y validar semánticamente las operaciones registradas.

use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::persistence::atomic_write;
use crate::workspace::{PersistedInstance, WorkspaceError, WorkspaceStore};

#[derive(Debug)]
pub(crate) enum JournalError {
    Io(io::Error),
    Json(serde_json::Error),
    InvalidName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum PendingOperation {
    Create {
        instance: PersistedInstance,
        parent_id: Option<Uuid>,
    },
    Remove {
        instance: PersistedInstance,
        parent_id: Option<Uuid>,
    },
}

pub(crate) fn path(
    workspaces: &WorkspaceStore,
    workspace_name: &str,
) -> Result<PathBuf, JournalError> {
    if workspace_name.trim().is_empty()
        || workspace_name == "."
        || workspace_name == ".."
        || workspace_name.contains('/')
        || workspace_name.contains('\\')
        || workspace_name.contains('\0')
    {
        return Err(JournalError::InvalidName);
    }
    Ok(workspaces
        .root()
        .join(format!(".{workspace_name}.journal.json")))
}

pub(crate) fn persist(
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    operation: &PendingOperation,
) -> Result<(), JournalError> {
    let path = path(workspaces, workspace_name)?;
    let temp = workspaces
        .root()
        .join(format!(".journal.{}.tmp", Uuid::now_v7()));
    let bytes = serde_json::to_vec_pretty(operation).map_err(JournalError::Json)?;
    atomic_write(&temp, &path, &bytes).map_err(JournalError::Io)?;
    Ok(())
}

pub(crate) fn clear(
    workspaces: &WorkspaceStore,
    workspace_name: &str,
) -> Result<(), JournalError> {
    let path = path(workspaces, workspace_name)?;
    match fs::remove_file(&path) {
        Ok(()) => {
            fs::File::open(workspaces.root())
                .and_then(|directory| directory.sync_all())
                .map_err(JournalError::Io)?;
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(JournalError::Io(error)),
    }
    Ok(())
}

pub(crate) fn read(
    workspaces: &WorkspaceStore,
    workspace_name: &str,
) -> Result<Option<PendingOperation>, JournalError> {
    let path = path(workspaces, workspace_name)?;
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content)
            .map(Some)
            .map_err(JournalError::Json),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(JournalError::Io(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn journal_path_rejects_names_that_escape_workspace_directory() {
        let dir = tempfile::tempdir().unwrap();
        let workspaces = WorkspaceStore::new(dir.path()).unwrap();

        for name in ["", "  ", ".", "..", "../outside", "nested/name", "nested\\name"] {
            assert!(matches!(path(&workspaces, name), Err(JournalError::InvalidName)));
        }
    }

    #[test]
    fn journal_round_trips_operation_without_changing_path_or_format() {
        let dir = tempfile::tempdir().unwrap();
        let workspaces = WorkspaceStore::new(Path::new(dir.path())).unwrap();
        let operation = PendingOperation::Remove {
            instance: PersistedInstance {
                id: Uuid::now_v7(),
                source: crate::workspace::PersistedSource { id: Uuid::now_v7() },
                semantic: None,
                content: None,
            },
            parent_id: None,
        };

        persist(&workspaces, "proyecto", &operation).unwrap();
        assert_eq!(read(&workspaces, "proyecto").unwrap().map(|value| {
            serde_json::to_value(value).unwrap()
        }), Some(serde_json::to_value(operation).unwrap()));
        assert!(path(&workspaces, "proyecto").unwrap().exists());
        clear(&workspaces, "proyecto").unwrap();
        assert!(read(&workspaces, "proyecto").unwrap().is_none());
    }
}
