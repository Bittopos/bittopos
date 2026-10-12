use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::resource::{
    normalize_description, normalize_name, validate_stored_name, Info, Management, Resource,
    ResourcePatch, ResourceWarning,
};

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
    TemporaryExists(PathBuf),
    PreparedContentMismatch(Uuid),
    AllocationConflict(Uuid),
    IncompleteManagement(Uuid),
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
            Self::AllocationConflict(id) => write!(
                f,
                "la instancia {id} ya figura en la asignación del recurso"
            ),
            Self::IncompleteManagement(id) => write!(
                f,
                "el recurso {id} no contiene management.allocation; se detuvo la recuperación para evitar perder instancias"
            ),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<io::Error> for StoreError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<toml::ser::Error> for StoreError {
    fn from(value: toml::ser::Error) -> Self {
        Self::TomlSerialize(value)
    }
}
impl From<toml::de::Error> for StoreError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlParse(value)
    }
}

/// Almacén local de recursos. El prototipo presupone un solo proceso escritor.
pub struct ResourceStore {
    root: PathBuf,
}

impl ResourceStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn create(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<(Resource, Vec<ResourceWarning>), StoreError> {
        let (name, _) = normalize_name(name)?;
        let mut warnings = Vec::new();
        let id = Uuid::now_v7();
        let description = normalize_description(description.map(str::to_owned));
        let resource = Resource {
            id,
            management: Management::default(),
            info: Info {
                title: name,
                description,
                tags: Vec::new(),
            },
        };
        self.write_new(&resource)?;
        if resource.info.title.starts_with(' ') {
            warnings.push(ResourceWarning::LeadingSpaces { id });
        }
        Ok((resource, warnings))
    }

    pub fn get(&self, id: Uuid) -> Result<Resource, StoreError> {
        let path = self.path_for(id);
        if !path.is_file() {
            return Err(StoreError::NotFound(id));
        }
        self.read_file(&path, id)
    }

    /// Lee todos los recursos confirmados y falla si alguno no puede validarse.
    /// Es deliberadamente estricto: una recuperación no debe confundir un recurso
    /// ilegible con una asignación vacía.
    pub(crate) fn all_strict(&self) -> Result<Vec<Resource>, StoreError> {
        let mut resources = Vec::new();
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }
            let stem = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
                StoreError::InvalidFileName(path.clone())
            })?;
            let expected_id = Uuid::parse_str(stem).map_err(|_| {
                StoreError::InvalidFileName(path.clone())
            })?;
            let content = fs::read_to_string(&path)?;
            let document: toml::Value = toml::from_str(&content)?;
            if document
                .get("management")
                .and_then(|management| management.get("allocation"))
                .is_none()
            {
                return Err(StoreError::IncompleteManagement(expected_id));
            }
            resources.push(self.read_file(&path, expected_id)?);
        }
        resources.sort_by_key(|resource| resource.id);
        Ok(resources)
    }

    /// Registra una identidad de instancia en el espacio administrativo de la definición.
    /// Es una operación de ciclo de vida; no permite editar directamente el segmento
    /// administrativo ni reemplazar la asignación completa.
    pub(crate) fn register_instance(
        &self,
        resource_id: Uuid,
        instance_id: Uuid,
    ) -> Result<(), StoreError> {
        let mut resource = self.get(resource_id)?;
        if resource.management.allocation.contains(&instance_id) {
            return Err(StoreError::AllocationConflict(instance_id));
        }
        resource.management.allocation.push(instance_id);
        self.write_existing(&resource)
    }

    /// Retira una identidad de instancia del registro administrativo.
    /// Devuelve false si el registro ya no contenía esa identidad.
    pub(crate) fn unregister_instance(
        &self,
        resource_id: Uuid,
        instance_id: Uuid,
    ) -> Result<bool, StoreError> {
        let mut resource = self.get(resource_id)?;
        let Some(position) = resource
            .management
            .allocation
            .iter()
            .position(|id| *id == instance_id)
        else {
            return Ok(false);
        };
        resource.management.allocation.remove(position);
        self.write_existing(&resource)?;
        Ok(true)
    }

    pub fn update(
        &self,
        id: Uuid,
        patch: ResourcePatch,
    ) -> Result<(Resource, Vec<ResourceWarning>), StoreError> {
        let mut resource = self.get(id)?;
        let mut warnings = Vec::new();

        if let Some(title) = patch.title {
            let (normalized, _) = normalize_name(&title)?;
            if normalized.starts_with(' ') {
                warnings.push(ResourceWarning::LeadingSpaces { id });
            }
            resource.info.title = normalized;
        }

        if let Some(description) = patch.description {
            resource.info.description = normalize_description(description);
        }

        if let Some(tags) = patch.tags {
            resource.info.tags = tags;
        }

        self.write_existing(&resource)?;
        Ok((resource, warnings))
    }

    /// Recupera todos los TOML válidos. Los archivos inválidos se omiten y generan advertencias.
    /// Los temporales de guardado se advierten, pero nunca se promueven automáticamente.
    pub fn recover(&self) -> Result<(Vec<Resource>, Vec<String>), StoreError> {
        let mut resources = Vec::new();
        let mut warnings = Vec::new();

        for entry in fs::read_dir(&self.root)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    warnings.push(format!(
                        "no se pudo leer una entrada del directorio: {error}"
                    ));
                    continue;
                }
            };
            let path = entry.path();
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            if file_name.starts_with('.') && file_name.ends_with(".toml.tmp") {
                warnings.push(format!(
                    "se encontró un temporal de guardado abandonado; no se recuperó automáticamente: {}",
                    path.display()
                ));
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let stem = match path.file_stem().and_then(|s| s.to_str()) {
                Some(stem) => stem,
                None => {
                    warnings.push(format!("nombre de archivo inválido: {}", path.display()));
                    continue;
                }
            };
            let expected_id = match Uuid::parse_str(stem) {
                Ok(id) => id,
                Err(_) => {
                    warnings.push(format!(
                        "nombre de archivo no contiene un UUID válido: {}",
                        path.display()
                    ));
                    continue;
                }
            };

            match self.read_file(&path, expected_id) {
                Ok(resource) => resources.push(resource),
                Err(error) => warnings.push(format!("se omitió {}: {error}", path.display())),
            }
        }

        resources.sort_by_key(|resource| resource.id);
        Ok((resources, warnings))
    }

    fn path_for(&self, id: Uuid) -> PathBuf {
        self.root.join(format!("{id}.toml"))
    }

    fn write_new(&self, resource: &Resource) -> Result<(), StoreError> {
        self.write_resource(resource, false)
    }

    fn write_existing(&self, resource: &Resource) -> Result<(), StoreError> {
        self.write_resource(resource, true)
    }

    fn temporary_path_for(&self, id: Uuid) -> PathBuf {
        self.root.join(format!(".{id}.toml.tmp"))
    }

    fn write_resource(&self, resource: &Resource, replace: bool) -> Result<(), StoreError> {
        let path = self.path_for(resource.id);
        let temporary_path = self.temporary_path_for(resource.id);
        let content = toml::to_string(resource)?;

        if replace {
            let confirmed_content = fs::read(&path)?;
            if confirmed_content == content.as_bytes() {
                return Ok(());
            }
        } else if path.exists() {
            return Err(StoreError::Io(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("ya existe el recurso {}", path.display()),
            )));
        }

        let mut temporary_file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
        {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                return Err(StoreError::TemporaryExists(temporary_path));
            }
            Err(error) => return Err(StoreError::Io(error)),
        };

        // Fase 1: preparar y validar. Si falla, el temporal no se considera recuperable
        // y se intenta limpiar; la limpieza no debe ocultar el error original.
        let prepared = (|| -> Result<(), StoreError> {
            temporary_file.write_all(content.as_bytes())?;
            temporary_file.sync_all()?;
            drop(temporary_file);

            let checked = self.read_file(&temporary_path, resource.id)?;
            if checked != *resource {
                return Err(StoreError::PreparedContentMismatch(resource.id));
            }
            Ok(())
        })();

        if let Err(error) = prepared {
            let _ = fs::remove_file(&temporary_path);
            return Err(error);
        }

        // Fase 2: confirmar. Si falla la promoción, se conserva el temporal ya validado
        // para no descartar la única copia de los nuevos datos.
        promote_temporary(&temporary_path, &path, replace)?;
        Ok(())
    }

    fn read_file(&self, path: &Path, expected_id: Uuid) -> Result<Resource, StoreError> {
        let content = fs::read_to_string(path)?;
        let first_line = content.lines().next().unwrap_or_default();
        if !first_line.starts_with("id = ") {
            return Err(StoreError::IdNotFirstLine(path.to_path_buf()));
        }
        let resource: Resource = toml::from_str(&content)?;
        if resource.id != expected_id {
            return Err(StoreError::IdMismatch {
                expected: expected_id,
                found: resource.id,
            });
        }
        validate_stored_name(&resource.info.title)?;
        Ok(resource)
    }
}

/// Promueve un temporal validado al destino definitivo.
/// En caso de error no elimina el temporal: el llamador puede conservarlo para inspección.
fn promote_temporary(temporary_path: &Path, path: &Path, replace: bool) -> Result<(), StoreError> {
    if replace {
        // En el entorno objetivo (Termux/Linux), rename en el mismo sistema de archivos
        // sustituye el destino sin exponer un archivo parcialmente escrito.
        // Las garantías concretas dependen de la plataforma y del sistema de archivos.
        fs::rename(temporary_path, path)?;
    } else {
        // hard_link evita reemplazar un destino que aparezca después de la comprobación.
        // El temporal y el destino están en el mismo directorio/sistema de archivos.
        fs::hard_link(temporary_path, path)?;
        let _ = fs::remove_file(temporary_path);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn store() -> (tempfile::TempDir, ResourceStore) {
        let dir = tempfile::tempdir().unwrap();
        let store = ResourceStore::new(dir.path()).unwrap();
        (dir, store)
    }

    #[test]
    fn creates_uuid_v7_resource_and_toml_file() {
        let (_dir, store) = store();
        let (resource, warnings) = store.create("Primer recurso", Some("Contexto")).unwrap();
        assert_eq!(resource.id.get_version_num(), 7);
        assert!(warnings.is_empty());
        let file = fs::read_to_string(store.path_for(resource.id)).unwrap();
        assert!(file.lines().next().unwrap().starts_with("id = "));
        assert!(file.contains("title = \"Primer recurso\""));
    }

    #[test]
    fn loads_legacy_resource_without_management_with_empty_allocation() {
        let (_dir, store) = store();
        let id = Uuid::now_v7();
        let path = store.path_for(id);
        fs::write(
            &path,
            format!(
                "id = \"{id}\"\n\n[info]\ntitle = \"Recurso antiguo\"\ntags = []\n"
            ),
        )
        .unwrap();

        let resource = store.get(id).unwrap();
        assert_eq!(resource.info.title, "Recurso antiguo");
        assert!(resource.management.allocation.is_empty());
    }

    #[test]
    fn strict_scan_rejects_resource_without_allocation_before_recovery() {
        let (_dir, store) = store();
        let id = Uuid::now_v7();
        fs::write(
            store.path_for(id),
            format!("id = \"{id}\"\n\n[info]\ntitle = \"Antiguo\"\ntags = []\n"),
        ).unwrap();

        assert!(matches!(
            store.all_strict(),
            Err(StoreError::IncompleteManagement(found)) if found == id
        ));
    }

    #[test]
    fn new_resource_persists_management_with_empty_allocation() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Nuevo", None).unwrap();
        let file = fs::read_to_string(store.path_for(resource.id)).unwrap();

        assert!(file.contains("[management]"));
        assert!(file.contains("allocation = []"));
        assert!(resource.management.allocation.is_empty());
    }

    #[test]
    fn allocation_registers_and_unregisters_instance_ids() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Asignación", None).unwrap();
        let instance_id = Uuid::now_v7();

        store.register_instance(resource.id, instance_id).unwrap();
        assert_eq!(store.get(resource.id).unwrap().management.allocation, vec![instance_id]);

        assert!(store.unregister_instance(resource.id, instance_id).unwrap());
        assert!(store.get(resource.id).unwrap().management.allocation.is_empty());
        assert!(!store.unregister_instance(resource.id, instance_id).unwrap());
    }

    #[test]
    fn allocation_rejects_duplicate_instance_ids() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Asignación", None).unwrap();
        let instance_id = Uuid::now_v7();

        store.register_instance(resource.id, instance_id).unwrap();
        assert!(matches!(
            store.register_instance(resource.id, instance_id),
            Err(StoreError::AllocationConflict(id)) if id == instance_id
        ));
        assert_eq!(store.get(resource.id).unwrap().management.allocation, vec![instance_id]);
    }

    #[test]
    fn same_name_gets_distinct_ids() {
        let (_dir, store) = store();
        let (a, _) = store.create("Igual", None).unwrap();
        let (b, _) = store.create("Igual", None).unwrap();
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn recovers_resource_from_disk() {
        let (_dir, store) = store();
        let (created, _) = store.create("Persistente", Some("Texto")).unwrap();
        let (recovered, warnings) = store.recover().unwrap();
        assert!(warnings.is_empty());
        assert_eq!(recovered, vec![created]);
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
    fn recovery_skips_corrupt_file_and_continues() {
        let (_dir, store) = store();
        let (valid, _) = store.create("Válido", None).unwrap();
        let corrupt_id = Uuid::now_v7();
        fs::write(store.path_for(corrupt_id), "esto no es TOML válido = [").unwrap();
        let (recovered, warnings) = store.recover().unwrap();
        assert_eq!(recovered, vec![valid]);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn recovery_rejects_filename_content_id_mismatch() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Original", None).unwrap();
        let other_id = Uuid::now_v7();
        fs::rename(store.path_for(resource.id), store.path_for(other_id)).unwrap();
        let (recovered, warnings) = store.recover().unwrap();
        assert!(recovered.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("no coincide"));
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
    #[test]
    fn update_does_not_overwrite_abandoned_temporary() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Confirmado", None).unwrap();
        let confirmed_path = store.path_for(resource.id);
        let confirmed_before = fs::read(&confirmed_path).unwrap();
        let temporary_path = store.temporary_path_for(resource.id);
        fs::write(&temporary_path, "borrador incompleto").unwrap();

        let result = store.update(
            resource.id,
            ResourcePatch {
                title: Some("Cambio".to_owned()),
                description: None,
                tags: None,
            },
        );

        assert!(matches!(result, Err(StoreError::TemporaryExists(path)) if path == temporary_path));
        assert_eq!(fs::read(&confirmed_path).unwrap(), confirmed_before);
        assert_eq!(
            fs::read_to_string(&temporary_path).unwrap(),
            "borrador incompleto"
        );
    }

    #[test]
    fn failed_promotion_preserves_temporary() {
        let (_dir, store) = store();
        let id = Uuid::now_v7();
        let resource = Resource {
            id,
            management: Management::default(),
            info: Info {
                title: "Temporal preparado".to_owned(),
                description: None,
                tags: Vec::new(),
            },
        };
        let temporary_path = store.temporary_path_for(id);
        let destination_path = store.path_for(id);

        // Un archivo no puede reemplazar un directorio mediante rename: fuerza un error
        // de promoción sin depender de permisos, que pueden no fallar bajo Termux/root.
        fs::write(&temporary_path, toml::to_string(&resource).unwrap()).unwrap();
        fs::create_dir(&destination_path).unwrap();

        assert!(promote_temporary(&temporary_path, &destination_path, true).is_err());
        assert_eq!(
            fs::read_to_string(&temporary_path).unwrap(),
            toml::to_string(&resource).unwrap()
        );
        assert!(destination_path.is_dir());
    }

    #[test]
    fn recovery_warns_about_temporary_without_promoting_it() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Confirmado", None).unwrap();
        let temporary_path = store.temporary_path_for(resource.id);
        fs::write(
            &temporary_path,
            "id = \"00000000-0000-7000-8000-000000000000\"\nname = \"No confirmado\"\n",
        )
        .unwrap();

        let (recovered, warnings) = store.recover().unwrap();
        assert_eq!(recovered, vec![resource.clone()]);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("no se recuperó automáticamente"));
        assert!(store.path_for(resource.id).exists());
        assert!(temporary_path.exists());
    }

    #[test]
    fn identical_update_leaves_confirmed_file_unchanged_and_creates_no_temporary() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Sin cambios", Some("Texto")).unwrap();
        let confirmed_path = store.path_for(resource.id);
        let before = fs::read(&confirmed_path).unwrap();

        let (updated, warnings) = store.update(resource.id, ResourcePatch::default()).unwrap();

        assert_eq!(updated, resource);
        assert!(warnings.is_empty());
        assert_eq!(fs::read(&confirmed_path).unwrap(), before);
        assert!(!store.temporary_path_for(resource.id).exists());
    }
}
