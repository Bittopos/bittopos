# Arquitectura actual

## Recorrido principal

```text
Perfil (profile.json: UUID único)
   └── proyectos/<UUID>/
       ├── project.json + project.toml
       ├── resources/*.toml
       └── workspaces/*.json
              ▲
              │
CLI / otras interfaces → casos de uso compartidos del núcleo
```

La estructura es el único contenedor de organización de las instancias. Los recursos, relaciones y referencias son archivos TOML independientes; las relaciones y referencias se leen directamente sin instanciación. El segmento de control `__relation` contiene únicamente `from` y `to`; el tipo de ambos componentes (`resource-id` o `instance-id`) determina el espacio de identidad. Una referencia contiene el componente `point` (`instance-id`) hacia una instancia y su ID se registra en `semantic.references` de la instancia propietaria. La semántica de una instancia no crea componentes de relaciones o referencias por defecto: ambos son opcionales y explícitos. `allocation` sigue siendo el componente de contención; `references` solo aparece cuando se asocia una referencia. La resolución combina la definición compartida con las diferencias locales de la instancia.

## Módulos

- `profile.rs`: identidad única del espacio de trabajo local en `profile.json`.
- `project.rs`: ficha de proyecto JSON/TOML, asociación al perfil y almacén de carpetas UUID.
- `resource.rs`: modelo de recurso, metadatos y referencias directas a instancias, identificadas por `__reference.point`.
- `relations.rs`: creación, lectura y validación de extremos de relaciones.
- `store.rs`: lectura y escritura de recursos TOML.
- `workspace_model.rs`: instancias, contención y reglas del modelo de estructura.
- `workspace.rs`: serialización JSON de la estructura.
- `semantic_masks.rs`: segmentos, conceptos, componentes, máscaras y resolución.
- `mask_operations.rs`: operaciones de edición compartida/local y resolución de instancias.
- `lifecycle.rs`: carga, creación y eliminación de instancias.
- `navigation.rs`: ubicación actual de navegación.
- `application.rs`: casos de uso compartidos de perfiles, proyectos y operaciones de estructura.
- `cli/src/main.rs`: interfaz de terminal.

Los módulos se separan por responsabilidad. Una separación adicional solo se justifica cuando un archivo contiene responsabilidades independientes que se puedan mover sin crear dependencias circulares.

## Perfil, proyectos y fichas

El perfil local contiene únicamente su UUID. Los proyectos se almacenan en `projects/<UUID>/`, llevan una ficha `project.json` y otra `project.toml` con los mismos metadatos, y tienen almacenes independientes `resources/` y `workspaces/`. La CLI selecciona explícitamente el proyecto cuando hay varios; con un único proyecto lo selecciona automáticamente. Las fichas y sus instancias se persisten dentro del proyecto activo, no en un almacén global compartido.

La creación/listado de proyectos está en `application.rs` y `project.rs`, por lo que otras interfaces pueden reutilizar los mismos casos de uso. La SPA aún necesita un adaptador de servidor/API para invocarlos desde el navegador.

## Persistencia y límites

Las operaciones escriben directamente los archivos implicados. La escritura del recurso y de la estructura no forma una transacción conjunta; un fallo entre ambas escrituras puede dejar un estado parcial. No hay journal ni reconciliación automática.

La prioridad es que el camino normal de datos sea visible. No se incorporan mecanismos de recuperación, temporales administrados, transacciones entre archivos o adaptadores futuros sin una decisión expresa.

## Fuera del núcleo actual

La interfaz web es experimental y guarda su propio estado en el navegador. No está conectada al núcleo Rust. Servidor, sincronización, autenticación, bases de datos externas y despliegue WASM no forman parte del flujo local actual.
