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

Unidad de contenido que agrupa información de una responsabilidad determinada dentro de un recurso.

Los contenidos iniciales son:

- Info: título, descripción y etiquetas.

La arquitectura conceptual admite varios contenidos por recurso, pero la implementación inicial solo necesita el contenido anterior.

2.2.1 Info

Información descriptiva del recurso:
title: título obligatorio.
description: descripción opcional.
tags: vector de etiquetas, que puede estar vacío.
La edición de estos datos no debe alterar el identificador del recurso.

2.3. Detail

Representa datos organizados mediante pares clave-valor.

Ejemplos actuales:

- Título.
- Descripción.
- Etiquetas, como un valor de tipo vector asociado a la clave correspondiente.

En este contrato, "Detail" describe una organización conceptual de los datos; no implica que todos sus valores sean cadenas de texto.

2.4. Component

Representa datos estructurados directamente como vectores, en lugar de una colección de pares clave-valor.

Las relaciones y otras estructuras especializadas podrán utilizar este concepto en versiones futuras. No se implementarán ahora.

Nota: No implementar en la versión 0.2.0 (solo conceptualmente)

2.5. Instance

Referencia a un recurso original mediante su identificador. No constituye una copia de su información descriptiva.

Al resolver una instancia se consulta la definición actual del recurso original. Una referencia cuyo recurso no existe debe producir un error identificable.

2.6. Workspace

Espacio de trabajo que registra explícitamente las instancias que contiene. Solo aparece en los recursos referenciados.

2.7. CLI

Interfaz de línea de comandos denominada bitto. Recibe argumentos y modificadores, invoca las operaciones del núcleo y presenta resultados y errores al usuario. Puede coordinar varias operaciones del núcleo para ofrecer un comando de mayor nivel.

___

3. Modelo de datos inicial

3.1. Management

Responsabilidades conceptuales:

- Identificar el recurso mediante un ID estable.
- Representar el vector de instancias asociadas al espacio de trabajo o a la estructura de gestión correspondiente.

Para evitar duplicar información, el identificador del recurso se almacena en su definición TOML y las instancias activas del espacio de trabajo se registran en su archivo JSON. El modelo lógico de gestión puede reunir ambas responsabilidades sin exigir que todos los datos residan físicamente en el mismo archivo.

Nota: No implementar en la versión 0.2.0 hasta tener pruebas que lo respalden o hasta futuros incrementos. Queda parcialmente implementado.

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

- Definir las estructuras de "Resource", "Management", "Info" y las referencias necesarias.
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

- Crear recursos.
- Consultar recursos.
- Editar título, descripción y etiquetas al momento de su creación.
- Conservar el ID durante las ediciones.
- Listar recursos almacenados.
- leer espacio de trabajo JSON por defecto.
- Incorporar explícitamente referencias a recursos.
- Resolver las referencias al abrir un espacio de trabajo.
- Informar sobre referencias inválidas o recursos inexistentes.
- Probar las operaciones del núcleo de forma independiente de la CLI.

La CLI actual muestra los recursos del workspace o consulta un recurso por UUID.

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

La versión se considerará funcional cuando:

- [ ] Se pueda crear un recurso y persistirlo como TOML.
- [ ] Se pueda leer un recurso y recuperar sus datos.
- [ ] Se puedan modificar los detalles sin alterar el ID.
- [ ] Se puedan registrar referencias en un espacio de trabajo JSON.
- [ ] Abrir un espacio de trabajo resuelva las referencias a sus recursos originales.
- [ ] Los recursos no referenciados no aparezcan automáticamente en el espacio de trabajo.
- [ ] Las modificaciones del recurso original se reflejen al volver a consultarlo.
- [ ] Los errores de archivos y referencias sean identificables.
- [ ] Las rutas de almacenamiento no dependan del directorio de ejecución.
- [ ] Las pruebas existentes y nuevas pasen.
- [ ] La CLI utilice las operaciones del núcleo sin duplicar su lógica.

Los perfiles, proyectos, `bitto init`, relaciones avanzadas, permisos configurables, servidor y web siguen fuera del alcance actual.

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
