# Modelo conceptual de Bittopos

**Estado:** modelo en definición. Esta página registra las reglas conceptuales acordadas; no afirma que todas estén implementadas.

## 1. Espacios de definición e instanciación

- **Espacio de definición:** contiene recursos y sus segmentos definidos. Un recurso puede ser usado por múltiples instancias.
- **Espacio de instanciación:** contiene instancias concretas de esos recursos. Una instancia tiene identidad propia y referencia su definición de origen.
- `Resource.id` identifica la definición.
- `Instance.id` identifica la instancia.
- `Instance.source.id` identifica el recurso de origen; no es la identidad de la instancia.

Una instancia sin máscara puede operar sobre información heredada de su recurso. En ese caso, modificar esa información desde la instancia puede modificar la definición compartida y afectar a otras instancias. La instancia no es automáticamente una copia privada por el mero hecho de existir.

## 2. Segmentos, conceptos y componentes

### Segment

Un segmento es una unidad con configuración propia que agrupa conceptos y componentes relacionados. Distintos segmentos pueden contener conceptos o componentes del mismo tipo; su pertenencia al segmento es parte de la organización del modelo, no una garantía de que el tipo sea globalmente único.

Por ahora, los conceptos y componentes se organizan dentro de segmentos. No se establece un catálogo universal de tipos ni se exige que cada tipo aparezca una sola vez en todo el recurso.

### Concept

Un concepto representa una unidad de información con una clave y un valor. **Para la etapa actual, los valores de concepto se restringen provisionalmente a texto.** No se diseñará todavía soporte general para binarios, tipos de base de datos ni un sistema dinámico de tipos.

### Component

Un componente representa una unidad estructurada o especializada dentro de un segmento. Los conceptos y componentes son unidades distintas; sus esquemas y reglas generales deben diseñarse como partes del mismo modelo, no como abstracciones aisladas.

La forma de representar componentes complejos, referencias tipadas y listas de casos se especificará cuando el modelo general y sus casos de uso estén suficientemente definidos.

## 3. Máscaras de segmento

Una **máscara es un segmento que vive en una instancia** y contiene como mínimo un concepto o un componente.

- Una máscara corresponde a un segmento definido del recurso y conserva una copia de su estado en el momento en que se crea o captura.
- La copia permite que el segmento enmascarado de la instancia y el segmento de la definición evolucionen sin sobrescribirse mutuamente.
- Si no existe una máscara para ese segmento, la instancia muestra la representación heredada tal como se resuelve desde la definición.
- La máscara protege el estado definido que captura frente a cambios posteriores ordinarios en la definición; no debe alterar por sí sola el recurso original.
- La ausencia de máscara no impide editar la información heredada: una operación de edición sin máscara puede actuar directamente sobre el recurso definido. Esa es una mutación compartida, no una edición privada de instancia.
- La máscara no se interpreta como una copia de toda la instancia ni como una protección automática de todos los segmentos. Su alcance es el segmento que representa.

La regla conceptual es, por tanto, **herencia por defecto y sustitución local mediante máscara de segmento**. Los detalles operativos —cómo se crea, actualiza o retira una máscara y cómo se representan conflictos— requieren una especificación y pruebas antes de implementar el mecanismo.

## 4. Permisos y máscaras de permisos

Los permisos y la herencia de contenido están relacionados, pero no son la misma regla.

- Un segmento de permisos en el recurso pertenece al espacio de definición. No se hereda como segmento de contenido en la instancia, pero sus reglas sí afectan al acceso y a las operaciones sobre esa instancia.
- Una instancia puede tener una máscara de permisos propia. Esta política es local a esa instancia y limita el acceso de terceros a esa instancia; no debe cambiar los permisos del recurso ni propagarse automáticamente a otras instancias.
- La política de permisos del recurso puede expresar reglas generales mediante conceptos o casos concretos mediante componentes, según el modelo que se termine de definir.
- La existencia de una regla documentada no equivale a una implementación de autorización. Toda operación protegida deberá comprobar permisos en una capa efectiva común, no confiar únicamente en lo que muestre la CLI.

Quedan por especificar la precedencia entre permisos del recurso y de la instancia, los actores, las operaciones protegidas, la denegación por defecto y la interacción entre máscaras de permisos y segmentos enmascarados.

## 5. Resolución del modelo

Debe existir un mecanismo de resolución que determine, a partir de la composición estructural del modelo general:

1. qué segmentos pertenecen a la definición de un recurso;
2. qué segmentos se heredan directamente en una instancia;
3. qué segmentos de instancia son máscaras que sustituyen la representación heredada;
4. cuándo se debe consultar el estado definido y cuándo el estado enmascarado;
5. qué política de permisos afecta a una operación, sin confundir permisos con contenido heredado.

El mecanismo debe basarse en reglas de composición explícitas, no en nombres especiales codificados para cada futura función. Más adelante deberá poder reutilizarse para determinar casos estructurales de tareas, responsabilidades y proyectos.

Esto **no significa** que haya que construir desde el inicio un motor genérico para cualquier dominio. La primera implementación debe cubrir los casos reales de Resource, Segment, Concept, Component e Instance, manteniendo puntos de extensión claros.

## 6. Invariantes y preguntas pendientes

### Reglas acordadas

- Una máscara es un segmento de instancia y contiene al menos un concepto o componente.
- Sin máscara, la instancia muestra el segmento heredado tal como se resuelve desde la definición.
- Una edición sin máscara puede mutar el recurso de origen y afectar otras instancias.
- Una máscara congela el estado del segmento definido que representa, evitando acoplamiento de edición entre ambos espacios.
- Los permisos del recurso afectan a las instancias sin heredarse como contenido.
- Una máscara de permisos de instancia limita esa instancia, sin cambiar la definición original.
- Los conceptos y componentes están segmentados y pueden repetirse entre segmentos.
- Los valores de Concept son texto provisionalmente.

### Decisiones necesarias antes de implementar máscaras

- ¿Qué significa exactamente «crear una máscara»: copiar todo el segmento en ese instante o permitir crearla con contenido parcial?
- Si la definición cambia después, ¿la máscara es un snapshot totalmente independiente o admite actualizarse explícitamente desde el origen?
- ¿Cómo se elimina o desactiva una máscara y qué sucede con los datos locales?
- ¿Cómo se distinguen los segmentos propios de instancia que no son máscaras?
- ¿Qué reglas de identidad y unicidad se aplican a los nombres/IDs de segmentos, conceptos y componentes?
- ¿Cuál es la precedencia exacta de los permisos de definición y de instancia?

Estas preguntas deben resolverse con ejemplos concretos y pruebas antes de convertirlas en restricciones de código.
