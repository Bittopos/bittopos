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

La lista anterior es un inventario general del repositorio, no una afirmación de que cada ruta de error o invariante esté exhaustivamente probada. El resultado de CI de la rama que se esté revisando es la fuente para conocer el estado real de las pruebas.

## Prototipo de interfaz

- Existe un prototipo web estático en `web/` para explorar el árbol de instancias, consultar metadatos y editar una copia local mediante importación/exportación.
- No está conectado a `bittopos-core`, no sustituye a la CLI y no escribe directamente en los archivos del dispositivo. Debe considerarse experimental.

## Acordado conceptualmente, pendiente de implementación completa

- Un modelo general de segmentos, conceptos y componentes configurables.
- Máscaras de segmento en instancias y resolución de herencia.
- Reglas efectivas de permisos en los espacios de definición e instanciación.
- Un mecanismo de resolución extensible a otros dominios estructurales, como tareas, responsabilidades y proyectos.
- Una capa de aplicación reutilizable por futuras interfaces, allí donde un caso de uso real justifique la separación.

## Aplazado

- Integración de la interfaz web con el núcleo mediante un servidor HTTP; daemon y servicio de larga duración.
- Interfaces móvil y de escritorio.
- Sincronización remota, colaboración y múltiples perfiles/proyectos.
- Adaptadores de bases de datos o persistencia alternativos.
- Modelo avanzado de relaciones y referencias.

## Próximo orden de trabajo recomendado

1. Acordar las reglas generales de segmentos, conceptos, componentes y máscaras.
2. Definir el comportamiento del mecanismo de resolución con ejemplos y casos límite.
3. Convertir esas reglas en pruebas de modelo independientes de persistencia.
4. Implementar la resolución mínima necesaria y documentar sus límites.
5. Revisar permisos por separado: una regla descrita no equivale a control de acceso implementado.
6. Mantener README, especificación y changelog sincronizados con cada PR relevante.

## Cómo leer el estado

- **Acordado:** decisión conceptual aprobada; puede seguir sin código.
- **Implementado:** el código contiene la capacidad.
- **Verificado:** pruebas o CI han confirmado el comportamiento en una revisión concreta.
- **Pendiente:** falta diseño, implementación o verificación.

No se asigna un porcentaje global de avance: mezclar conceptos, código y pruebas en un único número daría una impresión de precisión que no existe.
