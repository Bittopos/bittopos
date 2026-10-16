# Modelo conceptual de Bittopos

**Estado:** contrato conceptual acordado en sus reglas centrales; el motor semántico todavía no está implementado. Este documento describe el modelo previsto, no afirma que el código ya lo soporte.

## 1. Espacios de definición e instanciación

- **Espacio de definición:** contiene recursos y sus segmentos definidos. Un recurso puede ser usado por múltiples instancias.
- **Espacio de instanciación:** contiene instancias concretas de esos recursos. Una instancia tiene identidad propia y referencia su definición de origen.
- **Resource.id** identifica la definición.
- **Instance.id** identifica la instancia.
- **Instance.source.id** identifica el recurso de origen; no es la identidad de la instancia.

Una instancia no es automáticamente una copia privada por el mero hecho de existir. La información puede resolverse desde la definición, modificarse de forma compartida o quedar gobernada por una máscara local, según las reglas del modelo.

## 2. Segmentos, conceptos y componentes

### Segment

Un segmento es una unidad con configuración propia que agrupa conceptos y componentes relacionados. Distintos segmentos pueden contener elementos del mismo tipo; su pertenencia al segmento forma parte de la organización semántica.

Algunos segmentos tienen función de control o estructural. Pueden gobernar la organización, la instanciación o la resolución sin convertirse por ello en contenido semántico que deba mostrarse como un segmento ordinario.

### Concept

Un concepto representa una unidad de información con clave y valor. Para la etapa inicial, los valores de concepto se restringen provisionalmente a texto. El modelo no exige un catálogo universal de tipos ni que cada tipo aparezca una sola vez en todo el recurso.

### Component

Un componente representa una unidad estructurada o especializada dentro de un segmento. No todos los componentes admiten la misma operación de combinación: su comportamiento de resolución depende del tipo de componente. En particular, una referencia se expande siguiendo la referencia; no se trata como un valor escalar que simplemente reemplaza al contenido de destino.

## 3. Máscaras y resolución semántica

**La semántica de las máscaras ya fue revisada y acordada conceptualmente.** No es una tarea pendiente volver a decidir si las máscaras son diferencias locales o si la herencia debe ser dinámica. Lo pendiente es implementar el modelo y verificarlo mediante pruebas.

La lectura semántica parte de una instancia del workspace y sigue su topología hacia la definición de origen. La resolución se expresa mediante direcciones semánticas dentro de esa estructura, no depende de que los identificadores físicos de los archivos sean la semántica del modelo. El mismo criterio debe poder aplicarse aunque la persistencia futura use TOML, JSON, una base de datos o un protocolo remoto.

### 3.1 Máscaras puntuales

- Una máscara es un **Segment** local de una instancia y debe contener al menos un Concept o Component.
- Una máscara puntual puede dirigirse a un **Concept** o **Component** concreto.
- La creación de una máscara puntual registra diferencias locales; no copia indiscriminadamente todo el contenido heredado.
- Si no existe una modificación local, el valor se resuelve desde la definición vigente. Los cambios posteriores de la definición se reflejan en los elementos no modificados localmente.
- Editar un elemento heredado sin crear una máscara local modifica directamente el recurso compartido y puede afectar a otras instancias.
- Editar un elemento mediante una máscara puntual crea o cambia la diferencia local de esa instancia.
- Eliminar la máscara local devuelve ese elemento al comportamiento de herencia correspondiente.
- La redefinición del recurso de origen no elimina por sí sola las máscaras locales existentes. Una modificación de política que prohíba operaciones de máscara puede afectar a su efectividad, pero no debe confundirse con una redefinición ordinaria del contenido.

### 3.2 Máscaras Snapshot

Una máscara **Snapshot** congela la definición de origen vigente para las partes maskables comprendidas en la Snapshot y evita que cambios posteriores de esas partes se propaguen como herencia dinámica. No debe confundirse con una máscara puntual: son modos distintos con efectos distintos.

La semántica de una Snapshot no autoriza a congelar automáticamente todos los segmentos de una instancia si el modelo no los ha incluido en su alcance. El alcance de la máscara debe quedar explícito en la estructura semántica.

### 3.3 Lectura y exclusión de elementos ya representados localmente

La instancia puede declarar en su estructura semántica segmentos o elementos que ya están representados localmente. Al leer la definición de origen, el resolvedor debe reconocer esas direcciones y evitar incorporar de nuevo los elementos que ya fueron tratados por la estructura local.

Los segmentos de control/estructurales pueden gobernar la navegación o la instanciación sin añadirse como contenido semántico ordinario. La resolución debe seguir la topología del modelo, no una lista de nombres especiales dispersa por las interfaces.

### 3.4 Permisos

- Los permisos tienen un segmento propio en la definición; no son contenido ordinario que simplemente se hereda y se muestra.
- Los permisos definidos en el recurso siguen afectando al acceso y a las operaciones sobre sus instancias, aunque la política no se copie como contenido visible.
- Una instancia puede tener una máscara local de permisos que modifique la política aplicable a esa instancia.
- La máscara local no adquiere automáticamente precedencia sobre la jerarquía de permisos de origen ni modifica la política del recurso compartido.
- La autorización efectiva debe resolverse en un mecanismo común, no depender de lo que muestre una interfaz.

### 3.5 Límites pendientes del contrato

Las reglas centrales anteriores están acordadas; no significa que todos los casos estructurales estén decididos. Para evitar que la implementación invente comportamientos, los cambios estructurales del origen, la eliminación local de elementos heredados, la reubicación semántica, la eliminación del recurso de origen, la composición exacta de permisos y los cambios estructurales que afectan a Snapshot deben tratarse según [ADR 0005: reglas y límites de las máscaras](decisions/0005-mask-contract-boundaries.md). Esos casos requieren ejemplos y una decisión explícita antes de implementarse; no reabren por sí solos las reglas centrales de herencia dinámica, diferencias locales y Snapshot.

### 3.6 Reglas de combinación

La resolución debe distinguir, como mínimo:

1. la definición de origen vigente;
2. los elementos estructurales que indican qué segmentos o elementos participan en la lectura;
3. las máscaras puntuales locales;
4. las máscaras Snapshot y su alcance;
5. el comportamiento específico de cada tipo de componente;
6. las referencias que amplían la lectura hacia otras partes del modelo;
7. las reglas de permisos, separadas de la presentación del contenido.

No se debe reducir el sistema a una sobrescritura genérica de clave/valor: esa simplificación perdería la topología, las direcciones semánticas, los componentes con reglas propias y los segmentos de control.

## 4. Qué está acordado y qué queda por implementar

### Acordado conceptualmente

- La definición y la instancia tienen identidades distintas.
- La instancia resuelve información siguiendo la topología semántica hacia el recurso de origen.
- Las direcciones semánticas, y no los identificadores físicos de persistencia, gobiernan la correspondencia de elementos.
- Las máscaras puntuales representan diferencias locales; lo no modificado conserva la herencia dinámica.
- Editar sin máscara puede modificar el recurso compartido.
- Las máscaras Snapshot congelan explícitamente las partes maskables incluidas en su alcance.
- Los casos de borde pendientes se delimitan en [ADR 0005](decisions/0005-mask-contract-boundaries.md).
- Las referencias se expanden; el comportamiento de combinación depende del tipo de componente.
- Los segmentos estructurales/de control no se confunden con contenido semántico ordinario.
- Los permisos tienen reglas propias y una resolución efectiva común.

### Pendiente de implementación y verificación

- Incorporar al núcleo el modelo general Segment/Concept/Component y las estructuras semánticas de instancia.
- Implementar el recorrido por topología y la resolución por direcciones semánticas.
- Persistir y resolver máscaras puntuales y Snapshot según el contrato, sin atar su semántica a TOML/JSON.
- Implementar la combinación específica de componentes y la expansión de referencias.
- Implementar la resolución efectiva de permisos.
- Añadir pruebas unitarias de resolución y pruebas de integración con las definiciones y las instancias persistidas.

Estas tareas son de implementación, integración y verificación; no deben describirse como si el contrato de máscaras siguiera sin decidirse. Si durante la implementación aparece una contradicción real con estas reglas, se debe registrar el caso concreto y proponer una decisión explícita antes de cambiar el contrato.
