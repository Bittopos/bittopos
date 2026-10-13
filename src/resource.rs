use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resource {
    /// Identidad estable del recurso.
    /// Se serializa primero para facilitar la comprobación del archivo.
    pub id: Uuid,
    /// Metadatos administrativos reservados al núcleo.
    /// No se exponen a consumidores externos ni a operaciones de instancia.
    #[serde(default)]
    pub(crate) management: Management,
    pub info: Info,
}

/// Estado administrativo del recurso. Su contenido no forma parte de la API pública.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Management {
    /// Identidades de instancias creadas a partir de esta definición.
    #[serde(default, alias = "allocation")]
    pub(crate) structural: Vec<Uuid>,
}

/// Información descriptiva del recurso.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResourcePatch {
    /// None conserva el título; Some(value) solicita cambiarlo.
    pub title: Option<String>,

    /// None conserva la descripción; Some(None) la elimina.
    /// Some(Some("")) también elimina la descripción.
    pub description: Option<Option<String>>,

    /// None conserva las etiquetas; Some(tags) reemplaza la lista.
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceWarning {
    LeadingSpaces { id: Uuid },
}

pub(crate) fn normalize_name(name: &str) -> Result<(String, Vec<ResourceWarning>), ResourceError> {
    let normalized = name.trim_end_matches(' ').to_owned();

    if normalized.is_empty() {
        return Err(ResourceError::InvalidName);
    }

    // Los espacios iniciales se conservan por ahora.
    Ok((normalized, Vec::new()))
}

pub(crate) fn validate_stored_name(name: &str) -> Result<(), ResourceError> {
    if name.is_empty() || name.trim_matches(' ').is_empty() {
        return Err(ResourceError::InvalidName);
    }

    Ok(())
}

pub(crate) fn normalize_description(description: Option<String>) -> Option<String> {
    description.filter(|value| !value.is_empty())
}


/// Error produced by resource-domain validation, independent of persistence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResourceError {
    InvalidName,
}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName => write!(f, "el nombre no puede estar vacío ni contener solo espacios"),
        }
    }
}

impl std::error::Error for ResourceError {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_validation_reports_domain_error_without_storage_types() {
        assert_eq!(normalize_name("   ").unwrap_err(), ResourceError::InvalidName);
        assert_eq!(validate_stored_name("   ").unwrap_err(), ResourceError::InvalidName);
        assert!(normalize_name("valid name").is_ok());
    }
}
