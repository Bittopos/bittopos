# Bittopos Web — prototipo local

Esta carpeta contiene una primera interfaz web estática para el modelo que existe hoy en Bittopos. No forma parte todavía de la arquitectura ejecutable del núcleo Rust.

## Qué incluye

- Árbol de instancias con contención recursiva.
- Inspector que distingue la identidad de instancia (`id`) de la identidad del recurso de origen (`source.id`).
- Vista de árbol y vista jerárquica de mapa.
- Creación local de definiciones y de instancias.
- Importación de workspace JSON e importación múltiple de definiciones TOML.
- Exportación del workspace con `content.instances`, `semantic.allocation` y `source.id`.
- Exportación de definiciones como archivos TOML.
- Persistencia temporal en `localStorage` del navegador.

## Ejecutar

Abre `web/index.html` en un navegador moderno. No requiere Node.js, un paquete frontend ni un servidor HTTP para esta etapa.

## Uso recomendado

1. Importa `proyecto.json` desde el directorio de workspaces de Bittopos.
2. Importa los archivos TOML del directorio de recursos. Puedes seleccionar varios a la vez.
3. Explora el árbol, crea recursos o añade instancias.
4. Exporta `proyecto.json` y las definiciones TOML modificadas para guardarlas manualmente en sus directorios correspondientes.

Los archivos permanecen en el directorio elegido por el usuario: el navegador no puede leerlos ni sobrescribirlos automáticamente. Los recursos TOML se descargan uno por uno al exportar.

## Límites conocidos

- No existe integración HTTP con `bittopos-core`; los cambios no se escriben directamente en el workspace real.
- La persistencia del navegador no sustituye una copia de seguridad. Exporta los archivos después de editar.
- El importador TOML cubre los campos actuales (`id`, `management.structural`, `info.title`, `info.description` y `info.tags`); no es un parser TOML general.
- El editor de metadatos es deliberadamente sencillo. El modelo general Segment/Concept/Component, las máscaras y los permisos efectivos no están implementados en esta interfaz.
- La eliminación respeta la regla actual de no borrar implícitamente descendientes: solo permite eliminar instancias hoja.

## Siguiente paso de integración

Cuando se acuerde el servidor HTTP, la interfaz deberá consumir operaciones explícitas del núcleo y dejar de mantener una copia local independiente del workspace. La integración deberá validar los mismos invariantes que `bittopos-core`, no duplicar reglas de dominio en la interfaz.
