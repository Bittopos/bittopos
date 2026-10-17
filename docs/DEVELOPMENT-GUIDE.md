# Guía de desarrollo

## Requisitos

Instala Rust y Cargo. El workspace contiene el paquete `bittopos-core` y la CLI `bitto-cli`.

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

## Incrementos pequeños

1. Identifica una sola regla o comportamiento.
2. Localiza su modelo canónico y sus consumidores.
3. Cambia la regla en un lugar y evita repetirla en la interfaz o persistencia.
4. Añade o actualiza pruebas del comportamiento visible.
5. Ejecuta formato, compilación y pruebas.
6. Actualiza el documento que sea fuente de autoridad si cambia el modelo.

## Límites de diseño

- Recurso, estructura, instancia, segmento, máscara y resolución son los conceptos centrales.
- No separe estructura y espacio de trabajo en entidades paralelas.
- No duplique la definición completa del recurso en cada instancia.
- No añadas abstracciones sin consumidores reales.
- No añadas recuperación automática, mecanismos de rescate, telemetría, autenticación o infraestructura futura sin aprobación.
- La persistencia actual no es transaccional entre archivos; describe esa limitación con claridad.

## Organización documental

El modelo se define en `model.md`; la distribución del código se describe en `architecture.md`. El README presenta el proyecto y esta guía describe cómo construirlo y modificarlo. Evita mantener copias extensas de la misma información.
