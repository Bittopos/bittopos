# Bittopos

Bittopos es un proyecto desarrollado en Rust para crear, almacenar, consultar y organizar recursos mediante espacios de trabajo que mantienen referencias a sus recursos originales.

El proyecto se desarrolla de forma incremental, separando el núcleo reutilizable de las interfaces que lo utilizan.

## Estado del proyecto

- **Versión del paquete del núcleo:** `0.1.0`.
- **Línea de desarrollo:** contrato técnico `0.3.0` (identidad de instancias y registro inicial de asignaciones en implementación).
- **Interfaz disponible:** CLI `bitto`.
- **Entorno inicial:** local, incluido Termux en Android.

La versión del paquete indicada en `Cargo.toml` identifica la versión declarada del paquete. El contrato técnico describe el alcance previsto; no implica por sí solo que todas sus características estén implementadas ni que se haya publicado una versión con ese número.

## Arquitectura actual

El repositorio utiliza un workspace de Cargo con dos paquetes:

- **`bittopos-core`:** define recursos, instancias, persistencia, navegación y espacios de trabajo.
- **`bitto-cli`:** proporciona la interfaz de terminal y coordina las operaciones del núcleo.

La CLI delega las reglas de identidad, contención y ciclo de vida al núcleo. `store.rs` gestiona las definiciones de recursos; `workspace.rs` representa y persiste el árbol recursivo; `lifecycle.rs` coordina la creación, remoción y recuperación de instancias; `navigation.rs` persiste la ubicación actual en TOML. La separación conceptual completa de `Resource`, `Segment` y las demás entidades queda pendiente de una refactorización posterior.

## Modelo de datos

### Recursos

Cada recurso tiene un identificador UUID estable y contiene información descriptiva:

- `title`: título obligatorio.
- `description`: descripción opcional.
- `tags`: lista de etiquetas, que puede estar vacía.

Cada recurso se persiste en un archivo TOML independiente. El segmento administrativo `management.structural` registra los IDs de las instancias conocidas que proceden de esa definición; el núcleo mantiene ese registro fuera de las operaciones públicas de edición descriptiva.

### Espacios de trabajo

Un espacio de trabajo contiene instancias persistidas en JSON bajo `content.instances`. Cada instancia tiene un `id` propio y referencia su recurso de origen mediante `source.id`. Sus hijos completos se almacenan recursivamente en `content.instances`, mientras `semantic.allocation` contiene los IDs de sus hijos directos. Ambas vistas deben coincidir en contenido y orden; los componentes vacíos pueden omitirse. Varias instancias pueden compartir un recurso de origen sin compartir identidad.

El esquema recursivo define `Workspace.content.instances` como la lista de instancias raíz. Cada instancia contiene `content.instances` con sus hijos completos y `semantic.allocation` con los IDs de esos mismos hijos directos. Cuando existen hijos, ambos componentes deben coincidir en contenido y orden. Si no hay hijos, cualquiera puede omitirse o ser un arreglo vacío. Las reglas completas se especifican en [CONTRATO-0.3.md](CONTRATO-0.3.md).

El esquema actualmente implementado exige `id` propio y `source.id` en cada instancia. La compatibilidad con `resource_id` se ha eliminado deliberadamente: los JSON antiguos se rechazan sin modificarlos. `management.structural` registra las identidades de instancias por recurso de origen, pero no contiene información de padres e hijos y, por sí solo, no permite reconstruir el árbol. Antes de reconciliar, el núcleo valida estrictamente todos los recursos y exige que cada uno contenga `management.structural`; si un recurso es ilegible, carece de ese registro o hay IDs asignados a varios recursos, la recuperación se detiene sin reescribir el workspace.

Crear una definición y crear una instancia son operaciones distintas. En el código actual, la CLI delega el ciclo de vida al núcleo: primero confirma el registro en el recurso y después guarda el workspace. El contrato define un journal duradero y el orden de escritura journal → recurso TOML → workspace JSON → retirar journal. La remoción de instancia elimina su asignación y su aparición en el workspace, pero conserva el recurso de origen; la eliminación destructiva del recurso es una operación diferente y queda aplazada. Todo esto sigue siendo diseño documental, no funcionalidad ya implementada.

## Almacenamiento local

La ubicación raíz de los datos puede configurarse mediante la variable de entorno `BITTOPOS_HOME`.

Si no está definida, se utiliza por defecto:

```text
~/.local/share/bittopos/
├── resources/
└── workspaces/
```

El almacenamiento no debe depender accidentalmente del directorio desde el que se ejecuta el programa.

## Uso de la CLI

Durante el desarrollo, los comandos pueden ejecutarse desde la raíz del repositorio mediante Cargo.

Mostrar la ayuda:

```bash
cargo run -p bitto-cli -- --help
```

Crear un recurso y añadir su instancia en la ubicación actual (en la raíz, si no se ha navegado a otra instancia):

```bash
cargo run -p bitto-cli -- create "Aprender Rust" \
  --description "Notas sobre el lenguaje" \
  --tag software \
  --tag aprendizaje
```

Crear un recurso sin añadirlo al espacio de trabajo:

```bash
cargo run -p bitto-cli -- create "Idea pendiente" --no-instance
```

Mostrar los recursos referenciados por el espacio de trabajo:

```bash
cargo run -p bitto-cli -- show
```

Consultar un recurso mediante su UUID:

```bash
cargo run -p bitto-cli -- show UUID_DEL_RECURSO
```

Añadir una instancia de un recurso existente dentro de la ubicación actual (se permite repetir el mismo recurso):

```bash
cargo run -p bitto-cli -- add UUID_DEL_RECURSO
```

### Navegación

La ubicación actual se guarda en `workspaces/navigation.toml`, separada del árbol de instancias. La raíz se representa con un contexto vacío. Los comandos disponibles para navegar son:

```bash
# Mostrar los hijos directos de la ubicación actual
cargo run -p bitto-cli -- ls

# Entrar en una instancia hija (UUID propio de instancia)
cargo run -p bitto-cli -- enter UUID_DE_INSTANCIA

# Subir al padre, volver a la raíz o consultar la ubicación actual
cargo run -p bitto-cli -- up
cargo run -p bitto-cli -- root
cargo run -p bitto-cli -- pwd
```

`show` sin UUID continúa mostrando el árbol completo; `show UUID` consulta una definición de recurso. Al crear o añadir una instancia, esta se guarda dentro de la ubicación actual. Si la ubicación guardada ya no existe, la navegación se restablece a la raíz. Un archivo TOML malformado produce un error explícito y no se sobrescribe automáticamente.

El nombre `proyecto.json` es una convención provisional para el archivo del espacio de trabajo por defecto, no una implementación de proyectos o perfiles.

## Desarrollo y verificación

Se requiere una instalación de Rust y Cargo.

Comprobar el formato:

```bash
cargo fmt --all -- --check
```

Comprobar la compilación de todos los paquetes:

```bash
cargo check --workspace
```

Ejecutar las pruebas con las dependencias registradas en `Cargo.lock`:

```bash
cargo test --workspace --locked
```

Revisar los cambios antes de registrarlos:

```bash
git status --short
git diff --check
git diff
```

Las comprobaciones deben ejecutarse sobre el estado real del código antes de registrar un incremento.

## Contratos técnicos

- [CONTRATO-0.1.md](CONTRATO-0.1.md): contrato de la etapa inicial.
- [CONTRATO-0.2.md](CONTRATO-0.2.md): contrato de la etapa previa.
- [CONTRATO-0.3.md](CONTRATO-0.3.md): modelo previsto de recursos, instancias, segmentos y navegación; varias partes siguen pendientes de implementación.

Los contratos describen las responsabilidades y los criterios de aceptación de cada etapa. Deben mantenerse alineados con la implementación real.

## Fuera del alcance actual

Todavía no forman parte de esta etapa:

- Perfiles y gestión de proyectos.
- Relaciones avanzadas entre recursos.
- Permisos configurables.
- Sincronización remota y colaboración.
- Servidor HTTP y API de red.
- Interfaz web.

Estas funcionalidades se evaluarán en incrementos posteriores, sin anticipar estructuras que todavía no se necesitan.
