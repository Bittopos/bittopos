# ADR 0005: reglas y límites de las máscaras

- **Estado:** reglas centrales aceptadas conceptualmente; algunos casos estructurales siguen pendientes.
- **Fecha:** 2026-10-16
- **Relacionado con:** [ADR 0004](0004-semantic-mask-resolution.md) y [modelo conceptual](../model.md).

## Contexto

Las reglas centrales de las máscaras ya fueron discutidas. Este documento separa lo acordado de los casos de borde que aún no tienen una regla confirmada. No reabre el diseño central por defecto.

## Reglas acordadas

### Máscara puntual

- Una máscara es un **Segment local de una Instance** y contiene al menos un Concept o Component.
- Si no hay máscara aplicable a un segmento, la instancia muestra la representación heredada del recurso de origen tal cual.
- Crear una máscara puntual registra **solo diferencias locales**; no copia indiscriminadamente el contenido heredado.
- Los elementos no modificados localmente se resuelven desde la definición vigente y reflejan automáticamente sus cambios posteriores.
- Editar un elemento heredado **sin máscara local** modifica directamente el recurso compartido y puede afectar a otras instancias.
- Editar con una máscara puntual crea o actualiza la diferencia local.
- Retirar la máscara local reactiva el comportamiento de herencia correspondiente.

Estas reglas son el comportamiento ordinario de las máscaras puntuales; no deben confundirse con Snapshot.

### Snapshot

- Una máscara **Snapshot** congela la definición de origen vigente para las partes maskables incluidas explícitamente en su alcance.
- Los cambios posteriores no se propagan a las partes congeladas.
- Snapshot no congela implícitamente toda la instancia.

### Redefinición y resolución

- La redefinición ordinaria del recurso no elimina por sí sola las máscaras locales existentes.
- La lectura parte de una instancia y sigue la topología hacia la definición de origen.
- La correspondencia usa direcciones semánticas, no rutas físicas ni IDs de almacenamiento como sustituto de la semántica.
- La estructura semántica local puede declarar segmentos o elementos ya representados; el resolvedor evita incorporarlos por segunda vez.
- Los segmentos de control pueden gobernar organización e instanciación sin convertirse en contenido semántico ordinario.
- La combinación depende del tipo de Component. Las referencias se expanden siguiendo la referencia.

### Permisos

- El segmento de permisos de una definición protege y gobierna operaciones sobre ese **Resource**. No se copia como contenido visible de cada instancia ni se trata como un segmento ordinario heredado.
- Una **Instance** puede tener una máscara local de permisos para ajustar el acceso específico a esa instancia, por ejemplo limitar el acceso de terceros.
- La máscara local no cambia la política compartida del recurso ni obtiene automáticamente precedencia sobre la jerarquía de permisos de origen.
- La autorización efectiva debe resolverse mediante reglas comunes, no por una interfaz.

## Casos que requieren una decisión explícita

No se debe improvisar el comportamiento de estos casos durante la implementación. Hay que presentar un ejemplo concreto, acordar el resultado y añadir pruebas:

1. Qué ocurre con una máscara cuando el recurso añade, elimina, renombra o reestructura un Segment, Concept o Component.
2. Diferencia entre retirar una máscara (reactiva la herencia) y eliminar localmente un elemento que aún existe en el origen. No está fijado si esta última operación requiere una marca explícita de ocultación u otra representación.
3. Cómo se resuelven las direcciones cuando un elemento cambia de segmento, se renombra o se mueve.
4. Qué sucede si se elimina un Resource que todavía tiene instancias o máscaras referenciándolo.
5. Orden exacto de evaluación de permisos y límites a los ajustes locales dentro de la jerarquía de origen.
6. Qué ocurre con partes congeladas de Snapshot si el elemento de origen se elimina, mueve o reestructura.

Estos pendientes no invalidan las reglas centrales; delimitan los casos de borde de la especificación.

## Consecuencias

- El motor debe distinguir herencia directa, diferencias locales, Snapshot y estructura semántica local.
- No basta una fusión genérica de mapas clave/valor: se necesitan direcciones semánticas y reglas por tipo de componente.
- Las pruebas deben cubrir herencia dinámica, edición compartida sin máscara, cambios locales, retirada de máscaras, Snapshot, referencias y permisos.
- Los casos de borde anteriores deben permanecer sin comportamiento implícito hasta ser decididos.
- La semántica pertenece al núcleo compartido, no a implementaciones paralelas en CLI, web o adaptadores.

## Historia

- [ADR 0002](0002-segment-masks.md) conserva la primera formulación, en la que una máscara se trataba como copia del segmento. La copia completa no es la regla vigente para máscaras puntuales.
- [ADR 0003](0003-sparse-mask-overrides.md) documenta la transición a diferencias locales y herencia dinámica.
- [ADR 0004](0004-semantic-mask-resolution.md) establece la resolución semántica general.
- Este ADR precisa las reglas centrales y los límites pendientes sin fingir que todos los casos de borde están resueltos.
