# Bittopos

Bittopos es un **sistema de estructuración semántica, navegación y control de ejecución**. Permite representar sistemas diversos mediante definiciones reutilizables, instancias organizadas jerárquicamente, resolución semántica y relaciones explícitas.

La jerarquía describe dónde se organiza cada elemento; las relaciones describen cómo se conecta con elementos de otras ramas. Juntas permiten modelar desde software y planes de estudio hasta sistemas de especies e interacciones del mundo cotidiano, sin imponer una plantilla de dominio.

## Modelo

- **Recurso:** archivo TOML individual con una definición reutilizable.
- **Estructura:** archivo JSON que organiza las instancias. Es también el espacio de trabajo; no son dos entidades distintas.
- **Instancia:** aparición identificable de un recurso dentro de la estructura.
- **Segmento:** parte identificable de un recurso; también puede existir solo en una instancia.
- **Máscara:** segmento de instancia que guarda diferencias locales respecto al recurso.
- **Resolución semántica:** obtiene los datos efectivos aplicando las reglas de herencia y las diferencias locales.
- **Relación:** definición con identidad propia que conecta recursos o instancias compatibles sin alterar la contención.
- **Referencia:** vínculo explícito de una instancia hacia otra; no equivale a contención ni a una relación general.

El recurso conserva los datos compartidos. La estructura conserva las instancias y su organización. La resolución no debe duplicar la definición completa para cada instancia. El modelo normativo está en [docs/model.md](docs/model.md).

## Estado actual

El núcleo semántico y la CLI `bitto` están implementados en Rust. La SPA modular de `web/` consume una API HTTP v1 experimental; el adaptador Python/SQLite de `backend/` sirve como frontera de persistencia para desarrollo, pero aún no ejecuta la resolución semántica Rust ni es un servicio multiusuario listo para Internet.

La persistencia local del núcleo escribe varios archivos sin transacción conjunta; una interrupción puede dejar cambios parciales. No se debe confundir el prototipo de API con la integración terminada ni el despliegue de assets estáticos con el despliegue del backend.

## Relaciones

Las relaciones tienen identidad propia y no se instancian. Pueden unir recursos o instancias, sin mezclar tipos de extremos:

```bash
bitto relate-resources "Relacionado con" <ID_RECURSO_ORIGEN> <ID_RECURSO_DESTINO>
bitto relate-instances "Depende de" <ID_INSTANCIA_ORIGEN> <ID_INSTANCIA_DESTINO>
bitto show <ID_RELACION>
```

Una relación como «se alimenta de», «acicala a» o «utilizado por» representa un vínculo semántico, no una relación padre-hijo. La jerarquía y la red de relaciones son vistas complementarias del mismo sistema.

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

## Frontend y API experimental

La SPA modular está en `web/` y consume la API HTTP v1. El adaptador experimental Python/SQLite está en `backend/`; ambos se ejecutan juntos en local con `python3 backend/server.py`. Consulta [la guía web](web/README.md), [la guía del backend](backend/README.md) y [el límite arquitectónico actual](docs/api-backed-frontend.md).

La dirección acordada para la siguiente etapa es integrar frontend y núcleo mediante API, completar la refactorización del frontend por responsabilidades y adoptar persistencia remota. La API debe invocar el núcleo Rust real; no se debe duplicar su semántica en JavaScript o Python. El contexto explica la visión y distingue las capacidades actuales de los objetivos: [docs/PRODUCT-CONTEXT.md](docs/PRODUCT-CONTEXT.md).

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

- [Contexto de producto y desarrollo](docs/PRODUCT-CONTEXT.md): conexión entre visión, decisiones y dirección de arquitectura.
- [Visión](docs/VISION.md): propósito y principios.
- [Modelo](docs/model.md): conceptos y reglas semánticas.
- [Arquitectura](docs/architecture.md): módulos, archivos y flujo actual.
- [Límite frontend/API](docs/api-backed-frontend.md): situación y contratos experimentales.
- [Guía de desarrollo](docs/DEVELOPMENT-GUIDE.md): cambios pequeños y comprobaciones.
- [Contribución](CONTRIBUTING.md): ramas y Pull Requests.

El código demuestra el comportamiento disponible; el modelo define la semántica; el contexto de producto conecta ambos con la dirección acordada. No describir una capacidad futura como ya implementada.
