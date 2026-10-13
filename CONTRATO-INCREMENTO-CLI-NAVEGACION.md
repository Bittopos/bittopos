# Bittopos — Contrato del incremento: CLI básica y navegación persistente

**Estado:** propuesta para revisión; no implica que las funcionalidades descritas estén implementadas.  
**Base conceptual:** contrato técnico 0.3.0.  
**Alcance:** preparar una base operativa para comenzar el frontend, sin completar el MVP ni la beta.

---

## 1. Objetivo

Este incremento establece las operaciones básicas necesarias para trabajar desde la CLI y navegar por el espacio de trabajo, conservando la ubicación actual para que el usuario pueda retomar su trabajo después de cerrar y volver a abrir Bittopos.

El resultado debe ser suficientemente estable y estar organizado para comenzar el desarrollo del frontend. No se pretende implementar ahora todas las funcionalidades del producto ni terminar la arquitectura definitiva del núcleo.

## 2. Terminología

### 2.1. Resource

`Resource` es el término interno utilizado por el sistema para representar la definición de un objeto. El usuario trabaja provisionalmente con los recursos a través de sus instancias.

El término técnico `recurso` puede resultar confuso en la interfaz. Su presentación al usuario queda pendiente del diseño de la UI; este incremento no renombra el modelo interno.

### 2.2. Instance

`Instance` es un término reservado para el sistema y no es sinónimo de `Resource`. Una instancia es una aparición concreta de un recurso en el espacio de trabajo y constituye la unidad operativa mediante la cual se trabaja con objetos.

- `Resource.id` identifica la definición.
- `Instance.id` identifica una aparición concreta y es distinto del ID del recurso.
- `Instance.source.id` referencia el recurso de origen.
- Varias instancias pueden proceder del mismo recurso y conservan identidades distintas.

La interfaz futura podrá presentar estos conceptos con términos más sencillos, sin alterar la distinción interna.

## 3. Navegación persistente

### 3.1. Propósito

La navegación guarda la ubicación actual del usuario para permitirle retomar su trabajo donde lo dejó. En este incremento es un mecanismo sencillo de continuidad, no un sistema de selección o administración de contextos.

### 3.2. Persistencia y estado

- El estado de navegación se almacenará inicialmente en un archivo TOML.
- TOML es una solución transitoria durante la evaluación de una futura migración a almacenamiento binario.
- El estado identificará la instancia actual o la raíz del espacio de trabajo.
- El estado de navegación será independiente del archivo que almacena la estructura del espacio de trabajo.
- La aplicación validará el destino al recuperar el estado.
- Si el destino ya no existe o no es válido, se recuperará un estado seguro regresando a la raíz, sin inventar destinos ni alterar la estructura del workspace.

La implementación deberá mantener la persistencia sencilla y evitar acoplar el modelo conceptual al formato TOML.

### 3.3. Límites

No se implementarán en este incremento:

- Selección de instancias como sistema independiente.
- Listas o pilas de contextos configurables.
- Historial de navegación.
- Políticas de navegación.
- Preferencias de usuario o ajustes personalizados.
- Reglas de navegación propias de una UI todavía no diseñada.
- Sincronización de la navegación entre dispositivos o sesiones.

La navegación solo deberá resolver el desplazamiento básico por las ubicaciones válidas y conservar la ubicación actual.

## 4. Estructura del espacio de trabajo

Se conserva el modelo recursivo de instancias establecido en el contrato 0.3.0:

- `Workspace.content.instances` contiene las instancias raíz.
- Cada instancia contiene en `content.instances` sus instancias hijas directas.
- `semantic.allocation` contiene los IDs de los hijos directos y debe coincidir con el contenido físico, incluido el orden.
- La contención expresa composición estructural; no equivale a una relación o referencia genérica.
- La ubicación de navegación apunta a una instancia válida o a la raíz, nunca a una definición de recurso sin instanciar.

Este incremento no redefine el esquema de contención ni relaja las reglas de consistencia, identidad, unicidad y recuperación ya establecidas en el contrato 0.3.0.

## 5. Comandos básicos de la CLI

El incremento debe consolidar un conjunto mínimo y coherente de operaciones para:

- Crear y consultar recursos.
- Crear instancias de recursos.
- Consultar las instancias del espacio de trabajo.
- Entrar en una instancia y desplazarse por la estructura.
- Regresar a la raíz o a la ubicación anterior mediante las operaciones básicas de navegación acordadas.
- Recuperar la ubicación persistida al iniciar la aplicación.
- Remover instancias cuando la operación esté permitida por las reglas de integridad vigentes.

Los nombres, argumentos y la lista final de comandos deberán corresponder a las capacidades acordadas y a la revisión del estado actual del código. No se añadirán comandos solo por anticipar funciones futuras.

La CLI actúa como interfaz del sistema. Las reglas de identidad, contención, validación y consistencia pertenecen al núcleo, no deben existir únicamente en la presentación de terminal.

## 6. Organización del núcleo y refactorización pendiente

La refactorización completa del núcleo queda aplazada hasta después de definir con mayor precisión las jerarquías conceptuales y las responsabilidades de sus entidades y operaciones.

Como parte de esa refactorización deberá separarse correctamente la jerarquía de objetos y dejar `Resource`, `Segment` y los demás conceptos claramente delimitados en el núcleo. No se considera resuelto por el hecho de que actualmente existan structs o archivos con esos nombres.

Este incremento no reorganizará el núcleo de manera especulativa. Solo permitirá cambios puntuales necesarios para implementar la navegación y consolidar los comandos básicos, respetando las invariantes existentes y evitando acoplamientos innecesarios entre navegación, CLI y persistencia.

La refactorización posterior deberá definir responsabilidades conceptuales, no limitarse a redistribuir código entre archivos.

## 7. Rutas y espacios de trabajo

Se mantienen las rutas de almacenamiento actuales, aunque algunas estén definidas de forma fija. Se reconoce que son provisionales.

Quedan fuera de alcance:

- `bitto init` para inicializar limpiamente un espacio de trabajo nuevo.
- Perfiles.
- Proyectos.
- Gestión de varios espacios de trabajo.
- Registro, selección y resolución de metadatos de esos espacios.
- Configuración avanzada de rutas.

Estas funciones se abordarán después de establecer las operaciones básicas del núcleo y definir cómo se representan y gestionan los espacios de trabajo.

## 8. Preparación para el frontend

El resultado debe permitir comenzar a construir el frontend sobre operaciones del sistema que tengan responsabilidades claras, sin exigir que la interfaz adopte la estructura interna de la CLI.

No se exige en este incremento implementar:

- El frontend completo.
- Un servidor HTTP o daemon.
- Una API pública definitiva.
- Diseño visual, políticas de UX o preferencias personalizadas.
- Infraestructura de despliegue remoto.

La meta es disponer de una base operativa estable y extensible, no anticipar toda la infraestructura de la interfaz.

## 9. Criterios de aceptación

- [ ] Las operaciones básicas acordadas de la CLI funcionan de forma coherente.
- [ ] Se conserva la separación de identidad entre recurso e instancia.
- [ ] La navegación permite entrar en ubicaciones válidas y volver a la raíz según las operaciones definidas.
- [ ] La ubicación actual se guarda en TOML y se recupera al reiniciar la aplicación.
- [ ] Un destino inexistente o inválido se resuelve de forma segura, regresando a la raíz.
- [ ] La navegación no modifica accidentalmente los recursos ni la estructura del workspace.
- [ ] Se preservan las invariantes del árbol recursivo y las reglas de persistencia del contrato 0.3.0.
- [ ] Las pruebas cubren persistencia, recuperación, navegación válida y destinos inválidos.
- [ ] El proyecto compila y las pruebas existentes continúan pasando.
- [ ] Los cambios quedan suficientemente organizados para comenzar el frontend sin requerir una refactorización total previa.
- [ ] La documentación distingue claramente el comportamiento implementado de las funciones aplazadas.

## 10. Fuera de alcance

Este incremento no incluye:

- Completar el MVP o la beta.
- Implementar todos los comandos previstos para esas etapas.
- La refactorización arquitectónica completa del núcleo.
- La definición final de la jerarquía de `Resource`, `Segment` y demás entidades.
- Asignación de tareas.
- `bitto init`, perfiles y proyectos.
- Migración definitiva de TOML a almacenamiento binario.
- Selección avanzada, listas de contexto o políticas de navegación.
- Implementación del frontend completo, servidor HTTP o daemon.

Estas exclusiones son límites del incremento, no restricciones permanentes para la arquitectura de Bittopos.

---

**Fin de la propuesta de contrato para el incremento de CLI básica y navegación persistente.**
