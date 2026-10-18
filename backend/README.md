# Backend experimental de Bittopos

Este servicio es un adaptador HTTP/SQLite para explorar la separación entre la interfaz web y la persistencia. **No es todavía un backend de producción ni contiene el núcleo semántico Rust.**

## Ejecutar localmente

Requiere Python 3.11 o superior; no necesita paquetes externos.

Desde la raíz del repositorio:

```bash
python3 backend/server.py
```

Abre `http://127.0.0.1:8787`. La base se crea en `backend/data/bittopos.sqlite3` (ese directorio está excluido de Git). Para cambiar ubicación:

```bash
BITTOPOS_DB_PATH=/ruta/privada/bittopos.sqlite3 python3 backend/server.py
```

Variables opcionales:

- `BITTOPOS_HOST`: por defecto `127.0.0.1`; no lo cambies a una dirección pública sin autenticación y despliegue endurecido.
- `BITTOPOS_PORT`: por defecto `8787`.
- `BITTOPOS_API_TOKEN`: habilita un bearer token para las rutas de API, pero la SPA aún no incorpora un flujo seguro de sesión/token. Úsalo solo para pruebas directas de API.
- `BITTOPOS_CORS_ORIGIN`: origen exacto permitido si se prueba una SPA separada. No uses comodines para datos privados.

## API v1

- `GET /api/v1/health`: estado del proceso, sin datos.
- `GET /api/v1/state`: definiciones y estructura JSON actuales, con revisión de estructura.
- `POST /api/v1/resources`: crea una definición JSON.
- `POST /api/v1/resources/import-toml`: valida TOML con la biblioteca estándar y conserva el texto original.
- `PUT /api/v1/resources/{id}`: actualiza la definición JSON existente.
- `PUT /api/v1/workspace`: reemplaza la estructura si coincide `expectedRevision`.

La estructura mantiene `content.instances`, la identidad de cada instancia y `source.id`. Los campos desconocidos se conservan en el documento JSON; no se ejecuta resolución semántica ni se reinterpreta el contenido de segmentos.

## Persistencia y límites

SQLite guarda recursos como documentos JSON y, cuando se importan, conserva también el texto TOML original. La estructura se guarda como JSON con un contador de revisión para detectar escrituras concurrentes simples. `GET /api/v1/state` entrega el conjunto JSON que consume el navegador.

Esto no define aún el esquema relacional definitivo. No hay cuentas, permisos por usuario, multi-tenancy, borrado de recursos, auditoría, sincronización CLI, `push/pull/log`, gRPC ni migración automática de los archivos locales existentes. No expongas este servicio directamente a Internet ni lo consideres multiusuario seguro.

Antes de un despliegue remoto real se necesitan autenticación de usuario, autorización por espacio de trabajo, límites de abuso, HTTPS, copias de seguridad/migraciones y una política de concurrencia. Cloudflare Pages puede servir la SPA, pero no ejecuta este servidor Python: el API necesitará un entorno compatible separado. No se ha desplegado ningún servicio en la nube desde este cambio.
