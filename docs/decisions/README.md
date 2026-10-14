# Decisiones arquitectónicas

Las decisiones arquitectónicas importantes se registran como ADR (*Architecture Decision Records*). Un ADR explica el contexto, las opciones consideradas, la decisión tomada y sus consecuencias.

## Cuándo crear un ADR

Crea uno cuando una decisión:
- cambie una frontera entre módulos o capas;
- establezca una regla difícil de revertir sobre el modelo o la persistencia;
- tenga alternativas razonables con consecuencias diferentes;
- afecte de manera relevante a futuras interfaces, adaptadores o formatos.

No se necesita un ADR para cada tarea, refactor menor ni corrección localizada.

## Formato

Usa números secuenciales y títulos descriptivos, por ejemplo `0001-core-and-adapters.md`.

Cada ADR incluye:
- **Estado:** propuesto, aceptado, sustituido o rechazado.
- **Contexto:** problema y restricciones.
- **Decisión:** qué se acuerda.
- **Alternativas:** opciones consideradas y por qué no se eligieron.
- **Consecuencias:** beneficios, costes y riesgos.
- **Referencias:** especificación, código, Issue o PR relacionados.

No se reescriben los ADR aceptados para ocultar la historia. Si una decisión cambia, se crea otro ADR que indique cuál sustituye.

## Índice

- [ADR 0001: núcleo adaptable y adaptadores explícitos](0001-core-and-adapters.md)
- [ADR 0002: máscaras como segmentos locales de instancia](0002-segment-masks.md)
