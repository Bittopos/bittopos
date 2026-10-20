# Bittopos Web

Licencia del frontend: [Mozilla Public License 2.0](LICENSE) (MPL-2.0). El servidor HTTP Rust se encuentra en `server/`; consulta [su guía](../server/README.md). La SPA inicia sesión con `BITTOPOS_USERNAME` y `BITTOPOS_PASSWORD` y trabaja dentro del primer perfil/proyecto disponibles (crea uno inicial si no existen).

Frontend reescrito como SPA modular con HTML, CSS y módulos ES nativos. Consume `/api/v1` y no utiliza `localStorage` como almacén principal. La persistencia inicial la gestiona el adaptador SQLite del directorio `backend/`.

## Ejecutar el MVP local

Requiere Python 3.11 o superior y un navegador moderno. No necesita paquetes externos.

```bash
cargo run -p bittopos-server
```

Configura las variables descritas en `server/.env.example` y ejecuta el servidor. La SPA estática debe servirse por separado; el API Rust escucha por defecto en el puerto 3000 y guarda SQLite en `BITTOPOS_DB_PATH`.

Para comprobar sintaxis del frontend desde `web/`:

```bash
npm run check
```

Para las pruebas del adaptador desde la raíz:

```bash
python -m unittest discover -s backend/tests -v
```

## Funcionalidad actual

- Cargar recursos y estructura desde la API.
- Crear/editar definiciones de recurso.
- Crear/eliminar instancias, manteniendo separados el ID de recurso y el ID de instancia.
- Buscar en la estructura.
- Importar TOML y JSON; exportar una copia JSON de recursos y estructura.
- Persistir estructura con contador de revisión para detectar escrituras obsoletas.

La API preserva campos desconocidos en los documentos JSON. La edición de instancias aún no implementa máscaras ni resolución semántica completa; los cambios de contención actualizan `content.instances` y `semantic.allocation` juntos según el contrato provisional.

## Cloudflare Pages y backend

Cloudflare Pages puede servir los archivos estáticos, pero no ejecuta este servidor Python. Para usar la SPA desde un despliegue remoto hace falta alojar el API en un entorno compatible y resolver HTTPS, CORS, autenticación, autorización y secretos. No se ha desplegado el backend en la nube y no debe exponerse directamente a Internet.

Consulta [la guía del backend](../backend/README.md) y [el límite arquitectónico de esta iteración](../docs/api-backed-frontend.md).
