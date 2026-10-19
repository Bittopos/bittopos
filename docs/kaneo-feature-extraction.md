# Encuesta de extracción de características: Kaneo → Bittopos

**Estado:** propuesta de análisis, no especificación aprobada ni funcionalidad implementada.  
**Referencia examinada:** repositorio público [usekaneo/kaneo](https://github.com/usekaneo/kaneo), README y guía de arquitectura del repositorio.  
**Objetivo:** identificar capacidades útiles de Kaneo, decidir cómo se traducen al modelo de Bittopos y construir una hoja de ruta verificable sin convertir Bittopos en un clon de Kanban.

## 1. Principio de extracción

Kaneo se utiliza como referencia de producto y arquitectura. No se copiarán sus conceptos de dominio de forma literal cuando dependan de tareas y tableros.

Bittopos tiene un dominio distinto:

- **Recurso:** definición reutilizable, actualmente almacenada como TOML.
- **Instancia:** aparición identificable de un recurso.
- **Estructura:** organización jerárquica de instancias, actualmente almacenada como JSON; también representa el espacio de trabajo.
- **Máscara:** diferencias locales de una instancia respecto a su definición compartida.
- **Relación:** conexión explícita entre recursos o entre instancias.
- **Resolución semántica:** cálculo determinista de los datos efectivos.

Las funciones nuevas deben operar sobre estas entidades y respetar la semántica existente. No deben introducir una entidad paralela llamada tarea/proyecto solo para imitar Kaneo.

## 2. Qué se ha observado en Kaneo

Según su README y guía de arquitectura pública, Kaneo incluye estas familias de capacidades:

- Vistas de tablero, lista, calendario y Gantt; backlog, filtros y búsqueda.
- Descripciones, adjuntos, subtareas, dependencias, etiquetas, comentarios y seguimiento del tiempo.
- Espacios de trabajo compartidos, invitaciones, roles, actualizaciones en tiempo real y vistas públicas de proyectos.
- Columnas y campos personalizados; reglas que reaccionan a actividad de repositorios.
- Notificaciones dentro de la aplicación, correo, ntfy, Gotify y webhooks personales.
- Integraciones con GitHub, Gitea y chat; webhooks de proyecto, REST API, CLI y MCP.
- API con lógica de dominio y autorización en el servidor; cliente web tipado.
- Persistencia duradera con PostgreSQL, eventos y WebSockets; Redis opcional para distribuir eventos entre réplicas.
- Aplicaciones web y móvil, documentación de API, pruebas, despliegue self-hosted y Cloud.

Fuentes de referencia:
- [README de Kaneo](https://github.com/usekaneo/kaneo/blob/main/README.md)
- [AGENTS.md: arquitectura y convenciones del repositorio](https://github.com/usekaneo/kaneo/blob/main/AGENTS.md)
- [Documentación de instalación](https://kaneo.app/docs/core/installation)
- [Documentación de API](https://kaneo.app/docs/api-reference/introduction)
- [Integración MCP](https://kaneo.app/docs/core/integrations/mcp)

El inventario es un punto de partida basado en documentación pública; no constituye una auditoría línea por línea de todo el código de Kaneo. Las funciones deben volver a verificarse contra la versión de referencia cuando se apruebe su implementación.

## 3. Prioridades

- **P0 — Base necesaria:** sin esto, la función no es segura o no puede sostener las demás.
- **P1 — Alta:** aporta valor central a Bittopos y debería planificarse pronto.
- **P2 — Media:** útil después de estabilizar el núcleo, API y colaboración.
- **P3 — Opcional:** función avanzada, integración específica o mejora de producto.
- **Excluir:** no trasladar por ahora; no encaja con el dominio o añade complejidad sin beneficio claro.

La prioridad no equivale a aprobación. Cada característica necesita una decisión y criterios de aceptación antes de convertirse en una tarea de implementación.

## 4. Matriz de extracción

### A. Navegación, vistas y descubrimiento

| Capacidad de Kaneo | Traducción para Bittopos | Prioridad inicial | Decisión / pregunta |
|---|---|---:|---|
| Vista de tablero Kanban | No replicar columnas de tareas. Si el modelo lo necesita, permitir vistas derivadas configurables sobre instancias o estados explícitos. | Excluir por defecto | ¿Existe un estado de dominio real que justifique una vista de tablero? |
| Vista de lista | Lista de recursos e instancias, con columnas configurables y acciones contextuales. | P1 | ¿Qué campos deben verse sin abrir cada recurso? |
| Vista de calendario | Vista temporal solo para datos que tengan fechas explícitas. | P2 | ¿Qué concepto temporal tiene sentido en el modelo actual? |
| Vista Gantt | Grafo temporal derivado de relaciones y fechas, solo cuando el dominio soporte dependencias temporales. | P3 | ¿Hay dependencias temporales reales o serían metadatos añadidos artificialmente? |
| Backlog | No crear un backlog de tareas. Considerar una bandeja de captura para recursos/ideas sin clasificar. | P2 | ¿Se necesita una bandeja de entrada independiente de la estructura? |
| Filtros | Filtrar por tipo, identidad, etiquetas/metadatos, ubicación, relación y propiedades resueltas. | P1 | ¿Qué campos se pueden filtrar sin romper la semántica? |
| Búsqueda | Búsqueda de recursos, instancias, nombres, contenido y relaciones; distinguir búsqueda textual de consulta semántica. | P1 | ¿Cuál es el alcance inicial y cómo se actualiza el índice? |
| Vistas guardadas | Guardar consultas, filtros, orden y presentación sin duplicar datos del dominio. | P2 | ¿Deben ser personales o compartidas por organización? |
| Vistas públicas | Compartir una proyección explícita de solo lectura con acceso revocable. | P3 | ¿Qué información puede exponerse y cómo se limita? |

**Principio:** una vista es una proyección del modelo, no una nueva fuente de verdad. Cambiar la vista no debe cambiar las reglas de resolución.

### B. Contenido y operaciones del modelo

| Capacidad de Kaneo | Traducción para Bittopos | Prioridad inicial | Decisión / pregunta |
|---|---|---:|---|
| Descripción de tarea | Contenido y metadatos del recurso, manteniendo TOML y reglas de segmentos. | P1 | ¿Qué esquema de metadatos es estable y qué queda definido por el usuario? |
| Subtareas | Jerarquía de instancias dentro de la estructura; no crear una segunda jerarquía paralela. | P0 | ¿Qué operaciones de árbol faltan para crear, mover, copiar o eliminar instancias con seguridad? |
| Dependencias | Usar relaciones tipadas entre recursos o instancias; no confundir relación con contención. | P1 | ¿Qué tipos de relación se validan y cuáles son definidos por el usuario? |
| Etiquetas | Metadatos o segmentos definidos explícitamente, con reglas de consulta consistentes. | P2 | ¿Las etiquetas son primitivas del núcleo o una convención del modelo? |
| Comentarios | Anotaciones asociadas a un recurso/instancia, separadas del contenido semántico principal. | P2 | ¿Deben ser append-only, editables, versionadas y exportables? |
| Adjuntos | Archivos binarios referenciados desde recursos/instancias, con almacenamiento configurable. | P2 | ¿Cómo se preservan las referencias al exportar o mover un espacio? |
| Campos personalizados | Esquema extensible de segmentos/conceptos/componentes, no una tabla rígida de campos de tareas. | P1 | ¿Qué validaciones corresponden al núcleo y cuáles a esquemas del usuario? |
| Seguimiento del tiempo | Registro opcional de eventos de tiempo vinculados a una entidad, sin alterar la definición semántica. | P3 | ¿Es una necesidad real del producto o una función heredada de gestión de proyectos? |
| Historial de cambios | Registro de revisiones de operaciones y cambios del modelo, distinto de las máscaras y snapshots semánticos. | P0 | ¿Qué nivel de historial es necesario para deshacer, auditar y sincronizar? |
| Deshacer/rehacer | Operaciones reversibles o compensatorias con límites claros; no revertir silenciosamente datos compartidos. | P1 | ¿Qué operaciones pueden revertirse y cómo se detectan cambios concurrentes? |

**Distinción esencial:** las máscaras de Bittopos expresan diferencias locales respecto a una definición compartida. No deben confundirse con historial de cambios, comentarios, eventos ni snapshots de revisión.

### C. Colaboración, cuentas y permisos

| Capacidad de Kaneo | Traducción para Bittopos | Prioridad inicial | Decisión / pregunta |
|---|---|---:|---|
| Espacios de trabajo compartidos | Colaboración sobre un espacio Bittopos persistido por un servidor; mantener la estructura como concepto de dominio actual. | P0 | ¿Qué parte del espacio es la unidad de permisos y concurrencia? |
| Organizaciones | Agrupar usuarios, membresías, políticas y varios espacios de trabajo. | P0 para Cloud | ¿Una organización puede poseer varios espacios y tener distintos planes? |
| Invitaciones | Invitar usuarios con caducidad, alcance y rol explícitos. | P1 para Cloud | ¿Qué roles mínimos necesita el MVP? |
| Roles personalizados | Permisos definidos en servidor y asociados a operaciones/capacidades, no a controles de UI. | P0 | ¿Cuáles son los permisos atómicos de lectura, escritura, administración y compartición? |
| Acceso por recurso/espacio | Validar el alcance de cada solicitud y evitar accesos cruzados entre tenants. | P0 | ¿Se requiere autorización a nivel de organización, espacio o recurso? |
| Actualizaciones en tiempo real | Eventos de cambios autorizados y actualización de clientes conectados. | P2 | ¿Qué cambios requieren tiempo real y cuáles pueden usar actualización periódica? |
| Sesiones multidispositivo | Identidad común y acceso seguro al mismo espacio desde web, escritorio, móvil y CLI. | P1 | ¿Qué flujo de autenticación conviene para cada tipo de cliente? |
| Vistas públicas | Enlaces revocables y de solo lectura con alcance explícito. | P3 | ¿Qué amenazas y limitaciones deben cubrir los enlaces públicos? |
| Auditoría administrativa | Registrar cambios de membresías, permisos, facturación y acciones privilegiadas. | P0 para Cloud | ¿Qué eventos se conservan y quién puede consultarlos? |

**Regla de seguridad:** ocultar botones en Workspace no autoriza una operación. El servidor debe autenticar y autorizar cada lectura y escritura protegida. El identificador de tenant enviado por el cliente nunca concede acceso por sí mismo.

### D. Automatización, notificaciones e integraciones

| Capacidad de Kaneo | Traducción para Bittopos | Prioridad inicial | Decisión / pregunta |
|---|---|---:|---|
| Notificaciones internas | Notificar cambios relevantes en recursos, instancias, relaciones, invitaciones y permisos. | P2 | ¿Qué eventos merecen notificación y cuáles generarían ruido? |
| Correo | Invitaciones, recuperación de cuenta y alertas configuradas. | P1 para Cloud | ¿Qué flujos de correo son obligatorios en el MVP? |
| ntfy / Gotify | Adaptadores opcionales de notificación para instalaciones autogestionadas. | P3 | ¿Se deben soportar proveedores intercambiables mediante una interfaz común? |
| Webhooks personales | Suscripciones a eventos autorizados con firma, reintentos y protección contra abuso. | P2 | ¿Qué eventos pueden exponerse de forma segura? |
| Webhooks de proyecto | Eventos de un espacio de trabajo, filtrados por alcance y permisos. | P2 | ¿Cómo se administran secretos, reintentos e idempotencia? |
| Automatizaciones | Reglas explícitas que reaccionan a eventos y ejecutan acciones limitadas. | P2 | ¿Qué lenguaje/modelo de reglas es seguro y fácil de depurar? |
| GitHub / Gitea | Integraciones que relacionen cambios externos con recursos Bittopos sin convertirlos en el dominio principal. | P2 | ¿Qué objetos se vinculan y qué sistema es autoridad en cada campo? |
| Integraciones de chat | Enlaces, notificaciones y comandos con permisos mínimos. | P3 | ¿Qué casos de uso justifican mantener cada integración? |
| API REST | API versionada para Workspace, CLI, automatizaciones y clientes de terceros. | P0 | ¿Qué operaciones del núcleo forman el contrato público estable? |
| CLI remota | Usar la API autenticada cuando el destino sea remoto; conservar el núcleo local para modo offline/local. | P1 | ¿Cómo se elige entre modo local y remoto sin ambigüedad? |
| MCP / agentes | Adaptador opcional que llama a casos de uso existentes y hereda permisos del usuario. | P3 | ¿Qué operaciones pueden exponerse a agentes y cuáles requieren confirmación? |

No se deben crear integraciones antes de definir un contrato de eventos estable, autenticación, autorización, idempotencia y política de reintentos.

### E. Arquitectura, persistencia y operación

| Capacidad/patrón de Kaneo | Adaptación a Bittopos | Prioridad inicial | Decisión / pregunta |
|---|---|---:|---|
| API como autoridad de dominio y permisos | Servidor Rust invoca el núcleo canónico; evitar que el backend Python replique la semántica. | P0 | ¿Qué operaciones actuales del núcleo deben exponerse primero? |
| Cliente HTTP tipado | Generar o validar cliente contra un contrato OpenAPI versionado. | P1 | ¿Contrato OpenAPI, JSON Schema o ambos según el uso? |
| PostgreSQL | Persistencia remota para concurrencia, usuarios y aislamiento multi-tenant; conservar adaptadores locales. | P0 para servidor compartido | ¿Cómo se representa el modelo de archivos sin perder identidad ni semántica? |
| Eventos y WebSockets | Canal de eventos posterior a cambios confirmados, con filtrado por permisos. | P2 | ¿Se necesita tiempo real en el MVP o bastan revisiones y polling? |
| Redis opcional | Solo cuando varias réplicas necesiten distribución de eventos; no introducirlo como requisito inicial. | P3 | ¿Qué evidencia de carga justificaría añadirlo? |
| Self-hosting sencillo | Contenedor y Compose con persistencia, configuración clara, health checks y copias de seguridad documentadas. | P0 | ¿Puede una persona instalarlo sin cuentas comerciales? |
| Cloud gestionado | Plano de control privado que aprovisiona espacios y gestiona suscripciones; separado del servidor genérico. | P1 tras el API seguro | ¿Qué unidad se aprovisiona y qué límites comerciales se aplican? |
| Aplicación móvil | Cliente de la API con flujos de identidad adecuados; no asumir que el frontend web se empaqueta sin cambios. | P2 | ¿Qué capacidades mínimas debe ofrecer en Android/iOS? |
| CI y pruebas de integración | Pruebas de núcleo, API, aislamiento de tenant, migraciones y contenedor. | P0 | ¿Qué pruebas bloquean toda publicación? |
| Migraciones | Versionar cambios de esquema y probarlos con instalaciones existentes. | P0 para persistencia remota | ¿Cómo se conserva compatibilidad entre versiones de API y servidor? |
| Observabilidad | Logs estructurados, métricas y trazas sin filtrar datos privados de tenants. | P1 | ¿Qué métricas ayudan a diagnosticar fallos sin exponer contenido? |
| Copias de seguridad y restauración | Backups verificables y ejercicios de restauración; política distinta por instalación propia y Cloud. | P0 para Cloud | ¿Cuál es el objetivo de recuperación y retención por plan? |
| Internacionalización y accesibilidad | Catálogo de traducciones y criterios de accesibilidad para Workspace. | P1 | ¿Qué idiomas y criterios mínimos exige el primer lanzamiento? |

## 5. Decisiones de arquitectura que debemos preservar

1. **El núcleo Rust es la autoridad semántica.** El servidor valida la identidad, los permisos y la entrada; el núcleo decide las reglas del modelo.
2. **El cliente no es una frontera de confianza.** Workspace puede validar para dar respuesta rápida, pero la API valida de nuevo.
3. **La estructura no se duplica.** La jerarquía de instancias, las relaciones y las máscaras mantienen sus significados actuales.
4. **Local no significa Cloud obligatorio.** La CLI y la instalación propia deben funcionar sin una cuenta de Bittopos Cloud.
5. **La API es el límite remoto compartido.** Web, CLI, móvil y futuras integraciones usan el mismo contrato.
6. **Persistencia y semántica son responsabilidades diferentes.** Cambiar de archivos locales a almacenamiento remoto no debe cambiar el significado de una operación.
7. **Monolito modular primero.** No introducir microservicios, Redis ni un sistema de eventos distribuido sin necesidad demostrada.
8. **Compatibilidad y exportación.** El usuario debe poder respaldar y recuperar sus datos, y no quedar atrapado en el servicio alojado.

## 6. Encuesta de decisiones de producto

Completar antes de convertir el inventario en épicas de desarrollo. Se permiten respuestas parciales; las preguntas sin decisión quedan abiertas.

### Experiencia principal

- [ ] **A.** Navegación principal basada en árbol de instancias y exploración de recursos.
- [ ] **B.** Árbol más una vista de lista configurable.
- [ ] **C.** Árbol, lista y vistas guardadas como capacidades del primer MVP.
- [ ] Bandeja de captura rápida para recursos todavía no organizados.
- [ ] Búsqueda de texto como primera versión; indexación avanzada después.
- [ ] Comentarios y adjuntos separados del contenido semántico.
- [ ] Historial de cambios y deshacer desde la primera versión colaborativa.

### Colaboración y multi-tenancy

- [ ] Una organización puede tener varios espacios de trabajo.
- [ ] Roles iniciales: propietario, administrador, editor y lector.
- [ ] Permisos inicialmente por espacio de trabajo; permisos por recurso solo si se justifican.
- [ ] Compartir mediante invitaciones autenticadas.
- [ ] Enlaces públicos de solo lectura pospuestos.
- [ ] Aislamiento multi-tenant obligatorio antes de abrir el servicio Cloud a usuarios.

### Integraciones y automatización

- [ ] API REST estable antes de integraciones externas.
- [ ] CLI local primero; cliente remoto después de estabilizar autenticación.
- [ ] Eventos y webhooks después de fijar el contrato de cambios.
- [ ] GitHub/Gitea como primera integración, si aporta al flujo real de Bittopos.
- [ ] MCP/agentes como adaptador opcional posterior.

### Distribución y sostenibilidad

- [ ] Servidor autogestionable y sin dependencia de Bittopos Cloud.
- [ ] Servicio Cloud con cuentas, organizaciones y aprovisionamiento separado.
- [ ] Copias de seguridad y exportación como requisitos, no como extras comerciales.
- [ ] Planes de pago basados en operación gestionada, almacenamiento, colaboración o soporte, no en bloquear el núcleo.
- [ ] La configuración privada de producción queda fuera del repositorio público; los secretos no se guardan en Git.

## 7. Propuesta de fases

### Fase 0 — Modelo y contratos

- Definir las entidades de usuario, organización, espacio y membresía.
- Documentar operaciones del núcleo y límites de la API.
- Definir formato de errores, revisiones, concurrencia e idempotencia.
- Crear pruebas de contrato y de aislamiento antes de implementar funciones de cuenta.

### Fase 1 — Servidor compartido mínimo

- Servidor Rust que invoque el núcleo.
- Persistencia remota detrás de una interfaz explícita.
- API para leer/editar recursos, estructura, relaciones y resolución semántica.
- Autenticación y autorización para cualquier despliegue multiusuario.
- Contenedor portable, configuración por entorno, health checks y backups.

### Fase 2 — Workspace conectado

- Sustituir la fuente de datos experimental por el cliente de la API estable.
- Navegación de árbol y lista, búsqueda y edición.
- Manejo de errores, carga, revisiones y conflictos.
- Validación de entrada cliente sin duplicar reglas semánticas.

### Fase 3 — Colaboración y Cloud

- Organizaciones, membresías, roles e invitaciones.
- Aislamiento multi-tenant probado con solicitudes negativas.
- Portal privado de cuentas y plano de control.
- Aprovisionamiento, cuotas y facturación cuando el servicio básico sea seguro y operable.

### Fase 4 — Funciones avanzadas

- Historial, comentarios, adjuntos y notificaciones según prioridades acordadas.
- Eventos en tiempo real solo donde mejoren la experiencia.
- Integraciones, webhooks, CLI remota y MCP.
- Aplicaciones de escritorio y móvil sobre contratos estables.

## 8. Criterios de aceptación de la extracción

Una característica derivada de Kaneo solo se considera lista para planificarse si:

- [ ] El problema de usuario que resuelve está descrito.
- [ ] Se identificó si afecta al núcleo, API, Workspace, CLI o Cloud.
- [ ] Su comportamiento no contradice las reglas de recursos, instancias, estructura, máscaras o relaciones.
- [ ] Se definieron permisos y límites de tenant cuando se trate de datos compartidos.
- [ ] Se especificaron pruebas unitarias, de integración o de contrato.
- [ ] Se describió su efecto en exportación, migración y compatibilidad.
- [ ] Se determinó si es necesaria para el MVP o si puede aplazarse.
- [ ] Se verificó la función en la versión de Kaneo que se esté usando como referencia.

## 9. Licencia y alcance de reutilización

El repositorio público de Kaneo declara licencia MIT. Este documento extrae y adapta características y patrones a nivel de producto; no incorpora código, recursos visuales ni textos de la aplicación. Si más adelante se decide reutilizar código de Kaneo, habrá que revisar la licencia aplicable a los archivos concretos y conservar los avisos de copyright y licencia exigidos.

## 10. Recomendación inicial para Bittopos

No perseguir paridad funcional total desde el comienzo. El orden con mayor valor arquitectónico es:

1. núcleo semántico único y verificable;
2. servidor portable con API y autorización;
3. Workspace para navegar y editar recursos e instancias;
4. persistencia compartida, revisiones y concurrencia;
5. organizaciones y colaboración multi-tenant;
6. Cloud gestionado;
7. funciones complementarias e integraciones.

La ventaja competitiva de Bittopos no será tener todos los botones de Kaneo. Será combinar una base semántica y una organización de datos coherentes con una experiencia instalable, colaborativa y gestionada cuando el usuario lo necesite.
