# Modelo de Bittopos

Este documento define el modelo conceptual vigente.

## Recurso

Un recurso es un archivo TOML individual. Contiene una definición reutilizable y sus segmentos. Su identidad es distinta de la identidad de cualquier instancia que lo use.

## Estructura

La estructura es un archivo JSON que organiza las instancias de forma jerárquica. Es también el espacio de trabajo: no existe un segundo concepto independiente de workspace.

La estructura conserva cada instancia y sus hijos directos. La jerarquía completa se obtiene recorriendo esa organización.

## Instancia

Una instancia es una aparición identificable de un recurso dentro de la estructura. Conserva su propia identidad y una referencia al recurso de origen. No es una copia completa del recurso.

## Segmento, concepto y componente

Un segmento es una parte identificable de una definición. Puede estar en el archivo del recurso o existir solo en una instancia. Los conceptos y componentes son datos dentro de los segmentos; sus distinciones se mantienen cuando cambian el comportamiento o la representación.

## Máscaras y herencia

Una máscara es un segmento de una instancia que contiene diferencias locales.

- Crear una máscara guarda únicamente diferencias locales.
- Editar sin máscara modifica la definición compartida.
- Los valores no modificados localmente siguen la definición compartida vigente.
- Una diferencia local prevalece para el elemento que modifica.
- Retirar una diferencia permite que el valor vuelva a heredarse.
- Un snapshot solo congela los elementos incluidos explícitamente en su alcance.

La eliminación de un elemento heredado no tiene una semántica de ocultación definida por defecto. No debe inventarse una regla de borrado local sin una decisión explícita.

## Resolución semántica

La resolución parte de una instancia y aplica las reglas definidas para obtener una vista efectiva. Debe ser determinista: para los mismos datos y reglas, produce el mismo resultado. Si no puede satisfacer la consulta, debe indicarlo en lugar de fabricar datos.

La jerarquía de resolución se ampliará solo mediante reglas explícitas del modelo, no con condiciones dispersas en las interfaces.

## Persistencia actual

- Recursos: archivos TOML individuales.
- Estructura e instancias: archivo JSON.
- Navegación: archivo TOML local.
- Máscaras de instancia: datos persistidos dentro de la instancia.

La organización física actual conserva algunos campos históricos. No deben considerarse conceptos nuevos solo porque sigan presentes en el formato.
