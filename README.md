# Bittopos

Bittopos es un producto compuesto por componentes independientes. Este repositorio es el **centro de coordinación del producto**, no el repositorio fuente del núcleo, el servidor, la CLI ni la interfaz.

## Responsabilidades de este repositorio

- Mantener la visión del producto y la arquitectura transversal.
- Documentar la topología de repositorios, contratos entre componentes y políticas de versiones.
- Coordinar releases mediante manifiestos que fijan commits inmutables de los componentes.
- Documentar la generación y publicación de archivos fuente, artefactos, checksums y procedencia.
- Mantener workflows de coordinación y publicación cuando hayan sido probados.

## Repositorios de componentes

- [bittopos-core](https://github.com/Bittopos/bittopos-core): núcleo y modelo del producto.
- [bittopos-server](https://github.com/Bittopos/bittopos-server): servicio y API.
- [bittopos-cli](https://github.com/Bittopos/bittopos-cli): interfaz de línea de comandos.
- [bittopos-ui](https://github.com/Bittopos/bittopos-ui): interfaz web.
- `bittopos-packaging`: repositorio de empaquetado y recetas de instalación, que se creará por separado.

Los nombres y las responsabilidades no implican que todas las plataformas estén soportadas ya. Linux será el primer destino de referencia; FreeBSD debe ser un objetivo explícito y Windows se añadirá como destino de compilación y distribución cuando pueda verificarse. Los sistemas operativos no se separan en repositorios que dupliquen el dominio.

## Documentación de arquitectura

- [Fundamentos](docs/architecture/FOUNDATION.md)
- [Topología de repositorios](docs/architecture/REPOSITORY-TOPOLOGY.md)
- [Auditoría inicial](docs/architecture/INITIAL-REPOSITORY-AUDIT.md)
- [Política de releases](docs/releases/README.md)

## Releases reproducibles

Cada release debe fijar los SHAs exactos de los componentes y registrar la versión, los destinos, la configuración de compilación, las pruebas ejecutadas y los artefactos publicados. Los archivos fuente, los binarios y los paquetes de instalación se identificarán por separado. No se publicarán releases desde ramas móviles ni se sobrescribirán artefactos existentes.

## Estado

El producto está en rediseño experimental. Las propuestas de arquitectura se revisan mediante pull requests. No se debe interpretar la presencia de un componente o de un destino en la documentación como garantía de que está listo para producción.
