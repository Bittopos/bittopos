//! Relaciones persistidas como recursos de definición, sin instancias propias.
//!
//! Los extremos se guardan en un segmento de control reservado. El segmento
//! descriptivo `info` se deriva de los metadatos normales del recurso.

use std::fmt;
use uuid::Uuid;

use crate::{load_workspace, Resource, ResourceStore, SemanticSegment, SegmentRole, StoreError, WorkspaceStore};

const RELATION_SEGMENT: &str = "__relation";

/// Los dos extremos siempre pertenecen al mismo espacio de identidad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationEndpoints {
    Resources { source: Uuid, target: Uuid },
    Instances { source: Uuid, target: Uuid },
}

#[derive(Debug)]
pub enum RelationError {
    Resource(StoreError),
    Invalid(String),
    EndpointNotFound(Uuid),
}

impl fmt::Display for RelationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resource(error) => write!(f, "error de recurso: {error}"),
            Self::Invalid(message) => write!(f, "relación inválida: {message}"),
            Self::EndpointNotFound(id) => write!(f, "no existe el extremo de relación {id}"),
        }
    }
}
impl std::error::Error for RelationError {}
impl From<StoreError> for RelationError {
    fn from(value: StoreError) -> Self { Self::Resource(value) }
}

/// Crea una relación concreta en el mismo almacén que los recursos ordinarios.
/// La relación tiene identidad e info propios, pero nunca se añade a la estructura.
pub fn create_relation(
    resources: &ResourceStore,
    workspaces: &WorkspaceStore,
    workspace_name: &str,
    title: &str,
    description: Option<&str>,
    endpoints: RelationEndpoints,
) -> Result<Resource, RelationError> {
    match endpoints {
        RelationEndpoints::Resources { source, target } => {
            resources.get(source)?;
            resources.get(target)?;
        }
        RelationEndpoints::Instances { source, target } => {
            let workspace = load_workspace(resources, workspaces, workspace_name)
                .map_err(|error| RelationError::Invalid(error.to_string()))?;
            for id in [source, target] {
                if workspace.find_instance(id).is_none() {
                    return Err(RelationError::EndpointNotFound(id));
                }
            }
        }
    }

    let (resource, _) = resources.create(title, description)?;
    resources.update_segments(resource.id, vec![relation_segment(endpoints)])
        .map_err(RelationError::from)
}

/// Detecta una relación por su segmento de control reservado.
pub fn is_relation(resource: &Resource) -> bool {
    resource.segments.iter().any(|segment| {
        segment.key == RELATION_SEGMENT && segment.role == SegmentRole::Control
    })
}

/// Lee los extremos directamente, sin crear una instancia ni mutar datos.
pub fn relation_endpoints(resource: &Resource) -> Result<RelationEndpoints, RelationError> {
    let segment = resource.segments.iter().find(|segment| {
        segment.key == RELATION_SEGMENT && segment.role == SegmentRole::Control
    }).ok_or_else(|| RelationError::Invalid("el recurso no es una relación".to_owned()))?;
    let kind = component_string(segment, "kind")?;
    let source = component_string(segment, "source")?;
    let target = component_string(segment, "target")?;
    let parse_id = |value: &str| Uuid::parse_str(value)
        .map_err(|_| RelationError::Invalid(format!("UUID de extremo inválido: {value}")));
    match kind.as_str() {
        "resource_resource" => Ok(RelationEndpoints::Resources { source: parse_id(&source)?, target: parse_id(&target)? }),
        "instance_instance" => Ok(RelationEndpoints::Instances { source: parse_id(&source)?, target: parse_id(&target)? }),
        _ => Err(RelationError::Invalid(format!("tipo de extremos desconocido: {kind}"))),
    }
}

fn component_string(segment: &SemanticSegment, key: &str) -> Result<String, RelationError> {
    let component = segment.components.iter().find(|item| item.key == key)
        .ok_or_else(|| RelationError::Invalid(format!("falta el componente reservado '{key}'")))?;
    component.value.as_str().map(str::to_owned)
        .ok_or_else(|| RelationError::Invalid(format!("el componente '{key}' debe ser texto")))
}

fn relation_segment(endpoints: RelationEndpoints) -> SemanticSegment {
    let (kind, source_kind, target_kind, source, target) = match endpoints {
        RelationEndpoints::Resources { source, target } => ("resource_resource", "resource-id", "resource-id", source, target),
        RelationEndpoints::Instances { source, target } => ("instance_instance", "instance-id", "instance-id", source, target),
    };
    SemanticSegment {
        key: RELATION_SEGMENT.to_owned(),
        role: SegmentRole::Control,
        concepts: Vec::new(),
        components: vec![
            crate::Component { key: "kind".to_owned(), kind: "relation-kind".to_owned(), value: serde_json::Value::String(kind.to_owned()) },
            crate::Component { key: "source".to_owned(), kind: source_kind.to_owned(), value: serde_json::Value::String(source.to_string()) },
            crate::Component { key: "target".to_owned(), kind: target_kind.to_owned(), value: serde_json::Value::String(target.to_string()) },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_instance;

    #[test]
    fn resource_relation_is_persisted_and_read_without_instance_creation() {
        let dir = tempfile::tempdir().unwrap();
        let resources = ResourceStore::new(dir.path().join("resources")).unwrap();
        let workspaces = WorkspaceStore::new(dir.path().join("structures")).unwrap();
        let (source, _) = resources.create("Origen", None).unwrap();
        let (target, _) = resources.create("Destino", None).unwrap();
        let relation = create_relation(&resources, &workspaces, "principal", "Relacionado con", Some("Descripción"), RelationEndpoints::Resources { source: source.id, target: target.id }).unwrap();
        let stored = resources.get(relation.id).unwrap();
        assert!(is_relation(&stored));
        assert_eq!(stored.info.title, "Relacionado con");
        assert_eq!(relation_endpoints(&stored).unwrap(), RelationEndpoints::Resources { source: source.id, target: target.id });
        assert_eq!(stored.semantic_definition().segment("info").unwrap().concepts.len(), 2);
    }

    #[test]
    fn instance_relation_requires_both_instances_to_exist() {
        let dir = tempfile::tempdir().unwrap();
        let resources = ResourceStore::new(dir.path().join("resources")).unwrap();
        let workspaces = WorkspaceStore::new(dir.path().join("structures")).unwrap();
        let (resource, _) = resources.create("Recurso", None).unwrap();
        let source = create_instance(&resources, &workspaces, "principal", resource.id).unwrap();
        let missing = Uuid::now_v7();
        assert!(matches!(create_relation(&resources, &workspaces, "principal", "Relación", None, RelationEndpoints::Instances { source, target: missing }), Err(RelationError::EndpointNotFound(id)) if id == missing));
    }
}
