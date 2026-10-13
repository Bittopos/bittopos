use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use crate::{ResourceStore, StoreError, Workspace, WorkspaceError, WorkspaceStore};

#[derive(Debug)]
pub enum LifecycleError {
    Resource(StoreError),
    Workspace(WorkspaceError),
    InstanceNotFound(Uuid),
    DuplicateStructuralId(Uuid),
    StructuralMissing { resource_id: Uuid, instance_id: Uuid },
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
            Self::StructuralMissing { resource_id, instance_id } => write!(
                f,
                "la instancia {instance_id} no figura en structural del recurso {resource_id}"
            ),
        }
    }
}

impl std::error::Error for LifecycleError {}

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

/// Reconstruye el workspace a partir de las asignaciones confirmadas en los recursos.
/// Si algún recurso no puede validarse, no modifica el workspace.
pub fn reconcile_workspace(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
) -> Result<(Workspace, RecoveryReport), LifecycleError> {
    let definitions = resources.all_strict()?;
    let mut allocated: HashMap<Uuid, Uuid> = HashMap::new();

    for resource in &definitions {
        for instance_id in &resource.management.structural {
            if allocated.insert(*instance_id, resource.id).is_some() {
                return Err(LifecycleError::DuplicateStructuralId(*instance_id));
            }
        }
    }

    let existing = match workspaces.load(workspace_name) {
        Ok(workspace) => Some(workspace),
        Err(WorkspaceError::NotFound(_)) => None,
        Err(error) => return Err(LifecycleError::Workspace(error)),
    };

    let mut report = RecoveryReport::default();
    let mut reconciled = Workspace::new();
    let mut seen = std::collections::HashSet::new();

    if let Some(workspace) = &existing {
        for instance in &workspace.instances {
            match allocated.get(&instance.id) {
                Some(source_id) if *source_id == instance.source.id => {
                    reconciled.instances.push(instance.clone());
                    seen.insert(instance.id);
                }
                Some(_) => {
                    report.restored += 1;
                    report.removed += 1;
                }
                None => report.removed += 1,
            }
        }
    }

    let mut missing: Vec<_> = allocated
        .iter()
        .filter(|(instance_id, _)| !seen.contains(*instance_id))
        .map(|(instance_id, source_id)| (*instance_id, *source_id))
        .collect();
    missing.sort_by_key(|(instance_id, _)| *instance_id);

    for (instance_id, source_id) in missing {
        reconciled.instances.push(crate::Instance {
            id: instance_id,
            source: crate::Source { id: source_id },
        });
        if !existing.as_ref().is_some_and(|workspace| {
            workspace.instances.iter().any(|instance| instance.id == instance_id)
        }) {
            report.restored += 1;
        }
    }

    if existing.as_ref() != Some(&reconciled)
        && (existing.is_some() || !reconciled.instances.is_empty())
    {
        workspaces.save(workspace_name, &reconciled)?;
    }

    Ok((reconciled, report))
}

/// Registra primero la asignación. Si el guardado del workspace falla,
/// la siguiente reconciliación reconstruirá la instancia desde management.structural.
pub fn create_instance(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    resource_id: Uuid,
) -> Result<Uuid, LifecycleError> {
    let (mut workspace, _) = reconcile_workspace(resources, workspaces, workspace_name)?;
    resources.get(resource_id)?;

    let instance_id = Uuid::now_v7();
    resources.register_instance(resource_id, instance_id)?;
    workspace.instances.push(crate::Instance {
        id: instance_id,
        source: crate::Source { id: resource_id },
    });
    workspaces.save(workspace_name, &workspace)?;
    Ok(instance_id)
}

/// Retira primero la asignación. Si el guardado del workspace falla,
/// la siguiente reconciliación eliminará la instancia residual.
pub fn remove_instance(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    instance_id: Uuid,
) -> Result<(), LifecycleError> {
    let (mut workspace, _) = reconcile_workspace(resources, workspaces, workspace_name)?;
    let instance = workspace
        .instances
        .iter()
        .find(|instance| instance.id == instance_id)
        .cloned()
        .ok_or(LifecycleError::InstanceNotFound(instance_id))?;

    if !resources.unregister_instance(instance.source.id, instance_id)? {
        return Err(LifecycleError::StructuralMissing {
            resource_id: instance.source.id,
            instance_id,
        });
    }

    workspace.remove_instance(instance_id);
    workspaces.save(workspace_name, &workspace)?;
    Ok(())
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
    fn recovers_creation_interrupted_after_structural_commit() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Origen", None).unwrap();
        let instance_id = Uuid::now_v7();
        resources.register_instance(resource.id, instance_id).unwrap();

        let (workspace, report) =
            reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();

        assert_eq!(workspace.instances.len(), 1);
        assert_eq!(workspace.instances[0].id, instance_id);
        assert_eq!(workspace.instances[0].source.id, resource.id);
        assert_eq!(report.restored, 1);
        assert_eq!(report.removed, 0);
    }

    #[test]
    fn recovers_deletion_interrupted_after_structural_commit() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Origen", None).unwrap();
        let instance_id = Uuid::now_v7();
        resources.register_instance(resource.id, instance_id).unwrap();
        workspaces.save("proyecto", &Workspace {
            instances: vec![crate::Instance {
                id: instance_id,
                source: crate::Source { id: resource.id },
            }],
        }).unwrap();

        resources.unregister_instance(resource.id, instance_id).unwrap();
        let (workspace, report) =
            reconcile_workspace(&resources, &workspaces, "proyecto").unwrap();

        assert!(workspace.instances.is_empty());
        assert_eq!(report.removed, 1);
    }

    #[test]
    fn creation_registers_before_persisting_instance() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Origen", None).unwrap();

        let id = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();

        assert_eq!(resources.get(resource.id).unwrap().management.structural, vec![id]);
        assert_eq!(workspaces.load("proyecto").unwrap().instances[0].id, id);
    }

    #[test]
    fn removal_unregisters_before_persisting_workspace() {
        let (_dir, resources, workspaces) = setup();
        let (resource, _) = resources.create("Origen", None).unwrap();
        let id = create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();

        remove_instance(&resources, &workspaces, "proyecto", id).unwrap();

        assert!(resources.get(resource.id).unwrap().management.structural.is_empty());
        assert!(workspaces.load("proyecto").unwrap().instances.is_empty());
    }

    #[test]
    fn refuses_to_reconcile_if_any_resource_is_invalid() {
        let (_dir, resources, workspaces) = setup();
        let bad_id = Uuid::now_v7();
        fs::write(resources.root().join(format!("{bad_id}.toml")), "not valid TOML = [").unwrap();
        let workspace_path = workspaces.root().join("proyecto.json");
        let original = r#"{"instances":[]}"#;
        fs::write(&workspace_path, original).unwrap();

        assert!(reconcile_workspace(&resources, &workspaces, "proyecto").is_err());
        assert_eq!(fs::read_to_string(workspace_path).unwrap(), original);
    }

    #[test]
    fn rejects_instance_allocated_to_two_resources() {
        let (_dir, resources, workspaces) = setup();
        let (first, _) = resources.create("Uno", None).unwrap();
        let (second, _) = resources.create("Dos", None).unwrap();
        let instance_id = Uuid::now_v7();
        resources.register_instance(first.id, instance_id).unwrap();
        resources.register_instance(second.id, instance_id).unwrap();

        assert!(matches!(
            reconcile_workspace(&resources, &workspaces, "proyecto"),
            Err(LifecycleError::DuplicateStructuralId(id)) if id == instance_id
        ));
    }
}
