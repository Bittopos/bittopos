use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Resource, ResourceStore, StoreError};

/// Segmento físico que contiene instancias completas.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<Instance>,
}

/// Registro semántico de los IDs de los hijos directos.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Semantic {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocation: Vec<Uuid>,
}

/// Referencia protegida al recurso del que procede una instancia.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub id: Uuid,
}

/// Aparición concreta de un recurso dentro de un espacio de trabajo.
/// La identidad de instancia es independiente de la identidad de su recurso.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instance {
    pub id: Uuid,
    pub source: Source,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic: Option<Semantic>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
}

impl Instance {
    pub(crate) fn new(id: Uuid, source_id: Uuid) -> Self {
        Self {
            id,
            source: Source { id: source_id },
            semantic: None,
            content: None,
        }
    }

    pub fn children(&self) -> &[Instance] {
        self.content.as_ref().map_or(&[], |content| content.instances.as_slice())
    }

    pub fn child_ids(&self) -> &[Uuid] {
        self.semantic.as_ref().map_or(&[], |semantic| semantic.allocation.as_slice())
    }

    fn normalize_empty_children(&mut self) {
        if self.content.as_ref().is_some_and(|content| content.instances.is_empty()) {
            self.content = None;
        }
        if self.semantic.as_ref().is_some_and(|semantic| semantic.allocation.is_empty()) {
            self.semantic = None;
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workspace {
    #[serde(default)]
    pub content: Content,
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
    InvalidContainment(String),
    InstanceNotFound(Uuid),
    InstanceHasChildren(Uuid),
    ParentNotFound(Uuid),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "error de E/S: {error}"),
            Self::Json(error) => write!(f, "JSON inválido: {error}"),
            Self::Resource(error) => write!(f, "error de recurso: {error}"),
            Self::InvalidName => write!(f, "nombre de workspace inválido"),
            Self::NotFound(name) => write!(f, "no existe el workspace {name}"),
            Self::UnresolvedReference(id) => write!(f, "no se puede resolver la referencia al recurso {id}"),
            Self::DuplicateInstanceId(id) => write!(f, "el ID de instancia {id} está duplicado en el workspace"),
            Self::InvalidContainment(reason) => write!(f, "contención inválida: {reason}"),
            Self::InstanceNotFound(id) => write!(f, "no existe la instancia {id}"),
            Self::InstanceHasChildren(id) => write!(f, "no se puede remover la instancia {id}: todavía contiene instancias hijas"),
            Self::ParentNotFound(id) => write!(f, "no existe la instancia padre {id}"),
        }
    }
}

impl std::error::Error for WorkspaceError {}

impl From<io::Error> for WorkspaceError {
    fn from(error: io::Error) -> Self { Self::Io(error) }
}
impl From<serde_json::Error> for WorkspaceError {
    fn from(error: serde_json::Error) -> Self { Self::Json(error) }
}
impl From<StoreError> for WorkspaceError {
    fn from(error: StoreError) -> Self { Self::Resource(error) }
}

impl Workspace {
    pub fn new() -> Self { Self::default() }

    pub fn instances(&self) -> &[Instance] { &self.content.instances }

    pub fn find_instance(&self, id: Uuid) -> Option<&Instance> {
        find_in(&self.content.instances, id)
    }

    pub fn parent_of(&self, id: Uuid) -> Option<Uuid> {
        parent_in(&self.content.instances, id, None)
    }

    pub fn all_instances(&self) -> Vec<&Instance> {
        let mut output = Vec::new();
        collect_instances(&self.content.instances, &mut output);
        output
    }

    pub(crate) fn insert_root(&mut self, instance: Instance) -> Result<(), WorkspaceError> {
        if self.find_instance(instance.id).is_some() {
            return Err(WorkspaceError::DuplicateInstanceId(instance.id));
        }
        self.content.instances.push(instance);
        Ok(())
    }

    pub(crate) fn insert_child(&mut self, parent_id: Uuid, instance: Instance) -> Result<(), WorkspaceError> {
        if self.find_instance(instance.id).is_some() {
            return Err(WorkspaceError::DuplicateInstanceId(instance.id));
        }
        let parent = find_mut_in(&mut self.content.instances, parent_id)
            .ok_or(WorkspaceError::ParentNotFound(parent_id))?;
        parent.content.get_or_insert_with(Content::default).instances.push(instance);
        parent.semantic.get_or_insert_with(Semantic::default).allocation
            .push(parent.content.as_ref().unwrap().instances.last().unwrap().id);
        Ok(())
    }

    /// Remueve una instancia hoja a cualquier profundidad. Nunca elimina descendientes implícitamente.
    pub(crate) fn remove_leaf(&mut self, id: Uuid) -> Result<Instance, WorkspaceError> {
        remove_from(&mut self.content.instances, id)?.ok_or(WorkspaceError::InstanceNotFound(id))
    }

    pub fn resolve(&self, resources: &ResourceStore) -> Result<Vec<Resource>, WorkspaceError> {
        self.all_instances().into_iter().map(|instance| match resources.get(instance.source.id) {
            Ok(resource) => Ok(resource),
            Err(StoreError::NotFound(id)) => Err(WorkspaceError::UnresolvedReference(id)),
            Err(error) => Err(WorkspaceError::Resource(error)),
        }).collect()
    }

    pub fn validate(&self) -> Result<(), WorkspaceError> {
        let mut seen = HashSet::new();
        validate_instances(&self.content.instances, &mut seen)
    }
}

fn find_in(instances: &[Instance], id: Uuid) -> Option<&Instance> {
    for instance in instances {
        if instance.id == id { return Some(instance); }
        if let Some(found) = find_in(instance.children(), id) { return Some(found); }
    }
    None
}

fn find_mut_in(instances: &mut [Instance], id: Uuid) -> Option<&mut Instance> {
    for instance in instances {
        if instance.id == id { return Some(instance); }
        if let Some(found) = find_mut_in(instance.content.get_or_insert_with(Content::default).instances.as_mut_slice(), id) {
            return Some(found);
        }
    }
    None
}

fn parent_in(instances: &[Instance], id: Uuid, parent: Option<Uuid>) -> Option<Uuid> {
    for instance in instances {
        if instance.id == id { return parent; }
        if let Some(found) = parent_in(instance.children(), id, Some(instance.id)) { return Some(found); }
    }
    None
}

fn collect_instances<'a>(instances: &'a [Instance], output: &mut Vec<&'a Instance>) {
    for instance in instances {
        output.push(instance);
        collect_instances(instance.children(), output);
    }
}

fn remove_from(instances: &mut Vec<Instance>, id: Uuid) -> Result<Option<Instance>, WorkspaceError> {
    if let Some(index) = instances.iter().position(|instance| instance.id == id) {
        if !instances[index].children().is_empty() {
            return Err(WorkspaceError::InstanceHasChildren(id));
        }
        return Ok(Some(instances.remove(index)));
    }

    for parent in instances.iter_mut() {
        if let Some(removed) = remove_from(parent.content.get_or_insert_with(Content::default).instances.as_mut(), id)? {
            if let Some(semantic) = parent.semantic.as_mut() {
                semantic.allocation.retain(|child_id| *child_id != id);
            }
            parent.normalize_empty_children();
            return Ok(Some(removed));
        }
    }
    Ok(None)
}

fn validate_instances(instances: &[Instance], seen: &mut HashSet<Uuid>) -> Result<(), WorkspaceError> {
    for instance in instances {
        if !seen.insert(instance.id) {
            return Err(WorkspaceError::DuplicateInstanceId(instance.id));
        }
        let children = instance.children();
        let allocation = instance.child_ids();
        if children.is_empty() {
            if !allocation.is_empty() {
                return Err(WorkspaceError::InvalidContainment(format!(
                    "la instancia {} registra hijos semánticos pero no contiene hijos físicos", instance.id
                )));
            }
            continue;
        }
        if instance.semantic.is_none() {
            return Err(WorkspaceError::InvalidContainment(format!(
                "la instancia {} contiene hijos físicos, pero falta semantic.allocation", instance.id
            )));
        }
        let child_ids: Vec<_> = children.iter().map(|child| child.id).collect();
        if allocation != child_ids.as_slice() {
            return Err(WorkspaceError::InvalidContainment(format!(
                "semantic.allocation no coincide en contenido y orden con content.instances para {}", instance.id
            )));
        }
        validate_instances(children, seen)?;
    }
    Ok(())
}

pub struct WorkspaceStore { root: PathBuf }

impl WorkspaceStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, WorkspaceError> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path { &self.root }

    pub fn load(&self, name: &str) -> Result<Workspace, WorkspaceError> {
        let path = self.path_for(name)?;
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Err(WorkspaceError::NotFound(name.to_owned())),
            Err(error) => return Err(WorkspaceError::Io(error)),
        };
        let workspace: Workspace = serde_json::from_str(&content)?;
        workspace.validate()?;
        Ok(workspace)
    }

    pub fn save(&self, name: &str, workspace: &Workspace) -> Result<(), WorkspaceError> {
        workspace.validate()?;
        let path = self.path_for(name)?;
        let temporary_path = self.root.join(format!(".{name}.{}.tmp", Uuid::now_v7()));
        let content = serde_json::to_vec_pretty(workspace)?;
        atomic_write(&temporary_path, &path, &content)?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>, WorkspaceError> {
        let mut names = Vec::new();
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                    names.push(name.to_owned());
                }
            }
        }
        names.sort();
        Ok(names)
    }

    fn path_for(&self, name: &str) -> Result<PathBuf, WorkspaceError> {
        if name.trim().is_empty() || name == "." || name == ".." || name.contains('/')
            || name.contains('\\') || name.contains('\0') {
            return Err(WorkspaceError::InvalidName);
        }
        Ok(self.root.join(format!("{name}.json")))
    }
}

pub(crate) fn atomic_write(temporary_path: &Path, destination: &Path, content: &[u8]) -> Result<(), WorkspaceError> {
    let parent = destination.parent().ok_or(WorkspaceError::InvalidName)?;
    let mut file = OpenOptions::new().write(true).create_new(true).open(temporary_path)?;
    file.write_all(content)?;
    file.sync_all()?;
    drop(file);
    fs::rename(temporary_path, destination)?;
    FileSync::sync_directory(parent)?;
    Ok(())
}

struct FileSync;
impl FileSync {
    fn sync_directory(path: &Path) -> io::Result<()> {
        OpenOptions::new().read(true).open(path)?.sync_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup() -> (tempfile::TempDir, ResourceStore, WorkspaceStore) {
        let dir = tempfile::tempdir().unwrap();
        let resources = ResourceStore::new(dir.path().join("resources")).unwrap();
        let workspaces = WorkspaceStore::new(dir.path().join("workspaces")).unwrap();
        (dir, resources, workspaces)
    }

    #[test]
    fn serializes_recursive_physical_and_semantic_containment() {
        let (_dir, resources, store) = setup();
        let (root_resource, _) = resources.create("Raíz", None).unwrap();
        let (child_resource, _) = resources.create("Hija", None).unwrap();
        let root_id = Uuid::now_v7();
        let child_id = Uuid::now_v7();
        let mut root = Instance::new(root_id, root_resource.id);
        root.content = Some(Content { instances: vec![Instance::new(child_id, child_resource.id)] });
        root.semantic = Some(Semantic { allocation: vec![child_id] });
        let workspace = Workspace { content: Content { instances: vec![root] } };

        store.save("proyecto", &workspace).unwrap();
        let loaded = store.load("proyecto").unwrap();
        assert_eq!(loaded, workspace);
        let json = fs::read_to_string(store.root().join("proyecto.json")).unwrap();
        assert!(json.contains("\"content\""));
        assert!(json.contains("\"semantic\""));
        assert!(json.contains("\"allocation\""));
    }

    #[test]
    fn allows_empty_components_to_be_omitted_for_leaf_instances() {
        let (_dir, resources, store) = setup();
        let (resource, _) = resources.create("Hoja", None).unwrap();
        let workspace = Workspace { content: Content { instances: vec![Instance::new(Uuid::now_v7(), resource.id)] } };
        store.save("proyecto", &workspace).unwrap();
        let json = fs::read_to_string(store.root().join("proyecto.json")).unwrap();
        assert!(!json.contains("\"semantic\""));
        assert!(!json.contains("\"content\""));
    }

    #[test]
    fn rejects_semantic_physical_mismatch() {
        let (_dir, resources, store) = setup();
        let (resource, _) = resources.create("Raíz", None).unwrap();
        let child = Instance::new(Uuid::now_v7(), resource.id);
        let mut root = Instance::new(Uuid::now_v7(), resource.id);
        root.content = Some(Content { instances: vec![child] });
        root.semantic = Some(Semantic { allocation: vec![Uuid::now_v7()] });
        let workspace = Workspace { content: Content { instances: vec![root] } };
        assert!(matches!(store.save("proyecto", &workspace), Err(WorkspaceError::InvalidContainment(_))));
    }

    #[test]
    fn rejects_duplicate_ids_across_different_levels() {
        let (_dir, resources, store) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let id = Uuid::now_v7();
        let mut root = Instance::new(Uuid::now_v7(), resource.id);
        root.content = Some(Content { instances: vec![Instance::new(id, resource.id)] });
        root.semantic = Some(Semantic { allocation: vec![id] });
        let workspace = Workspace { content: Content { instances: vec![root, Instance::new(id, resource.id)] } };
        assert!(matches!(store.save("proyecto", &workspace), Err(WorkspaceError::DuplicateInstanceId(found)) if found == id));
    }

    #[test]
    fn rejects_legacy_flat_workspace_without_modifying_the_file() {
        let (_dir, _resources, store) = setup();
        let path = store.root().join("proyecto.json");
        let legacy = r#"{\"instances\":[]}"#;
        fs::write(&path, legacy).unwrap();
        assert!(matches!(store.load("proyecto"), Err(WorkspaceError::Json(_))));
        assert_eq!(fs::read_to_string(path).unwrap(), legacy);
    }

    #[test]
    fn removes_leaf_at_arbitrary_depth_and_updates_both_views() {
        let (_dir, resources, _store) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let leaf_id = Uuid::now_v7();
        let mut child = Instance::new(Uuid::now_v7(), resource.id);
        child.content = Some(Content { instances: vec![Instance::new(leaf_id, resource.id)] });
        child.semantic = Some(Semantic { allocation: vec![leaf_id] });
        let mut root = Instance::new(Uuid::now_v7(), resource.id);
        root.content = Some(Content { instances: vec![child] });
        root.semantic = Some(Semantic { allocation: vec![root.content.as_ref().unwrap().instances[0].id] });
        let mut workspace = Workspace { content: Content { instances: vec![root] } };

        workspace.remove_leaf(leaf_id).unwrap();
        workspace.validate().unwrap();
        assert_eq!(workspace.all_instances().len(), 2);
        assert!(workspace.find_instance(leaf_id).is_none());
        assert!(workspace.find_instance(workspace.content.instances[0].content.as_ref().unwrap().instances[0].id).unwrap().children().is_empty());
    }

    #[test]
    fn refuses_to_remove_parent_with_children() {
        let (_dir, resources, _store) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let child_id = Uuid::now_v7();
        let mut root = Instance::new(Uuid::now_v7(), resource.id);
        root.content = Some(Content { instances: vec![Instance::new(child_id, resource.id)] });
        root.semantic = Some(Semantic { allocation: vec![child_id] });
        let mut workspace = Workspace { content: Content { instances: vec![root] } };
        let parent_id = workspace.content.instances[0].id;
        assert!(matches!(workspace.remove_leaf(parent_id), Err(WorkspaceError::InstanceHasChildren(id)) if id == parent_id));
        assert!(workspace.find_instance(child_id).is_some());
    }
}
