# Contribuir a Bittopos

Este repositorio coordina el producto Bittopos. No es el lugar para implementar cambios en el core, servidor, CLI, UI ni recetas de instalación.

## Dónde contribuir

- Cambios del dominio: [bittopos-core](https://github.com/Bittopos/bittopos-core)
- API y servicio: [bittopos-server](https://github.com/Bittopos/bittopos-server)
- CLI: [bittopos-cli](https://github.com/Bittopos/bittopos-cli)
- Interfaz web: [bittopos-ui](https://github.com/Bittopos/bittopos-ui)
- Empaquetado: `bittopos-packaging` cuando esté creado.

## En este repositorio

Las contribuciones deben tratar sobre arquitectura transversal, contratos entre componentes, matriz de compatibilidad, manifiestos de producto, documentación o coordinación de releases.

- Usa una rama y una pull request.
- Describe el motivo, alcance, decisiones y efectos sobre otros componentes.
- Fija SHAs inmutables en los manifiestos de releases.
- No declares una plataforma compatible sin builds y pruebas reproducibles.
- No copies implementaciones de componentes a este repositorio.
- No sobrescribas artefactos de release existentes.

Los cambios de arquitectura que afecten varios componentes deben identificar qué repositorios necesitan cambios coordinados.
