# Bittopos

Bittopos es un proyecto en Rust para modelar, organizar y navegar información mediante **recursos**, **instancias** y relaciones estructurales. Su objetivo es ayudar a externalizar ideas y construir modelos que las personas definen y controlan.

> **Estado:** prototipo en desarrollo. La interfaz operativa actual es la CLI `bitto`, orientada al uso local. El modelo de segmentos, conceptos, componentes y máscaras está en definición; no todas las capacidades conceptuales están implementadas.

## Qué existe hoy

- Workspace de Cargo con la biblioteca `bittopos-core` y la CLI `bitto-cli`.
- Definiciones de recursos en TOML.
- Espacio de trabajo local en JSON, con instancias de identidad propia y contención recursiva.
- Navegación persistente.
- Validación estructural y recuperación mediante journal para operaciones del ciclo de vida.

Consulta [Estado del proyecto](docs/status.md) para distinguir las capacidades implementadas de las decisiones conceptuales y del trabajo pendiente.

## Prototipo web local

Existe una interfaz estática experimental en [`web/`](web/README.md). Permite importar y exportar un workspace y recursos desde el navegador, pero no está conectada al núcleo Rust ni modifica directamente los archivos locales de Bittopos.

## Inicio rápido

Requisitos: Rust y Cargo instalados.

```bash
cargo run -p bitto-cli -- --help
```

Crear un recurso (por defecto también crea una instancia en la ubicación actual):

```bash
cargo run -p bitto-cli -- create "Aprender Rust" \
  --description "Notas sobre el lenguaje" \
  --tag software --tag aprendizaje
```

Crear solo la definición, sin instancia:

```bash
cargo run -p bitto-cli -- create "Idea pendiente" --no-instance
```

Mostrar el espacio de trabajo:

```bash
cargo run -p bitto-cli -- show
```

Los comandos pueden cambiar durante el desarrollo. Usa siempre `--help` para consultar la revisión instalada.

## Desarrollo

Desde la raíz del repositorio:

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
git diff --check
```

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para ramas, commits y Pull Requests, y [la guía de ingeniería](docs/DEVELOPMENT-GUIDE.md) para criterios de diseño, pruebas, persistencia y uso de herramientas de IA.

## Documentación vigente

- [Visión del producto y arquitectura objetivo](docs/VISION.md)
- [Estado del proyecto y alcance](docs/status.md)
- [Modelo conceptual](docs/model.md)
- [Arquitectura actual y dirección](docs/architecture.md)
- [Decisiones arquitectónicas (ADR)](docs/decisions/README.md)
- [Guía de ingeniería](docs/DEVELOPMENT-GUIDE.md)
- [Desarrollo y contribución](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)
- [Documentación histórica](docs/archive/README.md)

Los contratos numerados antiguos están en [`docs/archive/`](docs/archive/README.md); no son la fuente normativa vigente.

## Principios de trabajo

1. La documentación distingue el modelo acordado del comportamiento implementado.
2. Las decisiones importantes conservan su motivo y sus consecuencias.
3. Cada cambio tiene un alcance verificable y pruebas adecuadas.
4. El changelog resume cambios relevantes, no cada commit.
5. Las versiones de Cargo, las etiquetas Git y las decisiones documentales cumplen funciones distintas.

## Licencia

Consulta [LICENSE](LICENSE).
