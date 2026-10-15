use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use crate::journal::{self, JournalError, PendingOperation};
use crate::workspace::PersistedInstance;
use crate::{
    Instance, ResourceStore, StoreError, Workspace, WorkspaceError, WorkspaceModelError,
    WorkspaceStore,
};

#[derive(Debug)]
pub enum LifecycleError {
    Resource(StoreError),
    Workspace(WorkspaceError),
    InstanceNotFound(Uuid),
    DuplicateStructuralId(Uuid),
    StructuralMissing {
        resource_id: Uuid,
        instance_id: Uuid,
    },
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
            Self::DuplicateStructuralId(id) => {
                write!(f, "la instancia {id} está registrada en más de un recurso")
            }
            Self::StructuralMissing {
                resource_id,
                instance_id,
            } => write!(
                f,
                "la instancia {instance_id} no figura en structural del recurso {resource_id}"
            ),
            Self::JournalIo(error) => write!(f, "error de E/S del journal: {error}"),
            Self::JournalJson(error) => write!(f, "journal inválido: {error}"),
            Self::JournalConflict(message) => {
                write!(f, "no se puede recuperar la operación pendiente: {message}")
            }
            Self::ParentRequired => write!(f, "la instancia padre indicada no existe"),
        }
    }
}
impl std::error::Error for LifecycleError {}

impl From<JournalError> for LifecycleError {
    fn from(error: JournalError) -> Self {
        match error {
            JournalError::Io(error) => Self::JournalIo(error),
            JournalError::Json(error) => Self::JournalJson(error),
            JournalError::InvalidName => Self::Workspace(WorkspaceError::InvalidName),
        }
    }
}
impl From<StoreError> for LifecycleError {
    fn from(error: StoreError) -> Self {
        Self::Resource(error)
    }
}
impl From<WorkspaceError> for LifecycleError {
    fn from(error: WorkspaceError) -> Self {
        Self::Workspace(error)
    }
}
impl From<WorkspaceModelError> for LifecycleError {
    fn from(error: WorkspaceModelError) -> Self {
        Self::Workspace(WorkspaceError::from(error))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecoveryReport {
    pub restored: usize,
    pub removed: usize,
}
impl RecoveryReport {
    pub fn changed(self) -> bool {
        self.restored > 0 || self.removed > 0
    }
}

fn apply_pending(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    operation: PendingOperation,
) -> Result<(), LifecycleError> {
    match operation {
        PendingOperation::Create {
            instance: persisted_instance,
            parent_id,
        } => {
            let instance = Instance::try_from(persisted_instance)?;
            let resource = resources.get(instance.source.id)?;
            if !resource.management.structural.contains(&instance.id) {
                resources.register_instance(instance.source.id, instance.id)?;
            }
            let mut workspace = load_workspace_or_empty(workspaces, workspace_name)?;
            if let Some(existing) = workspace.find_instance(instance.id) {
                if existing != &instance || workspace.parent_of(instance.id) != parent_id {
                    return Err(LifecycleError::JournalConflict(format!(
                        "la instancia {} ya existe en una ubicación o con datos distintos",
                        instance.id
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
        PendingOperation::Remove {
            instance: persisted_instance,
            parent_id,
        } => {
            let instance = Instance::try_from(persisted_instance)?;
            let resource = resources.get(instance.source.id)?;
            let registered = resource.management.structural.contains(&instance.id);
            let mut workspace = load_workspace_or_empty(workspaces, workspace_name)?;
            let existing = workspace.find_instance(instance.id).cloned();

            match (registered, existing) {
                (true, None) => {
                    // Si el registro sigue presente pero la instancia física no existe,
                    // no podemos afirmar que la remoción se haya completado.
                    return Err(LifecycleError::JournalConflict(format!(
                        "la instancia {} sigue registrada, pero no existe en el workspace",
                        instance.id
                    )));
                }
                (false, None) => {
                    // Ambos efectos ya se aplicaron; solo queda retirar el journal.
                }
                (is_registered, Some(existing)) => {
                    if existing.source.id != instance.source.id
                        || workspace.parent_of(instance.id) != parent_id
                    {
                        return Err(LifecycleError::JournalConflict(format!(
                            "la instancia {} ya no coincide con el origen o padre registrado",
                            instance.id
                        )));
                    }
                    if !existing.children().is_empty() {
                        return Err(LifecycleError::Workspace(
                            WorkspaceError::InstanceHasChildren(instance.id),
                        ));
                    }
                    // Validar la ubicación y que sea hoja antes de tocar el recurso.
                    if is_registered {
                        resources.unregister_instance(instance.source.id, instance.id)?;
                    }
                    workspace.remove_leaf(instance.id)?;
                    workspaces.save(workspace_name, &workspace)?;
                }
            }
        }
    }
    journal::clear(workspaces, workspace_name).map_err(LifecycleError::from)?;
    Ok(())
}

fn load_workspace_or_empty(
    workspaces: &WorkspaceStore,
    name: &str,
) -> Result<Workspace, LifecycleError> {
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
    if let Some(operation) = journal::read(workspaces, workspace_name).map_err(LifecycleError::from)? {
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
        resources
            .get(instance.source.id)
            .map_err(LifecycleError::Resource)?;
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
        if workspace.find_instance(parent).is_none() {
            return Err(LifecycleError::ParentRequired);
        }
    }
    let instance = Instance::new(Uuid::now_v7(), resource_id);
    let operation = PendingOperation::Create {
        instance: PersistedInstance::from(instance.clone()),
        parent_id,
    };
    journal::persist(workspaces, workspace_name, &operation).map_err(LifecycleError::from)?;
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
    let instance = workspace
        .find_instance(instance_id)
        .cloned()
        .ok_or(LifecycleError::InstanceNotFound(instance_id))?;
    if !instance.children().is_empty() {
        return Err(LifecycleError::Workspace(
            WorkspaceError::InstanceHasChildren(instance_id),
        ));
    }
    let parent_id = workspace.parent_of(instance_id);
    let resource = resources.get(instance.source.id)?;
    if !resource.management.structural.contains(&instance_id) {
        return Err(LifecycleError::StructuralMissing {
            resource_id: instance.source.id,
            instance_id,
        });
    }
    let operation = PendingOperation::Remove {
        instance: PersistedInstance::from(instance),
        parent_id,
    };
    journal::persist(workspaces, workspace_name, &operation)?;
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
        let root_id =
            create_instance(&resources, &workspaces, "proyecto", root_resource.id).unwrap();
        let child_id = create_instance_under(
            &resources,
            &workspaces,
            "proyecto",
            child_resource.id,
            Some(root_id),
        )
        .unwrap();
        let workspace = workspaces.load("proyecto").unwrap();
        let root = workspace.find_instance(root_id).unwrap();
        assert_eq!(root.child_ids(), &[child_id]);
        assert_eq!(root.children().len(), 1);
        assert_eq!(root.children()[0].source.id, child_resource.id);
        assert!(resources
            .get(root_resource.id)
            .unwrap()
            .management
            .structural
            .contains(&root_id));
        assert!(resources
            .get(child_resource.id)
            .unwrap()
            .management
            .structural
            .contains(&child_id));
    }

    #[test]
    fn removes_nested_leaf_but_rejects_parent() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let root = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();
        let child =
            create_instance_under(&resources, &workspaces, "proyecto", resource.id, Some(root))
                .unwrap();
        assert!(remove_instance(&resources, &workspaces, "proyecto", root).is_err());
        remove_instance(&resources, &workspaces, "proyecto", child).unwrap();
        let workspace = workspaces.load("proyecto").unwrap();
        assert_eq!(workspace.all_instances().len(), 1);
        assert!(workspace.find_instance(root).unwrap().children().is_empty());
        assert!(!resources
            .get(resource.id)
            .unwrap()
            .management
            .structural
            .contains(&child));
        assert!(resources
            .get(resource.id)
            .unwrap()
            .management
            .structural
            .contains(&root));
    }

    #[test]
    fn journal_recovers_creation_after_resource_write() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let instance = Instance::new(Uuid::now_v7(), resource.id);
        journal::persist(
            &workspaces,
            "proyecto",
            &PendingOperation::Create {
                instance: PersistedInstance::from(instance.clone()),
                parent_id: None,
            },
        )
        .unwrap();
        resources
            .register_instance(resource.id, instance.id)
            .unwrap();
        let (workspace, report) = reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();
        assert!(workspace.find_instance(instance.id).is_some());
        assert_eq!(report.restored, 1);
        assert!(!journal::path(&workspaces, "proyecto").unwrap().exists());
    }

    #[test]
    fn journal_recovers_removal_after_resource_write() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let root_id = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();
        let child_id = create_instance_under(
            &resources,
            &workspaces,
            "proyecto",
            resource.id,
            Some(root_id),
        )
        .unwrap();
        let workspace = workspaces.load("proyecto").unwrap();
        let child = workspace.find_instance(child_id).unwrap().clone();
        let operation = PendingOperation::Remove {
            instance: PersistedInstance::from(child),
            parent_id: Some(root_id),
        };
        journal::persist(&workspaces, "proyecto", &operation).unwrap();
        resources
            .unregister_instance(resource.id, child_id)
            .unwrap();

        let (recovered, report) = reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();

        assert_eq!(report.removed, 1);
        assert!(recovered.find_instance(child_id).is_none());
        let root = recovered.find_instance(root_id).unwrap();
        assert!(root.children().is_empty());
        assert!(root.child_ids().is_empty());
        assert!(!resources
            .get(resource.id)
            .unwrap()
            .management
            .structural
            .contains(&child_id));
        assert!(!journal::path(&workspaces, "proyecto").unwrap().exists());
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
    fn reconcile_rejects_instance_registered_by_multiple_resources() {
        let (_dir, resources, workspaces) = setup();
        let (first_resource, _) = resources.create("Primero", None).unwrap();
        let (second_resource, _) = resources.create("Segundo", None).unwrap();
        let duplicate_id = Uuid::now_v7();

        resources
            .register_instance(first_resource.id, duplicate_id)
            .unwrap();
        resources
            .register_instance(second_resource.id, duplicate_id)
            .unwrap();

        assert!(matches!(
            reconcile_workspace(&resources, &workspaces, "proyecto"),
            Err(LifecycleError::DuplicateStructuralId(id)) if id == duplicate_id
        ));
    }

    #[test]
    fn reconcile_rejects_structural_source_mismatch() {
        let (_dir, resources, workspaces) = setup();
        let (source_resource, _) = resources.create("Origen", None).unwrap();
        let (registered_resource, _) = resources.create("Registro", None).unwrap();
        let instance_id = Uuid::now_v7();
        let workspace = Workspace {
            content: crate::Content {
                instances: vec![Instance::new(instance_id, source_resource.id)],
            },
        };

        resources
            .register_instance(registered_resource.id, instance_id)
            .unwrap();
        workspaces.save("proyecto", &workspace).unwrap();

        assert!(matches!(
            reconcile_workspace(&resources, &workspaces, "proyecto"),
            Err(LifecycleError::JournalConflict(_))
        ));
    }

    #[test]
    fn journal_recovers_creation_before_either_persistent_effect() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let instance = Instance::new(Uuid::now_v7(), resource.id);

        journal::persist(
            &workspaces,
            "proyecto",
            &PendingOperation::Create {
                instance: PersistedInstance::from(instance.clone()),
                parent_id: None,
            },
        )
        .unwrap();

        let (workspace, report) = reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();

        assert_eq!(report.restored, 1);
        assert_eq!(workspace.all_instances().len(), 1);
        assert!(workspace.find_instance(instance.id).is_some());
        assert!(resources
            .get(resource.id)
            .unwrap()
            .management
            .structural
            .contains(&instance.id));
        assert!(!journal::path(&workspaces, "proyecto").unwrap().exists());
    }

    #[test]
    fn journal_recovers_creation_idempotently_after_both_effects() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let instance = Instance::new(Uuid::now_v7(), resource.id);

        journal::persist(
            &workspaces,
            "proyecto",
            &PendingOperation::Create {
                instance: PersistedInstance::from(instance.clone()),
                parent_id: None,
            },
        )
        .unwrap();
        resources
            .register_instance(resource.id, instance.id)
            .unwrap();
        workspaces
            .save(
                "proyecto",
                &Workspace {
                    content: crate::Content {
                        instances: vec![instance.clone()],
                    },
                },
            )
            .unwrap();

        let (workspace, report) = reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();

        assert_eq!(report.restored, 1);
        assert_eq!(workspace.all_instances().len(), 1);
        assert!(workspace.find_instance(instance.id).is_some());
        assert!(!journal::path(&workspaces, "proyecto").unwrap().exists());

        let (again, second_report) =
            reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();
        assert_eq!(second_report, RecoveryReport::default());
        assert_eq!(again.all_instances().len(), 1);
    }

    #[test]
    fn journal_recovers_removal_before_either_persistent_effect() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let root_id = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();
        let child_id = create_instance_under(
            &resources,
            &workspaces,
            "proyecto",
            resource.id,
            Some(root_id),
        )
        .unwrap();
        let child = workspaces
            .load("proyecto")
            .unwrap()
            .find_instance(child_id)
            .unwrap()
            .clone();

        journal::persist(
            &workspaces,
            "proyecto",
            &PendingOperation::Remove {
                instance: PersistedInstance::from(child),
                parent_id: Some(root_id),
            },
        )
        .unwrap();

        let (workspace, report) = reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();

        assert_eq!(report.removed, 1);
        assert!(workspace.find_instance(child_id).is_none());
        assert!(workspace
            .find_instance(root_id)
            .unwrap()
            .children()
            .is_empty());
        assert!(!resources
            .get(resource.id)
            .unwrap()
            .management
            .structural
            .contains(&child_id));
        assert!(!journal::path(&workspaces, "proyecto").unwrap().exists());
    }

    #[test]
    fn journal_finishes_removal_idempotently_after_both_effects() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let root_id = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();
        let child_id = create_instance_under(
            &resources,
            &workspaces,
            "proyecto",
            resource.id,
            Some(root_id),
        )
        .unwrap();
        let child = workspaces
            .load("proyecto")
            .unwrap()
            .find_instance(child_id)
            .unwrap()
            .clone();

        journal::persist(
            &workspaces,
            "proyecto",
            &PendingOperation::Remove {
                instance: PersistedInstance::from(child),
                parent_id: Some(root_id),
            },
        )
        .unwrap();
        resources
            .unregister_instance(resource.id, child_id)
            .unwrap();
        let mut workspace = workspaces.load("proyecto").unwrap();
        workspace.remove_leaf(child_id).unwrap();
        workspaces.save("proyecto", &workspace).unwrap();

        let (recovered, report) = reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();

        assert_eq!(report.removed, 1);
        assert!(recovered.find_instance(child_id).is_none());
        assert!(recovered
            .find_instance(root_id)
            .unwrap()
            .children()
            .is_empty());
        assert!(!journal::path(&workspaces, "proyecto").unwrap().exists());

        let (again, second_report) =
            reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();
        assert_eq!(second_report, RecoveryReport::default());
        assert!(again.find_instance(child_id).is_none());
        assert_eq!(again.all_instances().len(), 1);
    }

    #[test]
    fn conflicting_removal_journal_preserves_instance_and_journal() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let root_id = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();
        let child_id = create_instance_under(
            &resources,
            &workspaces,
            "proyecto",
            resource.id,
            Some(root_id),
        )
        .unwrap();
        let child = workspaces
            .load("proyecto")
            .unwrap()
            .find_instance(child_id)
            .unwrap()
            .clone();

        // El journal afirma que la instancia es raíz, pero el workspace la contiene bajo root_id.
        journal::persist(
            &workspaces,
            "proyecto",
            &PendingOperation::Remove {
                instance: PersistedInstance::from(child),
                parent_id: None,
            },
        )
        .unwrap();
        let journal = journal::path(&workspaces, "proyecto").unwrap();

        assert!(matches!(
            reconcile_workspace(&resources, &workspaces, "proyecto"),
            Err(LifecycleError::JournalConflict(_))
        ));

        let workspace = workspaces.load("proyecto").unwrap();
        assert_eq!(workspace.parent_of(child_id), Some(root_id));
        assert!(workspace.find_instance(child_id).is_some());
        assert!(resources
            .get(resource.id)
            .unwrap()
            .management
            .structural
            .contains(&child_id));
        assert!(journal.exists());
    }

    #[test]
    fn refuses_to_reconcile_corrupt_journal() {
        let (_dir, resources, workspaces) = setup();
        let (_resource, _) = resources.create("Recurso", None).unwrap();
        let path = workspaces.root().join(".proyecto.journal.json");
        fs::write(&path, "corrupto").unwrap();
        assert!(matches!(
            reconcile_workspace(&resources, &workspaces, "proyecto"),
            Err(LifecycleError::JournalJson(_))
        ));
        assert!(path.exists());
    }
}
