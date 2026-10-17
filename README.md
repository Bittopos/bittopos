# Bittopos

Bittopos es un sistema de estructuración semántica y control de ejecución. El proyecto permite modelar, organizar y navegar información mediante recursos, instancias y relaciones estructurales, con un modelo que las personas conservan bajo su control.

> **Estado:** prototipo local en desarrollo. La CLI bitto es la interfaz operativa actual. El motor de dominio de máscaras puntuales y Snapshot ya existe en el núcleo y su estado se persiste en los formatos locales; los comandos CLI, la expansión de referencias y la autorización efectiva todavía no están implementados.

## Qué existe hoy

- Workspace Rust con la biblioteca bittopos-core y la CLI bitto-cli.
- Definiciones de recursos en TOML con metadatos actuales: título, descripción, etiquetas y registro de instancias.
- Workspace local JSON con instancias de identidad propia, referencias a recursos de origen y contención recursiva.
- Registro estructural por recurso y asignación de contención directa en el workspace.
- Navegación persistente local.
- Operaciones de ciclo de vida coordinadas mediante journal durable y recuperación.
- Frontera de aplicación para preparar el workspace, crear instancias en la ubicación actual y eliminar instancias actualizando la navegación.
- Motor semántico inicial para Segment/Concept/Component, roles de segmento, máscaras puntuales y Snapshot, con resolución por definición vigente y diferencias locales.
- Máscaras serializadas en instancias y segmentos semánticos adicionales persistidos en TOML; la metadata histórica se proyecta como segmento reservado info.
- Operaciones de núcleo para editar conceptos/componentes locales o compartidos, crear Snapshot, retirar máscaras y resolver una instancia.
- Pruebas de integridad y recuperación para distintos estados intermedios e inconsistentes.

Consulta [Estado del proyecto](docs/status.md) para el inventario completo y [Estado de trabajo y continuidad](docs/WORKING-STATE.md) para el punto exacto de continuación.

## Prototipo web local

Existe una interfaz estática experimental en [web/](web/README.md). Permite importar y exportar un workspace y recursos desde el navegador, pero no está conectada al núcleo Rust ni modifica directamente los archivos locales de Bittopos.

## Inicio rápido

Requisitos: Rust y Cargo instalados.

    cargo run -p bitto-cli -- --help

Crear un recurso (por defecto también crea una instancia en la ubicación actual):

    cargo run -p bitto-cli -- create "Aprender Rust" --description "Notas sobre el lenguaje" --tag software --tag aprendizaje

Crear solo la definición, sin instancia:

    cargo run -p bitto-cli -- create "Idea pendiente" --no-instance

Mostrar el espacio de trabajo:

    cargo run -p bitto-cli -- show

Los comandos pueden cambiar durante el desarrollo. Usa siempre --help para consultar la revisión instalada.

## Desarrollo

Desde la raíz del repositorio:

    cargo fmt --all -- --check
    cargo build --workspace --locked
    cargo test --workspace --locked
    git diff --check

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para ramas, commits y Pull Requests, y [la guía de ingeniería](docs/DEVELOPMENT-GUIDE.md) para criterios de diseño, pruebas, persistencia y uso de herramientas de IA.

## Documentación vigente

- [Visión del producto y arquitectura objetivo](docs/VISION.md)
- [Estado del proyecto y alcance](docs/status.md)
- [Estado de trabajo y continuidad](docs/WORKING-STATE.md)
- [Modelo conceptual](docs/model.md)
- [Implementación del motor de máscaras](docs/semantic-masks-engine.md)
- [Arquitectura actual y dirección](docs/architecture.md)
- [Decisiones arquitectónicas (ADR)](docs/decisions/README.md)
- [Guía de ingeniería](docs/DEVELOPMENT-GUIDE.md)
- [Desarrollo y contribución](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)
- [Documentación histórica](docs/archive/README.md)

Los contratos numerados antiguos están en docs/archive/; se conservan por trazabilidad y no son la fuente normativa vigente.

## Principios de trabajo

1. La documentación distingue el modelo acordado del comportamiento implementado.
2. Las decisiones importantes conservan su motivo y sus consecuencias.
3. Cada cambio tiene un alcance verificable y pruebas adecuadas.
4. El changelog resume cambios relevantes, no cada commit.
5. Las versiones de Cargo, las etiquetas Git y las decisiones documentales cumplen funciones distintas.

## Licencia

Consulta LICENSE.
