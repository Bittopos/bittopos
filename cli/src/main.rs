use std::env;
use std::path::PathBuf;
use std::process;

use bittopos_core::{
    create_instance, reconcile_workspace, remove_instance, ResourcePatch, ResourceStore,
    Workspace, WorkspaceStore,
};
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
    /// Crea una definición y, salvo --no-instance, una instancia en el workspace.
    Create {
        title: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long = "tag")]
        tags: Vec<String>,
        #[arg(long)]
        no_instance: bool,
    },
    /// Muestra las instancias del workspace o consulta una definición por UUID.
    Show { id: Option<Uuid> },
    /// Crea una instancia de una definición existente.
    Add { id: Uuid },
    /// Elimina una instancia por su UUID propio.
    Remove { id: Uuid },
    /// Reconstruye el workspace a partir de las asignaciones confirmadas.
    Recover,
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
                PathBuf::from(home).join(".local").join("share").join("bittopos")
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
    let workspaces =
        WorkspaceStore::new(&paths.workspaces).map_err(|error| error.to_string())?;

    // Toda invocación comienza reconciliando el workspace con allocation.
    let (startup_workspace, startup_report) =
        reconcile_workspace(&resources, &workspaces, WORKSPACE_FILE)
            .map_err(|error| error.to_string())?;
    report_recovery(startup_report.restored, startup_report.removed);

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
                    ResourcePatch { tags: Some(tags), ..Default::default() },
                ) {
                    Ok((updated, _)) => resource = updated,
                    Err(error) => {
                        return Err(format!(
                            "el recurso se creó, pero no se pudieron guardar sus etiquetas: {} ({}). Detalle: {}",
                            resource.info.title, resource.id, error
                        ));
                    }
                }
            }

            let instance_id = if no_instance {
                None
            } else {
                Some(create_instance(&resources, &workspaces, WORKSPACE_FILE, resource.id)
                    .map_err(|error| format!(
                        "el recurso se creó, pero no se pudo crear su instancia: {error}"
                    ))?)
            };

            println!("Recurso creado: {} ({})", resource.info.title, resource.id);
            if let Some(instance_id) = instance_id {
                println!("Instancia creada: {instance_id}");
            }
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
            print_workspace(&startup_workspace, &resources)?;
        }
        Command::Add { id } => {
            let resource = resources.get(id).map_err(|error| error.to_string())?;
            let instance_id = create_instance(&resources, &workspaces, WORKSPACE_FILE, id)
                .map_err(|error| error.to_string())?;
            println!(
                "Instancia añadida: {} — recurso {} (instancia {})",
                resource.info.title, resource.id, instance_id
            );
        }
        Command::Remove { id } => {
            remove_instance(&resources, &workspaces, WORKSPACE_FILE, id)
                .map_err(|error| error.to_string())?;
            println!("Instancia eliminada: {id}");
        }
        Command::Recover => {
            println!(
                "Workspace consistente: {} instancia(s).",
                startup_workspace.instances.len()
            );
        }
    }

    Ok(())
}

fn report_recovery(restored: usize, removed: usize) {
    if restored > 0 || removed > 0 {
        eprintln!(
            "Recuperación aplicada: {restored} instancia(s) reconstruida(s), {removed} instancia(s) residual(es) eliminada(s)."
        );
    }
}

fn print_workspace(
    workspace: &Workspace,
    resources: &ResourceStore,
) -> Result<(), String> {
    let resolved = workspace.resolve(resources).map_err(|error| error.to_string())?;
    if resolved.is_empty() {
        println!("El workspace está vacío.");
    } else {
        for (instance, resource) in workspace.instances.iter().zip(resolved.iter()) {
            println!(
                "- {} [{}] — recurso {}",
                resource.info.title, instance.id, instance.source.id
            );
        }
    }
    Ok(())
}
