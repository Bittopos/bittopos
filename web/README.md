# Bittopos Web

La SPA estática usa HTML, CSS y módulos ES nativos. Su licencia es [Mozilla Public License 2.0](LICENSE) (MPL-2.0). El servidor Rust se encuentra en [server/](../server/) y su configuración se documenta en [server/README.md](../server/README.md).

## Ejecutar localmente

1. Configura las variables de entorno copiando `server/.env.example` a `.env`. Define una contraseña aleatoria de al menos 12 caracteres.
2. Desde la raíz del repositorio, ejecuta `cargo run -p bittopos-server`.
3. Sirve la carpeta `web/` en el origen configurado en `WEB_ORIGIN`. Por ejemplo, con Python 3: `python3 -m http.server 8787 --directory web`. Ajusta `WEB_ORIGIN=http://localhost:8787` y `SESSION_COOKIE_SECURE=false` solo para desarrollo local.
4. Abre el origen de la SPA e inicia sesión con las credenciales del servidor.

Tras iniciar sesión, la SPA utiliza el primer perfil y proyecto disponibles. Si todavía no existen, crea un perfil y un proyecto inicial para que puedas comenzar a probar el flujo.

## Comprobaciones

Desde la raíz del repositorio:

```bash
npm --prefix web run check
cargo fmt --all -- --check
cargo test --workspace --locked
```

## Estado y límites

- Las solicitudes incluyen la cookie de sesión; el servidor establece una cookie HttpOnly.
- La API permite listar y crear perfiles, crear/listar proyectos, listar/crear/editar recursos e importar TOML, y leer/guardar la estructura con control de revisión.
- La interfaz todavía no presenta un selector para cambiar entre múltiples perfiles o proyectos.
- La autenticación actual es una barrera de desarrollo de un solo administrador. Las sesiones se pierden al reiniciar el servidor y no hay permisos por proyecto ni protección CSRF dedicada.
- SQLite local es la persistencia actual. Cloudflare R2/D1 no está conectado todavía.

La SPA no debe considerarse lista para producción hasta completar la autorización multiusuario, la revisión de seguridad y la estrategia de persistencia remota.
