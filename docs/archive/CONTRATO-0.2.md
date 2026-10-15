> **Documento histórico — no es la fuente de verdad vigente.**
>
> Este archivo pertenece a una etapa anterior de Bittopos y puede contener reglas sustituidas, propuestas que nunca se implementaron o criterios de aceptación ya obsoletos. Para el modelo vigente consulta [docs/model.md](../model.md); para el estado real de implementación consulta [docs/status.md](../status.md); para la arquitectura consulta [docs/architecture.md](../architecture.md). Las decisiones arquitectónicas actuales se registran en [docs/decisions/](../decisions/README.md).
>
> Se conserva temporalmente para mantener el contexto histórico. No debe usarse para definir nuevos cambios ni interpretarse como una garantía de que las características descritas existen en el código.

---

Bittopos — Contrato técnico 0.2.0

Estado: Contrato técnico de la línea de desarrollo 0.2.0
Lenguaje principal: Rust
Formato de recursos: TOML
Formato de espacios de trabajo: JSON
Entorno inicial: Local, incluido Termux

___

1. Objetivo

Establecer el alcance funcional y las responsabilidades técnicas de Bittopos 0.2.0. Esta versión debe permitir crear, consultar y editar recursos, así como registrar instancias de recursos en un espacio de trabajo.

La implementación debe priorizar la simplicidad, la persistencia segura, la modularidad y la posibilidad de ampliar el sistema sin implementar anticipadamente características futuras.

Este contrato define el comportamiento requerido, no obliga a utilizar una arquitectura interna más compleja de lo necesario.

El núcleo y la CLI tienen responsabilidades distintas. Crear un recurso no significa necesariamente incorporarlo a un workspace; esa decisión la toma la interfaz al coordinar las operaciones.

___

2. Terminología

2.1. Resource

Unidad de información identificable y persistente. Cada recurso tiene un UUID estable y una estructura Info. Su definición se almacena en un archivo TOML independiente.

2.2. Content

Content es un concepto previsto para describir unidades de contenido con
responsabilidades diferenciadas dentro de un recurso.

En la implementación de esta versión, el recurso contiene únicamente la
estructura Info con título, descripción opcional y etiquetas. No existe
todavía una abstracción Content independiente.

2.3. Info

Información descriptiva del recurso:
title: título obligatorio.
description: descripción opcional.
tags: vector de etiquetas, que puede estar vacío.
La edición de estos datos no debe alterar el identificador del recurso.

2.4. Detail

Detail es un concepto de organización de datos mediante pares clave-valor.
En esta versión, los datos descriptivos se representan mediante la estructura
Info. No existe una estructura Detail independiente.

2.5. Component

Component es un concepto reservado para posibles estructuras especializadas,
incluidas estructuras representadas mediante vectores u otras composiciones.

No existe una estructura Component independiente en esta versión. Su diseño
e implementación se posponen hasta que un caso de uso concreto los justifique.

2.6. Instance

Referencia a un recurso original mediante su identificador. No constituye una copia de su información descriptiva.

Al resolver una instancia se consulta la definición actual del recurso original. Una referencia cuyo recurso no existe debe producir un error identificable.

2.7. Workspace

Espacio de trabajo que registra explícitamente las instancias que contiene. Solo aparece en los recursos referenciados.

2.8. CLI

Interfaz de línea de comandos denominada bitto. Recibe argumentos y modificadores, invoca las operaciones del núcleo y presenta resultados y errores al usuario. Puede coordinar varias operaciones del núcleo para ofrecer un comando de mayor nivel.

___

3. Modelo de datos inicial

3.1. Gestión conceptual (no implementada)

La gestión es un concepto de organización que podrá reunir responsabilidades
relacionadas con la identidad de los recursos y las instancias que los
referencian.

En la implementación actual, estas responsabilidades se distribuyen entre
Resource, ResourceStore, Instance, Workspace y WorkspaceStore.

No existe una estructura Management independiente en esta versión. No debe
crearse únicamente para representar este concepto; su implementación queda
pospuesta hasta que exista una necesidad concreta y un alcance definido.

3.2. Info

Contiene los detalles descriptivos del recurso:

Campo| Tipo conceptual| Requisito
"title"| Cadena de texto| Obligatorio
"description"| Texto opcional| Opcional
"tags"| Vector de cadenas| Puede estar vacío

El identificador del recurso es independiente de estos detalles y no debe cambiar al editar el título, la descripción o las etiquetas.

3.3. Representación TOML

Ejemplo ilustrativo:

id = "0199abcd-1234-7000-8000-123456789abc"

[info]
title = "Aprender Rust"
description = "Notas sobre el lenguaje."
tags = ["software", "aprendizaje"]

El formato definitivo debe ser coherente con las estructuras implementadas y con las capacidades de serialización de Serde y TOML.

El ejemplo no impone que el código Rust deba utilizar exactamente la misma estructura de anidamiento que el archivo.

3.4. Representación JSON del espacio de trabajo

Ejemplo ilustrativo:

{
  "instances": [
    {
      "resource_id": "0199abcd-1234-7000-8000-123456789abc"
    }
  ]
}

Cada entrada identifica un recurso original. El espacio de trabajo no debe almacenar una copia de sus detalles descriptivos.

El formato JSON debe poder ampliarse sin introducir campos innecesarios en esta versión.

___

4. Persistencia

1. Cada recurso se almacena en un archivo TOML independiente.
2. Cada espacio de trabajo se almacena en un archivo JSON.
3. Los identificadores de los recursos son estables y únicos.
4. Crear un recurso no implica incorporarlo automáticamente a un espacio de trabajo.
5. Incorporar un recurso a un espacio de trabajo registra su referencia, no una copia de sus datos.
6. Consultar una instancia resuelve la referencia al recurso original.
7. Editar un recurso modifica su definición original; las instancias normales reflejan los cambios al consultarse de nuevo.
8. La ausencia de un recurso referenciado debe producir un error identificable, no la creación automática de un recurso sustituto.
9. Las operaciones de escritura deben evitar dejar archivos parcialmente escritos cuando sea razonablemente posible.
10. La ubicación de los datos debe ser configurable y no depender accidentalmente del directorio desde el que se ejecuta el programa.
11. El núcleo crea recursos y gestiona referencias mediante operaciones separadas.
12. `bitto create` crea un recurso y, por defecto, lo incorpora al workspace por defecto.
13. `bitto create --no-instance` crea el recurso sin incorporarlo al workspace.
14. `bitto add UUID` incorpora un recurso existente.
15. El workspace por defecto utiliza provisionalmente el nombre de archivo proyecto.json; esto no significa que exista un sistema de proyectos o perfiles.

___


5. Responsabilidades del código

La implementación debe conservar un núcleo reutilizable, separado de la interfaz de comandos.

Núcleo

Responsable de:

- Definir las estructuras implementadas para los recursos, su información
  descriptiva, los espacios de trabajo y las referencias entre ellos.
- Mantener separadas las responsabilidades de persistencia de recursos y
  persistencia de espacios de trabajo.
- Validar los datos.
- Crear, consultar y editar recursos.
- Gestionar la lectura y escritura de TOML y JSON.
- Resolver referencias de instancias.
- Comunicar errores mediante tipos o resultados claros.

CLI

Responsable de:

- Recibir los argumentos del usuario.
- Invocar las operaciones del núcleo.
- Mostrar resultados y errores comprensibles.

La CLI no debe duplicar las reglas de validación ni implementar su propio mecanismo de persistencia.

Interfaz web

La interfaz web no necesita implementarse completamente antes de disponer de un núcleo funcional. Su diseño debe mantenerse separado de las reglas del modelo.

La persistencia directa desde el navegador requerirá un mecanismo local apropiado en una etapa posterior. Esta versión no incluye un servidor HTTP.

Nota: No implementar interfaz web en la versión 0.2.0 (mantener conceptualmente)

6. Organización del código

El repositorio utiliza un workspace de Cargo con dos paquetes:

- `bittopos-core`: paquete raíz que contiene el núcleo reutilizable.
- `bitto-cli`: paquete ubicado en cli/ que contiene la interfaz de terminal y depende del núcleo mediante una dependencia local.

La organización concreta conserva los módulos existentes cuando sus responsabilidades siguen siendo apropiadas.
La estructura principal es:

bittopos/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── CONTRATO-0.1.md
├── CONTRATO-0.2.md
├── src/
│   ├── lib.rs
│   ├── resource.rs
│   ├── store.rs
│   └── workspace.rs
└── cli/
    ├── Cargo.toml
    └── src/
        └── main.rs

El núcleo no depende de la CLI. La CLI utiliza las interfaces públicas del núcleo, sin duplicar sus reglas de validación ni su mecanismo de persistencia.

7. Alcance funcional

Incluido

- Crear recursos y persistirlos como archivos TOML.
- Consultar recursos por su identificador UUID.
- Editar título, descripción y etiquetas mediante las operaciones del núcleo.
- Conservar el identificador del recurso durante las ediciones.
- Leer y guardar espacios de trabajo en archivos JSON.
- Incorporar explícitamente referencias a recursos existentes.
- Resolver las referencias de un espacio de trabajo al consultar su contenido.
- Mostrar los títulos de los recursos referenciados por el espacio de trabajo
  mediante `bitto show`.
- Crear un recurso e incorporarlo al workspace por defecto mediante
  `bitto create`.
- Crear un recurso sin incorporarlo al workspace mediante
  `bitto create --no-instance`.
- Incorporar un recurso existente mediante `bitto add UUID`.
- Informar de errores de persistencia y de referencias no resolubles.
- Probar las operaciones del núcleo de forma independiente de la CLI.

La CLI no proporciona todavía un comando para listar todos los recursos
almacenados que no estén referenciados por el workspace. Esa capacidad queda
fuera del alcance funcional de esta versión.

Excluido

- Relaciones entre recursos.
- Permisos configurables.
- Contenidos especializados adicionales.
- Contenidos exclusivos de instancias.
- Máscaras y snapshots.
- Perfiles, cuentas y colaboración.
- Sincronización remota.
- Servidor HTTP y API de red.
- Docker, Cloud Run, Cloudflare y despliegue público.
- Bases de datos externas y sistemas de caché.

Las exclusiones son decisiones de alcance, no restricciones permanentes de la arquitectura.

8. Errores y seguridad de persistencia

Las operaciones deben distinguir, como mínimo:

- Recurso inexistente.
- Espacio de trabajo inexistente.
- Archivo TOML o JSON inválido.
- Error de lectura o escritura.
- Datos que no cumplen las reglas del modelo.
- Referencia de instancia no resoluble.

No se deben ocultar errores de persistencia ni comunicar que una modificación fue guardada cuando la escritura falló.

Las pruebas deben utilizar directorios temporales para evitar modificar los datos reales del usuario.

9. Compatibilidad de archivos

La estructura actual del prototipo utiliza "Resource" con campos como "id", "name" y "description".

La transición a "Info", "title" y "tags" constituye un cambio de esquema.

Antes de modificar la persistencia se debe comprobar cómo están implementados los archivos y las pruebas actuales. No se debe asumir compatibilidad automática con los archivos anteriores.

Para esta versión inicial podrá aceptarse un formato nuevo si no existe una necesidad real de conservar los datos de prueba anteriores. Si aparecen datos que deban preservarse, se decidirá explícitamente cómo migrarlos.

10. Criterios de aceptación

La versión se considerará funcional cuando se verifique lo siguiente:

- [ ] Se pueda crear un recurso y persistirlo como TOML.
- [ ] Se pueda leer un recurso por su identificador y recuperar sus datos.
- [ ] Se puedan modificar el título, la descripción y las etiquetas sin
      alterar el identificador.
- [ ] Se pueda guardar y cargar un espacio de trabajo JSON.
- [ ] Se puedan registrar referencias a recursos existentes.
- [ ] Al consultar un espacio de trabajo, las referencias se resuelvan a los
      recursos originales.
- [ ] Los recursos no referenciados no aparezcan automáticamente en el
      espacio de trabajo.
- [ ] Los errores de archivos y referencias sean identificables.
- [ ] Las rutas de almacenamiento no dependan accidentalmente del directorio
      desde el que se ejecuta el programa.
- [ ] La CLI permita crear recursos, consultarlos por UUID, mostrar el
      contenido del workspace y añadir recursos existentes.
- [ ] La CLI permita crear un recurso sin incorporarlo al workspace mediante
      `--no-instance`.
- [ ] La CLI utilice las operaciones del núcleo sin duplicar su lógica.
- [ ] Las pruebas y comprobaciones del workspace de Cargo pasen.

Estos puntos son criterios de verificación, no una declaración automática de
que todas las comprobaciones ya se hayan ejecutado.

La enumeración de recursos almacenados que no estén referenciados por un
workspace no forma parte de esta versión.

11. Método de implementación

La implementación se realizará de forma incremental:

1. Revisar las estructuras y pruebas existentes.
2. Ajustar el modelo "Resource" y definir "Info" y "Management".
3. Adaptar la persistencia TOML y sus pruebas.
4. Implementar o adaptar los espacios de trabajo JSON.
5. Conectar las operaciones necesarias con la CLI.
6. Verificar formato, compilación, pruebas y manejo de errores.
7. Actualizar la documentación con el comportamiento realmente implementado.

Cada etapa debe dejar el proyecto en un estado verificable. No se implementarán características futuras sin una decisión explícita de alcance.

---

Fin del contrato técnico de Bittopos 0.2.0.
