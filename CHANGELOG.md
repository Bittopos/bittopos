# Changelog

Los cambios relevantes de Bittopos se registran aquí en orden cronológico inverso. Este archivo sigue una estructura inspirada en Keep a Changelog; no reemplaza el historial de commits ni enumera cada cambio interno.

Mientras no existan versiones publicadas, Unreleased reúne los cambios que se preparan para la próxima versión. Las fechas y números de versión se añaden cuando existe un hito verificable.

## [Unreleased]

### Documentation
- Se crea un estado de continuidad que registra la dirección del producto, el inventario real, las decisiones vigentes, los PR integrados y el siguiente trabajo concreto.
- Se alinea la visión, el modelo conceptual, la arquitectura, el estado del proyecto y el README para distinguir el contrato acordado de su implementación pendiente.
- Se corrige la clasificación de las máscaras: su semántica está revisada conceptualmente; lo pendiente es implementar y verificar el motor, no reabrir el diseño.
- Se registra ADR 0004 para la resolución semántica mediante topología, direcciones, máscaras puntuales y Snapshot, comportamiento de componentes, referencias y permisos.
- Se aclara que la secuencia de ingeniería inmediata es una cola de trabajo posterior al PR #28 y no reemplaza la hoja de ruta general original de diez etapas.

### Engineering
- Se fija como siguiente tarea la auditoría del flujo combinado de bitto create (recurso, etiquetas e instancia opcional) y sus garantías ante fallos parciales, antes de extraerlo a la capa de aplicación.
- Se documenta que las pruebas de recuperación de creación/eliminación ya existen en lifecycle.rs; no deben añadirse duplicados sin identificar primero un hueco concreto.
- Se conserva el criterio de refactorización incremental: no cambiar formatos TOML/JSON ni simplificar journal, temporales o recuperación sin pruebas equivalentes.

### Model
- Se documenta la resolución desde la instancia hacia el recurso de origen mediante topología y direcciones semánticas.
- Se distinguen las máscaras puntuales de diferencias locales de las máscaras Snapshot, que congelan las partes maskables comprendidas en la Snapshot; las redefiniciones ordinarias no eliminan máscaras locales.
- Se aclara que los componentes pueden tener reglas específicas de combinación, que las referencias se expanden y que los permisos se resuelven separadamente del contenido.

### Notes
- Estos cambios son documentales. No implementan el motor semántico, las máscaras, la autorización efectiva, un servidor HTTP ni una integración de la interfaz web con el núcleo Rust.
