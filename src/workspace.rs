use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Resource, ResourceStore, StoreError};

/// Referencia protegida al recurso del que procede una instancia.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub id: Uuid,
}

/// Aparición concreta de un recurso dentro de un espacio de trabajo.
/// Su identidad no depende de la identidad del recurso de origen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instance {
    pub id: Uuid,
    pub source: Source,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workspace {
    #[serde(default)]
    pub instances: Vec<Instance>,
}

#[derive(Debug)]
pub enum WorkspaceError {
    Io(io::Error),
    Json(serde_json::Error),
    Resource(StoreError),
    InvalidName,
    NotFound(String),
    UnresolvedReference(Uuid),
    DuplicateInstanceId(Uuid),
    DuplicateAllocationId(Uuid),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "error de E/S: {error}"),
            Self::Json(error) => write!(f, "JSON inválido: {error}"),
            Self::Resource(error) => write!(f, "error de recurso: {error}"),
            Self::InvalidName => write!(f, "nombre de workspace inválido"),
            Self::NotFound(name) => write!(f, "no existe el workspace {name}"),
            Self::UnresolvedReference(id) => {
                write!(f, "no se puede resolver la referencia al recurso {id}")
            }
            Self::DuplicateInstanceId(id) => {
                write!(f, "el ID de instancia {id} está duplicado en el workspace")
            }
            Self::DuplicateAllocationId(id) => {
                write!(f, "el ID de instancia {id} aparece asignado a más de un recurso")
            }
        }
    }
}

impl std::error::Error for WorkspaceError {}

impl From<io::Error> for WorkspaceError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for WorkspaceError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<StoreError> for WorkspaceError {
    fn from(error: StoreError) -> Self {
        Self::Resource(error)
    }
}

impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea una instancia nueva de un recurso, incluso si ya hay otras
    /// instancias del mismo recurso en este workspace.
    pub fn add_resource(&mut self, resource_id: Uuid) -> Result<Uuid, WorkspaceError> {
        let instance_id = Uuid::now_v7();
        self.instances.push(Instance {
            id: instance_id,
            source: Source { id: resource_id },
        });
        Ok(instance_id)
    }

    /// Elimina una instancia por su identidad propia.
    pub fn remove_instance(&mut self, instance_id: Uuid) -> bool {
        let previous_len = self.instances.len();
        self.instances.retain(|instance| instance.id != instance_id);
        self.instances.len() != previous_len
    }

    /// Elimina todas las instancias cuyo origen sea el recurso indicado.
    pub fn remove_resource(&mut self, resource_id: Uuid) -> bool {
        let previous_len = self.instances.len();
        self.instances
            .retain(|instance| instance.source.id != resource_id);
        self.instances.len() != previous_len
    }

    pub fn resolve(&self, resources: &ResourceStore) -> Result<Vec<Resource>, WorkspaceError> {
        self.instances
            .iter()
            .map(|instance| match resources.get(instance.source.id) {
                Ok(resource) => Ok(resource),
                Err(StoreError::NotFound(id)) => Err(WorkspaceError::UnresolvedReference(id)),
                Err(error) => Err(WorkspaceError::Resource(error)),
            })
            .collect()
    }

    fn validate_instance_ids(&self) -> Result<(), WorkspaceError> {
        let mut seen = std::collections::HashSet::with_capacity(self.instances.len());
        for instance in &self.instances {
            if !seen.insert(instance.id) {
                return Err(WorkspaceError::DuplicateInstanceId(instance.id));
            }
        }
        Ok(())
    }
}

pub struct WorkspaceStore {
    root: PathBuf,
}

impl WorkspaceStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, WorkspaceError> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Carga exclusivamente el esquema vigente. Los formatos anteriores se rechazan;
    /// no se migran ni se sobrescriben automáticamente.
    pub fn load(&self, name: &str) -> Result<Workspace, WorkspaceError> {
        let path = self.path_for(name)?;
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(WorkspaceError::NotFound(name.to_owned()));
            }
            Err(error) => return Err(WorkspaceError::Io(error)),
        };

        let workspace: Workspace = serde_json::from_str(&content)?;
        workspace.validate_instance_ids()?;
        Ok(workspace)
    }

    pub fn save(&self, name: &str, workspace: &Workspace) -> Result<(), WorkspaceError> {
        workspace.validate_instance_ids()?;
        let path = self.path_for(name)?;
        let temporary_path = self.root.join(format!(".{}.{}.tmp", name, Uuid::now_v7()));
        let content = serde_json::to_vec_pretty(workspace)?;

        let result = (|| -> Result<(), WorkspaceError> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary_path)?;

            file.write_all(&content)?;
            file.sync_all()?;
            drop(file);

            // En Termux/Linux, ambos archivos están en el mismo directorio.
            // rename sustituye el destino sin exponer una escritura parcial.
            fs::rename(&temporary_path, &path)?;
            Ok(())
        })();

        if result.is_err() {
            let _ = fs::remove_file(&temporary_path);
        }

        result
    }

    pub fn list(&self) -> Result<Vec<String>, WorkspaceError> {
        let mut names = Vec::new();

        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                names.push(name.to_owned());
            }
        }

        names.sort();
        Ok(names)
    }

    fn path_for(&self, name: &str) -> Result<PathBuf, WorkspaceError> {
        if name.trim().is_empty()
            || name == "."
            || name == ".."
            || name.contains('/')
            || name.contains('\\')
            || name.contains('\0')
        {
            return Err(WorkspaceError::InvalidName);
        }

        Ok(self.root.join(format!("{name}.json")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (tempfile::TempDir, ResourceStore, WorkspaceStore) {
        let dir = tempfile::tempdir().unwrap();
        let resources = ResourceStore::new(dir.path().join("resources")).unwrap();
        let workspaces = WorkspaceStore::new(dir.path().join("workspaces")).unwrap();
        (dir, resources, workspaces)
    }

    #[test]
    fn saves_and_loads_workspace_json_with_distinct_instance_identity() {
        let (_dir, resources, store) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();

        let mut workspace = Workspace::new();
        let instance_id = workspace.add_resource(resource.id).unwrap();
        assert_ne!(instance_id, resource.id);

        store.save("proyecto", &workspace).unwrap();
        let loaded = store.load("proyecto").unwrap();
        assert_eq!(loaded, workspace);

        let json = fs::read_to_string(store.root().join("proyecto.json")).unwrap();
        assert!(json.contains(r#""id""#));
        assert!(json.contains(r#""source""#));
        assert!(json.contains(&format!(r#""id": "{}""#, instance_id)));
        assert!(json.contains(&format!(r#""id": "{}""#, resource.id)));
        assert!(!json.contains(r#""resource_id""#));
    }

    #[test]
    fn allows_multiple_instances_of_the_same_resource() {
        let (_dir, resources, store) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();

        let mut workspace = Workspace::new();
        let first = workspace.add_resource(resource.id).unwrap();
        let second = workspace.add_resource(resource.id).unwrap();

        assert_ne!(first, second);
        assert_eq!(workspace.instances.len(), 2);
        store.save("proyecto", &workspace).unwrap();
        assert_eq!(store.load("proyecto", &resources).unwrap(), workspace);
    }

    #[test]
    fn rejects_legacy_resource_id_without_modifying_the_file() {
        let (_dir, _resources, store) = setup();
        let path = store.root().join("proyecto.json");
        let legacy = r#"{"instances":[{"resource_id":"00000000-0000-7000-8000-000000000000"}]}"#;
        fs::write(&path, legacy).unwrap();

        assert!(matches!(store.load("proyecto"), Err(WorkspaceError::Json(_))));
        assert_eq!(fs::read_to_string(path).unwrap(), legacy);
    }

    #[test]
    fn rejects_duplicate_instance_ids() {
        let (_dir, resources, store) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let id = Uuid::now_v7();
        let workspace = Workspace {
            instances: vec![
                Instance { id, source: Source { id: resource.id } },
                Instance { id, source: Source { id: resource.id } },
            ],
        };

        assert!(matches!(
            store.save("proyecto", &workspace),
            Err(WorkspaceError::DuplicateInstanceId(found)) if found == id
        ));
    }

    #[test]
    fn resolves_current_resource_data() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Antes", None).unwrap();

        let mut workspace = Workspace::new();
        workspace.add_resource(resource.id).unwrap();
        workspaces.save("prueba", &workspace).unwrap();

        resources
            .update(
                resource.id,
                crate::ResourcePatch {
                    title: Some("Después".to_owned()),
                    ..Default::default()
                },
            )
            .unwrap();

        let loaded = workspaces.load("prueba").unwrap();
        let resolved = loaded.resolve(&resources).unwrap();

        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].info.title, "Después");
    }

    #[test]
    fn reports_missing_resource_reference() {
        let (_dir, resources, _workspaces) = setup();
        let mut workspace = Workspace::new();
        let missing_id = Uuid::now_v7();
        workspace.add_resource(missing_id).unwrap();

        assert!(matches!(
            workspace.resolve(&resources),
            Err(WorkspaceError::UnresolvedReference(id)) if id == missing_id
        ));
    }

    #[test]
    fn rejects_workspace_path_traversal() {
        let (_dir, resources, store) = setup();
        assert!(matches!(
            store.load("../otro"),
            Err(WorkspaceError::InvalidName)
        ));
    }
}
