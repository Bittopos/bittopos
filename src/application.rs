//! Casos de uso que coordinan el modelo persistido antes de una interfaz.

use std::fmt;

use crate::{
    create_instance_under, reconcile_workspace, remove_instance, LifecycleError, NavigationError,
    NavigationState, NavigationStore, RecoveryReport, ResourceStore, Workspace, WorkspaceStore,
};

/// Error producido al preparar el contexto operativo del workspace.
#[derive(Debug)]
pub enum ApplicationError {
    Lifecycle(LifecycleError),
    Navigation(NavigationError),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lifecycle(error) => write!(f, "no se pudo preparar el workspace: {error}"),
            Self::Navigation(error) => write!(f, "no se pudo preparar la navegación: {error}"),
        }
    }
}

impl std::error::Error for ApplicationError {}

impl From<LifecycleError> for ApplicationError {
    fn from(error: LifecycleError) -> Self {
        Self::Lifecycle(error)
    }
}

impl From<NavigationError> for ApplicationError {
    fn from(error: NavigationError) -> Self {
        Self::Navigation(error)
    }
}

/// Contexto validado que una interfaz necesita para comenzar una operación.
#[derive(Debug)]
pub struct PreparedWorkspace {
    pub workspace: Workspace,
    pub navigation_state: NavigationState,
    pub navigation_reset: bool,
    pub recovery: RecoveryReport,
}

/// Reconcilia las vistas persistidas y valida el destino de navegación antes de usar la CLI
/// u otra interfaz. Un destino inexistente se restablece a la raíz mediante NavigationStore.
pub fn prepare_workspace(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    navigation: &NavigationStore,
    workspace_name: &str,
) -> Result<PreparedWorkspace, ApplicationError> {
    let (workspace, recovery) = reconcile_workspace(resources, workspaces, workspace_name)?;
    let (navigation_state, navigation_reset) = navigation.load_validated(&workspace)?;

    Ok(PreparedWorkspace {
        workspace,
        navigation_state,
        navigation_reset,
        recovery,
    })
}

/// Crea una instancia de un recurso bajo la ubicación actualmente seleccionada.
/// La regla de ciclo de vida sigue siendo responsabilidad de `lifecycle`.
pub fn create_instance_at_current_location(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    resource_id: uuid::Uuid,
    navigation_state: &NavigationState,
) -> Result<uuid::Uuid, ApplicationError> {
    Ok(create_instance_under(
        resources,
        workspaces,
        workspace_name,
        resource_id,
        navigation_state.current_instance,
    )?)
}

/// Elimina una instancia hoja y restablece la navegación si apuntaba a esa instancia.
/// Devuelve `true` cuando fue necesario guardar una navegación nueva en la raíz.
pub fn remove_instance_and_update_navigation(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    navigation: &NavigationStore,
    workspace_name: &str,
    instance_id: uuid::Uuid,
    navigation_state: &mut NavigationState,
) -> Result<bool, ApplicationError> {
    remove_instance(resources, workspaces, workspace_name, instance_id)?;
    if navigation_state.current_instance == Some(instance_id) {
        *navigation_state = NavigationState::default();
        navigation.save(navigation_state)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_instance_under_current_navigation_location() {
        let dir = tempfile::tempdir().unwrap();
        let resources = ResourceStore::new(dir.path().join("resources")).unwrap();
        let workspaces = WorkspaceStore::new(dir.path().join("workspaces")).unwrap();
        let (root_resource, _) = resources.create("Raíz", None).unwrap();
        let (child_resource, _) = resources.create("Hija", None).unwrap();
        let root_id =
            crate::create_instance(&resources, &workspaces, "proyecto", root_resource.id).unwrap();
        let navigation_state = NavigationState {
            current_instance: Some(root_id),
        };

        let child_id = create_instance_at_current_location(
            &resources,
            &workspaces,
            "proyecto",
            child_resource.id,
            &navigation_state,
        )
        .unwrap();
        let workspace = workspaces.load("proyecto").unwrap();
        assert_eq!(workspace.parent_of(child_id), Some(root_id));
        assert_eq!(
            workspace.find_instance(child_id).unwrap().source.id,
            child_resource.id
        );
    }

    #[test]
    fn removing_current_instance_resets_and_persists_navigation() {
        let dir = tempfile::tempdir().unwrap();
        let resources = ResourceStore::new(dir.path().join("resources")).unwrap();
        let workspaces = WorkspaceStore::new(dir.path().join("workspaces")).unwrap();
        let navigation = NavigationStore::new(dir.path().join("workspaces"));
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let instance_id =
            crate::create_instance(&resources, &workspaces, "proyecto", resource.id).unwrap();
        let mut navigation_state = NavigationState {
            current_instance: Some(instance_id),
        };
        navigation.save(&navigation_state).unwrap();

        let reset = remove_instance_and_update_navigation(
            &resources,
            &workspaces,
            &navigation,
            "proyecto",
            instance_id,
            &mut navigation_state,
        )
        .unwrap();

        assert!(reset);
        assert_eq!(navigation_state.current_instance, None);
        assert_eq!(navigation.load().unwrap().current_instance, None);
        assert!(workspaces
            .load("proyecto")
            .unwrap()
            .find_instance(instance_id)
            .is_none());
    }

    #[test]
    fn preparation_reconciles_workspace_and_resets_missing_navigation_target() {
        let dir = tempfile::tempdir().unwrap();
        let resources = ResourceStore::new(dir.path().join("resources")).unwrap();
        let workspaces = WorkspaceStore::new(dir.path().join("workspaces")).unwrap();
        let navigation = NavigationStore::new(dir.path().join("workspaces"));
        navigation
            .save(&NavigationState {
                current_instance: Some(uuid::Uuid::now_v7()),
            })
            .unwrap();

        let prepared = prepare_workspace(&resources, &workspaces, &navigation, "proyecto").unwrap();

        assert!(prepared.workspace.all_instances().is_empty());
        assert_eq!(prepared.navigation_state.current_instance, None);
        assert!(prepared.navigation_reset);
        assert_eq!(prepared.recovery, RecoveryReport::default());
        assert_eq!(navigation.load().unwrap().current_instance, None);
    }
}
