# Interfaz web experimental

Esta carpeta contiene una interfaz estática que funciona con datos del navegador. No está conectada al núcleo Rust ni escribe directamente en los archivos locales.

## Uso local

Desde la raíz del repositorio, ejecuta:

```bash
python3 -m http.server 8000 --directory web
```

Abre `http://localhost:8000`. No requiere compilación ni dependencias de Node para servir la aplicación. Servirla por HTTP evita restricciones que algunos navegadores aplican a archivos abiertos con `file://`.

1. Importa los recursos TOML.
2. Importa la estructura JSON.
3. Organiza las instancias y crea recursos.
4. Exporta la estructura y los recursos para guardarlos manualmente.

## Despliegue en Cloudflare Pages

Para el frontend actual, que no requiere bundler:

- Repositorio: `jasperslimbell/bittopos`
- Rama: la rama que se decida publicar después de la revisión
- Directorio raíz: `/` (raíz del repositorio)
- Comando de compilación: vacío
- Directorio de salida: `web`
- Variables de entorno: ninguna

Esto publica solamente HTML, CSS y JavaScript. No compila Rust a WASM ni crea un backend. No se deben guardar secretos en el frontend.

## Modelo de archivos

- Cada recurso se exporta como un TOML individual con `id` e `info`.
- La estructura JSON contiene las instancias y su jerarquía.
- Cada instancia conserva su propio `id` y la referencia `source.id`.
- La contención se representa mediante `content.instances`; `semantic.allocation` enumera los hijos directos.

## Límites

La interfaz es un prototipo independiente. No implementa la resolución semántica completa ni todas las reglas de máscaras, relaciones y referencias del núcleo. Los datos de `localStorage` pertenecen al navegador y origen actual; no son sincronización ni copia de seguridad. El parser TOML del navegador solo admite el subconjunto implementado en `app.js`.
