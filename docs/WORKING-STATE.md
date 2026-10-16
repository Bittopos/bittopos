# Estado de trabajo y continuidad de Bittopos

**Última reconciliación documental:** 2026-10-16  
**Base de código examinada:** `main` en `296a76a84b28d181c904a6ab34e2db4a269f1acb` (PR #31 integrado).  
**Tipo de cambio de esta actualización:** documentación; no modifica el código de ejecución.

Esta página existe para que una conversación nueva pueda continuar el trabajo sin reconstruirlo por memoria, repetir pruebas existentes ni reabrir decisiones ya revisadas.

## 1. Dirección del producto

Bittopos busca ser una base semántica explícita y controlada por las personas para estructurar ideas y trabajo. Las definiciones reutilizables (**Resources**) se distinguen de las ocurrencias concretas (**Instances**); estas viven en un workspace navegable. El modelo previsto se organiza mediante **Segments**, **Concepts** y **Components**, y debe permitir que capacidades futuras —tareas, responsabilidades, cronogramas y colaboración— se compongan sobre la misma estructura.

La continuidad de Bittopos no debe depender de la memoria conversacional de un agente. El modelo persistido, los contratos, las decisiones registradas, las pruebas y el estado del repositorio son la fuente de continuidad.

## 2. Arquitectura elegida

- Las interfaces solicitan operaciones estructuradas.
- El núcleo valida invariantes y contiene las reglas del modelo.
- La capa de aplicación coordina casos de uso reutilizables.
- Los adaptadores se ocupan de archivos, formatos y mecanismos concretos de persistencia.
- Las fronteras se extraen cuando aclaran una responsabilidad o permiten reutilizar/probar comportamiento; no se crean capas genéricas por anticipación.
- Las escrituras atómicas de archivos individuales no hacen transaccional una operación de varios archivos.

El código actual todavía no realiza por completo esta arquitectura objetivo. La CLI es la interfaz operativa; el prototipo web es estático y experimental, no está conectado a `bittopos-core`.

## 3. Inventario actual — lo que sí está implementado

Según los archivos de código consultados en `main`:

- Workspace Rust con paquete `bittopos-core` y CLI `bitto-cli`.
- Definiciones de recursos en TOML; los metadatos actuales incluyen título, descripción y etiquetas.
- Workspace local JSON con instancias que tienen identidad propia y referencia `source.id` al recurso de origen.
- Contención recursiva de instancias; la ubicación padre-hijo directa se representa en `semantic.allocation`.
- Registro de instancias asociadas a una definición mediante `management.structural`, distinto de la jerarquía padre-hijo.
- Navegación local persistida en `navigation.toml`.
- CLI con `create`, `show`, `ls`, `enter`, `up`, `root`, `pwd`, `add`, `remove` y `recover`.
- Modelo del árbol de workspace separado de DTOs JSON y rutas.
- Journal durable para coordinar creación/eliminación de instancias y reconciliación al preparar el workspace.
- Reglas que impiden borrar instancias con descendientes y recuperación que falla explícitamente ante discrepancias que no puede resolver con seguridad.
- `journal.rs` separado de la coordinación de ciclo de vida.
- `application.rs` con tres casos de uso: `prepare_workspace`, `create_instance_at_current_location` y `remove_instance_and_update_navigation`.
- Prototipo web estático con árbol/mapa e importación/exportación local; mantiene una copia local del estado y no usa el núcleo Rust.

**Implementado no significa exhaustivamente verificado.** La suite contiene pruebas de recuperación para estados intermedios; la verificación de una revisión concreta requiere consultar el resultado de CI correspondiente. Esta actualización no ejecutó `cargo fmt`, `cargo build` ni `cargo test`.

## 4. Modelo semántico: decisiones ya revisadas

La dirección conceptual del modelo y las máscaras ya fue revisada. No iniciar la siguiente conversación preguntando otra vez si las máscaras deben ser diferencias locales o si la herencia es dinámica.

Las fuentes conceptuales vigentes son:

- [Modelo conceptual](model.md)
- [ADR 0004: resolución semántica y modos de máscara](decisions/0004-semantic-mask-resolution.md)
- [ADR 0005: reglas y límites de las máscaras](decisions/0005-mask-contract-boundaries.md)

Resumen de la dirección acordada:

- La lectura parte de una instancia y sigue la topología hacia la definición de origen.
- La correspondencia se hace mediante direcciones semánticas, no por rutas físicas ni por asumir que el ID de almacenamiento define la semántica.
- Una máscara es un Segment local de una instancia y contiene al menos un Concept o Component. Las máscaras puntuales dirigidas a Concept o Component registran diferencias locales; los elementos no modificados siguen la definición vigente.
- Editar un elemento heredado sin máscara local puede modificar la definición compartida.
- Una máscara Snapshot congela la definición vigente para las partes maskables comprendidas en la Snapshot. La redefinición ordinaria del recurso no elimina máscaras locales; cambios de política que prohíban esas operaciones pueden afectar a su efectividad.
- Los componentes pueden tener reglas específicas de combinación; las referencias se expanden siguiendo la referencia.
- Los segmentos estructurales/de control gobiernan la organización o instanciación sin confundirse con contenido semántico ordinario.
- Los permisos se resuelven como una preocupación propia y común; una máscara local no obtiene automáticamente precedencia sobre la jerarquía de permisos de origen.

El motor de resolución, las estructuras genéricas Segment/Concept/Component, las máscaras en código y la autorización efectiva todavía no están implementados. Además, algunos casos estructurales requieren una decisión explícita: cambios del origen, eliminación/ocultación local, reubicación semántica, eliminación de recursos referenciados, composición exacta de permisos y cambios estructurales bajo Snapshot. Están enumerados en ADR 0005. Esto no reabre las reglas centrales ya acordadas. No inventar una regla: presentar un ejemplo reproducible, decidir el resultado y documentarlo antes de implementar ese caso.

## 5. Trabajo de ingeniería reciente y por qué importa

PRs integrados relevantes en la secuencia actual:

- **#23 — documentación de la trayectoria de ingeniería:** prioriza el riesgo real en lugar de una lista genérica de refactors.
- **#24 — recuperación/journal:** añadió o reforzó garantías de recuperación y sus pruebas.
- **#25 y #26 — frontera de aplicación y navegación:** introdujeron casos de uso para preparar el workspace, crear en la ubicación actual y eliminar actualizando la navegación.
- **#27 — regresiones de estados inconsistentes:** protege instancias y journal cuando la información persistida contradice la operación pendiente.
- **#28 — separación física del journal:** mueve lectura/escritura/formato/limpieza a `journal.rs`, manteniendo en `lifecycle.rs` la coordinación semántica.
- **#29 — reconciliación documental:** alinea visión, modelo, ADR, arquitectura, estado y changelog; añade este documento de continuidad y registra que las máscaras están revisadas conceptualmente, mientras su motor sigue pendiente. Se integró mediante squash merge en el commit `c0c02605eebce36173d396381ea085c6ecfe2c92`; CI pasó en la rama del PR.
- **#30 — cierre del registro de continuidad:** actualiza la continuidad después de la integración del PR #29.
- **#31 — límites del contrato de máscaras:** añade ADR 0005 para separar las reglas centrales acordadas de los casos estructurales que todavía necesitan decisiones explícitas. El estado actual de `main` se verificó en el commit `296a76a84b28d181c904a6ab34e2db4a269f1acb`.

La secuencia se debe leer como una progresión de garantías y límites de responsabilidad, no como una invitación a repetir las pruebas o rehacer la separación del journal.

No se encontraron PR abiertos en la consulta realizada durante esta revisión. Antes de actuar en una conversación nueva, volver a verificar el estado real de GitHub.

## 6. Pruebas que ya existen: no duplicarlas sin un hueco demostrado

En `src/lifecycle.rs` existen, entre otras, pruebas para:

- creación recuperada tras escribir el recurso;
- creación recuperada antes de cualquiera de los efectos persistentes;
- recuperación idempotente de creación después de ambos efectos;
- eliminación recuperada antes de los efectos persistentes;
- eliminación idempotente después de aplicar los efectos;
- conflicto entre el journal de eliminación y el padre/origen real;
- journal corrupto;
- discrepancias estructurales y registro duplicado de IDs.

Los nombres confirmados incluyen `journal_recovers_creation_after_resource_write`, `journal_recovers_creation_before_either_persistent_effect`, `journal_recovers_creation_idempotently_after_both_effects`, `journal_recovers_removal_before_either_persistent_effect` y `conflicting_removal_journal_preserves_instance_and_journal`.

No proponer otra prueba con la misma precondición y el mismo resultado esperado. La línea base solo se puede marcar como cerrada después de revisar también las pruebas relevantes de temporales inválidos/ausentes en `store.rs` y comprobar el CI de la revisión actual. No se ha confirmado ese cierre aquí.

## 7. Siguiente trabajo concreto

La prioridad inmediata es preparar el núcleo para una reestructuración arquitectónica amplia. No continuar por ahora el flujo combinado de `bitto create` ni iniciar refactors de persistencia que no sean necesarios para preparar ese cambio.

El plan de preparación y sus criterios están en [Preparación para la reestructuración del núcleo](CORE-REWRITE-READINESS.md). Este documento ordena el alcance y el proceso; no autoriza por sí solo la implementación posterior.

Antes de iniciar código nuevo, consultar el PR de preparación, verificar el estado de `main` y CI, y acordar el alcance del siguiente incremento. Las reglas centrales de máscaras no se reabren por defecto; los casos no resueltos permanecen delimitados en el ADR 0005.

## 8. Prioridad y límites

La dirección estratégica de producto y la cola inmediata de ingeniería son escalas diferentes. La prioridad inmediata deja de ser la auditoría de `bitto create`: el usuario ha decidido pausar ese trabajo para preparar una reestructuración del núcleo centrada en el modelo semántico y las máscaras.

No se debe interpretar esta preparación como autorización para implementar todo el motor en un solo cambio. La reestructuración debe proponerse en incrementos revisables, conservar las garantías de persistencia salvo decisión explícita y no introducir migraciones silenciosas. No borrar ramas antiguas ni cambiar configuración de GitHub como parte de un PR documental.

## 9. Documentación y fuentes de verdad

- [README](../README.md): entrada rápida y capacidades actuales.
- [Visión](VISION.md): intención estratégica.
- [Modelo conceptual](model.md): semántica acordada.
- [ADR](decisions/README.md): decisiones y evolución.
- [Arquitectura](architecture.md): límites de módulos y estado arquitectónico.
- [Estado del proyecto](status.md): inventario, pendientes y aplazados.
- [Changelog](../CHANGELOG.md): cambios relevantes.
- [Guía de ingeniería](DEVELOPMENT-GUIDE.md): normas para desarrollar y verificar.
- [Documentación histórica](archive/README.md): contratos y planes antiguos, conservados para trazabilidad.

Cuando dos documentos parezcan discrepar, determinar cuál es la fuente normativa para esa cuestión y resolver la discrepancia explícitamente. No actualizar el estado solo porque una tarea aparezca en un resumen antiguo.

## 10. Lista de comprobación para una conversación nueva

- [ ] Leer este documento, `docs/status.md`, `docs/model.md` y el ADR relevante.
- [ ] Consultar `main`, últimos commits, PR abiertos y estado de CI actual.
- [ ] Revisar el código y las pruebas actuales del flujo que se vaya a tocar.
- [ ] Distinguir: acordado, implementado, verificado, pendiente y aplazado.
- [ ] No reabrir las máscaras como tema de diseño sin una contradicción concreta.
- [ ] No proponer pruebas ya presentes sin demostrar una brecha.
- [ ] No afirmar que se ejecutaron comandos, pasó CI o se modificó código si no se verificó.
- [ ] Antes de refactorizar persistencia, anotar las garantías existentes que deben conservarse.

**Última actualización de continuidad:** consultar el historial de Git y los PR más recientes; este documento no sustituye la comprobación del estado actual de `main` ni de CI.
