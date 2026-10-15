use std::env;
use std::path::PathBuf;
use std::process;

use bittopos_core::{
    children_of, create_instance_at_current_location, create_instance_under, prepare_workspace,
    remove_instance_and_update_navigation, Instance, NavigationState, NavigationStore,
    ResourcePatch, ResourceStore, Workspace, WorkspaceModelError, WorkspaceStore,
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
    /// Crea una definición y, salvo --no-instance, una instancia en la ubicación actual.
    Create {
        title: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long = "tag")]
        tags: Vec<String>,
        #[arg(long)]
        no_instance: bool,
    },
    /// Muestra el árbol completo o consulta una definición por UUID.
    Show { id: Option<Uuid> },
    /// Lista las instancias hijas directas de la ubicación actual.
    Ls,
    /// Entra en una instancia hija de la ubicación actual.
    Enter { id: Uuid },
    /// Sube a la instancia padre; desde la raíz permanece en la raíz.
    Up,
    /// Vuelve a la raíz del espacio de trabajo.
    Root,
    /// Muestra la ubicación de navegación actual.
    Pwd,
    /// Crea una instancia de una definición existente dentro de la ubicación actual.
    Add { id: Uuid },
    /// Elimina una instancia hoja por su UUID propio.
    Remove { id: Uuid },
    /// Verifica y recupera el workspace a partir de los registros confirmados.
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
    let navigation = NavigationStore::new(&paths.workspaces);

    // La capa de aplicación prepara el contexto consistente para la interfaz.
    let prepared = prepare_workspace(&resources, &workspaces, &navigation, WORKSPACE_FILE)
        .map_err(|error| error.to_string())?;
    report_recovery(prepared.recovery.restored, prepared.recovery.removed);
    let startup_workspace = prepared.workspace;
    let mut nav_state = prepared.navigation_state;
    let navigation_reset = prepared.navigation_reset;
    if navigation_reset {
        eprintln!("La ubicación guardada ya no existe; navegación restablecida a la raíz.");
    }

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
                            "el recurso se creó, pero no se pudieron guardar sus etiquetas: {} ({}). Detalle: {}",
                            resource.info.title, resource.id, error
                        ));
                    }
                }
            }

            let instance_id = if no_instance {
                None
            } else {
                Some(
                    create_instance_under(
                        &resources,
                        &workspaces,
                        WORKSPACE_FILE,
                        resource.id,
                        nav_state.current_instance,
                    )
                    .map_err(|error| {
                        format!("el recurso se creó, pero no se pudo crear su instancia: {error}")
                    })?,
                )
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
        Command::Ls => {
            let children =
                children_of(&startup_workspace, nav_state.current_instance).map_err(|error| {
                    match error {
                        WorkspaceModelError::ParentNotFound(id) => {
                            format!("la ubicación actual ya no existe: {id}")
                        }
                        other => other.to_string(),
                    }
                })?;
            if children.is_empty() {
                println!("No hay instancias en esta ubicación.");
            } else {
                print_instances(children, &resources, 0)?;
            }
        }
        Command::Enter { id } => {
            let children =
                children_of(&startup_workspace, nav_state.current_instance).map_err(|error| {
                    match error {
                        WorkspaceModelError::ParentNotFound(id) => {
                            format!("no existe la ubicación actual: {id}")
                        }
                        other => other.to_string(),
                    }
                })?;
            if !children.iter().any(|child| child.id == id) {
                return Err(format!(
                    "la instancia {id} no es hija directa de la ubicación actual; usa 'bitto ls' para consultar las opciones"
                ));
            }
            nav_state.current_instance = Some(id);
            navigation
                .save(&nav_state)
                .map_err(|error| error.to_string())?;
            print_location(&nav_state, &startup_workspace, &resources)?;
        }
        Command::Up => {
            nav_state.current_instance = nav_state
                .current_instance
                .and_then(|id| startup_workspace.parent_of(id));
            navigation
                .save(&nav_state)
                .map_err(|error| error.to_string())?;
            print_location(&nav_state, &startup_workspace, &resources)?;
        }
        Command::Root => {
            nav_state = NavigationState::default();
            navigation
                .save(&nav_state)
                .map_err(|error| error.to_string())?;
            println!("Ubicación actual: raíz del espacio de trabajo.");
        }
        Command::Pwd => {
            print_location(&nav_state, &startup_workspace, &resources)?;
        }
        Command::Add { id } => {
            let resource = resources.get(id).map_err(|error| error.to_string())?;
            let instance_id = create_instance_at_current_location(
                &resources,
                &workspaces,
                WORKSPACE_FILE,
                id,
                &nav_state,
            )
            .map_err(|error| error.to_string())?;
            println!(
                "Instancia añadida: {} — recurso {} (instancia {})",
                resource.info.title, resource.id, instance_id
            );
        }
        Command::Remove { id } => {
            let navigation_reset = remove_instance_and_update_navigation(
                &resources,
                &workspaces,
                &navigation,
                WORKSPACE_FILE,
                id,
                &mut nav_state,
            )
            .map_err(|error| error.to_string())?;
            if navigation_reset {
                println!(
                    "La ubicación eliminada era la actual; navegación restablecida a la raíz."
                );
            }
            println!("Instancia removida: {id}");
        }
        Command::Recover => {
            println!(
                "Workspace consistente: {} instancia(s).",
                startup_workspace.all_instances().len()
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

fn print_location(
    state: &NavigationState,
    workspace: &Workspace,
    resources: &ResourceStore,
) -> Result<(), String> {
    match state.current_instance {
        None => println!("Ubicación actual: raíz del espacio de trabajo."),
        Some(id) => {
            let instance = workspace
                .find_instance(id)
                .ok_or_else(|| format!("no existe la ubicación actual: {id}"))?;
            let resource = resources
                .get(instance.source.id)
                .map_err(|error| error.to_string())?;
            println!(
                "Ubicación actual: {} [{}]",
                resource.info.title, instance.id
            );
        }
    }
    Ok(())
}

fn print_workspace(workspace: &Workspace, resources: &ResourceStore) -> Result<(), String> {
    if workspace.all_instances().is_empty() {
        println!("El workspace está vacío.");
        return Ok(());
    }
    print_instances(workspace.instances(), resources, 0)
}

fn print_instances(
    instances: &[Instance],
    resources: &ResourceStore,
    depth: usize,
) -> Result<(), String> {
    for instance in instances {
        let resource = resources
            .get(instance.source.id)
            .map_err(|error| error.to_string())?;
        println!(
            "{}- {} [{}] — recurso {}",
            "  ".repeat(depth),
            resource.info.title,
            instance.id,
            instance.source.id
        );
        print_instances(instance.children(), resources, depth + 1)?;
    }
    Ok(())
}
