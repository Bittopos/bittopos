# Bittopos — Contrato técnico 0.3.0

**Estado:** contrato de diseño para la línea 0.3.0; no implica que las funcionalidades descritas ya estén implementadas.  
**Lenguaje del núcleo:** Rust.  
**Persistencia inicial:** recursos en TOML; espacio de trabajo y estado de instancias en formatos locales adecuados.  
**Entorno inicial:** local, incluido Termux en Android.  
**Interfaz prevista:** CLI `bitto` y una interfaz web local servida por un programa HTTP ligero.

---

## 1. Objetivo y principios

Este contrato define el modelo conceptual y el alcance previsto para Bittopos 0.3.0. La finalidad es evolucionar desde recursos descriptivos y referencias de instancia hacia recursos compuestos por segmentos, componentes y conceptos, conservando una separación clara entre definiciones e instancias.

El contrato describe comportamientos y límites; no obliga a adoptar una implementación interna más compleja de lo necesario. Las características marcadas como futuras o excluidas no deben implementarse como parte de esta versión.

Principios:

1. Separar el espacio de definición del espacio de instanciación.
2. Mantener separadas la identidad de un recurso y la identidad de cada instancia de ese recurso.
3. Distinguir `semantic.allocation` (contención directa) de `management.structural` (registro de identidades de instancia), así como de relaciones y referencias.
4. Permitir que una instancia conserve o sobreponga propiedades mediante máscaras sin modificar accidentalmente la definición original.
5. Consultar el almacenamiento bajo demanda, evitando cargar toda la estructura en memoria solo para mostrarla.
6. Mantener el núcleo independiente de la CLI, del servidor HTTP y de la interfaz web.
7. No implementar anticipadamente relaciones avanzadas, edición por referencias, permisos completos ni sincronización entre dispositivos.

## 2. Terminología

### 2.1. Espacio de definición

Conjunto de definiciones persistentes de recursos y de sus segmentos, conceptos y componentes. Una definición puede ser utilizada por varias instancias.

### 2.2. Espacio de instanciación

Conjunto de instancias concretas creadas a partir de recursos definidos. Una instancia tiene identidad propia y puede mantener datos particulares que no alteran la definición de origen.

### 2.3. Resource

Definición identificable y persistente. Cada recurso posee un identificador estable que no se hereda como identificador de sus instancias.

### 2.4. Instance

Aparición concreta de un recurso. Cada instancia tiene un identificador propio `id`, distinto del ID del recurso original. El identificador de instancia se almacena como campo de nivel superior, fuera de los segmentos. `Instance.id` identifica la instancia; `source.id` referencia el mismo identificador de recurso que `Resource.id`, aunque se encuentre en una ubicación distinta. La instancia identifica además su recurso de origen mediante `source.id`. Varias instancias pueden proceder del mismo recurso y deben distinguirse por su propio `id`. La instancia puede tener segmentos propios o segmentos máscara.

### 2.5. Segment

Parte diferenciada de un recurso o de una instancia. Los segmentos de sistema pertenecen al espacio de definición. Los segmentos de instancia existen únicamente en una instancia.

### 2.6. Segmento máscara

Segmento definido en una instancia que corresponde a un segmento del recurso original. Permite controlar la herencia de sus propiedades y sobreponer información propia de la instancia.

Una máscara puede preservar propiedades frente a cambios posteriores de la definición y aportar valores particulares. Las reglas de herencia se aplican por las unidades de información que la máscara protege; la mera existencia de una instancia no significa que todas sus propiedades estén congeladas.

Si una instancia no tiene máscara para una propiedad heredable, esta puede reflejar los cambios de la definición original. Una máscara parcial protege únicamente la parte que cubre. Una instancia completamente enmascarada no recibe cambios ordinarios de la definición mientras se mantenga ese estado. Al retirar una máscara, la parte liberada vuelve a seguir las reglas de herencia. Retirar una máscara no debe borrar datos particulares sin una regla explícita.

Un segmento de instancia que no corresponde a un segmento definido en el recurso original es un segmento exclusivo de la instancia, no una máscara.

### 2.7. Concept

Unidad de información compuesta por una clave y un valor. Por ejemplo, `title` y `description` son conceptos dentro del segmento `info`: cada uno representa una pieza de información con su propio significado. El valor puede ser un arreglo. En esta versión, un Concept no contiene recursivamente otros Concept como estructura arbitraria.

Los datos del sistema —por ejemplo, identificadores de recursos, instancias, relaciones o referencias— se representan mediante componentes especializados, no como Concept de usuario.

El contrato de implementación deberá fijar el conjunto de tipos admitidos para los valores de Concept. No se debe asumir que todos los valores son cadenas. Este modelo conceptual no obliga a implementar un motor genérico dinámico: los conceptos pueden representarse mediante campos tipados del lenguaje y serializarse con las herramientas correspondientes.

### 2.8. Component

Estructura especializada que representa datos del sistema o una organización semántica concreta dentro de un segmento. Los componentes pueden contener identificadores u otras estructuras tipadas.

### 2.9. Contención, registro de instancias, relación y referencia

- **Contención:** expresa la composición estructural de un objeto mediante otros objetos.
- **Registro de instancias:** `management.structural` registra en el espacio de definición las identidades de las instancias creadas a partir de un recurso.
- **Relación:** expresa una conexión semántica entre elementos, distinta de la contención.
- **Referencia:** identifica o apunta a otro elemento sin implicar por sí misma contención.

Estos conceptos no son intercambiables y no deben almacenarse bajo una única regla semántica.

### 2.10. Workspace

Contexto de trabajo local que organiza las instancias utilizadas. Su segmento `content` contiene `instances`, una lista de instancias raíz. En esta versión existe un espacio de trabajo por defecto; no existe todavía un sistema completo de perfiles o proyectos.

### 2.11. Navigation context

Posición persistente de navegación. Puede ser la raíz del espacio de trabajo o una instancia válida dentro de él. La posición no puede apuntar a un recurso no instanciado.

## 3. Segmentos iniciales del recurso y de la instancia

### 3.1. `management`

Segmento de sistema para responsabilidades administrativas globales de la definición del recurso, incluidas privacidad, permisos globales y demás propiedades de gestión. El campo superior `Resource.id` permanece fuera de los segmentos y no se hereda como identidad de una instancia.

`management` se administra desde el espacio de definición. Una instancia no puede utilizar operaciones ordinarias para leer o modificar componentes de gestión global ni cambiar la definición de origen. Cuando el contrato permita una máscara de otra propiedad de `management`, esta solo podrá producir un comportamiento local de la instancia; nunca modificará la definición original. `management.structural` es una excepción estricta: no puede consultarse, modificarse ni enmascararse desde el espacio de instanciación.

El componente `management.structural` registra las identidades de las instancias creadas a partir de una definición. No describe los objetos que una instancia contiene. Sus cambios son operaciones controladas del sistema asociadas al ciclo de vida de las instancias.

### 3.2. `info`

Segmento de sistema para información descriptiva de un recurso. En el modelo actual puede incluir título, descripción y etiquetas. Los datos descriptivos no sustituyen la identidad del recurso.

### 3.3. `semantic`

Segmento de sistema para la organización semántica de un recurso. Los componentes conceptuales previstos incluyen:

- `allocation`: contención directa.
- `relational`: relaciones semánticas.
- `referential`: referencias a otros elementos.

En este incremento se especifican `semantic.allocation` y `content.instances` como las dos representaciones coordinadas de la contención de instancias. La implementación debe seguir este contrato; no se afirma que ya esté presente en el código. Las relaciones y referencias avanzadas (`relational` y `referential`) quedan aplazadas.

### 3.4. `content` (segmento de workspace e instancia)

`content.instances` contiene físicamente las instancias hijas directas. En el `Workspace`, contiene las instancias raíz; dentro de cada `Instance`, contiene sus hijos directos completos y cada hijo puede volver a contener su propio `content.instances`. La estructura es recursiva y expresa la contención física del árbol.

`content.instances` no sustituye a `semantic.allocation`: almacena los objetos hijos completos, mientras `semantic.allocation` registra únicamente sus IDs directos. Ambos deben mantenerse consistentes.

### 3.5. `source` (segmento de instancia)

`source` identifica la definición de la que procede una instancia. Su componente `source.id` contiene el ID del recurso original. Es una referencia al origen, no la identidad de la instancia.

`source.id` es un componente de sistema protegido: las operaciones ordinarias de edición de segmentos, componentes o conceptos no pueden reemplazarlo ni alterarlo. Su cambio solo puede ocurrir mediante operaciones internas explícitas y validadas que formen parte de la creación de una instancia.

Aunque `source.id` se almacena dentro de la instancia y `Resource.id` en el nivel superior del recurso, ambos representan conceptualmente el mismo identificador del recurso de origen. La instancia conserva su identidad independiente en `Instance.id`. No debe añadirse un campo permanente paralelo `resource_id` al nuevo esquema.

## 4. Componentes y reglas de uso

### 4.1. `structural`

`structural` pertenece a `management` y al espacio de definición. Registra los identificadores de las instancias creadas a partir de un recurso. Mantiene la correspondencia entre una definición y las identidades de sus instancias y facilita la validación y recuperación ante fallos.

La identidad de cada instancia es independiente del ID del recurso original. No se debe inferir la identidad de una instancia a partir del ID de su recurso. Cada instancia mantiene un `id` propio y referencia el origen mediante `source.id`.

`structural` es un componente administrativo reservado al sistema: no puede leerse, modificarse ni enmascararse mediante la interfaz del espacio de instanciación. Su modificación corresponde a operaciones controladas de creación, recuperación o eliminación de instancias en el espacio de definición.

En esta versión, `structural` no representa contención ni almacena los identificadores de los objetos que contiene una instancia. Es un registro de ciclo de vida asociado a la definición.

### 4.2. `allocation`

`allocation` pertenece a `semantic` y expresa qué objetos contiene directamente una instancia. Registra solo los IDs de los hijos directos; no almacena copias completas de sus definiciones ni duplica en el padre todos los IDs de los descendientes.

Aunque la contención se considera universal y conceptualmente puede existir en el espacio de definición o en el de instanciación, **en 0.3 solo se utilizará en instancias**. No se implementará la contención semántica global en recursos originales, ya que podría afectar a todas sus instancias.

La interfaz podrá presentar esta operación al usuario con expresiones sencillas, como guardar un objeto dentro de otro. No debe exigir que el usuario conozca el término técnico `allocation`.

`semantic.allocation` y `content.instances` representan deliberadamente dos vistas complementarias de la misma contención directa:

- `semantic.allocation`: lista de IDs de los hijos directos de una instancia, sin copiar sus datos.
- `content.instances`: lista de esos mismos hijos como instancias completas, anidadas físicamente en el JSON.

`semantic.allocation` debe coincidir, en el mismo orden, con los valores `id` de `content.instances`. Si una instancia no tiene hijos, cualquiera de los dos componentes puede omitirse o representarse como un arreglo vacío; para validar la consistencia, un componente omitido equivale a una lista vacía únicamente en ese caso. Si hay hijos, ambas representaciones deben estar presentes y coincidir exactamente en contenido y orden.

La contención solo se aplica a instancias, nunca a las definiciones de recursos originales. El árbol físico no puede contener una instancia más de una vez, repetir IDs, contenerse a sí misma ni formar ciclos. Cada instancia debe tener un único padre físico como máximo; las instancias raíz solo aparecen en `Workspace.content.instances`. Las relaciones y referencias que no sean contención pertenecen a componentes futuros y no alteran estas reglas.

### 4.3. `relational` y `referential`

Son componentes conceptuales reservados para relaciones y referencias. No se implementarán como funcionalidades completas en esta versión. Su futura incorporación no debe obligar a reinterpretar la contención como una relación genérica.

### 4.4. `content.instances` y consistencia de la contención

`content.instances` es el componente físico que almacena instancias completas. En el workspace aparece bajo `content.instances` y contiene las raíces del árbol; en cada instancia aparece bajo `content.instances` y contiene recursivamente sus hijos directos.

Para cada instancia, el sistema debe validar que `semantic.allocation` y `content.instances` describan exactamente los mismos hijos directos y en el mismo orden. Cuando no hay hijos, cualquiera de los componentes puede omitirse o contener un arreglo vacío; la omisión se interpreta como vacío solo si la otra representación también está vacía. Cuando hay hijos, ambas representaciones deben estar presentes. Una discrepancia es un estado inválido y no debe corregirse silenciosamente eligiendo una de las representaciones.

La identidad de instancia debe ser única en todo el workspace, no solo entre hermanos. Una instancia no puede aparecer a la vez como raíz y como descendiente ni tener más de un padre. El árbol físico no puede tener ciclos. Antes de persistir o aceptar un workspace cargado, el núcleo debe validar estas invariantes y referencias a recursos de origen.

## 5. Identidad, definición e instancia

1. Cada recurso tiene un `id` estable en el nivel superior de su definición TOML, fuera de los segmentos.
2. Cada instancia tiene un `id` propio en el nivel superior de su representación JSON, fuera de los segmentos y distinto del ID del recurso original.
3. Cada instancia contiene el segmento `source`, cuyo componente `source.id` referencia el recurso de origen.
4. Varias instancias pueden compartir `source.id`, pero nunca la misma identidad de instancia dentro del ámbito de unicidad definido por el sistema.
5. Crear una instancia no crea una nueva definición de recurso.
6. Crear o modificar datos propios de una instancia no debe modificar accidentalmente la definición original.
7. `management.structural` registra en la definición las identidades de sus instancias.
8. El contexto de navegación identifica una instancia, no solo el recurso de origen.
9. Los identificadores internos del sistema no se representan como simples datos de usuario ni se modifican mediante ediciones genéricas de segmentos.
10. Los alias no forman parte del alcance funcional de 0.3. Si se añaden en el futuro, no sustituirán a los identificadores estables.

`management.structural` es el registro administrativo de las identidades de instancia asociadas a cada recurso de origen; no contiene la jerarquía de padres e hijos. La jerarquía y la contención se conservan en el estado JSON mediante `content.instances` y `semantic.allocation`. Por tanto, `management.structural` por sí solo no permite reconstruir el árbol completo.

Hay que distinguir tres operaciones: **instanciación** (crear una instancia a partir de un recurso), **remoción de instancia** (desinstanciarla del workspace sin destruir la definición del recurso) y **eliminación de recurso** (destruir la definición y limpiar las instancias e identidades asociadas). No son sinónimos y deben tener operaciones del núcleo separadas. En este incremento se implementan la instanciación necesaria para `bitto add` y la remoción de instancias; la eliminación destructiva del recurso queda definida como operación futura y no debe confundirse con la remoción.

Como `management.structural` se persiste en TOML y el árbol se persiste en JSON, no existe una escritura atómica que abarque ambos archivos. Las operaciones de instanciación y remoción deben utilizar un journal duradero de operación pendiente. El journal debe persistirse de forma duradera antes de modificar el recurso o el workspace y conservarse hasta que ambos cambios hayan sido persistidos y validados. Cada entrada identifica la acción, el ID de instancia, el ID del recurso de origen y el padre (`null` para una raíz); para instanciación también debe contener datos suficientes para reconstruir la nueva instancia y su destino. El formato debe permitir añadir en el futuro una acción de eliminación destructiva de recurso sin mezclarla con la remoción de una instancia.

El orden de escritura será explícito y común para mantener pequeño el journal: **journal → recurso TOML → workspace JSON → retirar journal**. Primero se persiste la intención; después se modifica y persiste `management.structural`; a continuación se modifica y persiste el workspace; solo después de validar ambos resultados se elimina el journal. Cada archivo se actualiza mediante escritura atómica, y el journal debe quedar durable antes de empezar la operación. Las operaciones deben ser idempotentes para que la recuperación pueda repetir los pasos de forma segura.

Para instanciar, el journal registra la intención de crear la instancia, su ID, el recurso de origen y el padre previsto. El núcleo registra el ID en `management.structural` y luego inserta la instancia en `Workspace.content.instances` o en los hijos del padre. La inserción física y la actualización de `semantic.allocation` del padre, cuando corresponda, se realizan en una sola escritura atómica del JSON. Para `bitto add`, que crea una instancia raíz, el padre es `null`.

Para remover una instancia, el journal registra su ID, el recurso de origen y su padre actual. El núcleo elimina primero el ID de `management.structural`; después elimina la instancia de su ubicación física y su ID de `semantic.allocation` del padre en una sola escritura atómica del JSON. La definición del recurso permanece intacta. La remoción de una raíz solo modifica la lista de raíces del workspace; la remoción de una hija también actualiza la contención directa de su padre.

En esta etapa, la remoción solo se permite para instancias hoja. La operación debe estar diseñada para recibir cualquier ID de instancia y localizarla recursivamente, de modo que en el futuro se pueda habilitar la remoción de subárboles sin cambiar la interfaz de la operación; por ahora, si la instancia contiene hijos, se rechaza sin modificar archivos. No se eliminan ni reubican descendientes de manera implícita.

La recuperación procesa el journal antes de exponer el workspace y completa la operación pendiente de forma idempotente siguiendo el mismo orden: registro del recurso y después workspace. Si el journal falta y los archivos discrepan, o si está corrupto o no permite resolver la operación sin ambigüedad, la recuperación se detiene sin aplanar el árbol ni convertir instancias desconocidas en raíces. La recuperación valida todos los recursos antes de modificar el workspace, exige `management.structural` en cada recurso y rechaza IDs registrados en más de un recurso. Si una operación pendiente no puede completarse de forma segura, debe conservarse el journal y comunicarse un error explícito.

## 6. Herencia y máscaras

### 6.1. Regla general

Las propiedades definidas en el recurso original pueden heredarse por una instancia, salvo las que estén protegidas o sobrepuestas por máscaras. Una máscara no es simplemente una copia completa del recurso ni una referencia que siempre refleja su estado más reciente.

### 6.2. Cambios posteriores en la definición

- Las instancias sin máscaras reciben los cambios heredables de la definición vigente.
- Las instancias parcialmente enmascaradas reciben cambios en las propiedades no protegidas.
- Las instancias totalmente enmascaradas no reciben cambios ordinarios mientras se mantenga la protección.
- Al eliminar una máscara, la parte liberada vuelve a seguir las reglas de herencia.
- Eliminar una máscara no debe borrar datos propios de la instancia sin una regla explícita y verificable.

### 6.3. Identidad y reglas de herencia

Heredar propiedades de `management` no implica heredar el ID del recurso. La identidad de la instancia permanece propia y se almacena fuera de los segmentos. `source.id` identifica el recurso original y no sustituye a `Instance.id`.

Algunos segmentos y propiedades pueden estar sujetos a reglas especiales que limiten la herencia. La implementación no debe asumir que todas las propiedades de todos los segmentos se heredan del mismo modo. Los identificadores de sistema, incluido `source.id`, no son propiedades editables mediante máscaras genéricas.

### 6.4. Privacidad y permisos

La gestión global de la definición y los permisos efectivos de una instancia son conceptos distintos. Cuando una propiedad de `management` admita máscara, esta solo crea un comportamiento local y no cambia la definición de origen. `management.structural` nunca admite acceso ni máscara desde una instancia. El sistema completo de autorización no forma parte de 0.3.

La política provisional permite que las instancias sean accesibles por terceros cuando se solicita la máscara de permisos correspondiente. Esto no debe interpretarse como autorización universal para modificar la definición original. La edición, la edición mediante referencias y la administración avanzada de permisos quedan fuera de alcance.

## 7. Espacio de trabajo y persistencia

Los recursos individuales se persisten en TOML; el espacio de trabajo y el árbol de instancias se persisten en JSON. TOML se mantiene como formato de las definiciones individuales para evitar duplicar en cada instancia toda la definición de origen. JSON representa la estructura física del workspace y también es el formato de exportación estructural previsto.

El objeto raíz del workspace contiene `content.instances`, un arreglo de instancias raíz. Cada instancia tiene `id` y `source.id`; `semantic.allocation` y `content.instances` representan sus hijos directos. Los hijos completos se anidan recursivamente dentro de `content.instances`, mientras `semantic.allocation` conserva solo sus IDs. Si una instancia no tiene hijos, cualquiera de esos componentes puede omitirse o aparecer como arreglo vacío. Si tiene hijos, ambos deben estar presentes y coincidir.

Ejemplo ilustrativo de una instancia raíz con una hija:

```json
{
  "content": {
    "instances": [
      {
        "id": "instancia-A",
        "source": { "id": "recurso-A" },
        "semantic": { "allocation": ["instancia-B"] },
        "content": {
          "instances": [
            {
              "id": "instancia-B",
              "source": { "id": "recurso-B" },
              "semantic": { "allocation": [] },
              "content": { "instances": [] }
            }
          ]
        }
      }
    ]
  }
}
```

Los IDs del ejemplo son ilustrativos, no UUIDs literales del formato real.

En el nuevo esquema, el recurso mantiene `id` en el nivel superior de su TOML. Cada instancia mantiene `id` propio en el nivel superior del JSON y `source.id` dentro del segmento `source`. No se conserva `resource_id` como campo permanente paralelo.

Se rompe deliberadamente la compatibilidad con archivos JSON que contengan `resource_id`: no se migran automáticamente ni se sobrescriben. El lector acepta exclusivamente el esquema vigente con `id` y `source.id`; los archivos antiguos o malformados producen un error explícito. El usuario deberá eliminar o recrear manualmente los datos antiguos si desea continuar.

La persistencia deberá permitir distinguir, como mínimo:

- Definiciones de recursos.
- Identidades y estado de instancias.
- Referencia `source.id` al recurso de origen.
- Árbol físico recursivo mediante `content.instances`, tanto desde la raíz del workspace como dentro de cada instancia.
- IDs de hijos directos mediante `semantic.allocation`, consistentes con el árbol físico.
- Registro `management.structural` de identidades de instancia asociado a cada definición.
- Contexto de navegación persistente.

El formato físico debe evitar duplicar sin necesidad la definición original. El espacio de trabajo por defecto no implica la existencia de perfiles, proyectos ni cuentas. Estos conceptos quedan fuera de esta versión.

## 8. Navegación

La navegación persistente puede apuntar a:

- La raíz del espacio de trabajo, representada por un contexto vacío.
- Una instancia válida contenida en el espacio de trabajo.

Reglas:

1. Solo las instancias pueden ser destinos de navegación.
2. Un recurso definido pero no instanciado puede consultarse directamente por ID, pero no convertirse en contexto de navegación.
3. Antes de aceptar una posición, el sistema debe comprobar que la instancia existe y es válida.
4. Si el destino deja de existir, la validación debe limpiar el contexto y devolverlo a la raíz.
5. Al retroceder, no se debe ofrecer un destino inexistente como posición válida.
6. La navegación se persiste mediante una operación explícita; reiniciar el programa no la restablece automáticamente.
7. El contexto actual es local y único. Su diseño no debe impedir que en el futuro se separe por perfil, proyecto, dispositivo y sesión.
8. Solo las instancias con el componente `semantic.allocation` y la raíz misma del espacio de trabajo pueden ser destino de navegación.

La expiración de sesiones, la navegación sincronizada entre dispositivos y la resolución de conflictos futuros no se implementan en 0.3.

## 9. Interfaz web local y servidor HTTP

### 9.1. Responsabilidades

El servidor HTTP será un programa local ligero que se ejecuta cuando el usuario necesita la interfaz web y termina cuando se detiene. No es un daemon persistente ni un servicio remoto.

Sus responsabilidades son servir los archivos de la interfaz y responder a las solicitudes de lectura o escritura que formen parte del alcance implementado. Debe consultar el almacenamiento bajo demanda y no cargar toda la estructura en memoria al iniciar.

La interfaz web presenta y navega los datos; el núcleo y el servidor validan la estructura y las operaciones. La lógica de dominio no debe duplicarse en JavaScript.

### 9.2. Vistas

La interfaz contempla dos vistas:

- **`map`:** representación expandida de la estructura disponible.
- **`tree`:** representación con expansión y contracción manual de ramas.

Los elementos se muestran como tarjetas compactas con título, información descriptiva y los datos o componentes pertinentes. La interfaz puede simplificar los nombres técnicos para que el usuario piense en acciones como guardar datos en un objeto o guardar un objeto dentro de otro.

### 9.3. Listados y navegación

La interfaz debe poder mostrar todos los recursos definidos, incluso aquellos que todavía no tienen instancia. Estos recursos pueden consultarse, pero no se pueden utilizar como destinos de navegación.

También debe poder explorar las instancias y su contención. La consulta de hijos directos de un padre es la operación estructural básica.

### 9.4. Paginación en dos dimensiones

La exploración utiliza dos estrategias complementarias:

1. **Precarga de `map`:** al abrir la raíz, cargar el primer nivel y, cuando resulte útil, parte del segundo nivel de los objetos presentes en ella.
2. **Expansión de `tree`:** solicitar niveles adicionales al explorar ramas, reutilizando los datos ya cargados cuando sea posible.

La precarga es una estrategia de presentación, no una obligación de cargar el árbol completo. Las consultas de hijos deben poder paginarse y limitarse al padre solicitado. El mecanismo de continuación debe evitar, en lo posible, omisiones y duplicados si la estructura cambia durante la exploración.

La carga de relaciones en la interfaz queda aplazada hasta que se implementen los componentes relacionales. En 0.3, la paginación se centra en la contención estructural.

### 9.5. Límites de seguridad

El servidor local no debe exponerse públicamente por defecto. El alcance de 0.3 no incluye cuentas, autenticación remota, colaboración ni despliegue público. Los controles de acceso completos siguen siendo trabajo futuro; la interfaz local no debe presentarse como una implementación terminada de los permisos globales descritos por el modelo.

## 10. Organización del código prevista

La separación conceptual recomendada es:

- `src/`: núcleo y persistencia.
- `cli/`: interfaz de terminal `bitto`.
- `server/`: servidor HTTP local ligero.
- `web/`: HTML, CSS y JavaScript de la interfaz.

El núcleo no debe depender de la CLI, del servidor ni de la interfaz web. El servidor puede utilizar las interfaces del núcleo y la persistencia; la interfaz web se comunica con el servidor mediante solicitudes locales.

Esta organización es una dirección de diseño, no una afirmación de que todos esos directorios o paquetes ya existan. La estructura definitiva debe respetar la organización del repositorio y evitar cambios innecesarios.

## 11. Alcance de 0.3

### Incluido en el modelo y la planificación de esta versión

- Formalizar los términos Resource, Instance, Segment, Concept y Component.
- Mantener `Resource.id` y `Instance.id` como identidades independientes fuera de los segmentos.
- Introducir el segmento de instancia `source` con el componente protegido `source.id`.
- Permitir múltiples instancias del mismo recurso, diferenciadas por `Instance.id`.
- Rechazar JSON antiguos que contengan `resource_id`; no existe migración automática.
- Restringir `management.structural` al espacio de definición.
- Distinguir el espacio de definición del espacio de instanciación.
- Definir los segmentos `management`, `info` y `semantic`.
- Ubicar `management.structural` en el espacio de definición como registro de identidades de instancia.
- Definir `semantic.allocation` como componente de contención directa por IDs de hijos.
- Introducir `content.instances` como almacenamiento físico recursivo de instancias completas.
- Validar la consistencia entre `semantic.allocation` y `content.instances`, la unicidad global de IDs y la ausencia de ciclos o múltiples padres.
- Mantener la identidad del recurso separada de la identidad de la instancia.
- Establecer reglas de herencia, máscaras y sobreposición de propiedades.
- Persistir el estado necesario para las instancias y la navegación.
- Diseñar navegación por instancias y recuperación a la raíz ante un destino inválido.
- Definir la arquitectura de una interfaz web local y un servidor HTTP bajo demanda.
- Diseñar la exploración paginada de la estructura por padre e hijos directos.

### Excluido o aplazado

- Contención mediante `semantic.allocation` en definiciones originales.
- Implementación completa de relaciones y referencias.
- Alias de recursos e instancias.
- Edición por referencias y edición avanzada.
- Sistema completo de permisos, cuentas y privacidad remota.
- Perfiles, proyectos múltiples y colaboración.
- Sincronización entre dispositivos, expiración de sesiones y resolución de conflictos.
- Daemon permanente, servicio público, servidor remoto o despliegue en la nube.
- Base de datos externa, gRPC, microVMs y sistemas de caché avanzados.
- Zoom avanzado y optimizaciones visuales posteriores.

Las exclusiones son límites de alcance, no restricciones permanentes de la arquitectura.

## 12. Criterios de aceptación del contrato

La línea 0.3 podrá considerarse coherente con este contrato cuando las implementaciones correspondientes permitan verificar que:

- [ ] `Resource.id` permanece en el nivel superior de la definición TOML, fuera de los segmentos.
- [ ] `Instance.id` permanece en el nivel superior del estado JSON, fuera de los segmentos, y es distinto del ID del recurso original.
- [ ] `source.id` referencia el recurso de origen y no puede modificarse mediante operaciones genéricas de edición.
- [ ] Varias instancias pueden referenciar el mismo recurso y distinguirse por su propio ID.
- [ ] Los JSON con `resource_id` se rechazan sin modificarlos.
- [ ] `management.structural` no puede consultarse, modificarse ni enmascararse desde el espacio de instanciación.
- [ ] `management.structural` se mantiene separado de la contención y registra las identidades de instancia asociadas a la definición.
- [ ] `semantic.allocation` registra solo los IDs de los hijos directos de una instancia.
- [ ] `Workspace.content.instances` contiene las instancias raíz y cada `Instance.content.instances` contiene recursivamente sus hijos completos.
- [ ] Cuando hay hijos, `semantic.allocation` y `content.instances` están presentes y describen los mismos hijos directos en el mismo orden; cuando no hay hijos, los componentes pueden omitirse o ser arreglos vacíos.
- [ ] Los IDs de instancia son únicos en todo el workspace; ninguna instancia aparece más de una vez en el árbol ni tiene más de un padre.
- [ ] El árbol físico no contiene ciclos y toda instancia anidada tiene una referencia `source.id` válida.
- [ ] Una discrepancia entre `semantic.allocation` y `content.instances` se rechaza explícitamente, sin corregirla silenciosamente.
- [ ] La recuperación no aplana instancias ni inventa padres cuando `management.structural` no permite conocer la jerarquía.
- [ ] La instanciación y la remoción usan un journal durable y siguen el orden journal → recurso → workspace → retirar journal.
- [ ] La remoción elimina la identidad de `management.structural` y la instancia del workspace, pero conserva la definición del recurso.
- [ ] La remoción está preparada para localizar instancias a cualquier profundidad, pero en esta etapa rechaza cualquier instancia que tenga hijos.
- [ ] La eliminación destructiva del recurso es una operación distinta y queda aplazada; no se implementa como alias de la remoción de instancia.
- [ ] La modificación de datos particulares de una instancia no modifica accidentalmente la definición original.
- [ ] Las máscaras distinguen las propiedades heredadas de las protegidas o sobrepuestas.
- [ ] La eliminación de una máscara no destruye silenciosamente datos propios de la instancia.
- [ ] La navegación solo apunta a instancias válidas o a la raíz.
- [ ] Un contexto inválido puede limpiarse y volver a la raíz.
- [ ] El listado de recursos definidos no exige que estén instanciados.
- [ ] Las consultas estructurales pueden limitarse a los hijos directos de un padre y paginarse.
- [ ] El servidor local no necesita cargar toda la estructura para iniciar o responder a una consulta puntual.
- [ ] Las reglas del modelo se validan en el núcleo y no solo en la interfaz.
- [ ] Los errores de persistencia o de referencias inválidas se comunican explícitamente.

Estos criterios describen lo que debe verificarse; no afirman que la implementación ya exista ni que las pruebas hayan sido ejecutadas.

## 13. Revisión de seguridad y límites conocidos

La implementación de este contrato debe terminar con una revisión explícita de fallas de seguridad evidentes o aparentes. La revisión deberá registrar los riesgos observados, las condiciones para reproducirlos, el impacto posible, las mitigaciones implementadas y los problemas que queden pendientes. No debe afirmarse que el sistema es seguro únicamente porque la interfaz ordinaria rechace operaciones prohibidas.

Como mínimo, se revisarán los caminos alternativos de escritura, la protección de `Resource.id`, `Instance.id` y `source.id`, el aislamiento de la definición original, el acceso a `management.structural`, la validación de archivos cargados, el rechazo del esquema JSON obsoleto, las referencias inexistentes y los estados parciales ante fallos de persistencia. La edición manual directa de los archivos locales queda fuera de la responsabilidad de la aplicación, pero los archivos que esta cargue deben validarse antes de confiar en su contenido.

Esta sección establece una revisión pendiente de la implementación; no afirma que se haya realizado una auditoría ni que se hayan encontrado o descartado vulnerabilidades.

## 14. Método de implementación

La implementación debe ser incremental:

1. Revisar el código, los contratos anteriores y las pruebas existentes.
2. Acordar el esquema de datos de instancias, segmentos y máscaras.
3. Adaptar el núcleo y la persistencia con pruebas aisladas.
4. Añadir operaciones estructurales y navegación.
5. Incorporar el servidor HTTP local cuando las operaciones del núcleo estén disponibles.
6. Construir la interfaz web sobre las operaciones expuestas por el servidor.
7. Verificar formato, compilación, pruebas, persistencia y manejo de errores.
8. Actualizar la documentación para reflejar el comportamiento realmente implementado.

No se debe declarar una característica terminada únicamente porque esté descrita en este contrato. Las decisiones pendientes de implementación deben resolverse antes de convertirlas en comportamiento observable.

---

Fin del contrato técnico de Bittopos 0.3.0.
