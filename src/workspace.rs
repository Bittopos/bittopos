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

/// Representación de transición para reconocer el esquema anterior sin
/// deserializarlo directamente como el modelo definitivo.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWorkspace {
    #[serde(default)]
    instances: Vec<RawInstance>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawInstance {
    id: Option<Uuid>,
    source: Option<Source>,
    resource_id: Option<Uuid>,
}

#[derive(Debug)]
pub enum WorkspaceError {
    Io(io::Error),
    Json(serde_json::Error),
    Resource(StoreError),
    InvalidName,
    InvalidInstanceFormat,
    NotFound(String),
    UnresolvedReference(Uuid),
    DuplicateInstanceId(Uuid),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "error de E/S: {error}"),
            Self::Json(error) => write!(f, "JSON inválido: {error}"),
            Self::Resource(error) => write!(f, "error de recurso: {error}"),
            Self::InvalidName => write!(f, "nombre de workspace inválido"),
            Self::InvalidInstanceFormat => write!(
                f,
                "formato de instancia inválido o mezcla de esquemas antiguo y nuevo"
            ),
            Self::NotFound(name) => write!(f, "no existe el workspace {name}"),
            Self::UnresolvedReference(id) => {
                write!(f, "no se puede resolver la referencia al recurso {id}")
            }
            Self::DuplicateInstanceId(id) => {
                write!(f, "el ID de instancia {id} está duplicado en el workspace")
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

    /// Carga un workspace. Los JSON antiguos se migran automáticamente solo
    /// después de convertirlos por completo y validar todos sus recursos de origen.
    pub fn load(
        &self,
        name: &str,
        resources: &ResourceStore,
    ) -> Result<Workspace, WorkspaceError> {
        let path = self.path_for(name)?;

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(WorkspaceError::NotFound(name.to_owned()));
            }
            Err(error) => return Err(WorkspaceError::Io(error)),
        };

        let raw: RawWorkspace = serde_json::from_str(&content)?;
        let is_legacy = !raw.instances.is_empty()
            && raw.instances.iter().all(|instance| {
                instance.id.is_none()
                    && instance.source.is_none()
                    && instance.resource_id.is_some()
            });

        let workspace = if is_legacy {
            let mut migrated = Workspace::new();
            for raw_instance in raw.instances {
                let resource_id = raw_instance
                    .resource_id
                    .ok_or(WorkspaceError::InvalidInstanceFormat)?;
                migrated.instances.push(Instance {
                    id: Uuid::now_v7(),
                    source: Source { id: resource_id },
                });
            }
            migrated
        } else {
            let mut instances = Vec::with_capacity(raw.instances.len());
            for raw_instance in raw.instances {
                match (raw_instance.id, raw_instance.source, raw_instance.resource_id) {
                    (Some(id), Some(source), None) => instances.push(Instance { id, source }),
                    _ => return Err(WorkspaceError::InvalidInstanceFormat),
                }
            }
            Workspace { instances }
        };

        workspace.validate_instance_ids()?;

        // Validamos todas las referencias antes de cambiar asignaciones o migrar el JSON.
        for instance in &workspace.instances {
            match resources.get(instance.source.id) {
                Ok(_) => {}
                Err(StoreError::NotFound(id)) => {
                    return Err(WorkspaceError::UnresolvedReference(id));
                }
                Err(error) => return Err(WorkspaceError::Resource(error)),
            }
        }

        // Reconcilia el registro administrativo con las instancias del workspace.
        // Solo se revierten las asignaciones que esta carga acaba de añadir.
        let added_allocations = Self::ensure_allocations(&workspace, resources)?;

        if is_legacy {
            // La migración se escribe solo después de validar y reconciliar el registro.
            if let Err(error) = self.save(name, &workspace) {
                Self::rollback_allocations(resources, &added_allocations);
                return Err(error);
            }
        }

        Ok(workspace)
    }

    fn ensure_allocations(
        workspace: &Workspace,
        resources: &ResourceStore,
    ) -> Result<Vec<(Uuid, Uuid)>, WorkspaceError> {
        let mut added = Vec::new();

        for instance in &workspace.instances {
            match resources.ensure_instance_registered(instance.source.id, instance.id) {
                Ok(true) => added.push((instance.source.id, instance.id)),
                Ok(false) => {}
                Err(error) => {
                    Self::rollback_allocations(resources, &added);
                    return Err(WorkspaceError::Resource(error));
                }
            }
        }

        Ok(added)
    }

    fn rollback_allocations(resources: &ResourceStore, added: &[(Uuid, Uuid)]) {
        for (resource_id, instance_id) in added.iter().rev() {
            let _ = resources.unregister_instance(*resource_id, *instance_id);
        }
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
        let loaded = store.load("proyecto", &resources).unwrap();
        assert_eq!(loaded, workspace);
        assert_eq!(
            resources.get(resource.id).unwrap().management.allocation,
            vec![instance_id]
        );

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
    fn migrates_legacy_workspace_once_and_persists_instance_ids() {
        let (_dir, resources, store) = setup();
        let (resource, _) = resources.create("Antiguo", None).unwrap();
        let path = store.root().join("proyecto.json");
        let legacy = format!(
            r#"{{ "instances": [{{ "resource_id": "{}" }}] }}"#,
            resource.id
        );
        fs::write(&path, &legacy).unwrap();

        let migrated = store.load("proyecto", &resources).unwrap();
        let instance_id = migrated.instances[0].id;
        assert_eq!(migrated.instances[0].source.id, resource.id);
        assert_eq!(
            resources.get(resource.id).unwrap().management.allocation,
            vec![instance_id]
        );

        let persisted = fs::read_to_string(&path).unwrap();
        assert!(!persisted.contains(r#""resource_id""#));
        assert!(persisted.contains(&instance_id.to_string()));

        let loaded_again = store.load("proyecto", &resources).unwrap();
        assert_eq!(loaded_again.instances[0].id, instance_id);
    }

    #[test]
    fn does_not_overwrite_legacy_json_when_reference_is_missing() {
        let (_dir, resources, store) = setup();
        let path = store.root().join("proyecto.json");
        let missing_id = Uuid::now_v7();
        let legacy = format!(
            r#"{{ "instances": [{{ "resource_id": "{}" }}] }}"#,
            missing_id
        );
        fs::write(&path, &legacy).unwrap();

        assert!(matches!(
            store.load("proyecto", &resources),
            Err(WorkspaceError::UnresolvedReference(id)) if id == missing_id
        ));
        assert_eq!(fs::read_to_string(&path).unwrap(), legacy);
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

        let loaded = workspaces.load("prueba", &resources).unwrap();
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
            store.load("../otro", &resources),
            Err(WorkspaceError::InvalidName)
        ));
    }
}
