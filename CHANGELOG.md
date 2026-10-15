# Changelog

Los cambios relevantes de Bittopos se registran aquí en orden cronológico inverso. Este archivo sigue una estructura inspirada en Keep a Changelog; no reemplaza el historial de commits ni enumera cada cambio interno.

Mientras no existan versiones publicadas, `Unreleased` reúne los cambios que se preparan para la próxima versión. Las fechas y números de versión se añaden cuando existe un hito verificable.

## [Unreleased]

### Documentation
- Se separa la visión estratégica, el modelo conceptual, la arquitectura actual, el estado, las decisiones y la guía interna de ingeniería.
- Se archivan los contratos anteriores sin eliminar su contenido histórico.
- Se refuerzan los enlaces del README y se aclara qué documentación es normativa y cuál es histórica.

### Model
- Se sustituye el modelo de máscara como instantánea congelada por una máscara dispersa de diferencias locales con herencia dinámica.
- Se mantiene la distinción entre edición compartida sin máscara y modificación local mediante máscara.
- Se distingue la gestión de permisos en el espacio de definición de la máscara de permisos local a una instancia.

### Notes
- Estos cambios documentales no implementan por sí solos máscaras, resolución de herencia ni permisos efectivos en el código.
