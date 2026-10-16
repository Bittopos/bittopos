# Changelog

Los cambios relevantes de Bittopos se registran aquí en orden cronológico inverso. Este archivo sigue una estructura inspirada en Keep a Changelog; no reemplaza el historial de commits ni enumera cada cambio interno.

Mientras no existan versiones publicadas, Unreleased reúne los cambios que se preparan para la próxima versión. Las fechas y números de versión se añaden cuando existe un hito verificable.

## [Unreleased]

### Engineering
- Se prepara una reestructuración del núcleo centrada en el modelo semántico y las máscaras; el flujo de `bitto create` queda pausado hasta nuevo alcance.

### Documentation
- Se añade una guía de preparación con límites de compatibilidad, criterios de aceptación y secuencia incremental para el cambio del núcleo.
- Se crea un estado de continuidad que registra la dirección del producto, el inventario real, las decisiones vigentes, los PR integrados y el siguiente trabajo concreto.
- Se alinea la visión, el modelo conceptual, la arquitectura, el estado del proyecto y el README para distinguir el contrato acordado de su implementación pendiente.
- Se corrige la clasificación de las máscaras: su semántica está revisada conceptualmente; lo pendiente es implementar y verificar el motor, no reabrir el diseño.
- Se registra ADR 0004 para la resolución semántica mediante topología, direcciones, máscaras puntuales y Snapshot, comportamiento de componentes, referencias y permisos.
- Se añade ADR 0005 para precisar que una máscara es un Segment local de instancia, conservar las reglas acordadas de diferencias locales/herencia dinámica/Snapshot y separar seis casos estructurales que requieren decisiones explícitas.
- Se aclara que la secuencia de ingeniería inmediata es una cola de trabajo posterior al PR #28 y no reemplaza la hoja de ruta general original de diez etapas.

### Engineering
- Se pausa la auditoría del flujo combinado de bitto create para priorizar el motor semántico de máscaras.
- Se añade en el PR #33 un motor de dominio para máscaras puntuales y Snapshot, operaciones persistidas de edición local/compartida y resolución de instancias.
- Se documenta que las pruebas de recuperación de creación/eliminación ya existen en lifecycle.rs; no deben añadirse duplicados sin identificar primero un hueco concreto.
- Se conserva el criterio de refactorización incremental: no cambiar formatos TOML/JSON ni simplificar journal, temporales o recuperación sin pruebas equivalentes.

### Model
- Se implementa una primera capa ejecutable Segment/Concept/Component con direcciones semánticas, máscaras puntuales y Snapshot de alcance explícito.
- Las máscaras se persisten en el workspace JSON y los segmentos semánticos adicionales en recursos TOML, con valores por defecto para leer datos históricos.
- Se documenta la resolución desde la instancia hacia el recurso de origen mediante topología y direcciones semánticas.
- Se distinguen las máscaras puntuales de diferencias locales de las máscaras Snapshot, que congelan las partes maskables comprendidas en la Snapshot; las redefiniciones ordinarias no eliminan máscaras locales.
- Se aclara que los componentes pueden tener reglas específicas de combinación, que las referencias se expanden y que los permisos se resuelven separadamente del contenido.
- Se documentan como pendientes explícitos la evolución estructural del origen, eliminación local/ocultación de elementos heredados, identidad ante reubicación, eliminación de recursos referenciados, precedencia exacta de permisos y evolución de elementos congelados por Snapshot.

### Notes
- El motor de máscaras está en revisión en el PR #33. No incluye aún autorización efectiva, expansión de referencias, comandos CLI ni integración de la interfaz web.
