# Auditoría inicial de la organización Bittopos

**Fecha:** 2026-10-27  
**Estado:** inspección documental y de metadatos; no se han movido ni eliminado archivos o repositorios.

## Hallazgos verificados

Se inspeccionaron los cinco repositorios públicos actuales de la organización: `bittopos`, `bittopos-core`, `bittopos-server`, `bittopos-cli` y `bittopos-ui`. Todos existen, están activos y usan `main` como rama predeterminada.

### 1. El repositorio principal ya contiene un workspace

La raíz de `Bittopos/bittopos` contiene `Cargo.toml`, `Cargo.lock`, código en `src/`, `cli/`, `server/`, `web/` y un adaptador experimental en `backend/`. Su `Cargo.toml` define un paquete llamado `bittopos-core` y un workspace cuyos miembros son `cli` y `server`.

Esto significa que el repositorio principal no es hoy solo un hub documental: ya es un monorepositorio de código funcional. La topología futura debe reconocer ese hecho y no proponer que el mismo repositorio sea simultáneamente fuente canónica y mero orquestador de fuentes externas.

### 2. Hay una segunda distribución del núcleo

El repositorio separado `bittopos-core` también declara el paquete Rust `bittopos-core`. Su README lo describe como núcleo canónico y especifica que CLI y servidor deben depender de él. La raíz de `bittopos` ya contiene un paquete con ese mismo nombre y propósito aproximado.

El servidor y la CLI separados consumen `bittopos-core` mediante dependencias Git fijadas a un SHA. Es una forma reproducible de consumir código, pero mantiene dos formas de organizar el producto y exige coordinar cambios de dominio entre repositorios.

**Conclusión:** antes de desarrollar más funciones, hay que comparar el código y las pruebas de ambas versiones y declarar una sola fuente de verdad. No debe asumirse que son idénticas solo por compartir el nombre del paquete.

### 3. El repositorio de UI es una aplicación web estática

`bittopos-ui` contiene HTML, CSS, JavaScript nativo y un `package.json` sin paso de empaquetado. Su README confirma que consume la API Rust. El repositorio principal también tiene `web/` y documentación sobre el frontend experimental.

**Conclusión:** se debe comparar `web/` con `bittopos-ui` para determinar si son versiones divergentes, si una es la interfaz vigente y qué configuración de despliegue depende de cada una.

### 4. Las licencias forman parte de la arquitectura

La documentación actual declara licencias distintas para el núcleo, la CLI, el frontend y el servidor (Apache-2.0, MPL-2.0 y AGPL-3.0-only). Cualquier consolidación debe preservar los avisos y el alcance de cada licencia; no debe cambiarse una licencia por inferencia ni copiarse código sin revisar sus avisos.

### 5. El servidor aún describe límites importantes

El README del servidor indica que la autenticación actual es una barrera temporal de desarrollo de un solo administrador, que las sesiones se invalidan al reiniciar, que no existen permisos por proyecto ni protección dedicada contra CSRF, y que no debe exponerse a Internet sin trabajo adicional de seguridad. También identifica SQLite local como persistencia actual y aclara que R2 no es un disco POSIX.

Estos límites deben tratarse como trabajo de arquitectura y seguridad, no como un detalle de empaquetado.

## Dirección recomendada

Adoptar `Bittopos/bittopos` como fuente canónica del código, contratos, pruebas y documentación durante el rediseño. Mantener los repositorios separados como consumidores o referencias de migración mientras se verifica la equivalencia; no permitir que nuevas funciones de dominio se implementen de forma divergente en ambos lugares.

La topología y la arquitectura fundacional deben contar una sola historia: un workspace de Rust con límites de responsabilidad reales, aplicaciones de plataforma y adaptadores por sistema operativo. Los repositorios separados solo se conservarán a largo plazo si una frontera de propiedad, licencia, seguridad o publicación independiente justifica el coste de sincronización.

## Próximas etapas

1. Comparar árbol, historial, dependencias y pruebas del núcleo de la raíz con `bittopos-core`.
2. Comparar `cli/` y `server/` del workspace con `bittopos-cli` y `bittopos-server`, incluyendo los SHAs de dependencia fijados.
3. Comparar `web/` con `bittopos-ui` y documentar cuál se despliega realmente.
4. Resolver diferencias de licencias, configuración, persistencia y CI antes de mover o archivar cualquier repositorio.
5. Fijar el contrato del dominio y los límites entre dominio, aplicación, adaptadores, API y CLI; solo después migrar o reimplementar.
6. Añadir pruebas de integración y builds reproducibles para Linux primero, FreeBSD como objetivo explícito y Windows como target futuro sin duplicar el dominio.
7. No archivar, renombrar ni borrar repositorios hasta que la fuente canónica compile, pase pruebas y tenga una ruta de recuperación verificable.

## Fuera del alcance de esta auditoría

Esta inspección no ejecutó los tests ni construyó los binarios, no comparó todos los archivos del repositorio, no comprobó despliegues en producción y no constituye una auditoría de seguridad completa. Esas verificaciones son necesarias antes de aprobar una migración.
