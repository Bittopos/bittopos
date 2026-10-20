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
- `GET /api/v1/state`: definiciones y estructura JSON heredadas, con revisión de estructura.
- `GET /api/v1/profiles`: lista los perfiles registrados.
- `POST /api/v1/profiles`: crea un perfil por ID; los IDs duplicados se rechazan.
- `GET /api/v1/profiles/{profile_id}/projects`: lista exclusivamente los proyectos del perfil.
- `POST /api/v1/profiles/{profile_id}/projects`: crea un proyecto ligado a un perfil existente y con estructura vacía.
- `GET /api/v1/projects/{project_id}`: devuelve metadatos, recursos, estructura y revisión del proyecto.
- `GET /api/v1/projects/{project_id}/resources`: lista recursos del proyecto.
- `POST /api/v1/projects/{project_id}/resources`: guarda un recurso JSON estructurando título, descripción y etiquetas.
- `POST /api/v1/projects/{project_id}/resources/import-toml`: valida y conserva el TOML original y su representación estructurada.
- `PUT /api/v1/projects/{project_id}/workspace`: guarda la estructura y actualiza las filas relacionales de instancias en una transacción.
- `POST /api/v1/resources`: crea una definición JSON.
- `POST /api/v1/resources/import-toml`: valida TOML con la biblioteca estándar y conserva el texto original.
- `PUT /api/v1/resources/{id}`: actualiza la definición JSON existente.
- `PUT /api/v1/workspace`: reemplaza la estructura si coincide `expectedRevision`.

La estructura mantiene `content.instances`, la identidad de cada instancia y `source.id`. Los campos desconocidos se conservan en el documento JSON; no se ejecuta resolución semántica ni se reinterpreta el contenido de segmentos.

## Persistencia y límites

SQLite conserva perfiles y proyectos en tablas independientes, con una clave foránea de proyecto a perfil. Cada proyecto tiene su propia estructura y contador de revisión. Los recursos de proyecto se guardan como documento JSON, con columnas consultables para título, descripción y etiquetas; cuando se importan, también se conserva el TOML original. Las instancias se guardan como estructura JSON de intercambio y se proyectan transaccionalmente en filas con ID, padre, recurso de origen, semántica y máscaras. El documento JSON sigue siendo una representación compatible; no se eliminan ni se reescriben archivos locales. La API heredada `/api/v1/state` continúa disponible para no romper la SPA actual.

Esto no define aún el esquema relacional definitivo. No hay autenticación de usuario ni autorización por perfil, multi-tenancy seguro, borrado de recursos, auditoría, sincronización CLI, `push/pull/log`, gRPC ni migración automática de los archivos locales existentes. Los nuevos endpoints de perfil/proyecto son una capa inicial de persistencia, no una promesa de despliegue multiusuario listo para Internet. No expongas este servicio directamente a Internet ni lo consideres multiusuario seguro.

Antes de un despliegue remoto real se necesitan autenticación de usuario, autorización por espacio de trabajo, límites de abuso, HTTPS, copias de seguridad/migraciones y una política de concurrencia. Cloudflare Pages puede servir la SPA, pero no ejecuta este servidor Python: el API necesitará un entorno compatible separado. No se ha desplegado ningún servicio en la nube desde este cambio.
