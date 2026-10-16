//! Núcleo mínimo de Bittopos.
//!
//! Los recursos se guardan individualmente en TOML. Los espacios de trabajo
//! se guardan en JSON y contienen instancias con identidad propia, separada
//! de la identidad del recurso de origen.

mod application;
mod journal;
mod lifecycle;
mod mask_operations;
mod model_operations;
mod navigation;
mod persistence;
mod resource;
mod semantic_masks;
mod store;
mod workspace;
mod workspace_model;

pub use application::{
    create_instance_at_current_location, prepare_workspace, remove_instance_and_update_navigation,
    ApplicationError, PreparedWorkspace,
};
pub use lifecycle::{
    create_instance, create_instance_under, reconcile_workspace, remove_instance, LifecycleError,
    RecoveryReport,
};
pub use mask_operations::{
    create_snapshot, edit_instance_component, edit_instance_concept, edit_shared_component,
    edit_shared_concept, remove_mask, resolve_instance, MaskOperationError,
};
pub use model_operations::children_of;
pub use navigation::{NavigationError, NavigationState, NavigationStore};
pub use resource::{Info, Resource, ResourcePatch, ResourceWarning};
pub use semantic_masks::{
    resolve as resolve_semantics, Component, ComponentMerge, Concept, ElementAddress,
    InstanceSemantics, Mask, MaskError, MaskMode, ReplaceComponent, ResourceDefinition,
    Segment as SemanticSegment, SegmentRole,
};
pub use store::{ResourceStore, StoreError, TemporaryInspection};
pub use workspace::{WorkspaceError, WorkspaceStore};
pub use workspace_model::{Content, Instance, Semantic, Source, Workspace, WorkspaceModelError};
