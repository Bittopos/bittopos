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
3. Distinguir la contención estructural de las asignaciones, relaciones y referencias.
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

Aparición concreta de un recurso. Cada instancia tiene un identificador propio `id`, distinto del ID del recurso original. El identificador de instancia se almacena como campo de nivel superior, fuera de los segmentos, igual que `Resource.id` en la definición. La instancia identifica además su recurso de origen mediante `source.id`. Varias instancias pueden proceder del mismo recurso y deben distinguirse por su propio `id`. La instancia puede tener segmentos propios o segmentos máscara.

### 2.5. Segment

Parte diferenciada de un recurso o de una instancia. Los segmentos de sistema pertenecen al espacio de definición. Los segmentos de instancia existen únicamente en una instancia.

### 2.6. Segmento máscara

Segmento definido en una instancia que corresponde a un segmento del recurso original. Permite controlar la herencia de sus propiedades y sobreponer información propia de la instancia.

Una máscara puede preservar propiedades frente a cambios posteriores de la definición y aportar valores particulares. Las reglas de herencia se aplican por las unidades de información que la máscara protege; la mera existencia de una instancia no significa que todas sus propiedades estén congeladas.

Si una instancia no tiene máscara para una propiedad heredable, esta puede reflejar los cambios de la definición original. Una máscara parcial protege únicamente la parte que cubre. Una instancia completamente enmascarada no recibe cambios ordinarios de la definición mientras se mantenga ese estado. Al retirar una máscara, la parte liberada vuelve a seguir las reglas de herencia. Retirar una máscara no debe borrar datos particulares sin una regla explícita.

Un segmento de instancia que no corresponde a un segmento definido en el recurso original es un segmento exclusivo de la instancia, no una máscara.

### 2.7. Concept

Unidad de datos de usuario compuesta por una clave y un valor. El valor puede ser un arreglo. En esta versión, un Concept no contiene recursivamente otros Concept como estructura arbitraria.

Los datos del sistema —por ejemplo, identificadores de recursos, instancias, relaciones o referencias— se representan mediante componentes especializados, no como Concept de usuario.

El contrato de implementación deberá fijar el conjunto de tipos admitidos para los valores de Concept. No se debe asumir que todos los valores son cadenas.

### 2.8. Component

Estructura especializada que representa datos del sistema o una organización semántica concreta dentro de un segmento. Los componentes pueden contener identificadores u otras estructuras tipadas.

### 2.9. Contención, asignación, relación y referencia

- **Contención:** expresa la composición estructural de un objeto mediante otros objetos.
- **Asignación:** registra en el espacio de definición identificadores asignados, incluidas las identidades de las instancias creadas a partir de un recurso.
- **Relación:** expresa una conexión semántica entre elementos, distinta de la contención.
- **Referencia:** identifica o apunta a otro elemento sin implicar por sí misma contención.

Estos conceptos no son intercambiables y no deben almacenarse bajo una única regla semántica.

### 2.10. Workspace

Contexto de trabajo local que organiza las instancias utilizadas. En esta versión existe un espacio de trabajo por defecto; no existe todavía un sistema completo de perfiles o proyectos.

### 2.11. Navigation context

Posición persistente de navegación. Puede ser la raíz del espacio de trabajo o una instancia válida dentro de él. La posición no puede apuntar a un recurso no instanciado.

## 3. Segmentos iniciales del recurso y de la instancia

### 3.1. `management`

Segmento de sistema para responsabilidades administrativas globales de la definición del recurso, incluidas privacidad, permisos globales y demás propiedades de gestión. El campo superior `Resource.id` permanece fuera de los segmentos y no se hereda como identidad de una instancia.

`management` se administra desde el espacio de definición. Una instancia no puede utilizar operaciones ordinarias para leer o modificar componentes de gestión global ni cambiar la definición de origen. Cuando el contrato permita una máscara de otra propiedad de `management`, esta solo podrá producir un comportamiento local de la instancia; nunca modificará la definición original. `management.allocation` es una excepción estricta: no puede consultarse, modificarse ni enmascararse desde el espacio de instanciación.

El componente `allocation` pertenece a `management`, porque registra las identidades de las instancias asignadas a una definición. No describe los objetos que un recurso contiene. Sus cambios son operaciones controladas del sistema asociadas al ciclo de vida de las instancias.

### 3.2. `info`

Segmento de sistema para información descriptiva de un recurso. En el modelo actual puede incluir título, descripción y etiquetas. Los datos descriptivos no sustituyen la identidad del recurso.

### 3.3. `semantic`

Segmento de sistema para la organización semántica de un recurso. Los componentes conceptuales previstos incluyen:

- `structural`: contención directa.
- `relational`: relaciones semánticas.
- `referential`: referencias a otros elementos.

En 0.3 estos nombres sirven para fijar el modelo y el glosario, pero solo se implementará la funcionalidad estructural expresamente incluida en el alcance. Las relaciones y referencias avanzadas quedan aplazadas.

### 3.4. `source` (segmento de instancia)

`source` identifica la definición de la que procede una instancia. Su componente `source.id` contiene el ID del recurso original. Es una referencia al origen, no la identidad de la instancia.

`source.id` es un componente de sistema protegido: las operaciones ordinarias de edición de segmentos, componentes o conceptos no pueden reemplazarlo ni alterarlo. Su cambio solo puede ocurrir mediante operaciones internas explícitas y validadas que formen parte de la creación de una instancia.

La denominación y ubicación de `source.id` se consideran provisionales hasta comprobar su estabilidad durante la implementación, pero constituyen el modelo previsto para esta línea. No debe añadirse un campo permanente paralelo `resource_id` al nuevo esquema.

## 4. Componentes y reglas de uso

### 4.1. `allocation`

`allocation` pertenece a `management` y al espacio de definición. Registra los identificadores asignados a las instancias creadas a partir de un recurso. Sirve para conservar la correspondencia entre la definición y las identidades de sus instancias y facilita la validación y recuperación ante fallos.

La identidad de cada instancia es independiente del ID del recurso original. No se debe inferir la identidad de una instancia a partir del ID de su recurso. Cada instancia mantiene `id` propio y referencia el origen mediante `source.id`.

`allocation` es un componente administrativo reservado al sistema: no puede leerse, modificarse ni enmascararse mediante la interfaz del espacio de instanciación. Su modificación debe corresponder a operaciones controladas de creación, recuperación o eliminación de instancias en el espacio de definición.

En esta versión, `allocation` no representa contención ni almacena los identificadores de los objetos que contiene una instancia. La implementación de relaciones y referencias generales queda fuera del alcance.

### 4.2. `structural`

`structural` expresa qué objetos contiene directamente una instancia. Registra solo el primer nivel de contención; los descendientes se descubren recorriendo la estructura, no duplicando todos sus IDs en el padre.

Aunque la contención se considera universal y conceptualmente puede existir en el espacio de definición o en el de instanciación, **en 0.3 solo se utilizará en instancias**. No se implementará la contención estructural global en recursos originales, ya que podría afectar a todas sus instancias.

La interfaz podrá presentar esta operación al usuario con expresiones sencillas, como guardar un objeto dentro de otro. No debe exigir que el usuario conozca el término técnico `structural`.

### 4.3. `relational` y `referential`

Son componentes conceptuales reservados para relaciones y referencias. No se implementarán como funcionalidades completas en esta versión. Su futura incorporación no debe obligar a reinterpretar la contención como una relación genérica.

## 5. Identidad, definición e instancia

1. Cada recurso tiene un `id` estable en el nivel superior de su definición TOML, fuera de los segmentos.
2. Cada instancia tiene un `id` propio en el nivel superior de su representación JSON, fuera de los segmentos y distinto del ID del recurso original.
3. Cada instancia contiene el segmento `source`, cuyo componente `source.id` referencia el recurso de origen.
4. Varias instancias pueden compartir `source.id`, pero nunca la misma identidad de instancia dentro del ámbito de unicidad definido por el sistema.
5. Crear una instancia no crea una nueva definición de recurso.
6. Crear o modificar datos propios de una instancia no debe modificar accidentalmente la definición original.
7. `allocation` registra en la definición las identidades asignadas a sus instancias.
8. El contexto de navegación identifica una instancia, no solo el recurso de origen.
9. Los identificadores internos del sistema no se representan como simples datos de usuario ni se modifican mediante ediciones genéricas de segmentos.
10. Los alias no forman parte del alcance funcional de 0.3. Si se añaden en el futuro, no sustituirán a los identificadores estables.

La implementación mantiene `allocation` como fuente de verdad para reconstruir el workspace. La creación confirma primero la asignación y luego persiste la instancia; la eliminación retira primero la asignación y luego persiste el workspace. Si el proceso se interrumpe entre ambos pasos, la siguiente reconciliación reconstruye las instancias asignadas y elimina las residuales. La recuperación valida todos los recursos antes de modificar el workspace, exige que cada recurso tenga `management.allocation` y se detiene ante archivos ilegibles, registros ausentes o IDs asignados a más de un recurso. El núcleo permite varias instancias del mismo origen y valida la identidad propia de cada instancia.

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

La gestión global de la definición y los permisos efectivos de una instancia son conceptos distintos. Cuando una propiedad de `management` admita máscara, esta solo crea un comportamiento local y no cambia la definición de origen. `management.allocation` nunca admite acceso ni máscara desde una instancia. El sistema completo de autorización no forma parte de 0.3.

La política provisional permite que las instancias sean accesibles por terceros cuando se solicita la máscara de permisos correspondiente. Esto no debe interpretarse como autorización universal para modificar la definición original. La edición, la edición mediante referencias y la administración avanzada de permisos quedan fuera de alcance.

## 7. Espacio de trabajo y persistencia

Los recursos individuales se persisten en TOML; el espacio de trabajo, las identidades de instancia y su estado se persisten en JSON. TOML se mantiene como formato de las definiciones individuales para evitar duplicar en cada instancia toda la definición de origen. JSON se utiliza para representar la estructura del espacio de trabajo y también será el formato de exportación estructural previsto.

En el nuevo esquema, el recurso mantiene `id` en el nivel superior de su TOML. Cada instancia mantiene `id` propio en el nivel superior del JSON y `source.id` dentro del segmento `source`. No se conserva `resource_id` como campo permanente paralelo.

Se rompe deliberadamente la compatibilidad con archivos JSON que contengan `resource_id`: no se migran automáticamente ni se sobrescriben. El lector acepta exclusivamente el esquema vigente con `id` y `source.id`; los archivos antiguos o malformados producen un error explícito. El usuario deberá eliminar o recrear manualmente los datos antiguos si desea continuar.

La persistencia deberá permitir distinguir, como mínimo:

- Definiciones de recursos.
- Identidades y estado de instancias.
- Referencia `source.id` al recurso de origen.
- Contención directa de cada instancia.
- Registro de asignaciones de instancia asociado a cada definición.
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
8. Solo las instancias con componente `structural` y la raiz misma del espacio de trabajo pueden ser destino de navegación.

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
- Restringir `management.allocation` al espacio de definición.
- Distinguir el espacio de definición del espacio de instanciación.
- Definir los segmentos `management`, `info` y `semantic`.
- Ubicar `allocation` en `management`.
- Definir `structural` como componente de contención directa.
- Mantener la identidad del recurso separada de la identidad de la instancia.
- Establecer reglas de herencia, máscaras y sobreposición de propiedades.
- Persistir el estado necesario para las instancias y la navegación.
- Diseñar navegación por instancias y recuperación a la raíz ante un destino inválido.
- Definir la arquitectura de una interfaz web local y un servidor HTTP bajo demanda.
- Diseñar la exploración paginada de la estructura por padre e hijos directos.

### Excluido o aplazado

- Contención estructural global en definiciones originales.
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
- [ ] `management.allocation` no puede consultarse, modificarse ni enmascararse desde el espacio de instanciación.
- [ ] `allocation` se mantiene separado de la contención y registra las identidades de instancia asociadas a la definición.
- [ ] `structural` registra solo los hijos directos de una instancia.
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

Como mínimo, se revisarán los caminos alternativos de escritura, la protección de `Resource.id`, `Instance.id` y `source.id`, el aislamiento de la definición original, el acceso a `management.allocation`, la validación de archivos cargados, el rechazo del esquema JSON obsoleto, las referencias inexistentes y los estados parciales ante fallos de persistencia. La edición manual directa de los archivos locales queda fuera de la responsabilidad de la aplicación, pero los archivos que esta cargue deben validarse antes de confiar en su contenido.

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
