# Estado del proyecto

**Estado general:** prototipo local en desarrollo. La CLI `bitto` es la interfaz operativa actual. Esta página resume lo que existe y evita confundir objetivos del modelo con capacidades implementadas.

## Implementado en el repositorio

- Workspace Rust con `bittopos-core` y `bitto-cli`.
- Definiciones de recursos en TOML.
- Workspace en JSON con instancias que tienen identidad propia, referencia de origen y contención recursiva.
- Registro de IDs de instancias en `management.structural`, distinto de la contención directa `semantic.allocation`.
- Persistencia de navegación local.
- Operaciones de ciclo de vida coordinadas con journal y recuperación.
- Separación del modelo de workspace respecto a sus DTOs de persistencia.
- Operación compartida `children_of` para consultar los hijos directos de una ubicación, utilizada por la CLI en `ls` y `enter`.
- CI que exige formato Rust, compilación del workspace y pruebas con dependencias bloqueadas.
- Pruebas de integridad para IDs estructurales duplicados y discrepancias entre el recurso de origen de una instancia y su registro.

La lista anterior es un inventario general del repositorio, no una afirmación de que cada ruta de error o invariante esté exhaustivamente probada. El resultado de CI de la rama que se esté revisando es la fuente para conocer el estado real de las pruebas.

## Prototipo de interfaz

- Existe un prototipo web estático en `web/` para explorar el árbol de instancias, consultar metadatos y editar una copia local mediante importación/exportación.
- No está conectado a `bittopos-core`, no sustituye a la CLI y no escribe directamente en los archivos del dispositivo. Debe considerarse experimental.

## Acordado conceptualmente, pendiente de implementación completa

- Un modelo general de segmentos, conceptos y componentes configurables.
- Máscaras dispersas con diferencias locales y herencia dinámica, según [ADR 0003](decisions/0003-sparse-mask-overrides.md).
- Resolución de máscaras, incluidos reinicio de diferencias y eliminación local de valores heredados.
- Reglas efectivas de permisos en los espacios de definición e instanciación.
- Un mecanismo de resolución extensible a otros dominios estructurales, como tareas, responsabilidades y proyectos.
- Una capa de aplicación completa y común para futuras interfaces; ya existen `prepare_workspace`, `create_instance_at_current_location` y `remove_instance_and_update_navigation`. La creación combinada de recursos, etiquetas e instancia sigue en la CLI y requiere definir sus garantías ante fallos antes de extraerla.

## Aplazado

- Integración de la interfaz web con el núcleo mediante un servidor HTTP; daemon y servicio de larga duración.
- Interfaces móvil y de escritorio.
- Sincronización remota, colaboración y múltiples perfiles/proyectos.
- Adaptadores de bases de datos o persistencia alternativos.
- Modelo avanzado de relaciones y referencias.

## Trayectoria de ingeniería revisada

La secuencia evita combinar cambios de comportamiento con refactorizaciones de estructura. El orden siguiente es deliberado y puede ajustarse si las pruebas descubren un riesgo nuevo.

### 1. Completar la línea base de regresión — en curso

Ampliar las pruebas de `lifecycle.rs` y `store.rs` alrededor de los estados intermedios persistidos, especialmente:

- creación con journal escrito, antes y después del registro estructural;
- repetición de la recuperación después de haber aplicado los efectos, pero antes de retirar el journal;
- eliminación en distintos puntos de la operación multiarchivo;
- rechazo de estados inconsistentes sin inventar padres ni perder datos;
- validación de recursos temporales y archivos corruptos en la recuperación.

Las pruebas deben expresar estados concretos y resultados esperados. No se añadirá infraestructura general de inyección de fallos hasta que los casos reales demuestren que hace falta.

### 2. Establecer una frontera de aplicación mínima — en curso

Primer incremento: `prepare_workspace` coordina la reconciliación del workspace y la validación del estado de navegación antes de que la CLI ejecute un comando. La CLI conserva el análisis de argumentos, los mensajes y la presentación.

Segundo incremento: `create_instance_at_current_location` reutiliza el estado de navegación para crear bajo la ubicación actual; `remove_instance_and_update_navigation` coordina la eliminación y el restablecimiento de navegación cuando se elimina la ubicación activa. Ambos reutilizan las operaciones de ciclo de vida existentes.

Pendiente: delimitar la creación combinada de recurso, etiquetas e instancia. No convertirla en una única operación de aplicación hasta especificar y probar qué permanece persistido si falla un paso intermedio.

No trasladar toda la CLI de una sola vez. El caso de creación de recurso con etiquetas e instancia opcional debe delimitar explícitamente qué queda persistido si falla un paso intermedio antes de tratarlo como una única operación de aplicación.

### 3. Refactorizar persistencia por responsabilidades

Solo después de reforzar las pruebas, separar `store.rs` y `lifecycle.rs` en incrementos que conserven las interfaces y formatos existentes. Cada cambio debe indicar qué responsabilidad mueve y qué garantía protege.

- No cambiar TOML ni JSON en una refactorización interna.
- No simplificar el journal, las escrituras atómicas ni la recuperación sin pruebas equivalentes.
- Mantener la reconciliación como una decisión explícita ante dos vistas persistidas; no corregir silenciosamente datos dudosos.
- Evitar abstraer cada operación de archivo antes de que exista un límite útil para reutilización o pruebas.

### 4. Cerrar el contrato del modelo semántico — en paralelo, sin adelantar código

Resolver mediante ejemplos y decisiones documentadas las cuestiones pendientes de máscaras: identidad estable de conceptos/componentes, modificaciones iguales al valor de origen, eliminación local, restablecimiento, cambios estructurales de la definición y precedencia de permisos.

Estas decisiones no bloquean las pruebas ni la refactorización interna. Sí bloquean la implementación de máscaras y del motor de resolución hasta que exista una semántica completa y verificable.

### 5. Implementar el modelo semántico de forma incremental

Cuando el contrato esté cerrado, comenzar con pruebas de resolución independientes de TOML/JSON. Implementar solo los conceptos necesarios para los casos acordados; no construir un motor universal hipotético.

La primera cobertura debe distinguir claramente definición compartida, valores heredados, diferencias locales y errores de resolución. Los permisos efectivos se diseñarán y probarán como una preocupación separada.

### 6. Revisar la API pública y habilitar futuras interfaces

Una vez establecida la capa de aplicación y estabilizadas las reglas del modelo, revisar qué tipos y operaciones deben exponerse desde `lib.rs`. Mantener detalles de almacenamiento internos si no forman parte del contrato público deseado.

La interfaz web seguirá siendo experimental hasta que pueda consumir operaciones comunes sin duplicar reglas. No iniciar un daemon, servidor HTTP, sincronización remota o adaptadores de bases de datos dentro de esta trayectoria.

### Criterio transversal de aceptación

Cada PR debe tener un propósito acotado, conservar compatibilidad de datos salvo que una migración explícita esté aprobada, pasar formato/compilación/pruebas de CI y distinguir con honestidad las capacidades implementadas de las únicamente documentadas. Las operaciones que toquen varios archivos requieren pruebas sobre sus estados intermedios y su recuperación.

## Cómo leer el estado

- **Acordado:** decisión conceptual aprobada; puede seguir sin código.
- **Implementado:** el código contiene la capacidad.
- **Verificado:** pruebas o CI han confirmado el comportamiento en una revisión concreta.
- **Pendiente:** falta diseño, implementación o verificación.

No se asigna un porcentaje global de avance: mezclar conceptos, código y pruebas en un único número daría una impresión de precisión que no existe.
