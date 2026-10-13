use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    Instance, ResourceStore, StoreError, Workspace, WorkspaceError, WorkspaceStore,
};

#[derive(Debug)]
pub enum LifecycleError {
    Resource(StoreError),
    Workspace(WorkspaceError),
    InstanceNotFound(Uuid),
    DuplicateStructuralId(Uuid),
    StructuralMissing { resource_id: Uuid, instance_id: Uuid },
    JournalIo(std::io::Error),
    JournalJson(serde_json::Error),
    JournalConflict(String),
    ParentRequired,
}

impl fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resource(error) => write!(f, "error de recurso: {error}"),
            Self::Workspace(error) => write!(f, "error de workspace: {error}"),
            Self::InstanceNotFound(id) => write!(f, "no existe la instancia {id}"),
            Self::DuplicateStructuralId(id) => write!(f, "la instancia {id} está registrada en más de un recurso"),
            Self::StructuralMissing { resource_id, instance_id } => write!(
                f, "la instancia {instance_id} no figura en structural del recurso {resource_id}"
            ),
            Self::JournalIo(error) => write!(f, "error de E/S del journal: {error}"),
            Self::JournalJson(error) => write!(f, "journal inválido: {error}"),
            Self::JournalConflict(message) => write!(f, "no se puede recuperar la operación pendiente: {message}"),
            Self::ParentRequired => write!(f, "la instancia padre indicada no existe"),
        }
    }
}
impl std::error::Error for LifecycleError {}
impl From<StoreError> for LifecycleError {
    fn from(error: StoreError) -> Self { Self::Resource(error) }
}
impl From<WorkspaceError> for LifecycleError {
    fn from(error: WorkspaceError) -> Self { Self::Workspace(error) }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecoveryReport {
    pub restored: usize,
    pub removed: usize,
}
impl RecoveryReport {
    pub fn changed(self) -> bool { self.restored > 0 || self.removed > 0 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
enum PendingOperation {
    Create {
        instance: Instance,
        parent_id: Option<Uuid>,
    },
    Remove {
        instance: Instance,
        parent_id: Option<Uuid>,
    },
}

fn journal_path(workspaces: &WorkspaceStore, workspace_name: &str) -> Result<PathBuf, WorkspaceError> {
    if workspace_name.trim().is_empty()
        || workspace_name == "."
        || workspace_name == ".."
        || workspace_name.contains('/')
        || workspace_name.contains('\\\\')
        || workspace_name.contains('\\0')
    {
        return Err(WorkspaceError::InvalidName);
    }
    Ok(workspaces.root().join(format!(".{workspace_name}.journal.json")))
}

fn persist_journal(workspaces: &WorkspaceStore, workspace_name: &str, operation: &PendingOperation) -> Result<(), LifecycleError> {
    let path = journal_path(workspaces, workspace_name)?;
    let temp = workspaces.root().join(format!(".journal.{}.tmp", Uuid::now_v7()));
    let bytes = serde_json::to_vec_pretty(operation).map_err(LifecycleError::JournalJson)?;
    crate::workspace::atomic_write(&temp, &path, &bytes)?;
    Ok(())
}

fn clear_journal(workspaces: &WorkspaceStore, workspace_name: &str) -> Result<(), LifecycleError> {
    let path = journal_path(workspaces, workspace_name)?;
    match fs::remove_file(&path) {
        Ok(()) => {
            fs::File::open(workspaces.root()).and_then(|directory| directory.sync_all())
                .map_err(LifecycleError::JournalIo)?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(LifecycleError::JournalIo(error)),
    }
    Ok(())
}

fn read_journal(workspaces: &WorkspaceStore, workspace_name: &str) -> Result<Option<PendingOperation>, LifecycleError> {
    let path = journal_path(workspaces, workspace_name)?;
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).map(Some).map_err(LifecycleError::JournalJson),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(LifecycleError::JournalIo(error)),
    }
}

fn apply_pending(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    operation: PendingOperation,
) -> Result<(), LifecycleError> {
    match operation {
        PendingOperation::Create { instance, parent_id } => {
            let resource = resources.get(instance.source.id)?;
            if !resource.management.structural.contains(&instance.id) {
                resources.register_instance(instance.source.id, instance.id)?;
            }
            let mut workspace = load_workspace_or_empty(workspaces, workspace_name)?;
            if let Some(existing) = workspace.find_instance(instance.id) {
                if existing != &instance || workspace.parent_of(instance.id) != parent_id {
                    return Err(LifecycleError::JournalConflict(format!(
                        "la instancia {} ya existe en una ubicación o con datos distintos", instance.id
                    )));
                }
            } else {
                match parent_id {
                    None => workspace.insert_root(instance.clone())?,
                    Some(parent) => workspace.insert_child(parent, instance.clone())?,
                }
            }
            workspace.validate()?;
            workspaces.save(workspace_name, &workspace)?;
        }
        PendingOperation::Remove { instance, parent_id } => {
            let resource = resources.get(instance.source.id)?;
            if resource.management.structural.contains(&instance.id) {
                resources.unregister_instance(instance.source.id, instance.id)?;
            }
            let mut workspace = load_workspace_or_empty(workspaces, workspace_name)?;
            if let Some(existing) = workspace.find_instance(instance.id) {
                if existing.source.id != instance.source.id || workspace.parent_of(instance.id) != parent_id {
                    return Err(LifecycleError::JournalConflict(format!(
                        "la instancia {} ya no coincide con el origen o padre registrado", instance.id
                    )));
                }
                workspace.remove_leaf(instance.id)?;
                workspaces.save(workspace_name, &workspace)?;
            }
        }
    }
    clear_journal(workspaces, workspace_name)?;
    Ok(())
}

fn load_workspace_or_empty(workspaces: &WorkspaceStore, name: &str) -> Result<Workspace, LifecycleError> {
    match workspaces.load(name) {
        Ok(workspace) => Ok(workspace),
        Err(WorkspaceError::NotFound(_)) => Ok(Workspace::new()),
        Err(error) => Err(LifecycleError::Workspace(error)),
    }
}

/// Resuelve primero cualquier operación interrumpida y después valida las dos vistas persistidas.
/// Sin journal, una discrepancia no permite inventar la ubicación de una instancia.
pub fn reconcile_workspace(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
) -> Result<(Workspace, RecoveryReport), LifecycleError> {
    let mut report = RecoveryReport::default();
    if let Some(operation) = read_journal(workspaces, workspace_name)? {
        match &operation {
            PendingOperation::Create { .. } => report.restored += 1,
            PendingOperation::Remove { .. } => report.removed += 1,
        }
        apply_pending(resources, workspaces, workspace_name, operation)?;
    }

    let definitions = resources.all_strict()?;
    let mut structural_ids: HashMap<Uuid, Uuid> = HashMap::new();
    for resource in &definitions {
        for instance_id in &resource.management.structural {
            if structural_ids.insert(*instance_id, resource.id).is_some() {
                return Err(LifecycleError::DuplicateStructuralId(*instance_id));
            }
        }
    }

    let workspace = match workspaces.load(workspace_name) {
        Ok(workspace) => workspace,
        Err(WorkspaceError::NotFound(_)) => Workspace::new(),
        Err(error) => return Err(LifecycleError::Workspace(error)),
    };
    workspace.validate()?;
    for instance in workspace.all_instances() {
        match structural_ids.get(&instance.id) {
            Some(source_id) if *source_id == instance.source.id => {}
            Some(_) => return Err(LifecycleError::JournalConflict(format!(
                "el registro structural de {} apunta a un recurso distinto", instance.id
            ))),
            None => return Err(LifecycleError::JournalConflict(format!(
                "la instancia {} existe en el workspace pero no en management.structural y no hay journal para resolverlo", instance.id
            ))),
        }
    }
    for (instance_id, source_id) in structural_ids {
        if workspace.find_instance(instance_id).is_none() {
            return Err(LifecycleError::JournalConflict(format!(
                "la instancia {instance_id} figura en structural del recurso {source_id}, pero no está en el workspace y no hay journal para recuperar su ubicación"
            )));
        }
    }
    for instance in workspace.all_instances() {
        resources.get(instance.source.id).map_err(LifecycleError::Resource)?;
    }
    Ok((workspace, report))
}

/// Crea una instancia raíz o una instancia hija. El journal se confirma antes de tocar TOML o JSON.
pub fn create_instance(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    resource_id: Uuid,
) -> Result<Uuid, LifecycleError> {
    create_instance_under(resources, workspaces, workspace_name, resource_id, None)
}

pub fn create_instance_under(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    resource_id: Uuid,
    parent_id: Option<Uuid>,
) -> Result<Uuid, LifecycleError> {
    let (workspace, _) = reconcile_workspace(resources, workspaces, workspace_name)?;
    resources.get(resource_id)?;
    if let Some(parent) = parent_id {
        if workspace.find_instance(parent).is_none() { return Err(LifecycleError::ParentRequired); }
    }
    let instance = Instance::new(Uuid::now_v7(), resource_id);
    let operation = PendingOperation::Create { instance: instance.clone(), parent_id };
    persist_journal(workspaces, workspace_name, &operation)?;
    apply_pending(resources, workspaces, workspace_name, operation)?;
    Ok(instance.id)
}

/// Remueve solo instancias hoja, localizándolas recursivamente. Conserva el recurso de origen.
pub fn remove_instance(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    instance_id: Uuid,
) -> Result<(), LifecycleError> {
    let (workspace, _) = reconcile_workspace(resources, workspaces, workspace_name)?;
    let instance = workspace.find_instance(instance_id).cloned()
        .ok_or(LifecycleError::InstanceNotFound(instance_id))?;
    if !instance.children().is_empty() {
        return Err(LifecycleError::Workspace(WorkspaceError::InstanceHasChildren(instance_id)));
    }
    let parent_id = workspace.parent_of(instance_id);
    let resource = resources.get(instance.source.id)?;
    if !resource.management.structural.contains(&instance_id) {
        return Err(LifecycleError::StructuralMissing { resource_id: instance.source.id, instance_id });
    }
    let operation = PendingOperation::Remove { instance, parent_id };
    persist_journal(workspaces, workspace_name, &operation)?;
    apply_pending(resources, workspaces, workspace_name, operation)
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
    fn creates_nested_instances_and_keeps_both_containment_views() {
        let (_dir, resources, workspaces) = setup();
        let (root_resource, _) = resources.create("Raíz", None).unwrap();
        let (child_resource, _) = resources.create("Hija", None).unwrap();
        let root_id = create_instance(&resources, &workspaces, "proyecto", root_resource.id).unwrap();
        let child_id = create_instance_under(&resources, &workspaces, "proyecto", child_resource.id, Some(root_id)).unwrap();
        let workspace = workspaces.load("proyecto").unwrap();
        let root = workspace.find_instance(root_id).unwrap();
        assert_eq!(root.child_ids(), &[child_id]);
        assert_eq!(root.children().len(), 1);
        assert_eq!(root.children()[0].source.id, child_resource.id);
        assert!(resources.get(root_resource.id).unwrap().management.structural.contains(&root_id));
        assert!(resources.get(child_resource.id).unwrap().management.structural.contains(&child_id));
    }

    #[test]
    fn removes_nested_leaf_but_rejects_parent() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let root = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();
        let child = create_instance_under(&resources, &workspaces, "proyecto", resource.id, Some(root)).unwrap();
        assert!(remove_instance(&resources, &workspaces, "proyecto", root).is_err());
        remove_instance(&resources, &workspaces, "proyecto", child).unwrap();
        let workspace = workspaces.load("proyecto").unwrap();
        assert_eq!(workspace.all_instances().len(), 1);
        assert!(workspace.find_instance(root).unwrap().children().is_empty());
        assert!(!resources.get(resource.id).unwrap().management.structural.contains(&child));
        assert!(resources.get(resource.id).unwrap().management.structural.contains(&root));
    }

    #[test]
    fn journal_recovers_creation_after_resource_write() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let instance = Instance::new(Uuid::now_v7(), resource.id);
        persist_journal(&workspaces, "proyecto", &PendingOperation::Create { instance: instance.clone(), parent_id: None }).unwrap();
        resources.register_instance(resource.id, instance.id).unwrap();
        let (workspace, report) = reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();
        assert!(workspace.find_instance(instance.id).is_some());
        assert_eq!(report.restored, 1);
        assert!(!journal_path(&workspaces, "proyecto").unwrap().exists());
    }

    #[test]
    fn does_not_guess_missing_parent_without_journal() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let id = Uuid::now_v7();
        resources.register_instance(resource.id, id).unwrap();
        assert!(matches!(
            reconcile_workspace(&resources, &workspaces, "proyecto"),
            Err(LifecycleError::JournalConflict(_))
        ));
    }

    #[test]
    fn refuses_to_reconcile_corrupt_journal() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let path = workspaces.root().join(".proyecto.journal.json");
        fs::write(&path, "corrupto").unwrap();
        assert!(matches!(reconcile_workspace(&resources, &workspaces, "proyecto"), Err(LifecycleError::JournalJson(_))));
        assert!(path.exists());
    }
}
