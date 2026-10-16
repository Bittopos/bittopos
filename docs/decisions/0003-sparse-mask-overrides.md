# ADR 0003: máscaras dispersas y herencia dinámica (marco parcial sustituido)

- **Estado:** sustituido por [ADR 0004](0004-semantic-mask-resolution.md)
- **Fecha:** 2026-10-14

## Contexto histórico

Este ADR registró la decisión de que las modificaciones locales no congelaran por accidente los valores heredados: una máscara puntual conserva diferencias locales y los elementos no modificados siguen resolviéndose desde la definición vigente. También mantuvo la regla de que editar sin máscara local puede modificar el recurso compartido.

Esas reglas continúan formando parte del contrato, pero describir toda la semántica de máscaras únicamente como «sobrescrituras dispersas» resultó insuficiente. Bittopos también necesita lectura desde la instancia del workspace hacia la definición de origen mediante topología y direcciones semánticas, modos de máscara distintos, comportamiento de componentes, expansión de referencias y resolución separada de permisos.

## Resultado

La formulación vigente está en [ADR 0004](0004-semantic-mask-resolution.md) y en el [modelo conceptual](../model.md). Este ADR se conserva para explicar la evolución del diseño, no como especificación completa independiente.

No se debe reabrir la decisión de herencia dinámica de las máscaras puntuales como si estuviera pendiente. La implementación de la semántica acordada sigue pendiente.
