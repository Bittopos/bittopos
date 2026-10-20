use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Perfil de usuario dentro del dominio de Bittopos.
///
/// La estructura de persistencia JSON es una representación de este modelo;
/// las reglas de validez se aplican mediante `validate`, no solo al deserializar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub projects: Vec<Project>,
}

/// Proyecto perteneciente a un perfil y raíz de una estructura de fichas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub objects: Vec<Ficha>,
}

/// Ficha identificable. Su contención jerárquica no representa relaciones semánticas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ficha {
    pub id: Uuid,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub children: Vec<Ficha>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainModelError {
    EmptyName { entity: &'static str, id: Uuid },
    DuplicateId(Uuid),
}

impl std::fmt::Display for DomainModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName { entity, id } => {
                write!(f, "el nombre/título de {entity} {id} no puede estar vacío")
            }
            Self::DuplicateId(id) => write!(f, "el ID de entidad {id} está duplicado"),
        }
    }
}

impl std::error::Error for DomainModelError {}

impl Profile {
    /// Valida nombres y unicidad de IDs en todo el árbol del perfil.
    pub fn validate(&self) -> Result<(), DomainModelError> {
        let mut ids = HashSet::new();
        insert_id(&mut ids, self.id)?;
        validate_label("perfil", self.id, &self.name)?;

        for project in &self.projects {
            insert_id(&mut ids, project.id)?;
            validate_label("proyecto", project.id, &project.name)?;
            for object in &project.objects {
                validate_ficha(object, &mut ids)?;
            }
        }

        Ok(())
    }
}

fn validate_ficha(ficha: &Ficha, ids: &mut HashSet<Uuid>) -> Result<(), DomainModelError> {
    insert_id(ids, ficha.id)?;
    validate_label("ficha", ficha.id, &ficha.title)?;
    for child in &ficha.children {
        validate_ficha(child, ids)?;
    }
    Ok(())
}

fn insert_id(ids: &mut HashSet<Uuid>, id: Uuid) -> Result<(), DomainModelError> {
    if ids.insert(id) {
        Ok(())
    } else {
        Err(DomainModelError::DuplicateId(id))
    }
}

fn validate_label(entity: &'static str, id: Uuid, value: &str) -> Result<(), DomainModelError> {
    if value.trim().is_empty() {
        Err(DomainModelError::EmptyName { entity, id })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_project_and_nested_fichas_round_trip_as_json() {
        let profile = Profile {
            id: Uuid::from_u128(1),
            name: "Ana García".into(),
            projects: vec![Project {
                id: Uuid::from_u128(2),
                name: "Huerta familiar".into(),
                objects: vec![
                    Ficha {
                        id: Uuid::from_u128(3),
                        title: "Cultivos".into(),
                        description: Some("Organización de los cultivos".into()),
                        children: vec![Ficha {
                            id: Uuid::from_u128(4),
                            title: "Tomates".into(),
                            description: None,
                            children: vec![],
                        }],
                    },
                    Ficha {
                        id: Uuid::from_u128(5),
                        title: "Riego".into(),
                        description: None,
                        children: vec![Ficha {
                            id: Uuid::from_u128(6),
                            title: "Riego por goteo".into(),
                            description: None,
                            children: vec![],
                        }],
                    },
                    Ficha {
                        id: Uuid::from_u128(7),
                        title: "Herramientas".into(),
                        description: None,
                        children: vec![],
                    },
                ],
            }],
        };

        profile.validate().unwrap();
        let json = serde_json::to_string_pretty(&profile).unwrap();
        let decoded: Profile = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, profile);
        assert_eq!(decoded.projects[0].objects.len(), 3);
        assert_eq!(decoded.projects[0].objects[0].children.len(), 1);
        assert_eq!(decoded.projects[0].objects[1].children.len(), 1);
        assert!(decoded.projects[0].objects[2].children.is_empty());
    }

    #[test]
    fn validation_rejects_duplicate_ids_across_entity_types() {
        let duplicate = Uuid::from_u128(42);
        let profile = Profile {
            id: Uuid::from_u128(1),
            name: "Perfil".into(),
            projects: vec![Project {
                id: duplicate,
                name: "Proyecto".into(),
                objects: vec![Ficha {
                    id: duplicate,
                    title: "Ficha".into(),
                    description: None,
                    children: vec![],
                }],
            }],
        };

        assert_eq!(profile.validate(), Err(DomainModelError::DuplicateId(duplicate)));
    }

    #[test]
    fn validation_rejects_blank_titles() {
        let profile = Profile {
            id: Uuid::from_u128(1),
            name: "Perfil".into(),
            projects: vec![Project {
                id: Uuid::from_u128(2),
                name: "Proyecto".into(),
                objects: vec![Ficha {
                    id: Uuid::from_u128(3),
                    title: "   ".into(),
                    description: None,
                    children: vec![],
                }],
            }],
        };

        assert!(matches!(
            profile.validate(),
            Err(DomainModelError::EmptyName { entity: "ficha", .. })
        ));
    }
}
