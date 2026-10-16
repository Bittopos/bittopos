# Arquitectura de Bittopos

**Estado:** describe la dirección arquitectónica y distingue esa dirección de la implementación presente. Las separaciones se introducen cuando eliminan dependencias, aclaran responsabilidades o facilitan pruebas; no por cumplir un diagrama ideal.

## Dirección

    Interfaces y adaptadores
      CLI / web / móvil / escritorio / adaptadores de IA
                     |
                     v
    Núcleo Bittopos
      modelo + reglas + resolución semántica + operaciones de dominio
                     |
                     v
    Adaptadores de datos
      TOML / JSON / base de datos / otros almacenes

El diagrama expresa dependencias deseadas, no una lista de componentes ya terminados. La CLI es la interfaz operativa actual; la web es experimental y no está conectada al núcleo Rust. Móvil, escritorio, adaptadores de IA y servicios remotos son objetivos futuros.

## Principio de dependencia

Las interfaces solicitan operaciones estructuradas. El núcleo valida las reglas del modelo. La coordinación de cambios que involucran varios recursos o archivos se sitúa en operaciones de aplicación/ciclo de vida donde aporte valor.

El núcleo no debe depender innecesariamente de una interfaz concreta ni de un formato de archivo. Al mismo tiempo, no es necesario crear una abstracción genérica de almacenamiento para cada módulo si todavía no elimina una dependencia real.

El workspace puede entenderse como una base de datos estructurada. JSON es el formato inicial de persistencia del workspace y puede servir para exportar información legible por modelos de lenguaje, pero no define por sí solo la semántica del modelo ni debe ser el límite conceptual del núcleo.

## Estado del código

La implementación actual contiene, entre otros:

- **workspace_model.rs:** tipos e invariantes del modelo del árbol de instancias, separados de Serde y las rutas.
- **workspace.rs:** DTOs JSON, conversiones y persistencia del workspace.
- **resource.rs y store.rs:** definición y almacenamiento de recursos TOML, con validación y promoción controlada de temporales.
- **journal.rs:** formato, rutas, lectura, escritura y limpieza del registro durable de operaciones pendientes.
- **lifecycle.rs:** coordinación de operaciones de ciclo de vida, aplicación del journal y reconciliación.
- **navigation.rs:** persistencia del estado de navegación.
- **application.rs:** frontera reutilizable con prepare_workspace, create_instance_at_current_location y remove_instance_and_update_navigation.
- **cli/src/main.rs:** parsing, presentación y parte de la coordinación de la interfaz de terminal.

La creación combinada de recurso, etiquetas e instancia opcional todavía permanece en la CLI. Antes de extraerla como un caso de uso de aplicación, hay que definir y probar qué debe quedar persistido cuando falle un paso intermedio.

## Persistencia y recuperación

La escritura atómica de un archivo no es una transacción atómica entre varios archivos. Las operaciones que actualizan tanto una definición como un workspace necesitan coordinación durable y recuperación idempotente.

Las invariantes estructurales actuales incluyen identidad única de instancias en el workspace, referencias a recursos de origen, contención recursiva y coherencia entre las instancias registradas en management.structural de un recurso y su presencia en el workspace. La contención padre-hijo directa se expresa en semantic.allocation; el registro estructural de una definición no codifica por sí solo la jerarquía padre-hijo.

Los archivos persistidos deben validarse al cargarse. Ante una discrepancia que no pueda resolverse de forma segura, la recuperación debe fallar de manera explícita, sin inventar padres ni aplanar la estructura.

## Modelo semántico y máscaras

El contrato conceptual de máscaras está revisado y registrado en [ADR 0004](decisions/0004-semantic-mask-resolution.md) y [model.md](model.md). La implementación del motor de resolución, las máscaras puntuales y Snapshot, los componentes con comportamiento específico, la expansión de referencias y la autorización efectiva sigue pendiente.

No debe volver a plantearse como tarea de diseño abierta la decisión básica sobre herencia dinámica de máscaras puntuales. La siguiente prioridad de ingeniería inmediata está en la frontera de aplicación de la CLI, no en reabrir el contrato de máscaras.

## Cómo evolucionar la arquitectura

1. Definir el comportamiento del modelo y sus ejemplos antes de introducir un motor general.
2. Probar las reglas de resolución del modelo sin depender de TOML/JSON.
3. Mantener conversión y persistencia en los adaptadores correspondientes.
4. Coordinar operaciones multiarchivo en un lugar identificable, con pruebas de fallos y recuperación.
5. Hacer que la CLI consuma operaciones del núcleo en lugar de duplicar reglas.
6. Añadir interfaces y adaptadores cuando exista un caso de uso real.
7. Revisar límites arquitectónicos cuando una dependencia real lo justifique.

No se implementan todavía el motor de máscaras, permisos efectivos, servidor HTTP ni persistencia alternativa. Esta documentación describe la dirección y el vocabulario compartido, no declara esas capacidades terminadas.
