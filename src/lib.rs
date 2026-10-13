//! Núcleo mínimo de Bittopos.
//!
//! Los recursos se guardan individualmente en TOML. Los espacios de trabajo
//! se guardan en JSON y contienen instancias con identidad propia, separada
//! de la identidad del recurso de origen.

mod lifecycle;
mod persistence;
mod resource;
mod store;
mod workspace;

pub use lifecycle::{create_instance, create_instance_under, reconcile_workspace, remove_instance, LifecycleError, RecoveryReport};
pub use resource::{Info, Resource, ResourcePatch, ResourceWarning};
pub use store::{ResourceStore, StoreError, TemporaryInspection};
pub use workspace::{Content, Instance, Semantic, Source, Workspace, WorkspaceError, WorkspaceStore};
