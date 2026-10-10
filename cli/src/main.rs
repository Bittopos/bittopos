use std::env;
use std::path::PathBuf;
use std::process;

use bittopos_core::{ResourcePatch, ResourceStore, Workspace, WorkspaceError, WorkspaceStore};
use clap::{Parser, Subcommand};
use uuid::Uuid;

const WORKSPACE_FILE: &str = "proyecto";

#[derive(Debug, Parser)]
#[command(name = "bitto", version, about = "Interfaz de terminal de Bittopos")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Crea un recurso y lo incorpora al workspace por defecto.
    Create {
        /// Título del recurso.
        title: String,

        /// Descripción del recurso.
        #[arg(long)]
        description: Option<String>,

        /// Etiqueta del recurso. Puede repetirse.
        #[arg(long = "tag")]
        tags: Vec<String>,

        /// Crea el recurso sin incorporarlo al workspace.
        #[arg(long)]
        no_instance: bool,
    },

    /// Muestra los títulos del workspace o un recurso completo.
    Show {
        /// UUID del recurso que se quiere consultar.
        id: Option<Uuid>,
    },

    /// Incorpora al workspace un recurso existente.
    Add {
        /// UUID del recurso que se quiere incorporar.
        id: Uuid,
    },
}

struct Paths {
    resources: PathBuf,
    workspaces: PathBuf,
}

impl Paths {
    fn discover() -> Result<Self, String> {
        let home = match env::var_os("BITTOPOS_HOME") {
            Some(path) => PathBuf::from(path),
            None => {
                let home =
                    env::var_os("HOME").ok_or("no se pudo determinar el directorio del usuario")?;

                PathBuf::from(home)
                    .join(".local")
                    .join("share")
                    .join("bittopos")
            }
        };

        Ok(Self {
            resources: home.join("resources"),
            workspaces: home.join("workspaces"),
        })
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        Cli::parse_from(["bitto", "--help"]);
        return Ok(());
    };

    let paths = Paths::discover()?;

    let resources = ResourceStore::new(&paths.resources).map_err(|error| error.to_string())?;

    let workspaces = WorkspaceStore::new(&paths.workspaces).map_err(|error| error.to_string())?;

    match command {
        Command::Create {
            title,
            description,
            tags,
            no_instance,
        } => {
            let (mut resource, _) = resources
                .create(&title, description.as_deref())
                .map_err(|error| error.to_string())?;

            if !tags.is_empty() {
                match resources.update(
                    resource.id,
                    ResourcePatch {
                        tags: Some(tags),
                        ..Default::default()
                    },
                ) {
                    Ok((updated, _)) => resource = updated,
                    Err(error) => {
                        return Err(format!(
                            "el recurso se creó, pero no se pudieron guardar \
                             sus etiquetas: {} ({}). Detalle: {}",
                            resource.info.title, resource.id, error
                        ));
                    }
                }
            }

            if !no_instance {
                let mut workspace = load_workspace(&workspaces)?;

                workspace
                    .add_resource(resource.id)
                    .map_err(|error| error.to_string())?;

                workspaces
                    .save(WORKSPACE_FILE, &workspace)
                    .map_err(|error| {
                        format!(
                            "el recurso se creó, pero no se pudo incorporar \
                             al workspace: {} ({}). Detalle: {}",
                            resource.info.title, resource.id, error
                        )
                    })?;
            }

            println!("Recurso creado: {} ({})", resource.info.title, resource.id);
        }

        Command::Show { id: Some(id) } => {
            let resource = resources.get(id).map_err(|error| error.to_string())?;

            println!("ID: {}", resource.id);
            println!("Título: {}", resource.info.title);

            if let Some(description) = &resource.info.description {
                println!("Descripción: {description}");
            }

            if !resource.info.tags.is_empty() {
                println!("Etiquetas: {}", resource.info.tags.join(", "));
            }
        }

        Command::Show { id: None } => {
            let workspace = load_workspace(&workspaces)?;
            let resolved = workspace
                .resolve(&resources)
                .map_err(|error| error.to_string())?;

            if resolved.is_empty() {
                println!("El workspace está vacío.");
            } else {
                for resource in resolved {
                    println!("- {}", resource.info.title);
                }
            }
        }

        Command::Add { id } => {
            // Comprobamos que el recurso exista antes de registrar la referencia.
            let resource = resources.get(id).map_err(|error| error.to_string())?;

            let mut workspace = load_workspace(&workspaces)?;

            workspace
                .add_resource(id)
                .map_err(|error| error.to_string())?;

            workspaces
                .save(WORKSPACE_FILE, &workspace)
                .map_err(|error| error.to_string())?;

            println!(
                "Recurso añadido al workspace: {} ({})",
                resource.info.title, resource.id
            );
        }
    }

    Ok(())
}

fn load_workspace(store: &WorkspaceStore) -> Result<Workspace, String> {
    match store.load(WORKSPACE_FILE) {
        Ok(workspace) => Ok(workspace),
        Err(WorkspaceError::NotFound(_)) => Ok(Workspace::new()),
        Err(error) => Err(error.to_string()),
    }
}
