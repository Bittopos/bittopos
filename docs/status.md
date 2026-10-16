# Estado del proyecto

**Estado general:** prototipo local en desarrollo. La CLI bitto es la interfaz operativa actual. Esta página distingue capacidades implementadas, decisiones conceptuales acordadas y trabajo pendiente.

Para el punto de continuación tras el PR #28, consulta [Estado de trabajo y continuidad](WORKING-STATE.md). Esta secuencia de ingeniería inmediata no sustituye la hoja de ruta general original de diez etapas ni debe renumerarse como si lo hiciera.

## Implementado en el repositorio

- Workspace Rust con bittopos-core y bitto-cli.
- Definiciones de recursos en TOML, con ID, metadatos info (título, descripción y etiquetas) y registro management.structural.
- Workspace en JSON con instancias de identidad propia, referencia source.id al recurso de origen y contención recursiva.
- Registro de instancias en management.structural separado de la contención padre-hijo directa en semantic.allocation.
- Persistencia local de navegación mediante navigation.toml.
- Comandos CLI: create, show, ls, enter, up, root, pwd, add, remove y recover.
- Operaciones de ciclo de vida de creación/eliminación coordinadas mediante journal durable y recuperación idempotente.
- Modelo del árbol de workspace separado de DTOs JSON y detalles de rutas.
- Consulta compartida children_of para obtener los hijos directos de una ubicación.
- Capa de aplicación con prepare_workspace, create_instance_at_current_location y remove_instance_and_update_navigation.
- Validación estructural y pruebas de recuperación frente a varios estados intermedios e inconsistentes.
- CI para formato, compilación y pruebas del workspace; el resultado de una revisión concreta debe verificarse en sus ejecuciones.
- Prototipo web estático experimental en web/, con árbol/mapa e importación/exportación local, no conectado al núcleo Rust.

La lista es un inventario del código; no implica que toda ruta de error esté exhaustivamente probada ni que la CI haya pasado en una revisión que no se haya consultado.

## Acordado conceptualmente, pendiente de implementación

- Modelo semántico general de Segment, Concept y Component.
- Resolución desde la instancia hacia la definición mediante topología y direcciones semánticas.
- Máscaras puntuales de diferencias locales y máscaras Snapshot, según [ADR 0004](decisions/0004-semantic-mask-resolution.md).
- Combinación específica por tipo de componente y expansión de referencias.
- Resolución efectiva de permisos, separada de la lectura de contenido.
- Motor reutilizable de resolución para futuros dominios como tareas, responsabilidades y proyectos.

**El contrato conceptual de máscaras ya fue revisado.** No es un pendiente de diseño ni debe volver a abrirse por defecto. Lo pendiente es implementar el contrato y probarlo. Las fuentes normativas actuales son [model.md](model.md) y [ADR 0004](decisions/0004-semantic-mask-resolution.md).

## Aplazado

- Integración de la web con el núcleo Rust mediante servidor HTTP o daemon.
- Interfaces móvil y de escritorio.
- Sincronización remota, colaboración y múltiples perfiles/proyectos.
- Adaptadores de bases de datos o persistencia alternativa.
- Capacidades avanzadas de relaciones y referencias más allá de la resolución definida conceptualmente.
- Publicación y distribución de versiones estables.

## Trayectoria de ingeniería inmediata

Esta cola ordena el trabajo técnico cercano al estado posterior al PR #28. No reemplaza las diez etapas de la hoja de ruta general del producto.

### 1. Verificar y cerrar la línea base de regresión

La suite de lifecycle.rs ya contiene pruebas para creación y eliminación interrumpidas, recuperación idempotente, journal corrupto y conflictos entre el journal y el estado persistido. Entre otras, existen journal_recovers_creation_after_resource_write, journal_recovers_creation_before_either_persistent_effect, journal_recovers_creation_idempotently_after_both_effects, journal_recovers_removal_before_either_persistent_effect y conflicting_removal_journal_preserves_instance_and_journal.

No añadas otra prueba de esos mismos casos sin demostrar primero una condición distinta que esté sin cubrir. El cierre de esta línea requiere revisar los casos de temporales inválidos/ausentes en store.rs y el resultado de CI actual; no debe declararse completa solo porque existan algunas pruebas.

### 2. Auditar el flujo combinado de bitto create

Este es el siguiente trabajo concreto. Hoy la CLI:
1. crea el recurso;
2. si se solicitaron etiquetas, actualiza el recurso por separado;
3. salvo --no-instance, crea una instancia mediante la operación de ciclo de vida.

La CLI ya informa de forma explícita si el recurso quedó creado pero falló el guardado de etiquetas o la creación de la instancia. Antes de extraer el flujo a application.rs, documentar qué estados parciales pueden quedar, cuál es el comportamiento deseado y qué garantías de recuperación son necesarias. Después, comparar esos criterios con las pruebas existentes y añadir solo la cobertura que falte.

No convertirlo automáticamente en una transacción ni borrar un recurso al fallar un paso sin definir antes el contrato de consistencia y recuperación.

### 3. Continuar la separación de persistencia solo donde exista un límite real

journal.rs ya separa la persistencia física del journal de la coordinación semántica en lifecycle.rs. Revisar store.rs y workspace.rs para identificar una frontera útil, no para reducir el tamaño de los archivos por sí mismo.

El almacenamiento de recursos tiene una ruta especializada de preparar, validar y promover temporales. No reemplazarla sin más por la escritura atómica genérica del workspace: sus garantías y su tratamiento de errores son distintos. No cambiar formatos TOML/JSON ni simplificar la recuperación sin pruebas equivalentes.

### 4. Completar la frontera de aplicación y la API después de la auditoría

Una vez especificadas y probadas las garantías de bitto create, decidir si ese flujo debe convertirse en un caso de uso de aplicación. No trasladar toda la CLI de golpe. Revisar la API pública cuando las operaciones compartidas y el modelo de dominio estén más estables.

### 5. Implementar el motor semántico conforme al contrato acordado

No reabrir el diseño de máscaras como primer paso. Implementar el contrato de [ADR 0004](decisions/0004-semantic-mask-resolution.md) de forma incremental: pruebas puras de resolución, topología/direcciones, máscaras puntuales y Snapshot, comportamiento de componentes, referencias y permisos. Mantener la persistencia como adaptador y no duplicar la semántica en la CLI o en la web.

### 6. Integrar futuras interfaces cuando exista una operación compartida

La web seguirá siendo experimental hasta consumir operaciones comunes del núcleo. No iniciar servidor, sincronización remota o adaptadores de bases de datos dentro de la tarea inmediata.

## Criterio transversal de aceptación

Cada cambio debe tener un objetivo acotado, preservar los datos persistidos salvo migración aprobada, pasar las comprobaciones aplicables y actualizar la documentación afectada. Las operaciones multiarchivo requieren pruebas de estados intermedios y recuperación. Los resultados de CI deben comprobarse en GitHub; no se infieren.

## Cómo leer el estado

- **Acordado:** decisión conceptual revisada; no equivale a implementación.
- **Implementado:** el código contiene la capacidad.
- **Verificado:** pruebas o CI confirmaron el comportamiento en una revisión concreta.
- **Pendiente:** falta implementación, integración o una verificación específica.
- **Aplazado:** no pertenece al alcance actual.

No se asigna un porcentaje global de avance: mezclar modelo, código, integración y pruebas en una cifra daría una precisión engañosa.
