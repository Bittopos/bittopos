# Auditoría inicial de la organización Bittopos

**Fecha:** 2026-10-27  
**Alcance:** inspección de metadatos, configuración y archivos principales; no se ejecutaron builds ni pruebas.

## Hallazgos

La organización contiene cinco repositorios de producto: `bittopos`, `bittopos-core`, `bittopos-server`, `bittopos-cli` y `bittopos-ui`.

- El repositorio central contenía un workspace Rust con un paquete llamado `bittopos-core`, miembros `cli` y `server`, una SPA en `web/` y un backend experimental en `backend/`.
- El repositorio `bittopos-core` también contiene un paquete Rust con el mismo nombre. No debe suponerse que ambos núcleos sean equivalentes sin una comparación exhaustiva.
- Los archivos `cli/src/main.rs` y `bittopos-cli/src/main.rs` tenían el mismo SHA en la inspección.
- Los archivos `server/src/main.rs` y `bittopos-server/src/main.rs` tenían el mismo SHA.
- En la UI, `main.js`, `model.js` y `ui.js` coincidían entre `web/` y `bittopos-ui`; `api.js` era diferente.
- La documentación de componentes declara licencias distintas. Cualquier movimiento de código debe preservar los avisos y revisar la compatibilidad de licencias.
- La documentación del servidor describe límites de seguridad que deben resolverse antes de declararlo listo para producción multiusuario.

## Decisión de topología

El repositorio central se utilizará como hub de documentación, contratos de producto, manifiestos y coordinación de releases. Los repositorios de componentes conservarán sus implementaciones. Las recetas de instalación y distribución específica se organizarán en `bittopos-packaging`, que el propietario creará por separado.

## Alcance pendiente

La inspección no comparó todos los archivos del core, historiales, CI, dependencias completas ni despliegues; tampoco ejecutó tests o builds. Antes de cambiar dependencias entre componentes, se debe completar esa auditoría en sus repositorios. Esta documentación no constituye una auditoría de seguridad completa.
