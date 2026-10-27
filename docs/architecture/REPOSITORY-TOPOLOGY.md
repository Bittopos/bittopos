# Topología de repositorios de Bittopos

**Estado:** propuesta de organización para el rediseño experimental.

## Principio

Cada repositorio tiene una responsabilidad clara y una fuente de verdad. Los repositorios de componentes mantienen el código y su CI; el repositorio central coordina la composición y publicación del producto. Los sistemas operativos son destinos de compilación, prueba e instalación, no razones para duplicar el dominio.

## Repositorios

| Repositorio | Responsabilidad | Regla |
|---|---|---|
| `Bittopos/bittopos` | Visión, arquitectura transversal, contratos de producto, matriz de compatibilidad, manifiestos y orquestación de releases | No alberga implementaciones de componentes ni recetas específicas de distribución. |
| `Bittopos/bittopos-core` | Modelo y lógica fundamental del producto | Única fuente canónica del core. |
| `Bittopos/bittopos-server` | API, autenticación, autorización y ejecución del servicio | Consume el core mediante una frontera explícita. |
| `Bittopos/bittopos-cli` | CLI para personas operadoras y automatización | No duplica reglas del dominio. |
| `Bittopos/bittopos-ui` | Interfaz web | Consume contratos documentados del servidor. |
| `Bittopos/bittopos-packaging` | Paquetes, recetas de instalación, manifiestos de distribución y pruebas de instalación | Se creará por separado; no asumir que ya existe. |

## Reglas de límites

1. Cada componente mantiene su código, pruebas, documentación local y CI en su repositorio.
2. El repositorio central mantiene documentación transversal y manifiestos que fijan SHAs exactos de cada componente.
3. Las releases nunca dependen de ramas móviles: cada componente se referencia por commit inmutable.
4. Las recetas de paquetes, servicios de sistema y configuración específica de instalación viven en `bittopos-packaging`, no en repositorios por SO.
5. Un adaptador específico de Linux, FreeBSD o Windows vive en el componente al que pertenece, salvo que sea una receta de empaquetado.
6. No crear repositorios de Android, iOS o escritorio hasta que exista una aplicación nativa con propiedad y ciclo de publicación propios.
7. No borrar ni archivar los repositorios actuales de componentes como parte de la limpieza del repositorio central.

## Manifiesto de producto

Un manifiesto de release debe registrar al menos:

- versión y canal de Bittopos;
- SHA y versión de core, server, CLI y UI;
- SHA de packaging si interviene en la release;
- destinos soportados y toolchains;
- pruebas requeridas y su resultado;
- artefactos resultantes, checksums y metadatos de procedencia.

La matriz de compatibilidad debe declarar qué combinaciones de versiones de componentes han sido probadas. Si una combinación no está probada, se marca como tal y no se supone compatible.

## Política de cambios

Los cambios transversales se coordinan mediante PRs en los repositorios afectados. El repositorio central no copia el código de los componentes: actualiza los manifiestos y documentación después de que las revisiones de cada componente hayan sido aprobadas.

## Migración

1. Mantener el historial del repositorio central y retirar de su árbol actual el código fuente de los componentes y la configuración de compilación que solo les corresponde.
2. Conservar la documentación de arquitectura, la visión del producto, la licencia aplicable al repositorio y las reglas de contribución de este repositorio.
3. Retirar workflows que compilan el código eliminado y sustituirlos por coordinación de releases cuando el proceso esté definido y probado.
4. Crear `bittopos-packaging` y trasladar allí recetas y pruebas de instalación, sin copiar implementaciones del dominio.
5. Confirmar que cada componente separado compila y mantiene sus fuentes antes de considerar terminada la transición.
6. No eliminar, renombrar ni archivar repositorios de componentes durante esta migración.
