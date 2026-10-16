# Estado de trabajo y continuidad de Bittopos

**Última reconciliación documental:** 2026-10-15  
**Base de código examinada:** rama `main` posterior al PR #28, según los archivos consultados durante esta revisión.  
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
- Navegación local persistida en `navigation.toml).
- CLI con `create`, `show`, `ls`, `enter`, `up`, `root`, `pwd`, `add`, `remove` y `recover`.
- Modelo del árbol de workspace separado de DTOs JSON y rutas.
- Journal durable para coordinar creación/eliminación de instancias y reconciliación al preparar el workspace.
- Reglas que impiden borrar instancias con descendientes y recuperación que falla explícitamente ante discrepancias que no puede resolver con seguridad.
- `journal.rs` separado de la coordinación de ciclo de vida.
- `application.rs` con tres casos de uso: `prepare_workspace`, `create_instance_at_current_location` y `remove_instance_and_update_navigation).
- Prototipo web estático con árbol/mapa e importación/exportación local; mantiene una copia local del estado y no usa el núcleo Rust.

**Implementado no significa exhaustivamente verificado.** La suite contiene pruebas de recuperación para estados intermedios; la verificación de una revisión concreta requiere consultar el resultado de CI correspondiente. Esta actualización no ejecutó `cargo fmt`, `cargo build` ni `cargo test`.

## 4. Modelo semántico: decisiones ya revisadas

La dirección conceptual del modelo y las máscaras ya fue revisada. No iniciar la siguiente conversación preguntando otra vez si las máscaras deben ser diferencias locales o si la herencia es dinámica.

Las fuentes conceptuales vigentes son:

- [Modelo conceptual](model.md)
- [ADR 0004: resolución semántica y modos de máscara](decisions/0004-semantic-mask-resolution.md)

Resumen de la dirección acordada:

- La lectura parte de una instancia y sigue la topología hacia la definición de origen.
- La correspondencia se hace mediante direcciones semánticas, no por rutas físicas ni por asumir que el ID de almacenamiento define la semántica.
- Las máscaras puntuales dirigidas a Concept o Component registran diferencias locales; los elementos no modificados siguen la definición vigente.
- Editar un elemento heredado sin máscara local puede modificar la definición compartida.
- Una máscara Snapshot congela la definición vigente para las partes explícitamente incluidas en su alcance.
- Los componentes pueden tener reglas específicas de combinación; las referencias se expanden siguiendo la referencia.
- Los segmentos estructurales/de control gobiernan la organización o instanciación sin confundirse con contenido semántico ordinario.
- Los permisos se resuelven como una preocupación propia y común; una máscara local no obtiene automáticamente precedencia sobre la jerarquía de permisos de origen.

El motor de resolución, las estructuras genéricas Segment/Concept/Component, las máscaras en código y la autorización efectiva todavía no están implementados. Esta es una brecha de implementación, no una razón para volver a abrir el diseño por defecto. Si al implementar aparece una contradicción concreta, documentar el caso reproducible y su impacto antes de proponer modificar el contrato.

## 5. Trabajo de ingeniería reciente y por qué importa

PRs integrados relevantes en la secuencia actual:

- **#23 — documentación de la trayectoria de ingeniería:** prioriza el riesgo real en lugar de una lista genérica de refactors.
- **#24 — recuperación/journal:** añadió o reforzó garantías de recuperación y sus pruebas.
- **#25 y #26 — frontera de aplicación y navegación:** introdujeron casos de uso para preparar el workspace, crear en la ubicación actual y eliminar actualizando la navegación.
- **#27 — regresiones de estados inconsistentes:** protege instancias y journal cuando la información persistida contradice la operación pendiente.
- **#28 — separación física del journal:** mueve lectura/escritura/formato/limpieza a `journal.rs`, manteniendo en `lifecycle.rs` la coordinación semántica.

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

No proponer otra prueba con la misma precondición y el mismo resultado esperado. La línea base solo se puede marcar como cerrada después de revisar también las pruebas relevantes de temporales inválidos/ausentes en `store.rs) y comprobar el CI de la revisión actual. No se ha confirmado ese cierre aquí.

## 7. Siguiente tarea concreta

**Auditar el flujo de `bitto create` antes de extraerlo a la capa de aplicación.** El código de la CLI hace estos pasos por separado:

1. crear el recurso;
2. si se solicitaron etiquetas, actualizar el recurso para guardarlas;
3. salvo `--no-instance`, crear una instancia bajo la ubicación de navegación actual.

La CLI ya informa explícitamente cuando el recurso se creó pero falla el guardado de etiquetas o la creación de la instancia. La siguiente conversación debe examinar el código, los errores y las pruebas actuales para precisar las garantías deseadas ante fallos parciales.

Secuencia de trabajo:

1. Enumerar cada efecto persistente y su orden.
2. Describir los estados alcanzables si falla cada paso.
3. Decidir qué se considera éxito parcial, qué debe permanecer y qué debe recuperarse; no asumir una transacción ni borrar automáticamente el recurso.
4. Comparar los criterios con las pruebas existentes.
5. Añadir solo las pruebas que cubran una garantía realmente ausente.
6. Cuando las garantías estén definidas y probadas, decidir si el flujo combinado merece convertirse en un caso de uso de `application.rs`.
7. Mantener el cambio pequeño y revisar el diff y CI antes de integrarlo.

Después, continuar la revisión de `store.rs` y `workspace.rs` solo para identificar límites reales de persistencia. El almacenamiento de recursos tiene una secuencia especializada de preparación, validación y promoción de temporales; no sustituirla a ciegas por la escritura atómica genérica del workspace.

## 8. Hoja de ruta y niveles de prioridad

Hay dos escalas que no deben confundirse:

- **Dirección de producto a largo plazo:** modelo general, instanciación, estructura/navegación, resolución semántica, capacidades superiores y futuras interfaces. La visión no significa que esas capacidades estén implementadas.
- **Cola de ingeniería inmediata posterior al PR #28:** cerrar con evidencia la línea base de regresión; auditar la creación combinada; definir y extraer la frontera de aplicación cuando corresponda; continuar la separación de persistencia sin cambiar garantías; después implementar el motor semántico según el contrato y revisar la API pública.

La conversación previa también manejó una hoja de ruta general de diez etapas, pero la trayectoria técnica registrada en el PR #23 es una priorización de seis frentes cercanos, no una sustitución de esa hoja de ruta. No inventar una equivalencia entre ambas ni renumerar los frentes como etapas del producto. Si se necesita restaurar el detalle literal de las diez etapas, recuperar la fuente histórica exacta antes de transcribirla.

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

- [ ] Leer este documento, `docs/status.md), `docs/model.md) y el ADR relevante.
- [ ] Consultar `main`, últimos commits, PR abiertos y estado de CI actual.
- [ ] Revisar el código y las pruebas actuales del flujo que se vaya a tocar.
- [ ] Distinguir: acordado, implementado, verificado, pendiente y aplazado.
- [ ] No reabrir las máscaras como tema de diseño sin una contradicción concreta.
- [ ] No proponer pruebas ya presentes sin demostrar una brecha.
- [ ] No afirmar que se ejecutaron comandos, pasó CI o se modificó código si no se verificó.
- [ ] Antes de refactorizar persistencia, anotar las garantías existentes que deben conservarse.

**Estado al cerrar esta actualización:** se han preparado cambios de documentación en una rama dedicada. La documentación no debe considerarse integrada en `main` hasta que se revise y se fusione el PR correspondiente.
