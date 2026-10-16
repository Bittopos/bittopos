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
- Máscaras puntuales de diferencias locales y máscaras Snapshot, según [ADR 0004](decisions/0004-semantic-mask-resolution.md) y [ADR 0005](decisions/0005-mask-contract-boundaries.md).
- Combinación específica por tipo de componente y expansión de referencias.
- Resolución efectiva de permisos, separada de la lectura de contenido.
- Motor reutilizable de resolución para futuros dominios como tareas, responsabilidades y proyectos.

**Las reglas centrales del contrato conceptual de máscaras ya fueron revisadas.** No deben reabrirse por defecto. Además de implementar y probar esas reglas, quedan por decidir algunos casos de borde estructurales enumerados en [ADR 0005](decisions/0005-mask-contract-boundaries.md): cambios estructurales del origen, ocultación/eliminación local, reubicación semántica, eliminación de recursos referenciados, precedencia precisa de permisos y cambios estructurales bajo Snapshot. No inventar esas reglas al implementar. Las fuentes normativas actuales son [model.md](model.md), [ADR 0004](decisions/0004-semantic-mask-resolution.md) y [ADR 0005](decisions/0005-mask-contract-boundaries.md).

## Aplazado

- Integración de la web con el núcleo Rust mediante servidor HTTP o daemon.
- Interfaces móvil y de escritorio.
- Sincronización remota, colaboración y múltiples perfiles/proyectos.
- Adaptadores de bases de datos o persistencia alternativa.
- Capacidades avanzadas de relaciones y referencias más allá de la resolución definida conceptualmente.
- Publicación y distribución de versiones estables.

## Próxima fase de ingeniería: preparar el cambio del núcleo

El trabajo de `bitto create` queda pausado por decisión de producto. La siguiente prioridad es preparar una reestructuración del núcleo para implementar de forma coherente el modelo semántico y las máscaras, sin modificar aún el comportamiento ejecutable.

La guía de alcance y aceptación está en [Preparación para la reestructuración del núcleo](CORE-REWRITE-READINESS.md).

### Límites de la fase actual

- Es documental y preparatoria: no implementa máscaras ni el motor de resolución.
- No continúa la auditoría/extracción del flujo de `bitto create`.
- No modifica formatos TOML/JSON ni realiza migraciones.
- No borra ramas ni cambia la configuración de protección de GitHub.
- Conserva las reglas centrales de los ADR 0004 y 0005; los casos pendientes del ADR 0005 no se resuelven por inferencia.

### Criterios para iniciar una implementación posterior

Antes de abrir el primer PR de código para la reestructuración, debe existir un alcance concreto que indique tipos y límites afectados, invariantes que se conservan o cambian, compatibilidad/migración de datos, pruebas necesarias y comprobaciones reales de CI. La línea base debe establecerse sobre el `main` vigente; no se considera verificada por el mero hecho de que existan pruebas.

La secuencia sugerida es modelo puro y sus invariantes, topología/direcciones semánticas, resolución incremental, máscaras puntuales y Snapshot, componentes/referencias/autorización según sus dependencias, y después integración de adaptadores e interfaces. Esta secuencia no autoriza automáticamente cada fase; el alcance posterior se decidirá tras revisar el PR de preparación.

## Criterio transversal de aceptación

Cada cambio debe tener un objetivo acotado, preservar los datos persistidos salvo migración aprobada, pasar las comprobaciones aplicables y actualizar la documentación afectada. Las operaciones multiarchivo requieren pruebas de estados intermedios y recuperación. Los resultados de CI deben comprobarse en GitHub; no se infieren.

## Cómo leer el estado

- **Acordado:** decisión conceptual revisada; no equivale a implementación.
- **Implementado:** el código contiene la capacidad.
- **Verificado:** pruebas o CI confirmaron el comportamiento en una revisión concreta.
- **Pendiente:** falta implementación, integración o una verificación específica.
- **Aplazado:** no pertenece al alcance actual.

No se asigna un porcentaje global de avance: mezclar modelo, código, integración y pruebas en una cifra daría una precisión engañosa.
