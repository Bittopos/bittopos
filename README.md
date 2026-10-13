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

- **`bittopos-core`:** define los recursos, la persistencia y los espacios de trabajo.
- **`bitto-cli`:** proporciona la interfaz de terminal y coordina las operaciones del núcleo.

La CLI no debe duplicar la lógica de validación ni implementar un sistema de persistencia independiente. Dentro del núcleo, `store.rs` gestiona archivos de definiciones, `workspace.rs` representa y persiste el estado del workspace, y `lifecycle.rs` coordina las operaciones de creación, eliminación y recuperación. La implementación actual todavía conserva una lista plana de instancias; la contención recursiva descrita en el contrato es el siguiente incremento de diseño y no debe confundirse con funcionalidad ya implementada.

## Modelo de datos

### Recursos

Cada recurso tiene un identificador UUID estable y contiene información descriptiva:

- `title`: título obligatorio.
- `description`: descripción opcional.
- `tags`: lista de etiquetas, que puede estar vacía.

Cada recurso se persiste en un archivo TOML independiente. El segmento administrativo `management.structural` registra los IDs de las instancias conocidas que proceden de esa definición; el núcleo mantiene ese registro fuera de las operaciones públicas de edición descriptiva.

### Espacios de trabajo

Un espacio de trabajo contiene instancias persistidas en JSON. Cada instancia tiene un `id` propio y referencia su recurso de origen mediante `source.id`. Varias instancias pueden compartir el mismo recurso de origen sin compartir identidad. La información descriptiva del recurso no se copia en el espacio de trabajo.

El esquema recursivo propuesto para el siguiente incremento define `Workspace.content.instances` como la lista de instancias raíz. Cada instancia contendrá `content.instances` con sus hijos completos y `semantic.allocation` con los IDs de esos mismos hijos directos. Ambos arreglos deben coincidir en contenido y orden, incluso cuando estén vacíos. Este esquema aún no está implementado en el código actual; se especifica en [CONTRATO-0.3.md](CONTRATO-0.3.md).

El esquema actualmente implementado exige `id` propio y `source.id` en cada instancia. La compatibilidad con `resource_id` se ha eliminado deliberadamente: los JSON antiguos se rechazan sin modificarlos. `management.structural` registra las identidades de instancias por recurso de origen, pero no contiene información de padres e hijos y, por sí solo, no permite reconstruir el árbol. Antes de reconciliar, el núcleo valida estrictamente todos los recursos y exige que cada uno contenga `management.structural`; si un recurso es ilegible, carece de ese registro o hay IDs asignados a varios recursos, la recuperación se detiene sin reescribir el workspace.

Crear una definición y crear una instancia son operaciones distintas. En el código actual, la CLI delega el ciclo de vida al núcleo: primero confirma el registro en el recurso y después guarda el workspace. El protocolo con journal duradero para coordinar estos archivos durante la contención recursiva está definido en el contrato, pero aún no está implementado.

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

Crear un recurso y añadirlo al espacio de trabajo por defecto:

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

Añadir una nueva instancia de un recurso existente al espacio de trabajo (se permite repetir el mismo recurso):

```bash
cargo run -p bitto-cli -- add UUID_DEL_RECURSO
```

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
