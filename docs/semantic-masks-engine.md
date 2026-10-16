# Motor de dominio de máscaras semánticas

**Estado:** implementación del motor de dominio y de los casos de uso persistidos; integración de comandos CLI y autorización efectiva sigue pendiente.

## Responsabilidad

`semantic_masks` introduce un modelo independiente de los formatos TOML/JSON actuales. Representa definiciones semánticas, instancias, segmentos, conceptos, componentes, direcciones y máscaras. La resolución no consulta rutas físicas ni modifica archivos.

## Comportamiento implementado

- Una máscara es un segmento local dirigido a un segmento semántico de origen y debe contener al menos un Concept o Component.
- La máscara puntual conserva diferencias locales y no copia el segmento heredado.
- Los valores no modificados se resuelven desde la definición vigente.
- La edición compartida cambia la definición de origen; las instancias sin diferencia local observan el cambio.
- Editar localmente un concepto o componente crea o actualiza una diferencia puntual.
- Si el valor local vuelve a coincidir con el origen, la diferencia se elimina.
- Retirar una diferencia o máscara reactiva la herencia.
- Snapshot congela únicamente los elementos enumerados explícitamente en su alcance.
- Los componentes usan una estrategia de combinación inyectada por tipo; el reemplazo completo se ofrece solo como estrategia explícita.
- Las máscaras puntuales y Snapshot no pueden modificar el mismo elemento simultáneamente.
- Las claves y duplicados estructurales se validan.
- Si un elemento enmascarado desaparece del origen, la resolución devuelve un error explícito; no inventa una regla de ocultación, eliminación local o reubicación.

## Límites deliberados

Las máscaras se serializan en cada Instance del workspace JSON y los segmentos semánticos adicionales se guardan en el TOML del Resource. Los datos históricos siguen siendo compatibles: el campo de máscaras y los segmentos adicionales son opcionales al leer formatos anteriores. El segmento reservado info se proyecta desde los campos existentes de Resource.info, sin duplicarse en TOML. Las operaciones de dominio se exponen mediante casos de uso del núcleo; la CLI todavía no ofrece comandos para administrarlas.

Tampoco se implementan aquí la política efectiva de permisos ni la expansión semántica de referencias. Las reglas estructurales pendientes enumeradas en ADR 0005 siguen devolviendo errores explícitos cuando afectan la resolución; no se inventa precedencia de permisos ni se confunde una referencia con un valor escalar.

## Pruebas

El módulo contiene pruebas unitarias para herencia dinámica, edición compartida, diferencias locales, retirada de diferencias, Snapshot de alcance explícito, edición de componentes y rechazo explícito de eliminación de elementos de origen. Las comprobaciones completas deben confirmarse en GitHub Actions para el PR.
