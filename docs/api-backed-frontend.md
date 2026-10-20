# Frontend API-backed: alcance de la iteración

## Decisiones vigentes

- La SPA se rehace con módulos ES nativos y mantiene un modelo de UI independiente.
- La interfaz consume una API HTTP versionada (`/api/v1`) y no accede directamente a archivos ni a SQLite.
- El adaptador experimental guarda documentos de recursos y estructura en SQLite; la API entrega JSON al navegador.
- Rust sigue siendo la autoridad del comportamiento semántico existente. Esta API todavía no invoca ni reimplementa su resolución.
- No se define aún qué responsabilidades finales vivirán en frontend o backend.
- Las interfaces evolucionan por separado, con contratos versionados y pruebas de compatibilidad compartidas. La coordinación entre extremos se pospone hasta después del MVP.
- CLI `push`, `pull` y `log`, gRPC y sincronización multiplataforma quedan explícitamente fuera de alcance.

## Camino de datos provisional

```text
SPA modular
  │ HTTP / JSON
  ▼
API v1 experimental
  │ validación estructural + persistencia
  ▼
SQLite: recursos JSON (+ TOML original importado) y estructura JSON
  │ GET /api/v1/state
  ▼
JSON para el navegador
```

La API no es el núcleo semántico. La base de datos incluye tablas de perfiles y proyectos con relaciones por clave foránea; recursos e instancias de cada proyecto también tienen proyecciones relacionales consultables. La importación TOML utiliza `tomllib`, conserva el texto original y guarda los campos estructurados. No se convierte todo TOML arbitrario a TOML canónico automáticamente porque eso podría perder detalles o tipos que el contrato aún no ha definido.

## Contrato de compatibilidad inicial

- Recurso: conserva `id` y `info`; los campos desconocidos no se eliminan al editar el recurso desde la SPA.
- Estructura: conserva `content.instances`, los IDs de instancias, `source.id`, y campos adicionales.
- Contención: al añadir/quitar hijos, `content.instances` y `semantic.allocation` se actualizan juntos en el cliente. Este comportamiento necesita fixtures del modelo completo antes de ser considerado canónico.
- Escrituras de estructura: la SPA envía `expectedRevision`; el API rechaza revisiones obsoletas con HTTP 409.
- Exportación de la SPA: JSON con `resources` y `workspace`, para respaldar y mover datos sin reconstruir un TOML incompleto.

## Seguridad y despliegue

El servidor Python está limitado a loopback por defecto y no debe exponerse directamente a Internet. No se incluye un sistema de cuentas ni autenticación de sesión para la SPA. Cloudflare Pages sirve archivos estáticos, no este proceso Python; el backend remoto requerirá otro entorno de ejecución y un diseño de autenticación antes de producción.

## Próximos pasos antes de aprobar el MVP

1. Ejecutar tests de contrato sobre recursos TOML reales y estructuras que incluyan máscaras, referencias, relaciones y campos desconocidos.
2. Diseñar y probar autenticación/autorización por perfil y proyecto antes de exponer datos privados.
2. Definir un formato de exportación JSON versionado y una estrategia de migración de archivos locales a SQLite.
3. Decidir si la base inicial debe ser SQLite o un servicio gestionado según el entorno real de despliegue y los requisitos multiusuario.
4. Diseñar autenticación/autorización y separación de datos antes de aceptar usuarios externos.
5. Repartir responsabilidades semánticas entre frontend/backend con pruebas, no por suposición.
6. Publicar fixtures compartidos y checks de contrato para que cada interfaz pueda evolucionar independientemente sin romper compatibilidad.
