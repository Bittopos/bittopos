//! Núcleo mínimo de Bittopos.
//!
//! Los recursos se guardan individualmente en TOML. Los espacios de trabajo
//! se guardan en JSON y contienen instancias con identidad propia, separada
//! de la identidad del recurso de origen.

mod resource;
mod store;
mod workspace;

pub use resource::{Info, Resource, ResourcePatch, ResourceWarning};
pub use store::{ResourceStore, StoreError};
pub use workspace::{Instance, Source, Workspace, WorkspaceError, WorkspaceStore};
