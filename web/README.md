# Interfaz web experimental

Esta carpeta contiene una interfaz estática que funciona con datos del navegador. No está conectada al núcleo Rust ni escribe directamente en los archivos locales.

## Uso

Abre `index.html` en un navegador moderno. No requiere Node.js ni un servidor.

1. Importa los recursos TOML.
2. Importa la estructura JSON.
3. Organiza las instancias, crea recursos o modifica sus metadatos.
4. Exporta la estructura y los recursos modificados para guardarlos manualmente.

## Modelo de archivos

- Cada recurso se exporta como un TOML individual con `id` e `info`.
- La estructura JSON contiene las instancias y su jerarquía.
- Cada instancia conserva su propio `id` y la referencia `source.id`.
- La contención de hijos se representa en `content.instances`; `semantic.allocation` enumera los hijos directos.

La interfaz no mantiene un registro de instancias dentro de los archivos de recursos.

## Límites

La interfaz web es un prototipo independiente. No implementa la resolución semántica completa ni las máscaras y no comparte automáticamente las reglas del núcleo Rust. Las operaciones y los formatos pueden cambiar mientras se desarrolla el modelo.
