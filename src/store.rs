use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::resource::{
    normalize_description, normalize_name, validate_stored_name, Info, Management, Resource,
    ResourceError, ResourcePatch, ResourceWarning,
};
use crate::semantic_masks::{ResourceDefinition, Segment, SegmentRole};

#[derive(Debug)]
pub enum StoreError {
    Io(io::Error),
    TomlSerialize(toml::ser::Error),
    TomlParse(toml::de::Error),
    InvalidName,
    NotFound(Uuid),
    InvalidFileName(PathBuf),
    IdMismatch { expected: Uuid, found: Uuid },
    IdNotFirstLine(PathBuf),
    StructuralConflict(Uuid),
    InvalidSemanticModel(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "error de E/S: {e}"),
            Self::TomlSerialize(e) => write!(f, "no se pudo serializar TOML: {e}"),
            Self::TomlParse(e) => write!(f, "TOML inválido: {e}"),
            Self::InvalidName => write!(
                f,
                "el nombre no puede estar vacío ni contener solo espacios"
            ),
            Self::NotFound(id) => write!(f, "no existe el recurso {id}"),
            Self::InvalidFileName(path) => write!(
                f,
                "nombre de archivo de recurso inválido: {}",
                path.display()
            ),
            Self::IdMismatch { expected, found } => write!(
                f,
                "el ID del archivo ({expected}) no coincide con el ID del contenido ({found})"
            ),
            Self::IdNotFirstLine(path) => write!(
                f,
                "el ID no aparece en la primera línea de {}",
                path.display()
            ),
            Self::TemporaryExists(path) => write!(
                f,
                "ya existe un temporal de guardado; se conserva sin sobrescribir: {}",
                path.display()
            ),
            Self::PreparedContentMismatch(id) => write!(
                f,
                "el contenido temporal preparado no coincide con el recurso esperado {id}"
            ),
            Self::StructuralConflict(id) => write!(
                f,
                "la instancia {id} ya figura en la asignación del recurso"
            ),
            Err(StoreError::StructuralConflict(id)) if id == instance_id
        ));
        assert_eq!(
            store.get(resource.id).unwrap().management.structural,
            vec![instance_id]
        );
    }

    #[test]
    fn same_name_gets_distinct_ids() {
        let (_dir, store) = store();
        let (a, _) = store.create("Igual", None).unwrap();
        let (b, _) = store.create("Igual", None).unwrap();
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn patch_preserves_omitted_fields_and_identity() {
        let (_dir, store) = store();
        let (created, _) = store.create("Antes", Some("Descripción")).unwrap();
        let (updated, _) = store
            .update(
                created.id,
                ResourcePatch {
                    title: Some("Después  ".to_owned()),
                    description: None,
                    tags: None,
                },
            )
            .unwrap();
        assert_eq!(updated.id, created.id);
        assert_eq!(updated.info.title, "Después");
        assert_eq!(updated.info.description.as_deref(), Some("Descripción"));
    }

    #[test]
    fn rejects_blank_names_and_warns_about_leading_space() {
        let (_dir, store) = store();
        assert!(matches!(
            store.create("   ", None),
            Err(StoreError::InvalidName)
        ));
        let (resource, warnings) = store.create(" nombre", None).unwrap();
        assert_eq!(resource.info.title, " nombre");
        assert_eq!(
            warnings,
            vec![ResourceWarning::LeadingSpaces { id: resource.id }]
        );
    }

    #[test]
    fn missing_id_returns_controlled_error() {
        let (_dir, store) = store();
        let missing = Uuid::now_v7();
        assert!(matches!(store.get(missing), Err(StoreError::NotFound(id)) if id == missing));
    }

    #[test]
    fn empty_description_clears_it() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Recurso", Some("Texto")).unwrap();
        let (updated, _) = store
            .update(
                resource.id,
                ResourcePatch {
                    title: None,
                    description: Some(Some(String::new())),
                    tags: None,
                },
            )
            .unwrap();
        assert_eq!(updated.info.description, None);
    }

}
