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

Una **máscara es una representación local y dispersa de las diferencias de un segmento dentro de una instancia**. Corresponde a un segmento definido del recurso, pero no es una copia completa ni una captura congelada de su estado.

Las reglas conceptuales acordadas son:

- **Solo diferencias locales:** una máscara almacena únicamente los valores que se hayan modificado localmente; no duplica como estado propio los valores que siguen heredándose.
- **Herencia dinámica:** si un valor no tiene una modificación local, la instancia lo resuelve desde la definición actual del recurso. Por tanto, los cambios posteriores en la definición se reflejan automáticamente en los valores no modificados localmente.
- **Edición compartida sin máscara:** editar un valor heredado sin crear una modificación local puede cambiar directamente la definición del recurso. Es una mutación compartida y puede afectar a otras instancias que hereden ese valor.
- **Edición local con máscara:** editar un valor mediante una máscara modifica la representación local de esa instancia, no la definición compartida.
- **Alcance por segmento:** una máscara afecta al segmento que representa; no convierte toda la instancia en una copia independiente ni aísla automáticamente los demás segmentos.
- **Permisos separados:** los permisos y su resolución no se confunden con la herencia de contenido. Su precedencia y aplicación efectiva requieren reglas propias.

La resolución debe consultar la definición vigente para cada valor no sobrescrito y usar el valor local cuando exista una modificación local válida. La lógica debe residir en un mecanismo común del modelo, no duplicarse en cada interfaz o adaptador.

### Detalles pendientes antes de implementar

La decisión conceptual anterior no determina todavía todos los detalles operativos. Antes de implementar máscaras se deben especificar y probar, como mínimo:

- cómo se identifica un concepto o componente al comparar la definición con una modificación local;
- qué sucede si una modificación local vuelve a ser igual al valor definido;
- cómo se representa la eliminación local de un valor heredado sin que reaparezca inmediatamente por herencia;
- qué significa retirar o restablecer una modificación local y qué datos se descartan;
- cómo se valida una máscara cuando cambia la estructura del segmento definido;
- cuál es la precedencia exacta de los permisos de definición y de instancia.

Estas preguntas no deben resolverse mediante comportamiento accidental del formato TOML/JSON ni mediante reglas independientes en la CLI.

## 4. Permisos y máscaras de permisos

Los permisos y la herencia de contenido están relacionados, pero no son la misma regla.

- Un segmento de permisos en el recurso pertenece al espacio de definición. No se hereda como segmento de contenido en la instancia, pero sus reglas sí afectan al acceso y a las operaciones sobre esa instancia.
- Una instancia puede tener una máscara de permisos propia. Esta política es local a esa instancia y limita el acceso de terceros a esa instancia; no debe cambiar los permisos del recurso ni propagarse automáticamente a otras instancias.
- La política de permisos del recurso puede expresar reglas generales mediante conceptos o casos concretos mediante componentes, según el modelo que se termine de definir.
- La existencia de una regla documentada no equivale a una implementación de autorización. Toda operación protegida deberá comprobar permisos en una capa efectiva común, no confiar únicamente en lo que muestre la CLI.

Quedan por especificar los actores, las operaciones protegidas, la denegación por defecto y la interacción entre máscaras de permisos y segmentos enmascarados.

## 5. Resolución del modelo

Debe existir un mecanismo de resolución que determine, a partir de la composición estructural del modelo general:

1. qué segmentos pertenecen a la definición de un recurso;
2. qué segmentos se heredan directamente en una instancia;
3. qué valores se resuelven desde la definición vigente y cuáles están modificados localmente;
4. cómo se aplican las reglas de permisos sin confundir autorización con contenido heredado.

El mecanismo debe basarse en reglas de composición explícitas, no en nombres especiales codificados para cada futura función. Más adelante deberá poder reutilizarse para determinar casos estructurales de tareas, responsabilidades y proyectos.

Esto **no significa** que haya que construir desde el inicio un motor genérico para cualquier dominio. La primera implementación debe cubrir los casos reales de Resource, Segment, Concept, Component e Instance, manteniendo puntos de extensión claros.

## 6. Invariantes y preguntas pendientes

### Reglas acordadas

- Una máscara representa diferencias locales de un segmento en una instancia; no es una instantánea completa.
- Los valores sin modificación local se resuelven desde la definición vigente.
- Los cambios en la definición actualizan los valores heredados que no estén modificados localmente.
- Sin máscara local, una edición puede mutar el recurso de origen y afectar otras instancias.
- Una modificación mediante máscara es local a la instancia.
- Los permisos del recurso afectan a las instancias sin heredarse como contenido.
- Una máscara de permisos de instancia limita esa instancia, sin cambiar la definición original.
- Los conceptos y componentes están segmentados y pueden repetirse entre segmentos.
- Los valores de Concept son texto provisionalmente.

### Decisiones necesarias antes de implementar máscaras

- ¿Cómo se identifican de forma estable los conceptos y componentes entre definición y máscara?
- ¿Cómo se trata una modificación local cuyo valor coincide con el valor actual de la definición?
- ¿Cómo se representa una eliminación local de un valor heredado?
- ¿Cómo se retira una modificación local sin confundirlo con eliminar el valor heredado?
- ¿Qué ocurre cuando cambia la estructura de un segmento definido?
- ¿Cuál es la precedencia exacta de los permisos de definición y de instancia?

Estas preguntas deben resolverse con ejemplos concretos y pruebas antes de convertirlas en restricciones de código.
