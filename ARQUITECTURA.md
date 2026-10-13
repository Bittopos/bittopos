# Arquitectura de Bittopos: límites y plan de refactorización

**Estado:** documento de trabajo para orientar la refactorización de la línea 0.3.0. Describe el estado observado en el código y la dirección acordada; no afirma que la arquitectura objetivo ya esté implementada.

## 1. Objetivo

Preparar el núcleo para que la CLI actual y futuras interfaces puedan utilizar las mismas operaciones del modelo, sin duplicar reglas ni depender de los formatos de persistencia. La refactorización debe ser incremental, comprobable y no introducir funcionalidades futuras por anticipado.

La regla general es:

> La interfaz expresa una intención mediante una operación estructurada; la capa de aplicación coordina el caso de uso; el modelo valida las invariantes; la persistencia traduce y materializa el estado.

Esto no exige crear una capa o abstracción para cada tipo. Cada frontera debe responder a una responsabilidad real y tener pruebas que justifiquen su existencia.

## 2. Límites de responsabilidad deseados

### 2.1. Adaptadores de interfaz

La CLI y las interfaces futuras:
- interpretan argumentos, eventos y entradas del usuario;
- convierten esas entradas en solicitudes estructuradas;
- presentan resultados y errores de manera adecuada al medio;
- no editan directamente archivos ni manipulan representaciones persistidas para imponer reglas del modelo.

La CLI seguirá siendo una interfaz pequeña. Una futura interfaz web debe consumir las mismas operaciones de aplicación, no reimplementar las reglas estructurales.

### 2.2. Capa de aplicación

La aplicación coordina casos de uso completos, por ejemplo crear una definición, instanciar un recurso, añadir una instancia dentro de otra, navegar o remover una instancia.

Es responsable de:
- cargar el estado requerido;
- invocar las validaciones y operaciones del modelo;
- coordinar escrituras que abarcan más de un archivo;
- invocar la recuperación cuando corresponda;
- devolver resultados y errores de caso de uso.

No debe convertirse en una segunda implementación del modelo ni exponer detalles accidentales de TOML, JSON o rutas de archivos a las interfaces.

### 2.3. Modelo del núcleo

El modelo expresa conceptos y reglas independientes del medio de entrada o almacenamiento:
- identidad de recursos e instancias;
- definición y origen de una instancia;
- contención directa y estructura recursiva;
- consistencia entre la estructura física y la asignación semántica;
- reglas de remoción de instancias hoja;
- validación de referencias e invariantes.

El modelo no debe depender de errores concretos de archivos, serializadores, rutas o tipos de almacenamiento. Los errores de dominio deben poder entenderse sin conocer dónde se guardan los datos.

### 2.4. Persistencia

La persistencia convierte los datos del modelo a sus formatos y los recupera de ellos. En el estado actual, las definiciones de recursos se guardan en TOML y el workspace en JSON; la navegación usa TOML y el journal usa JSON.

La persistencia es responsable de:
- leer y escribir formatos;
- informar errores de E/S y serialización;
- realizar escrituras atómicas de archivos individuales;
- validar que los datos cargados puedan convertirse a las estructuras del modelo.

No debe decidir silenciosamente qué estructura conceptual es correcta cuando encuentra datos contradictorios. Las discrepancias deben devolverse como errores explícitos.

### 2.5. Recuperación y durabilidad

La recuperación coordina operaciones interrumpidas que afectan más de un archivo. Debe poder reutilizarse desde cualquier interfaz y ejecutarse antes de exponer un estado que todavía requiera reconciliación.

Para las operaciones cubiertas por el contrato 0.3, se mantiene el orden:

**journal durable → recurso TOML → workspace JSON → validar resultado → retirar journal.**

La recuperación debe ser idempotente. No debe inventar padres ni aplanar el árbol si no existe información suficiente para reconstruirlo. Los detalles del journal no deben convertirse en responsabilidades de la CLI.

La escritura atómica de un archivo individual y la transacción lógica que abarca varios archivos son responsabilidades diferentes. Una no sustituye a la otra.

## 3. Inventario del código actual

Este inventario describe las responsabilidades observadas en el código de la rama principal al iniciar esta refactorización. No significa que todos los módulos deban reemplazarse ni que sus límites actuales sean incorrectos en todos los casos.

| Módulo | Responsabilidad actual | Frontera que conviene mejorar |
| --- | --- | --- |
| `resource.rs` | Tipos de recurso, información descriptiva, parches y validación/normalización | El modelo importa `StoreError`, por lo que conoce errores de almacenamiento |
| `store.rs` | Persistencia TOML, operaciones sobre recursos, registro estructural y temporales | Mezcla serialización, operaciones de almacenamiento y ciertos conflictos de dominio |
| `workspace.rs` | Tipos de instancia/workspace, árbol recursivo, validación, JSON y escritura atómica | Modelo, validación y persistencia comparten módulo |
| `lifecycle.rs` | Creación/remoción de instancias, journal y reconciliación | Coordina el caso de uso y contiene detalles del protocolo de recuperación |
| `navigation.rs` | Estado de navegación, validación del destino y persistencia TOML | Depende de `workspace.rs` para la escritura atómica |
| `lib.rs` | Declaración y reexportación de módulos públicos | La API pública deberá reflejar las fronteras una vez que se estabilicen |
| `cli/src/main.rs` | Parseo de comandos, resolución de rutas, recuperación de inicio, navegación, coordinación y presentación | Concentra coordinación de aplicación que conviene extraer del adaptador de terminal |

La refactorización debe conservar primero el comportamiento y las invariantes existentes. Los módulos solo deben dividirse cuando eso reduzca una dependencia concreta o permita probar una responsabilidad de manera aislada.

## 4. Invariantes que no deben perderse

1. `Resource.id` y `Instance.id` son identidades distintas.
2. `Instance.source.id` referencia el ID del recurso de origen.
3. `Workspace.content.instances` contiene las raíces; cada instancia guarda sus hijos directos completos en su propio `content.instances`.
4. `semantic.allocation` contiene únicamente los IDs de los hijos directos y debe coincidir con los hijos físicos en contenido y orden.
5. Los IDs de instancia son únicos en todo el workspace; no se permiten ciclos, instancias duplicadas ni múltiples padres físicos.
6. `management.structural` registra las identidades de instancias asociadas a una definición; no representa la jerarquía de contención.
7. La remoción actual solo permite instancias hoja y no elimina la definición del recurso.
8. Una discrepancia estructural no se repara escogiendo silenciosamente una de las representaciones.
9. La recuperación es idempotente y no inventa ubicaciones que no puedan deducirse del estado durable.
10. Los JSON del esquema antiguo que no cumplan el contrato actual se rechazan explícitamente; no se añade migración automática como parte de esta refactorización.

Estas invariantes pertenecen al modelo y a los casos de uso, no exclusivamente a la CLI.

## 5. Errores y dependencias

Conviene distinguir al menos estas categorías, sin exigir que cada una tenga desde el primer momento un módulo o una jerarquía compleja de tipos:

- **Dominio:** una operación viola una regla del modelo, por ejemplo remover una instancia con hijos.
- **Validación de datos:** el estado leído no cumple el esquema o las invariantes.
- **Persistencia:** falló la lectura, escritura, serialización o sustitución atómica de un archivo.
- **Recuperación/consistencia:** journal o registros entre archivos contradictorios, incompletos o irrecuperables.
- **Interfaz:** argumentos inválidos o una presentación/operación no disponible en el adaptador.

Las capas superiores pueden contextualizar los errores, pero no deben tener que analizar cadenas de texto para distinguir sus causas. La dirección de dependencias buscada es: interfaz → aplicación → modelo; la aplicación utiliza puertos/servicios de persistencia y recuperación. El modelo no depende de la CLI ni de la persistencia.

La forma exacta de los puertos y los tipos de error se decidirá al implementar cada paso, no mediante una abstracción genérica prematura.

## 6. Secuencia de refactorización

### Paso 1 — Contrato e inventario
Documentar los límites deseados, el inventario real, las invariantes y los criterios de aceptación. No cambia el comportamiento del programa.

### Paso 2 — Separar modelo y persistencia
Extraer los tipos y reglas de dominio de los errores y detalles de almacenamiento. Separar serialización/lectura/escritura de las estructuras del modelo con el menor cambio público viable. Añadir pruebas para demostrar que las invariantes se conservan.

### Paso 3 — Consolidar primitivas de persistencia
Definir una frontera común para las operaciones reales de lectura, escritura y reemplazo atómico. Mantener separadas la escritura atómica de un archivo y la coordinación durable entre varios archivos.

### Paso 4 — Introducir casos de uso de aplicación
Mover la coordinación reutilizable fuera de la CLI. La aplicación debe coordinar operaciones estructuradas; el modelo sigue siendo la autoridad sobre las reglas de dominio.

### Paso 5 — Reducir la CLI a adaptador
Conservar comandos y comportamiento observables, delegando los casos de uso a la aplicación y limitándose a presentar resultados.

### Paso 6 — Validar antes de ampliar
Ejecutar formato, compilación y pruebas del workspace; añadir pruebas de persistencia y recuperación para fallos parciales. Revisar que el contrato y el README describan el comportamiento realmente implementado.

Las máscaras son el siguiente incremento funcional después de estabilizar esta refactorización. Las relaciones, referencias avanzadas, permisos, perfiles, sincronización, servidor y web no se deben introducir incidentalmente en estos pasos.

## 7. Criterios de aceptación

- [ ] Las invariantes estructurales actuales continúan cubiertas por pruebas.
- [ ] El modelo no depende de tipos de error propios de almacenamiento.
- [ ] Las reglas de dominio pueden probarse sin crear archivos reales cuando no se necesita persistencia.
- [ ] Los formatos TOML/JSON y las rutas no se filtran a las reglas del modelo.
- [ ] Las operaciones que modifican varios archivos siguen utilizando journal durable y recuperación idempotente.
- [ ] La escritura atómica por archivo no se confunde con una transacción entre archivos.
- [ ] La coordinación de los casos de uso es reutilizable por la CLI y futuras interfaces.
- [ ] La CLI no edita directamente los archivos ni reimplementa las reglas estructurales.
- [ ] Los errores conservan suficiente tipo/contexto para que la interfaz los presente sin inspeccionar cadenas.
- [ ] `cargo fmt --all -- --check`, `cargo check --workspace` y `cargo test --workspace --locked` pasan antes de fusionar los cambios de implementación.
- [ ] La documentación diferencia con claridad el comportamiento implementado de los objetivos futuros.

## 8. Fuera de alcance

Este documento no introduce máscaras, relaciones, referencias avanzadas, permisos, perfiles, proyectos múltiples, colaboración, sincronización, servidor HTTP ni interfaz web. Tampoco autoriza migrar automáticamente archivos antiguos. Estas decisiones permanecen en sus incrementos correspondientes.

---

Este documento se revisará a medida que la implementación revele límites mejores o más simples. El objetivo es mantener las responsabilidades explícitas, no congelar una estructura interna innecesaria.
