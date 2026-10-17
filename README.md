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

Requisitos: Rust y Cargo.

```bash
cargo run -p bitto-cli -- --help
cargo run -p bitto-cli -- create "Aprender Rust" --description "Notas del lenguaje"
cargo run -p bitto-cli -- show
```

Crear una definición sin instancia:

```bash
cargo run -p bitto-cli -- create "Idea pendiente" --no-instance
```

## Verificación

Desde la raíz:

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
git diff --check
```

## Documentación

- [Visión](docs/VISION.md): propósito y dirección.
- [Modelo](docs/model.md): conceptos y reglas semánticas.
- [Arquitectura](docs/architecture.md): archivos, módulos y flujo de datos.
- [Guía de desarrollo](docs/DEVELOPMENT-GUIDE.md): cambios pequeños y comandos.
- [Contribución](CONTRIBUTING.md): ramas y Pull Requests.

La documentación describe el comportamiento vigente. Las decisiones sustituidas y los contratos históricos no se conservan como parte de la documentación operativa.
