//! Núcleo mínimo de Bittopos.
//!
//! En esta versión, un recurso contiene identidad, título y descripción opcional y etiquetas
//! opcionales.
//! Cada recurso se guarda en un archivo TOML cuyo nombre es su UUID.
//! Todas las instancias se guardan en un archivo JSON cuyo nombre sera proyecto.

mod resource;
mod store;
mod workspace;

pub use resource::{Info, Resource, ResourcePatch, ResourceWarning};
pub use store::{ResourceStore, StoreError};
pub use workspace::{Instance, Workspace, WorkspaceError, WorkspaceStore};
