# Guía de desarrollo

## Requisitos

Instala Rust y Cargo. El workspace contiene el paquete `bittopos-core` y la CLI `bitto-cli`.

## Antes de cambiar código

Lee [el contexto de producto](PRODUCT-CONTEXT.md), [el modelo](model.md) y [la arquitectura](architecture.md), además de la guía específica del directorio que vayas a tocar. Confirma el comportamiento en el código: la documentación distingue el núcleo Rust actual de la SPA/API experimental y de la arquitectura objetivo.

## Construir y probar

Desde la raíz del repositorio:

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
git diff --check
```

Para ejecutar la CLI:

```bash
cargo run -p bitto-cli -- --help
```

Para las comprobaciones del frontend y del adaptador experimental:

```bash
npm --prefix web run check
python -m unittest discover -s backend/tests -v
```

## Incrementos revisables

1. Identifica una regla, capacidad o comportamiento que pueda verificarse.
2. Localiza su autoridad canónica y todos sus consumidores.
3. Cambia la regla en un lugar; no dupliques semántica en UI, API y persistencia.
4. Añade o actualiza pruebas y fixtures que representen el comportamiento observable.
5. Ejecuta las verificaciones pertinentes y comunica con precisión cuáles se ejecutaron.
6. Actualiza el documento canónico si cambia una decisión aprobada.
7. Trabaja en una rama y no fusiones en `main` sin autorización expresa.

## Límites de diseño

- Recurso, estructura, instancia, segmento, máscara, relación, referencia y resolución son conceptos distintos.
- No separe estructura y espacio de trabajo en entidades paralelas.
- No duplique la definición completa del recurso en cada instancia.
- No convierta relaciones en contención ni intercambie IDs de recursos e instancias.
- No descarte campos desconocidos o semánticos silenciosamente.
- No añadas abstracciones sin consumidores reales.
- No declares una integración terminada hasta que las operaciones semánticas pasen por el núcleo Rust real y existan pruebas de contrato.
- No expongas el adaptador experimental a Internet sin autenticación, autorización, aislamiento y revisión de seguridad.
- La persistencia local actual no es transaccional entre archivos; describe esa limitación con claridad.

## Organización documental

- `PRODUCT-CONTEXT.md`: propósito, decisiones de producto y dirección acordada.
- `model.md`: reglas normativas del modelo.
- `architecture.md`: distribución real del código y flujo actual.
- `api-backed-frontend.md`: contrato y limitaciones de la integración experimental.

Evita mantener copias extensas de la misma información. Si código y documentación discrepan, registra la discrepancia con evidencia en vez de suponer que el objetivo ya está implementado.
