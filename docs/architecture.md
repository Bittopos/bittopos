# Arquitectura de Bittopos

**Estado:** describe la dirección arquitectónica y distingue esa dirección de la implementación presente. Las separaciones se introducen cuando eliminan dependencias, aclaran responsabilidades o facilitan pruebas; no por cumplir un diagrama ideal.

## Dirección

```text
Interfaces y adaptadores
  CLI / web / móvil / escritorio / adaptadores de IA
                 |
                 v
Núcleo Bittopos
  modelo + reglas + operaciones de dominio
                 |
                 v
Adaptadores de datos
  TOML / JSON / base de datos / otros almacenes
```

El diagrama expresa dependencias deseadas, no una lista de componentes ya terminados. La CLI es la interfaz disponible hoy; web, móvil, escritorio y adaptadores de IA son objetivos futuros.

## Principio de dependencia

Las interfaces solicitan operaciones estructuradas. El núcleo valida las reglas del modelo. La coordinación de cambios que involucran varios recursos o archivos se sitúa en operaciones de aplicación/ciclo de vida donde aporte valor.

El núcleo no debe depender innecesariamente de una interfaz concreta ni de un formato de archivo. Al mismo tiempo, no es necesario crear una abstracción genérica de almacenamiento para cada módulo si todavía no elimina una dependencia real.

El workspace puede entenderse como una base de datos estructurada. JSON es el formato de persistencia inicial del workspace y puede ser útil para exportar información legible por modelos de lenguaje, pero no define por sí solo la semántica del modelo ni debe ser el límite conceptual del núcleo.

## Estado del código

La implementación actual contiene, entre otros:

- `workspace_model.rs`: tipos e invariantes del modelo del árbol de instancias, separados de Serde y las rutas.
- `workspace.rs`: DTOs JSON, conversiones y persistencia del workspace.
- `resource.rs` y `store.rs`: definición y almacenamiento de recursos TOML, con validación.
- `journal.rs`: formato, rutas, lectura, escritura y limpieza del registro durable de operaciones pendientes.
- `lifecycle.rs`: coordinación de operaciones de ciclo de vida, aplicación del journal y reconciliación.
- `navigation.rs`: persistencia del estado de navegación.
- `cli/src/main.rs`: parsing, presentación y parte de la coordinación de la interfaz de terminal.

Existe una frontera de aplicación reutilizable con tres casos de uso: `prepare_workspace` coordina recuperación y validación de navegación; `create_instance_at_current_location` crea una instancia bajo la ubicación seleccionada; y `remove_instance_and_update_navigation` elimina una instancia hoja y restablece la navegación si apuntaba a ella. La creación combinada de recursos, etiquetas e instancia aún permanece en la CLI porque sus garantías ante fallos necesitan definirse. El modelo general Segment/Concept/Component tampoco está implementado de forma genérica.

## Persistencia y recuperación

La escritura atómica de un archivo no es una transacción atómica entre varios archivos. Las operaciones que actualizan tanto una definición como un workspace necesitan coordinación durable y recuperación idempotente.

Las invariantes estructurales conocidas incluyen identidad única de instancias en el workspace, referencias a recursos de origen, contención recursiva y coherencia entre los hijos físicos directos y sus IDs registrados en `semantic.allocation`. `management.structural` registra las instancias asociadas a una definición; no codifica la jerarquía padre-hijo.

Los archivos persistidos deben validarse al cargarse. Ante una discrepancia que no pueda resolverse de forma segura, la recuperación debe fallar de manera explícita, sin inventar padres ni aplanar la estructura.

## Cómo evolucionar la arquitectura

1. Definir el comportamiento del modelo y ejemplos de resolución antes de introducir un motor general.
2. Probar las reglas del modelo sin depender del sistema de archivos.
3. Mantener conversión y persistencia en los adaptadores correspondientes.
4. Coordinar operaciones multiarchivo en un lugar identificable, con pruebas de fallos y recuperación.
5. Hacer que la CLI consuma operaciones del núcleo en lugar de duplicar reglas.
6. Añadir interfaces y adaptadores cuando exista un caso de uso real.
7. Revisar límites arquitectónicos cada vez que una dependencia real lo justifique.

No se implementan en este cambio máscaras, permisos efectivos, servidor, web ni persistencia alternativa. Esta documentación solo establece dirección y vocabulario compartido.
