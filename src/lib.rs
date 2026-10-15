//! Núcleo mínimo de Bittopos.
//!
//! Los recursos se guardan individualmente en TOML. Los espacios de trabajo
//! se guardan en JSON y contienen instancias con identidad propia, separada
//! de la identidad del recurso de origen.

mod lifecycle;
mod navigation;
mod persistence;
mod resource;
mod store;
mod workspace;
mod workspace_model;

pub use lifecycle::{
    create_instance, create_instance_under, reconcile_workspace, remove_instance, LifecycleError,
    RecoveryReport,
};
pub use navigation::{NavigationError, NavigationState, NavigationStore};
pub use resource::{Info, Resource, ResourcePatch, ResourceWarning};
pub use store::{ResourceStore, StoreError, TemporaryInspection};
pub use workspace::{WorkspaceError, WorkspaceStore};
pub use workspace_model::{Content, Instance, Semantic, Source, Workspace, WorkspaceModelError};
