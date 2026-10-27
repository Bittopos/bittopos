# Fundamentos de arquitectura de Bittopos

**Estado:** propuesta para el rediseño experimental.  
**Principio rector:** una sola fuente de verdad por componente, contratos explícitos y portabilidad verificada mediante adaptadores y pruebas, no mediante duplicación por sistema operativo.

## 1. Responsabilidad del repositorio central

`Bittopos/bittopos` es el centro de coordinación del producto. Contiene visión, arquitectura transversal, topología de repositorios, políticas de versiones, manifiestos de releases y workflows de orquestación. No mantiene implementaciones del núcleo, servidor, CLI, UI ni recetas específicas de instalación.

El código vive en los repositorios de sus componentes. Los releases del producto reúnen revisiones exactas de esos repositorios, fijadas por SHA inmutable.

## 2. Responsabilidades de los componentes

- **Core:** modelo de dominio, entidades, valores, invariantes y operaciones fundamentales; sin dependencia de UI, HTTP o un sistema operativo concreto.
- **Server:** API, autenticación, autorización, sesiones, coordinación de casos de uso y operación como servicio.
- **CLI:** interfaz para operadores y automatización; consume contratos/casos de uso publicados por los componentes, sin reimplementar reglas de negocio.
- **UI:** experiencia web que consume la API documentada.
- **Packaging:** recetas, manifiestos y pruebas de instalación específicas de distribución, en `Bittopos/bittopos-packaging` una vez creado.
- **Central:** contratos entre componentes a nivel de producto, matriz de compatibilidad, manifiestos de composición y coordinación de releases.

Cada componente es dueño de su código, pruebas y CI. Un cambio transversal debe coordinar las versiones de los componentes afectados y actualizar el manifiesto de producto.

## 3. Modelo de dependencias

1. Las reglas de dominio pertenecen al core y no dependen del framework HTTP, la interfaz ni del sistema operativo.
2. El servidor y la CLI consumen la misma semántica del core mediante una frontera explícita y versionada.
3. La UI depende de contratos de API documentados, no de estructuras internas de implementación.
4. Las integraciones específicas de SO quedan en adaptadores o módulos de plataforma dentro del componente que los necesita.
5. No crear capas, crates ni repositorios vacíos por anticipado: una frontera se extrae cuando existe una responsabilidad o variación verificable.
6. No permitir que el mismo componente tenga dos implementaciones canónicas mantenidas en paralelo.

## 4. Sistemas operativos y entornos

- **Linux:** primera plataforma de referencia para desarrollo, CI, servidor y empaquetado.
- **FreeBSD:** destino real de portabilidad, con builds y pruebas propias; no basta con que el código compile en Linux.
- **Windows:** destino futuro previsto, sin repositorio preventivo por SO. Su soporte se declarará solo después de verificar toolchain, servicios, rutas, permisos, señales, almacenamiento y distribución.
- **Android y otros clientes móviles:** se incorporan como aplicaciones o repositorios propios cuando exista código nativo y un ciclo de publicación real.

La portabilidad se organiza por contratos, adaptadores, destinos de compilación y pruebas. No por copias del dominio.

## 5. Inventario y observabilidad

Bittopos debe distinguir el estado del host que ejecuta el producto del estado del producto administrado.

- El inventario de hardware/software registra la fuente de la observación, el momento, el alcance y las limitaciones de permisos.
- La ausencia de permiso o de una fuente de datos significa “desconocido/no accesible”, no “inexistente”.
- Las métricas de infraestructura (salud del host, CPU, memoria, almacenamiento, disponibilidad) se separan de las métricas de producto (usuarios, solicitudes, acciones y latencia).
- La observabilidad no debe requerir privilegios administrativos para las operaciones ordinarias. Los colectores con privilegios se aíslan y aplican mínimo privilegio.

## 6. Seguridad y datos

La autenticación y autorización pertenecen a la arquitectura del servidor, no a una configuración de despliegue opcional. Antes de declarar soporte multiusuario o producción se deben validar identidad, permisos por recurso, gestión de sesiones, protección CSRF cuando aplique, límites de intentos, transporte seguro, auditoría y pruebas de seguridad.

Los cambios de diseño pueden ser incompatibles durante la fase experimental. Sin embargo, una migración o reinicialización de datos persistentes debe ser explícita, documentada y recuperable; no debe destruir datos silenciosamente.

## 7. Releases y procedencia

Una release de producto fija:
- versión y canal;
- SHA de cada repositorio componente;
- targets y requisitos mínimos;
- toolchain, features y opciones relevantes;
- pruebas ejecutadas y limitaciones conocidas;
- lista de artefactos, SHA-256 y metadatos de firma/procedencia cuando estén configurados.

Fuentes, binarios y paquetes de instalación son artefactos distintos. Nunca se reconstruye una release a partir de una rama móvil ni se sobrescriben artefactos publicados.

## 8. Secuencia

1. Dejar este repositorio central libre de implementaciones de componentes.
2. Auditar cada repositorio componente y confirmar que sus fuentes, pruebas, licencias y CI están completas.
3. Fijar contratos entre core, server, CLI y UI.
4. Crear `bittopos-packaging` y trasladar allí las recetas específicas de instalación cuando el repositorio esté disponible.
5. Definir un manifiesto de producto con SHAs exactos y comprobaciones de compatibilidad.
6. Unificar la coordinación de releases sin ocultar la independencia de CI de cada componente.
7. Verificar Linux primero, FreeBSD como target explícito y Windows cuando existan pruebas reales.

## Criterio de éxito

El repositorio central describe y compone el producto sin duplicar el código de sus componentes; cada componente tiene una fuente canónica; y cada release puede reconstruirse y auditarse a partir de commits y herramientas declarados.
