# ADR 0004: resolución semántica y modos de máscara

- **Estado:** aceptado conceptualmente; implementación pendiente
- **Fecha:** 2026-10-15
- **Sustituye como especificación completa:** [ADR 0003](0003-sparse-mask-overrides.md)
- **Especificación conceptual relacionada:** [Modelo conceptual](../model.md)

## Contexto

La formulación previa describía las máscaras principalmente como sobrescrituras dispersas. Esa regla es necesaria para las máscaras puntuales, pero no basta para expresar la lectura semántica desde una instancia hacia la definición de origen, la topología del modelo, las direcciones semánticas, las máscaras Snapshot, el comportamiento específico de los componentes, la expansión de referencias y los permisos.

El contrato de máscaras fue revisado conceptualmente. Este ADR registra la dirección acordada para que el desarrollo futuro la implemente en lugar de reabrirla como una pregunta de diseño.

## Decisión

### 1. Recorrido y direcciones semánticas

- La resolución parte de la instancia del workspace y recorre la topología hacia la definición de origen.
- La correspondencia entre elementos se expresa mediante direcciones semánticas en el modelo, no mediante rutas físicas ni por asumir que un ID de almacenamiento define la semántica.
- El enfoque debe ser independiente del formato de persistencia; TOML y JSON son adaptadores actuales, no el contrato de resolución.
- La estructura local de la instancia puede declarar segmentos o elementos ya representados. La lectura del origen debe evitar duplicarlos.
- Los segmentos de control/estructurales pueden gobernar la organización y la instanciación sin incorporarse como contenido semántico ordinario.

### 2. Máscaras puntuales

- Una máscara es un Segment local de una instancia y contiene al menos un Concept o Component.
- Una máscara puntual puede dirigirse a un Concept o Component.
- Registra diferencias locales sin copiar indiscriminadamente todos los valores heredados.
- Los elementos no modificados se resuelven desde la definición vigente y reflejan cambios posteriores de esa definición.
- Editar un elemento heredado sin máscara local puede modificar el recurso compartido y afectar a otras instancias.
- Editar un elemento con máscara crea o cambia la diferencia local.
- Retirar la máscara local devuelve el elemento al comportamiento de herencia correspondiente.
- Una redefinición ordinaria del recurso de origen no elimina las máscaras locales ya existentes. Si cambia una política que prohíbe operaciones de máscara, puede cambiar su efectividad; esto es distinto de una redefinición ordinaria del contenido.

### 3. Máscaras Snapshot

- Una máscara Snapshot congela la definición de origen vigente para las partes maskables comprendidas en la Snapshot.
- Los cambios posteriores de la definición no se propagan a las partes congeladas.
- Una Snapshot es un modo distinto de una máscara puntual; no debe aplicarse de forma implícita a toda la instancia ni confundirse con el comportamiento de herencia dinámica.

### 4. Componentes y referencias

- La combinación no es una sobrescritura universal de clave/valor: el comportamiento depende del tipo de componente.
- Las referencias se expanden siguiendo la referencia; no se tratan como valores escalares que reemplazan automáticamente al contenido referido.
- Las reglas de resolución pertenecen al núcleo compartido y no deben duplicarse en cada interfaz o adaptador.

### 5. Permisos

- Los permisos tienen un segmento propio en la definición y no se tratan como contenido ordinario heredado.
- Los permisos del recurso siguen afectando a sus instancias.
- Una instancia puede tener una máscara local de permisos para modificar la política aplicable a esa instancia.
- La máscara local no tiene precedencia automática sobre la jerarquía de permisos de origen ni cambia la política compartida.
- La autorización efectiva debe calcularse en una capa común.

## Consecuencias

- El motor semántico necesita distinguir topología, dirección semántica, definición de origen, estructura local y modos de máscara.
- Las pruebas deben cubrir herencia dinámica, edición compartida, diferencias locales, snapshots, componentes, referencias y permisos.
- El modelo no puede depender de detalles accidentales de serialización.
- La CLI y la web no deben implementar resoluciones independientes.
- La implementación debe ser incremental y no necesita un motor universal hipotético antes de resolver los casos reales.

## Alcance de este ADR

Este ADR cierra la dirección conceptual de las máscaras y su resolución. No afirma que el código ya la implemente. La tarea pendiente es traducir estas reglas en estructuras de dominio, operaciones y pruebas. Si aparece una contradicción concreta durante la implementación, debe registrarse con un ejemplo reproducible antes de proponer una modificación del contrato.
