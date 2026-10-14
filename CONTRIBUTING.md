# Desarrollo y contribución

Esta guía describe el flujo normal de trabajo de Bittopos. Está pensada para cambios incrementales y revisables; no exige crear un documento formal para cada tarea.

## 1. Antes de cambiar código

1. Lee [Estado del proyecto](docs/status.md) y la sección pertinente de la documentación.
2. Busca una Issue existente. Si el trabajo es sustancial, crea una Issue con problema, alcance y criterios de aceptación.
3. Si el cambio altera una regla conceptual o una frontera arquitectónica, actualiza la especificación o propone un ADR.
4. Asegúrate de que el cambio no mezcla limpieza incidental con nuevas funcionalidades.

Las tareas pequeñas y obvias pueden ir directamente a una rama y PR sin crear una Issue por obligación.

## 2. Ramas y commits

Trabaja en una rama por cambio lógico; evita desarrollar continuamente sobre `main`.

Nombres recomendados:

- `docs/...`: documentación.
- `feat/...`: funcionalidad nueva.
- `fix/...`: corrección.
- `refactor/...`: cambio interno sin alterar el comportamiento intencionado.
- `test/...`: pruebas.
- `ci/...`: automatización.

Usa mensajes claros, preferiblemente con Conventional Commits:

- `docs: clarify mask inheritance rules`
- `feat(core): resolve inherited segments`
- `fix(workspace): reject duplicate instance ids`
- `refactor(core): separate model validation`
- `test(core): cover mask fallback`

Conventional Commits ayuda a leer el historial y automatizar versiones; no sustituye la revisión ni exige que cada commit sea una versión.

## 3. Pull Request

Una PR debe explicar:

- **Motivo:** qué problema resuelve.
- **Cambio:** qué se modificó y qué queda fuera.
- **Criterios de aceptación:** cómo se comprobará.
- **Pruebas:** qué comandos se ejecutaron y sus resultados.
- **Documentación:** qué páginas o decisiones se actualizaron.

Mantén las PR pequeñas cuando eso facilite la revisión. Una PR puede incluir documentación y código si describen el mismo cambio.

No marques como implementado algo que solo está documentado. Integra en `main` después de revisar el diff y el resultado de CI.

## 4. Verificación local

Desde la raíz del repositorio:

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
git diff --check
```

Si un comando no se ejecutó, indícalo explícitamente en la PR. Las pruebas del workspace importan porque el repositorio contiene más de un paquete Cargo.

## 5. Responsabilidades documentales

- **README:** presentación y puerta de entrada.
- **`docs/VISION.md`:** propósito y dirección estratégica.
- **`docs/model.md`:** reglas conceptuales vigentes.
- **`docs/architecture.md`:** límites actuales y dirección técnica.
- **`docs/status.md`:** capacidades implementadas, pendientes y aplazadas.
- **ADR:** decisiones técnicas importantes, alternativas y consecuencias.
- **`docs/DEVELOPMENT-GUIDE.md`:** normas de ingeniería y límites de responsabilidad.
- **Issues:** trabajo pendiente y criterios de aceptación.
- **PR:** registro revisable de un cambio concreto.
- **CHANGELOG:** cambios relevantes agrupados por versión.
- **Histórico:** contratos antiguos y decisiones sustituidas.

Evita mantener la misma especificación completa en varios documentos. Enlaza la fuente de autoridad en lugar de copiarla.

## 6. Versiones y changelog

La versión de un paquete Rust se declara en su `Cargo.toml`; Git registra commits y referencias, pero no decide automáticamente qué versión debe tener la aplicación. Las etiquetas Git y GitHub Releases se crean cuando se decide publicar una versión.

Mientras Bittopos sea un prototipo, registra en `CHANGELOG.md` los cambios funcionales y arquitectónicos relevantes. No incrementes la versión del paquete por cada PR. Al preparar una publicación, decide el incremento SemVer según compatibilidad y alcance, actualiza los metadatos pertinentes, valida el workspace y etiqueta el commit publicado.
