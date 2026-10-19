# Estructura objetivo del repositorio

## Propósito

Bittopos es un solo producto con cuatro componentes ejecutables o reutilizables: núcleo, CLI, servidor y web. Se mantienen en un monorepositorio para compartir contratos, pruebas y automatización, pero cada componente debe tener límites explícitos y poder construirse y publicarse de manera independiente cuando corresponda.

La estructura de Docker sirve como referencia organizativa: el código se agrupa por responsabilidad, mientras que documentación, pruebas, herramientas de desarrollo y empaquetado tienen lugares reconocibles. No se pretende copiar sus directorios literalmente.

## Estructura objetivo

Esta es la estructura hacia la que migraremos gradualmente. No implica que todos estos directorios o componentes estén implementados hoy.

```text
bittopos/
├── .github/
│   └── workflows/          # CI, publicación de artefactos y despliegue
├── apps/
│   ├── cli/                # Interfaz de terminal y modos local/remoto
│   ├── server/              # API HTTP; adapta solicitudes a casos de uso del núcleo
│   └── web/                 # SPA; presentación y cliente de la API
├── crates/
│   └── bittopos-core/       # Modelo de dominio, semántica y casos de uso
├── contracts/
│   └── api/                 # Contrato versionado de API (cuando se estabilice)
├── deploy/
│   ├── compose/             # Entorno local y despliegue portable del servidor
│   └── cloudflare/          # Configuración específica de publicación de la SPA
├── docs/
│   ├── architecture/        # Decisiones y diagramas arquitectónicos
│   └── ...                  # Modelo, visión y guías existentes
├── tests/
│   ├── integration/         # Pruebas entre componentes
│   └── contracts/           # Compatibilidad de API y formatos
├── scripts/                 # Automatización repetible para desarrollo y CI
├── Cargo.toml               # Workspace Rust
└── README.md
```

## Responsabilidades y dependencias permitidas

### Núcleo — `crates/bittopos-core`

- Es la fuente canónica de las reglas del modelo y de la resolución semántica.
- Conserva las identidades distintas de recursos e instancias, la organización de instancias, las máscaras y las reglas de relaciones y referencias.
- No depende de HTTP, bases de datos remotas, JavaScript, interfaces de usuario ni detalles de despliegue.
- Expone operaciones reutilizables a través de una API Rust estable dentro del workspace.

### CLI — `apps/cli`

- En modo local llama al núcleo directamente y usa adaptadores de persistencia local.
- En modo remoto actúa como cliente HTTP y utiliza el contrato público del servidor.
- No implementa una segunda versión de la semántica ni accede directamente a la base de datos remota.
- Los futuros comandos de sincronización deben añadirse después de definir revisiones, conflictos e idempotencia.

### Servidor — `apps/server`

- Es el adaptador de entrada para HTTP y JSON.
- Convierte solicitudes en operaciones del núcleo y convierte resultados/errores en respuestas de API.
- Es responsable de autenticación, autorización, límites de abuso, configuración, observabilidad y coordinación de persistencia remota.
- No debe mantener una implementación alternativa de la resolución semántica.
- Debe poder empaquetarse como contenedor sin depender de un proveedor de infraestructura concreto.

### Web — `apps/web`

- Es responsable de presentación, estado visual, accesibilidad, navegación y validaciones de entrada útiles para el usuario.
- Usa la API HTTP para leer y modificar datos compartidos; no accede directamente a la base de datos ni es autoridad sobre reglas de dominio.
- Puede ejecutarse localmente contra un servidor local o desplegarse separada en Cloudflare Pages contra una API remota.
- La validación cliente mejora la experiencia, pero el servidor vuelve a validar toda operación no confiable.

### Contratos, pruebas y despliegue

- Los contratos HTTP se versionan y se verifican para evitar que CLI, web y servidor diverjan.
- Las pruebas unitarias del núcleo viven junto al código Rust; las pruebas de integración y contrato cubren los límites entre componentes.
- La configuración y los secretos se inyectan en ejecución. No se incluyen credenciales en el repositorio.
- La configuración de despliegue se separa del código de aplicación para mantener portable el servidor.

## Límites de confianza

El navegador y la CLI remota son clientes no confiables. El servidor autentica y autoriza cada operación remota y el núcleo aplica las reglas del dominio. Una validación de JavaScript nunca sustituye una validación del servidor.

En modo local, la CLI puede ejecutar el núcleo sin red. La persistencia local y la remota son adaptadores diferentes; no deben crear semánticas distintas. Si una operación requiere capacidades exclusivas del servidor —por ejemplo, autorización multiusuario o coordinación de escrituras compartidas— debe declararse como tal, en lugar de simular que existe en modo local.

## Migración desde el estado actual

La organización actual se conserva durante esta primera etapa:

- `src/` contiene el paquete Rust del núcleo.
- `cli/` contiene el binario `bitto`.
- `web/` contiene la SPA.
- `backend/` contiene el adaptador experimental Python/SQLite; todavía no es el servidor Rust definitivo.
- `docs/` contiene la documentación vigente.

No se deben mover directorios ni cambiar manifiestos de compilación en la misma entrega que establece los límites arquitectónicos. La migración se hará en pasos verificables:

1. Documentar los límites y la estructura destino.
2. Convertir el núcleo en un crate explícito del workspace sin cambiar sus reglas ni su comportamiento.
3. Crear el servidor Rust como adaptador del núcleo y definir sus pruebas de integración.
4. Trasladar la CLI y la web a la organización `apps/`, manteniendo los comandos y el comportamiento existentes.
5. Definir contrato de API, persistencia remota y sincronización en cambios posteriores, con pruebas de compatibilidad.

Cada paso debe tener su propia rama o PR revisable. `main` no se modifica directamente y no se fusiona ninguna propuesta sin revisión explícita.
