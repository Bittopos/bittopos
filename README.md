# Bittopos

Bittopos es un **sistema de estructuración semántica y control de ejecución**. Su modelo local se construye con recursos reutilizables, instancias organizadas en una estructura y una resolución que determina los datos efectivos.

## Modelo

- **Recurso:** archivo TOML individual con una definición reutilizable.
- **Estructura:** archivo JSON que organiza las instancias. Es el espacio de trabajo; no son dos entidades distintas.
- **Instancia:** aparición identificable de un recurso dentro de la estructura.
- **Segmento:** parte identificable de un recurso; también puede existir solo en una instancia.
- **Máscara:** segmento de instancia que guarda diferencias locales respecto al recurso.
- **Resolución semántica:** obtiene los datos efectivos aplicando las reglas de herencia y las diferencias locales.

El recurso conserva los datos compartidos. La estructura conserva las instancias y su organización. La resolución no debe duplicar la definición completa para cada instancia.

## Estado actual

Prototipo local en Rust. La CLI `bitto` es la interfaz operativa principal. Hay una interfaz web experimental que funciona con datos del navegador y todavía no utiliza el núcleo Rust.

La persistencia entre archivos no es transaccional: las operaciones escriben directamente sus archivos y una interrupción puede dejar cambios parciales. No existe journal ni recuperación automática.

## Relaciones

Las relaciones son recursos de definición con identidad propia; no se instancian. Se pueden crear relaciones entre recursos o entre instancias, sin mezclar los tipos de extremos:

```bash
bitto relate-resources "Relacionado con" <ID_RECURSO_ORIGEN> <ID_RECURSO_DESTINO>
bitto relate-instances "Depende de" <ID_INSTANCIA_ORIGEN> <ID_INSTANCIA_DESTINO>
bitto show <ID_RELACION>
```

`bitto show <ID_RELACION>` muestra los metadatos y los extremos de la relación.

## Inicio rápido

Requisitos: Rust y Cargo. Cada perfil contiene proyectos; cada proyecto tiene un UUID propio y guarda sus recursos y estructuras dentro de su directorio. Crea el perfil y después un proyecto:

```bash
cargo run -p bitto-cli -- profile create
cargo run -p bitto-cli -- project create "Aprender Rust" --description "Notas del lenguaje"
cargo run -p bitto-cli -- project list
```

Si hay un único proyecto, los comandos de fichas lo seleccionan automáticamente. Si hay varios, indica `--project <UUID>`:

```bash
cargo run -p bitto-cli -- --project <UUID> create "Primera ficha"
cargo run -p bitto-cli -- --project <UUID> show
```

Crear una definición sin instanciarla dentro de la estructura:

```bash
cargo run -p bitto-cli -- --project <UUID> create "Idea pendiente" --no-instance
```

Los datos se guardan bajo `BITTOPOS_HOME` si está definida esa variable; por defecto se usa `~/.local/share/bittopos/`.

## Servidor HTTP y frontend

El servicio HTTP Rust está en `server/`, reutiliza el núcleo y ofrece inicio de sesión de desarrollo y operaciones por perfil/proyecto. Consulta [la guía del servidor](server/README.md). La imagen Docker se construye desde la raíz con `docker build -t bittopos-server .` y necesita un volumen persistente para SQLite.

El frontend modular está en `web/` y se publica bajo la licencia MPL 2.0; el núcleo, CLI y servidor están bajo GNU AGPLv3. El adaptador Python/SQLite de `backend/` sigue siendo experimental y no es el servidor remoto recomendado. Consulta [la guía web](web/README.md), [la guía del backend](backend/README.md) y [el límite arquitectónico](docs/api-backed-frontend.md).

## Adaptador Python experimental

El directorio `backend/` conserva el prototipo Python/SQLite usado durante la exploración del frontend; no es el servidor remoto objetivo. La aplicación en desarrollo utiliza el servidor Rust de `server/`. Consulta [la guía web](web/README.md), [la guía del servidor](server/README.md) y [la documentación histórica del adaptador](backend/README.md).

## Verificación

Desde la raíz:

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
git diff --check
npm --prefix web run check
python -m unittest discover -s backend/tests -v
```

## Documentación

- [Visión](docs/VISION.md): propósito y dirección.
- [Modelo](docs/model.md): conceptos y reglas semánticas.
- [Arquitectura](docs/architecture.md): archivos, módulos y flujo de datos.
- [Persistencia SQLite](docs/database-storage.md): esquema estructurado para perfiles, proyectos, recursos e instancias.
- [Guía de desarrollo](docs/DEVELOPMENT-GUIDE.md): cambios pequeños y comandos.
- [Contribución](CONTRIBUTING.md): ramas y Pull Requests.

La documentación describe el comportamiento vigente. Las decisiones sustituidas y los contratos históricos no se conservan como parte de la documentación operativa.
