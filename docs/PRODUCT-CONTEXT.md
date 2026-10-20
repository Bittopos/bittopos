# Contexto de producto y desarrollo de Bittopos

Este documento conecta las decisiones de producto con el modelo semántico y la arquitectura. Sirve para que una persona o un agente de desarrollo trabaje sobre una comprensión común del proyecto. No sustituye el contrato del modelo ni convierte objetivos futuros en funcionalidades ya disponibles.

## 1. Qué es Bittopos

Bittopos es un sistema general de **estructuración semántica, navegación y control de ejecución**. Ayuda a describir, organizar, relacionar e inspeccionar sistemas completos sin imponer a todos los dominios una plantilla rígida.

La idea unificadora es que muchos ámbitos distintos pueden representarse con una combinación de:

1. una **estructura jerárquica** que responde «¿dónde está contenido esto?»;
2. **definiciones reutilizables** que expresan qué es cada tipo de elemento;
3. **instancias identificables** que sitúan esas definiciones en un sistema concreto;
4. **diferencias locales y resolución semántica** que determinan los datos efectivos sin duplicar el contenido compartido;
5. **relaciones tipadas** que responden «¿qué vínculo existe entre estos elementos?»;
6. interfaces y operaciones que permiten explorar, modificar y verificar el sistema.

Por eso Bittopos puede servir tanto para estructurar un proyecto de software o un plan de estudios como para modelar especies, interacciones ecológicas o la organización de un paseo de olla. Los dominios cambian; los mecanismos de estructura, identidad, herencia, resolución y relación se mantienen. Bittopos debe adaptarse al sistema descrito, no obligar al sistema a imitar un tablero o una taxonomía predefinida.

## 2. La conexión fundamental: árbol más red semántica

La jerarquía y las relaciones complementan funciones diferentes; ninguna sustituye a la otra.

- **La estructura** expresa pertenencia y organización. Es el árbol de instancias y el espacio de trabajo único. Su comportamiento debe ser tan predecible y navegable como el de un sistema de archivos.
- **Las relaciones** expresan vínculos con significado entre recursos o entre instancias, respetando el tipo de identidad de ambos extremos. Ejemplos de etiquetas posibles son «se alimenta de», «acicala a», «utilizado por», «depende de» o «implementa».
- Una relación no mueve un elemento, no lo convierte en hijo de otro y no altera la contención. Un nodo puede estar ubicado en una rama y mantener relaciones con elementos de otras ramas.
- La interfaz puede ofrecer una vista jerárquica para la organización, una vista de relaciones para la red semántica y un inspector para consultar el elemento seleccionado. Estas vistas deben mostrar el mismo modelo subyacente, no mantener modelos paralelos.

Esta distinción permite que un sistema sea navegable sin perder vínculos transversales. Las relaciones tienen identidad propia y se consultan directamente; no se instancian como elementos ordinarios.

## 3. Vocabulario canónico

El documento [model.md](model.md) define las reglas normativas. Resumen práctico:

| Concepto | Función |
|---|---|
| Recurso | Definición reutilizable persistida como TOML. |
| Instancia | Aparición identificable de un recurso dentro de la estructura. No es una copia completa de la definición. |
| Estructura | Organización jerárquica de instancias persistida como JSON; es también el espacio de trabajo. No crear otro objeto paralelo llamado workspace. |
| Segmento | Parte identificable de una definición o de una instancia. |
| Máscara | Diferencia local de una instancia respecto de su recurso compartido. |
| Resolución semántica | Obtiene los datos efectivos aplicando las reglas explícitas de herencia y diferencias locales. |
| Relación | Recurso especial con identidad propia que conecta dos extremos compatibles sin cambiar la contención. |
| Referencia | Vínculo explícito de una instancia a otra instancia mediante el componente `point`; no equivale a contención ni a una relación general. |

Reglas esenciales:

- Los identificadores de recurso e instancia pertenecen a espacios distintos; no se intercambian.
- La definición compartida vive en el recurso. La estructura conserva las instancias y su organización.
- Las máscaras guardan solo diferencias locales. Lo no modificado sigue la definición compartida vigente.
- La resolución debe ser determinista y pertenecer al núcleo, no recrearse en componentes de UI o adaptadores de persistencia.
- Los vínculos y componentes opcionales se crean de forma explícita; no se inventan por normalización.
- Los campos desconocidos o semánticos deben preservarse en importaciones, exportaciones y ediciones cuando el contrato lo permita.
- No se debe inventar una semántica de borrado u ocultación local que el modelo no defina.

## 4. Principios de producto e interfaz

### 4.1 La estructura es una herramienta, no una plantilla

Bittopos no es un gestor Kanban al que se le cambia la etiqueta, ni debe reducir todo sistema a tareas. El modelo base es una estructura semántica configurable y extensible. Los patrones de interfaz de otros productos solo son útiles si se adaptan al modelo propio y mejoran una tarea real del usuario.

### 4.2 La UI hace visible el modelo

La interfaz debe ayudar a entender dónde está cada instancia, de qué recurso procede, qué datos hereda, qué diferencias locales tiene y con qué elementos se relaciona. La representación visual no debe ocultar las distinciones que el núcleo conserva.

El árbol de estructura, el mapa de relaciones, la búsqueda y el inspector son formas complementarias de explorar un mismo sistema. No son almacenes independientes.

### 4.3 Definición, ubicación y estado son dimensiones distintas

La identidad del recurso, la ubicación de la instancia y el estado operativo no deben mezclarse. En particular, un panel de estado puede distinguir elementos creados, modificados y finalizados, pero eso no convierte el modelo en Git ni sustituye el historial de auditoría.

### 4.4 Estado visible e historial auditable cumplen fines distintos

El panel de estado está orientado al trabajo cotidiano: debe dejar claro qué ha cambiado y qué requiere atención. El registro de auditoría es un mecanismo distinto, principalmente para trazabilidad administrativa/legal y diagnóstico de fallos. No se debe usar una lista de actividad visual como sustituto de un registro fiable, ni exponer todos los detalles de auditoría como ruido en la interfaz diaria.

### 4.5 Notificaciones bajo control del usuario

Las preferencias de notificación pertenecen a la configuración del usuario. La interfaz normal y la API son vías equivalentes de interacción con las mismas capacidades autorizadas; no deben existir reglas de negocio distintas según el canal utilizado.

### 4.6 Aplicación y cuentas se separan

La interfaz de trabajo con estructuras y la interfaz de cuentas son áreas distintas. La primera se centra en los sistemas que el usuario organiza; la segunda, en la identidad y el acceso a la cuenta. La separación visual debe acompañar una separación real de responsabilidades y permisos en la arquitectura.

## 5. Arquitectura objetivo acordada

Para la siguiente etapa se ha elegido:

- **Integración frontend–núcleo mediante API.** La SPA no debe leer archivos internos ni reimplementar reglas semánticas.
- **Alcance de refactorización del frontend: completo.** Se debe ordenar toda la SPA por responsabilidades sin intentar transformar toda la aplicación en un único cambio opaco. Trabajar en incrementos revisables, con pruebas y preservación del comportamiento.
- **Persistencia inicial remota.** No tratar el almacenamiento en el navegador ni una base local de desarrollo como la fuente compartida final.
- **Núcleo semántico Rust como autoridad.** La API debe invocar la lógica del núcleo mediante una frontera explícita. Un adaptador que solo valida y guarda documentos no equivale a integrar el núcleo.
- **CLI y UI como interfaces del mismo sistema.** A medida que madure el contrato remoto, las operaciones comunes deben conservar reglas y resultados coherentes. La sincronización remota de la CLI es una necesidad posterior ya identificada, no una funcionalidad que deba darse por existente.

Dirección conceptual:

```text
Interfaz web ───────┐
                    ├── API versionada ── capa de aplicación ── núcleo semántico Rust
CLI local/remota ───┘                                      │
                                                           ▼
                                               persistencia remota
```

El diagrama expresa la dirección deseada, no el estado completo que ya esté desplegado. La forma concreta de ejecutar el núcleo Rust en el entorno de backend, el proveedor de persistencia remoto y la estrategia final de migración deben quedar definidos y probados; no deben resolverse con duplicación silenciosa de lógica en JavaScript o Python.

### Situación actual frente a destino

El repositorio contiene una SPA modular conectada a un API HTTP v1 experimental y un adaptador Python/SQLite. Ese adaptador conserva documentos, pero todavía no ejecuta la resolución semántica Rust, no ofrece un sistema completo de cuentas y no debe exponerse directamente a Internet. Por tanto:

- la existencia de endpoints no significa que el núcleo esté integrado;
- la persistencia SQLite de desarrollo no equivale a persistencia remota multiusuario;
- alojar los assets estáticos en Cloudflare Pages no despliega el backend;
- autenticación, autorización, aislamiento de datos, concurrencia, migraciones, copias de seguridad y auditoría deben resolverse antes de abrir un servicio remoto a usuarios reales.

Cloudflare Pages es el destino de los assets estáticos ya explorado. El runtime remoto y la tecnología concreta de almacenamiento deben elegirse por compatibilidad verificable con la API, el núcleo, las necesidades de seguridad y el modelo de datos; no se debe inferir un proveedor solo por el alojamiento de la SPA.

## 6. Contratos y seguridad de datos

Toda modificación de interfaz, API o persistencia debe proteger la continuidad del modelo:

1. Preservar identidades, segmentos, máscaras, referencias, relaciones y campos desconocidos admitidos.
2. Validar antes de sustituir el estado activo y comunicar errores accionables.
3. Detectar escrituras obsoletas y conflictos; no tratar una revisión antigua como válida silenciosamente.
4. Compartir fixtures de contrato para estructura y documentos de recursos entre UI, API y núcleo.
5. Separar autenticación (quién es la persona), autorización (qué puede hacer) y ámbito de datos (a qué cuenta/espacio accede).
6. No registrar secretos ni convertir datos de auditoría en información visible para usuarios que no tienen permiso.
7. No declarar como implementada una capacidad solo porque esté descrita como objetivo.

La API debe ofrecer operaciones explícitas y versionadas. Los controles de la UI pueden facilitar esas operaciones, pero no son una frontera de seguridad: la validación y autorización deben hacerse también en el servidor.

## 7. Prioridad de desarrollo

La refactorización completa debe avanzar por cortes revisables:

1. Inventariar el comportamiento actual y los contratos reales, no asumirlos a partir de nombres o prototipos visuales.
2. Acordar contratos y fixtures que representen el modelo Rust, incluyendo casos con máscaras, relaciones, referencias y campos desconocidos.
3. Separar UI, casos de uso, transporte API y adaptadores de persistencia con límites visibles.
4. Integrar el núcleo real detrás de la API y demostrarlo con pruebas de contrato; evitar una segunda implementación semántica.
5. Completar persistencia remota segura y el aislamiento por cuenta/espacio antes del uso multiusuario.
6. Construir panel de estado, auditoría y notificaciones sobre eventos y permisos definidos, sin acoplarlos al árbol visual.
7. Incorporar sincronización de la CLI cuando el contrato remoto y la resolución de conflictos estén definidos.

La prioridad no autoriza añadir capas por anticipación. Cada abstracción debe tener un consumidor real y cada fase debe dejar el repositorio verificable.

## 8. Cómo debe trabajar un agente de desarrollo

Antes de cambiar código:

- Leer este documento, [model.md](model.md), [architecture.md](architecture.md), [api-backed-frontend.md](api-backed-frontend.md), las guías de los directorios tocados y el código real.
- Distinguir siempre el comportamiento implementado de las decisiones y objetivos pendientes.
- Localizar la autoridad del comportamiento antes de añadir una regla. No solventar una limitación del núcleo con lógica dispersa en la SPA.
- Realizar el cambio en una rama revisable y no fusionar en `main` sin autorización expresa.
- Ejecutar las verificaciones pertinentes y registrar qué se ejecutó y qué no. Nunca inventar resultados de pruebas.
- Actualizar el documento canónico cuando una decisión aprobada cambie el modelo o la arquitectura.

Si la documentación y el código discrepan, documentar primero la discrepancia con evidencia. El modelo define la semántica; el código demuestra lo que hoy existe; este documento conecta el propósito con la dirección aprobada. No resolver una contradicción suponiendo silenciosamente que la funcionalidad más ambiciosa ya está disponible.


## 2.1 Una estructura, múltiples proyecciones: directorio, diagrama, mapa y árbol radial

La imagen de especies en blanco y negro, con una vista ampliada circular, ilustra un principio central de Bittopos: **es un árbol jerárquico como el de un sistema de archivos Linux, dispuesto visualmente en forma circular**. La forma circular no cambia la estructura lógica; cambia el modo en que se proyecta y se recorre visualmente.

De ahí se desprende una regla de diseño de producto: un directorio, un diagrama y un mapa pueden ser representaciones diferentes de la misma estructura subyacente. No son necesariamente tres modelos de datos ni tres documentos que haya que mantener sincronizados. Pueden ser distintas vistas, proyecciones o modos de navegación sobre los mismos nodos, identidades, segmentos y vínculos.

- **Vista de directorio o árbol:** privilegia la contención, los niveles y la navegación ordenada, como un filesystem Linux.
- **Vista de árbol radial/circular:** conserva las relaciones padre-hijo, pero organiza los niveles alrededor de un centro para aprovechar el espacio y revelar agrupaciones.
- **Vista de diagrama:** privilegia la disposición espacial, los caminos y las relaciones visibles entre elementos.
- **Vista de mapa:** privilegia orientación, vecindad y exploración espacial o conceptual; según el caso puede proyectar jerarquía, relaciones o ambas.

La geometría puede cambiar —vertical, horizontal, radial, circular, espacial o esquemática— mientras la identidad y la semántica del modelo permanecen estables. Las conexiones transversales como «se alimenta de», «acicala a» o «utilizado por» deben distinguirse visualmente de las aristas de contención, aunque aparezcan en el mismo lienzo.

Consecuencias para implementación:

1. Separar el **modelo semántico** del **motor de disposición visual** y de la **vista** que lo presenta.
2. Permitir cambiar de vista sin duplicar ni reconstruir las entidades semánticas.
3. Mantener estables las identidades y la selección del elemento al pasar de árbol a diagrama, mapa o vista circular.
4. Hacer explícito qué representa cada línea: contención jerárquica, referencia o relación tipada.
5. Permitir zoom y enfoque sobre una zona sin que el zoom cambie la jerarquía real.
6. No asumir que todo mapa debe ser geográfico ni que todo diagrama es un árbol; la representación elegida depende de qué aspecto del mismo sistema se quiera explorar.

Este principio es una de las ventajas distintivas de Bittopos: **un solo sistema estructurado, múltiples ángulos de lectura**. La interfaz aprovecha la estructura en lugar de obligar a cada vista a inventar su propia organización.
