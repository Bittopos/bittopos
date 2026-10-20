# Servidor Rust de Bittopos

Este servicio HTTP utiliza `bittopos-core` como autoridad semántica. La primera iteración ofrece salud, sesión de desarrollo y lectura/creación básica de recursos. No es todavía un servicio multiusuario ni implementa permisos por cuenta/proyecto.

## Configuración

Copia `.env.example` a `.env` y establece un secreto propio de al menos 12 caracteres. No subas `.env` al repositorio.

- `BITTOPOS_USERNAME`: usuario de desarrollo (por defecto `admin`).
- `BITTOPOS_PASSWORD`: contraseña obligatoria; el proceso se niega a arrancar si falta o es demasiado corta.
- `BITTOPOS_DATA_DIR`: directorio persistente de recursos y espacios de trabajo.
- `BIND_ADDR`: dirección de escucha; por defecto `0.0.0.0:3000` para contenedores.
- `WEB_ORIGIN`: origen exacto autorizado por CORS.
- `SESSION_COOKIE_SECURE`: `true` para Cloudflare/HTTPS; `false` solo para desarrollo HTTP local.

## Ejecutar localmente

```bash
cp server/.env.example .env
# Edita .env y define BITTOPOS_PASSWORD con una contraseña aleatoria propia.
set -a
. ./.env
set +a
cargo run -p bittopos-server
```

Con `SESSION_COOKIE_SECURE=false`, la SPA local debe ejecutarse en el origen configurado en `WEB_ORIGIN`. El frontend debe enviar `credentials: "include"` en las solicitudes `fetch`.

## API inicial

- `GET /health`: comprobación de disponibilidad sin autenticación.
- `POST /api/v1/login`: JSON `{ "username": "admin", "password": "..." }`; establece una cookie HttpOnly.
- `POST /api/v1/logout`: invalida la sesión actual.
- `GET /api/v1/me`: confirma la sesión.
- `GET /api/v1/resources`: lista recursos.
- `GET /api/v1/resources/{id}`: obtiene un recurso.
- `POST /api/v1/resources`: crea un recurso con `title` y `description` opcional.
- `GET /api/v1/workspaces`: lista los espacios de trabajo guardados.

Todos los endpoints salvo `/health` y `/api/v1/login` requieren una sesión válida. Las sesiones viven en memoria durante 24 horas y se invalidan al reiniciar el proceso; es una decisión temporal adecuada para desarrollo, no para escalar a varias réplicas.

## Persistencia y Cloudflare

La API usa por ahora los almacenes de archivos TOML/JSON que ya entiende el núcleo. En Docker, `/var/lib/bittopos` debe estar en un volumen persistente. Cloudflare Pages sirve la SPA y el servidor Rust se ejecuta en Wispbyte. R2/D1 aún no son el almacén principal: la persistencia actual del núcleo usa rutas locales, por lo que conectar almacenamiento remoto requiere una adaptación explícita del contrato de persistencia. No se debe montar R2 como si fuera un disco POSIX ni declarar la aplicación lista para datos remotos antes de probar esa adaptación.

## Seguridad conocida

Esta es una barrera sencilla para una instancia de desarrollo de un solo administrador. No hay cuentas, permisos por proyecto, protección CSRF dedicada, limitación de intentos ni persistencia de sesiones. Usa HTTPS, contraseña aleatoria y `SESSION_COOKIE_SECURE=true` en Wispbyte detrás de Cloudflare. No expongas el servicio sin un proxy HTTPS y sin revisar las limitaciones anteriores.
