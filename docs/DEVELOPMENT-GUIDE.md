# Bittopos — Guía de desarrollo e ingeniería

**Estado:** guía interna vigente  
**Ámbito:** diseño, implementación, pruebas, documentación y evolución del repositorio  
**Autoridad:** complementa la visión y los contratos; no los sustituye.

## 1. Propósito y fuentes de verdad

El objetivo es mantener la coherencia entre el modelo, los contratos, la arquitectura, el código y el comportamiento observable, sin introducir complejidad organizativa desproporcionada.

Cada fuente tiene una función distinta:

- **Visión:** propósito, principios y dirección estratégica.
- **Contrato vigente y ADR aceptados:** conceptos, estructuras, invariantes y semántica acordada.
- **Pruebas:** verifican comportamientos concretos que la implementación afirma soportar.
- **Código:** determina lo que está implementado hoy; no convierte por sí solo una desviación en una regla correcta.
- **README y guías:** explican el estado, el uso y las rutas de entrada.
- **Estado de trabajo:** indica el punto actual de continuación y las tareas inmediatas.
- **Histórico:** conserva decisiones y requisitos de etapas anteriores; no es fuente normativa vigente.

Una prueba que contradice el contrato señala un problema que debe resolverse; no modifica automáticamente el contrato. Una visión tampoco demuestra que una capacidad esté implementada. Ante una contradicción, documenta el impacto y resuélvela explícitamente antes de ampliar la funcionalidad.

## 2. Desarrollo orientado por contratos

Antes de implementar cambios que afecten al modelo o a la persistencia, determina:
1. qué conceptos y operaciones están involucrados;
2. qué invariantes deben mantenerse;
3. qué datos se crean, cambian, eliminan o resuelven;
4. qué ocurre ante errores, interrupciones y estados inconsistentes;
5. qué pruebas demostrarán el resultado esperado.

Si falta una regla necesaria, acuerda una decisión explícita; no conviertas una interpretación improvisada del código en una norma conceptual. Actualiza el contrato y las pruebas en el mismo incremento lógico que la implementación.

## 3. Límites de responsabilidad

### Dominio
Contiene conceptos, invariantes, reglas semánticas y errores de dominio. No depende de presentación, rutas físicas ni formatos de persistencia.

### Aplicación
Coordina casos de uso que combinan componentes del sistema. Expone operaciones reutilizables para distintas interfaces sin duplicar reglas.

### Adaptadores
Gestionan serialización, lectura, escritura, filesystem, journal y recuperación. Preservan las garantías requeridas, pero no son la autoridad semántica principal.

### Interfaces
Interpretan entradas, invocan operaciones y presentan resultados. No mantienen implementaciones paralelas de las invariantes.

Estos límites describen responsabilidades, no una obligación de crear una carpeta, módulo o crate por cada elemento. La estructura física debe responder a cohesión, acoplamiento y complejidad reales.

Antes de añadir una abstracción, justifica el problema que resuelve, las invariantes que representa y por qué no bastan los conceptos existentes. No añadas capas genéricas solo para anticipar posibilidades hipotéticas.

## 4. Persistencia, consistencia y recuperación

Para cualquier operación que modifique varias representaciones persistidas, especifica, según corresponda:
- orden de escrituras y punto de confirmación;
- estados intermedios posibles;
- recuperación después de una interrupción;
- detección de inconsistencias;
- idempotencia de la recuperación.

Una escritura atómica de un archivo no hace atómica una operación entre varios archivos. No elimines ni simplifiques journal, temporales, reconciliación o recuperación durante un refactor sin demostrar que se conservan sus garantías o que se ha aprobado explícitamente el cambio.

Los datos persistidos deben validarse al cargarse. Ante una discrepancia que no pueda resolverse con seguridad, falla explícitamente; no inventes relaciones ni descartes datos para hacer que el estado parezca válido.

## 5. Cambios incrementales y Git

Flujo recomendado:
1. identifica el problema y el comportamiento esperado;
2. revisa contrato, implementación y pruebas pertinentes;
3. delimita el cambio mínimo correcto;
4. registra decisiones importantes;
5. implementa y prueba;
6. actualiza documentación afectada;
7. ejecuta verificaciones y revisa el diff;
8. integra mediante la revisión establecida.

Usa una rama por cambio lógico y commits claros, preferiblemente Conventional Commits. Las Issues y PRs deben ser proporcionales al tamaño y riesgo; no son burocracia obligatoria para cada línea. Evita mezclar refactorizaciones sin relación con nuevas funciones.

La guía de ramas, commits y Pull Requests para colaboradores está en [CONTRIBUTING.md](../CONTRIBUTING.md).

## 6. Estrategia de pruebas

Prioriza pruebas acordes al riesgo:
- **Dominio:** invariantes, validaciones y resolución.
- **Persistencia:** serialización, lectura, escritura y errores de E/S.
- **Aplicación:** coordinación y errores de casos de uso.
- **Integración:** interacción entre modelo y adaptadores.
- **Recuperación:** interrupciones, temporales abandonados y reconciliación.
- **Regresión:** preservación de comportamientos ya establecidos.

Las pruebas deben ser reproducibles y evitar servicios externos cuando la lógica pueda verificarse localmente. No todas las modificaciones necesitan todas las categorías; la cobertura debe corresponder al riesgo.

## 7. Verificación y aceptación

Compilar no basta para considerar terminado un cambio. Como base, ejecuta desde la raíz:

    cargo fmt --all -- --check
    cargo build --workspace --locked
    cargo test --workspace --locked
    git diff --check

Comprueba además que no se debilitaron invariantes, que los errores mantienen contexto útil, que las garantías de persistencia sobreviven y que la documentación refleja el resultado real. Si una comprobación no se ejecutó, indícalo; nunca declares que pasó por inferencia.

## 8. Versiones y documentación

Cargo declara versiones de paquetes; Git registra commits y referencias, pero no elige automáticamente la versión del producto. Usa SemVer al preparar una publicación, no por cada PR. El changelog registra cambios relevantes, no cada commit.

Mantén las responsabilidades documentales separadas:
- README.md: puerta de entrada y estado resumido.
- docs/VISION.md: dirección estratégica.
- docs/model.md: semántica conceptual vigente.
- docs/architecture.md: arquitectura actual y dirección técnica.
- docs/status.md: capacidades implementadas, pendientes y aplazadas.
- docs/WORKING-STATE.md: punto de continuación y siguiente trabajo concreto.
- docs/decisions/: decisiones arquitectónicas.
- docs/archive/: documentación histórica.
- CONTRIBUTING.md: flujo práctico de contribución.
- Esta guía: normas de ingeniería.

Evita mantener la misma especificación completa en varios documentos. En caso de duda, enlaza la fuente de autoridad en lugar de copiarla.

## 9. Uso de herramientas de IA y continuidad

Toda herramienta de IA que contribuya al repositorio debe consultar las fuentes vigentes, distinguir requisitos de inferencias, no inventar APIs ni resultados de pruebas, respetar el alcance autorizado y declarar verificaciones y limitaciones. Debe señalar contradicciones entre contrato y código en lugar de resolverlas silenciosamente.

Antes de retomar trabajo en una conversación nueva:
1. leer docs/WORKING-STATE.md y docs/status.md;
2. comprobar la rama main, los PR abiertos y los últimos commits;
3. consultar el contrato o ADR relevante y verificar el código/pruebas actuales;
4. distinguir tareas pendientes de capacidades ya implementadas y decisiones ya acordadas;
5. no volver a proponer pruebas existentes ni reabrir decisiones cerradas sin una contradicción concreta demostrada.

La continuidad del proyecto debe apoyarse en documentación persistente, decisiones y estado verificable del repositorio, no solo en el historial conversacional.

## 10. Evolución de esta guía

Mantén la guía proporcional a la madurez del proyecto. Cambia una regla cuando la experiencia revele un riesgo, una ambigüedad o un procedimiento innecesario, y explica el motivo. Esta guía no autoriza a contradecir contratos ni sustituye ADRs específicos.
