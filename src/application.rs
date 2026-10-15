//! Casos de uso que coordinan el modelo persistido antes de una interfaz.

use std::fmt;

use crate::{
    reconcile_workspace, LifecycleError, NavigationError, NavigationState, NavigationStore,
    RecoveryReport, ResourceStore, Workspace, WorkspaceStore,
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

#[cfg(test)]
mod tests {
    use super::*;

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
