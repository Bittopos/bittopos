# Bittopos

Bittopos es un proyecto en Rust para modelar, organizar y navegar información mediante **recursos**, **instancias** y relaciones estructurales. Su objetivo es ayudar a externalizar ideas y construir modelos que las personas definen y controlan.

> **Estado:** prototipo en desarrollo. La interfaz disponible es la CLI `bitto`, orientada al uso local. El modelo de segmentos, conceptos, componentes y máscaras está en definición; no todas las capacidades descritas en la documentación conceptual están implementadas.

## Qué existe hoy

- Workspace de Cargo con la biblioteca `bittopos-core` y la CLI `bitto-cli`.
- Definiciones de recursos almacenadas como TOML.
- Espacio de trabajo local almacenado como JSON, con instancias de identidad propia y contención recursiva.
- Navegación persistente de la CLI.
- Validación del árbol, registro estructural de instancias y recuperación mediante journal para operaciones de ciclo de vida.

Consulta [Estado y alcance](docs/status.md) para distinguir las capacidades implementadas de las decisiones del modelo y el trabajo pendiente.

## Inicio rápido

Requisitos: Rust y Cargo instalados.

Desde la raíz del repositorio:

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

Los comandos disponibles pueden cambiar durante el desarrollo. Usa siempre `--help` para consultar la CLI de la revisión que tengas instalada.

## Desarrollo

Verifica los paquetes del workspace, no solo el paquete raíz:

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
```

Antes de abrir una PR, revisa:

```bash
git status --short
git diff --check
git diff
```

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para el flujo de ramas, commits, Pull Requests y criterios de revisión.

## Documentación

- [Estado del proyecto y próximos pasos](docs/status.md)
- [Modelo conceptual vigente](docs/model.md)
- [Arquitectura: estado actual y dirección](docs/architecture.md)
- [Decisiones arquitectónicas (ADR)](docs/decisions/README.md)
- [Guía de desarrollo y contribución](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)
- [Documentación histórica](docs/archive/README.md)

Los documentos `CONTRATO-*.md` que permanecen en la raíz son referencias históricas de etapas anteriores. **No son la fuente de verdad vigente**: las reglas actuales están en `docs/`, y el estado de implementación se registra por separado.

## Principios de trabajo

1. La documentación distingue el modelo acordado del comportamiento que el código implementa hoy.
2. Las decisiones importantes registran su motivo y sus consecuencias; las decisiones sustituidas no se borran del historial.
3. Cada cambio de código tiene un alcance pequeño, criterios verificables y pruebas adecuadas.
4. El changelog resume cambios relevantes para usuarios y desarrolladores; no duplica cada commit.
5. Las versiones de Cargo, las versiones publicadas y las decisiones/documentos no tienen que compartir números.

## Licencia

Consulta [LICENSE](LICENSE).
