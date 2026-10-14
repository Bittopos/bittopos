# ADR 0002: máscaras como segmentos locales de instancia

- **Estado:** sustituido por [ADR 0003](0003-sparse-mask-overrides.md)
- **Fecha:** 2026-10-13

## Contexto

Bittopos necesita distinguir la representación heredada de un recurso de una representación que una instancia conserva de forma local. También debe permitir que las instancias trabajen directamente con información definida cuando no se ha creado una máscara.

## Decisión original

- Una máscara es un segmento que vive en una instancia y contiene al menos un concepto o componente.
- Si no existe máscara para un segmento, la instancia muestra la representación heredada desde el recurso.
- Editar información heredada sin máscara puede modificar directamente el recurso de definición y, por tanto, afectar otras instancias.
- Una máscara conserva una copia del estado del segmento definido que representa, separando la evolución de la instancia de la definición.
- Los permisos no son contenido heredado: los permisos definidos en el recurso afectan a la instancia, aunque no se copien como segmento de contenido.
- Una máscara de permisos de instancia es local a esa instancia y limita el acceso de terceros sin modificar la política del recurso.

## Alternativas consideradas originalmente

- Copiar automáticamente todos los segmentos al instanciar: se descartó porque haría que toda instancia fuera una copia independiente y ocultaría la diferencia entre herencia y edición compartida.
- Hacer que todas las ediciones fueran locales: se descartó porque contradice la regla de que una instancia sin máscara puede modificar la definición compartida.
- Tratar permisos como un segmento de contenido ordinario: se descartó porque la política de autorización debe afectar a una instancia sin confundirse con los datos heredados que muestra.

## Resultado histórico

La regla de congelar una copia completa del segmento fue reemplazada por una máscara dispersa que almacena diferencias locales y conserva la herencia dinámica para los valores no modificados. La decisión vigente está en ADR 0003.

Las reglas originales sobre edición compartida sin máscara y la separación conceptual entre permisos y contenido se mantienen, sujetas a la especificación de autorización pendiente.
