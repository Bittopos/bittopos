# Changelog

Los cambios relevantes de Bittopos se registran aquí en orden cronológico inverso. Este archivo sigue una estructura inspirada en Keep a Changelog; no reemplaza el historial de commits ni enumera cada cambio interno.

Mientras no existan versiones publicadas, `Unreleased` reúne los cambios que se preparan para la próxima versión. Las fechas y números de versión se añaden cuando existe un hito verificable.

## [Unreleased]

### Documentation
- Se establece una documentación de trabajo separada en modelo, arquitectura, estado del proyecto y decisiones arquitectónicas.
- Se define el flujo de desarrollo mediante Issues, ramas, Conventional Commits, Pull Requests y pruebas del workspace.
- Se aclara que los contratos numerados anteriores son documentos históricos, no la fuente de verdad vigente.

### Model
- Se documenta como regla conceptual provisional la máscara de segmento en instancias, la herencia por defecto y el propósito de congelar una copia de un segmento definido.
- Se distingue la gestión de permisos en el espacio de definición de la máscara de permisos local a una instancia.

### Notes
- Estos cambios documentales no implementan por sí solos máscaras, resolución de herencia ni permisos efectivos en el código.
