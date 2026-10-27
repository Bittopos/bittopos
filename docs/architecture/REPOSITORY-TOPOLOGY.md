# Topología de repositorios y modelo de publicación

**Estado:** propuesta revisada a partir de la auditoría inicial del código actual.  
**Objetivo:** definir una única fuente de verdad y evitar implementaciones divergentes.

## Principio

Los repositorios siguen límites estables de responsabilidad, no nombres de sistemas operativos. Linux, FreeBSD y Windows son destinos de compilación, prueba, integración y distribución; no requieren copias del dominio por sistema operativo.

## Hecho actual y decisión objetivo

La auditoría de `main` confirma que `Bittopos/bittopos` ya contiene un workspace Rust, el paquete raíz `bittopos-core`, miembros `cli` y `server`, una SPA en `web/` y un backend Python experimental. También existen repositorios separados con otro paquete `bittopos-core`, una CLI, un servidor y una SPA.

Por tanto, la propuesta anterior de tratar `bittopos/bittopos` como un hub sin código fuente contradice el estado real y la arquitectura fundacional. **La decisión propuesta es hacer de `Bittopos/bittopos` la fuente canónica del producto y su código durante el rediseño.** Los repositorios separados se mantienen temporalmente mientras se comparan; no se asumirán equivalentes ni se archivarán automáticamente.

## Responsabilidades de destino

| Ruta o repositorio | Responsabilidad objetivo | Estado y regla |
|---|---|---|
| `Bittopos/bittopos` | Fuente canónica: workspace Rust, contratos, pruebas, documentación, aplicaciones y CI | Ya contiene un workspace y varias aplicaciones. Ampliarlo por límites reales de responsabilidad. |
| `crates/bittopos-domain` | Entidades, valores, invariantes y reglas puras del dominio | Crear al extraer límites comprobables; no crear crates vacíos por adelantado. |
| `crates/bittopos-application` | Casos de uso y coordinación | No conoce frameworks ni adaptadores concretos. |
| `crates/bittopos-contracts` y `crates/bittopos-ports` | Contratos públicos, errores, eventos e interfaces requeridas | Versionados y probados; separar solo cuando aporte una frontera clara. |
| `crates/bittopos-storage` | Adaptadores de persistencia y transacciones | El dominio no depende de SQLite, TOML, archivos ni servicios externos. |
| `crates/bittopos-inventory` | Modelo y procedencia de observaciones de host | Diferencia hardware/software del host del producto que se administra. |
| `crates/bittopos-observability` | Salud, métricas, trazas y correlación | Separa métricas de infraestructura y métricas de producto. |
| `crates/bittopos-runtime` | Configuración, ciclo de vida, apagado y señales | Encapsula diferencias de sistema operativo mediante adaptadores. |
| `apps/web` | SPA web | Comparar primero con `web/` y `bittopos-ui`; elegir una sola versión canónica. |
| `apps/android` | Cliente Android nativo | Incorporar cuando exista código y requisitos reales de plataforma. |
| `apps/desktop` | Shell de escritorio | Solo si hay integración nativa que justifique una aplicación separada. |
| `platforms/linux`, `platforms/freebsd`, `platforms/windows` | Adaptadores, pruebas y configuración específicos | Añadir código real por plataforma; nunca duplicar reglas de dominio. |
| `deploy/` y `packaging/` | Servicios, instalaciones, paquetes y pruebas de instalación | Mantener recetas pequeñas y verificables junto al producto mientras no exista un motivo de propiedad independiente. |

## Tratamiento de los repositorios separados actuales

- `bittopos-core`: comparar su historial, módulos, persistencia y pruebas con el paquete raíz del mismo nombre. Declarar un ganador tras una comparación explícita, no por nombre ni por tamaño.
- `bittopos-server`: comparar con `server/`, incluidas autenticación, sesión, SQLite, configuración, Docker y licencias.
- `bittopos-cli`: comparar con `cli/`, incluidos nombres de comandos, formatos, pruebas y licencias.
- `bittopos-ui`: comparar con `web/`, incluidos adaptador API, orígenes de despliegue, validaciones y licencias.

Hasta completar esas comparaciones, no hacer nuevas funciones que deban mantenerse en paralelo. Los repositorios pueden quedar como archivos de transición o convertirse después en repositorios de distribución solo si existe una necesidad demostrable. La decisión final se documentará antes de cualquier archivo o renombre.

No crear ahora un repositorio de empaquetado solo por anticipación: primero evaluar si `packaging/` y `deploy/` dentro del monorepo cubren las necesidades. Separarlo más adelante solo si tiene ciclo de publicación, permisos o responsables realmente independientes.

## Límites de dependencia

1. El dominio no depende del sistema operativo, red, UI, framework HTTP ni motor de persistencia.
2. Los casos de uso consumen contratos/puertos; adaptadores concretos se conectan en los puntos de entrada.
3. Servidor, CLI y clientes llaman a los mismos casos de uso o consumen los mismos contratos; no reimplementan reglas de negocio.
4. La aplicación normal no necesita privilegios de administrador. La recopilación de inventario usa permisos mínimos y comunica explícitamente capacidades no disponibles.
5. Las observaciones de host incluyen fuente, instante, alcance y confianza. Falta de permiso significa desconocido/no accesible, no inexistente.
6. No introducir capas genéricas ni repositorios por SO sin una variación real, una frontera de seguridad o un ciclo de mantenimiento independiente.

## Publicaciones reproducibles

La fuente canónica publica releases de producto con:
- versión y canal de producto;
- SHA inmutable del commit y versión de cada componente relevante;
- targets y versiones mínimas soportadas;
- toolchain, features y configuración de compilación;
- resultados de pruebas, limitaciones conocidas y procedencia;
- nombres de artefactos, checksums SHA-256 y firmas/metadatos verificables cuando estén configurados.

Los archivos de código fuente y los paquetes binarios se identifican por separado. Un archivo `.tar.gz` de fuentes no se presenta como binario portable. Nunca se construye una release desde una rama móvil ni se sobrescriben artefactos publicados.

## Secuencia de migración

1. Auditar contenidos, historiales, licencias, dependencias, workflows y despliegues de cada repositorio.
2. Comparar las implementaciones actuales y elegir la fuente canónica de cada responsabilidad.
3. Definir modelo de dominio, contratos, persistencia y límites de seguridad antes de reorganizar módulos.
4. Reestructurar el workspace en cambios pequeños, conservando los avisos de licencia por componente.
5. Unificar CI y pruebas de contrato/integración; comprobar instalación limpia y recuperación.
6. Añadir pruebas reales para Linux y FreeBSD; mantener Windows como target previsto y verificarlo antes de anunciar soporte.
7. Crear manifiestos de releases y ensamblado de fuentes/artefactos a partir de commits exactos.
8. Solo tras validar la migración, decidir si los repositorios separados se archivan, redirigen o siguen con un papel distinto.

## Seguridad y cambios incompatibles

Bittopos sigue en fase experimental y no promete compatibilidad con APIs, CLI ni formatos todavía en diseño. Los cambios incompatibles se pueden adoptar deliberadamente, pero los datos persistentes no se deben destruir silenciosamente: toda reinicialización o migración debe ser explícita y documentada.

La autenticación actual del servidor se describe como una barrera temporal de desarrollo de un solo administrador. No se debe declarar listo para producción multiusuario ni exponerlo a Internet hasta completar la revisión de autenticación, autorización por recurso, protección CSRF, limitación de intentos, gestión de sesiones, HTTPS y pruebas de seguridad pertinentes.

## Criterio de éxito

Existe una sola implementación canónica del dominio; las interfaces comparten su semántica; los targets se verifican con pruebas y artefactos reproducibles; y cada repositorio que permanezca separado tiene una frontera de propiedad justificada.
