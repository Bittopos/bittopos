# Bittopos: arquitectura base Unix-first

**Estado:** propuesta fundacional para revisión  
**Alcance:** reemplaza la organización experimental actual; no promete retrocompatibilidad con APIs, formatos, CLI ni esquemas que aún estén en diseño.

## Decisión central

Bittopos se organizará como un **monorepositorio con un workspace de Rust y aplicaciones de plataforma**, contratos versionados en el propio árbol y una única fuente canónica del dominio. El repositorio `Bittopos/bittopos` será la fuente de verdad para código, pruebas, especificaciones y releases. Los repositorios separados actuales no deben seguir conteniendo copias independientes de módulos que evolucionan juntos; se migrarán cuando exista una implementación nueva comprobable y después se archivarán o convertirán en repositorios de distribución según corresponda. No se borrará ni archivará nada automáticamente durante esta etapa.

La razón no es que un monorepo sea universalmente mejor: es que Bittopos comparte tipos, semántica de dominio, protocolos y contratos entre núcleo, servidor, CLI y clientes. Esos elementos necesitan cambios atómicos, pruebas conjuntas y releases coordinadas. Separar repositorios ahora impone coste de sincronización sin aportar todavía límites de producto suficientemente estables.

## Objetivo del sistema

Un único modelo de producto, operado a través de interfaces diferentes:

- Usuario cotidiano: interfaz móvil/escritorio/web sencilla, segura y limitada por las capacidades del dispositivo.
- Operador: CLI, API administrativa, diagnósticos, configuración y automatización.
- Administrador de sistemas: inventario de hardware/software visible desde el host, estado de servicios, recursos, dependencias, eventos y trazabilidad.
- Despliegue: ejecución local, servicio de servidor o instalación gestionada, usando el mismo dominio y contratos.

Bittopos administra productos y su estructura. El host que ejecuta Bittopos es una entidad distinta del producto administrado; no se deben confundir los recursos de la máquina con los componentes inventariados del producto.

## Estructura de destino

```text
bittopos/
├── Cargo.toml                  # workspace y política común de dependencias
├── Cargo.lock
├── crates/
│   ├── bittopos-domain/        # entidades, valores, invariantes, reglas puras
│   ├── bittopos-application/   # casos de uso y coordinación
│   ├── bittopos-contracts/     # DTO, errores, eventos y contratos públicos
│   ├── bittopos-ports/         # interfaces requeridas por los casos de uso
│   ├── bittopos-storage/       # persistencia y transacciones
│   ├── bittopos-inventory/     # modelo de inventario y procedencia de observaciones
│   ├── bittopos-observability/ # métricas, trazas y salud de aplicación
│   ├── bittopos-runtime/       # configuración, ciclo de vida, apagado y señales
│   ├── bittopos-server/        # API, autenticación y autorización de red
│   └── bittopos-cli/           # CLI de usuario y administración
├── apps/
│   ├── web/                    # cliente web; consume contratos/API
│   ├── desktop/                # shell de escritorio si aporta valor
│   └── android/                # cliente Android y adaptadores Android
├── platforms/
│   ├── unix-common/            # comportamiento portable POSIX cuando aplique
│   ├── linux/                  # systemd, rutas, capacidades y empaquetado Linux
│   └── freebsd/                # rc.d, pkg, rutas y capacidades FreeBSD
├── deploy/
│   ├── containers/
│   ├── linux/
│   └── freebsd/
├── schemas/                    # OpenAPI, eventos y formatos publicados
├── tests/
│   ├── contracts/
│   ├── integration/
│   └── platform/
├── docs/
│   ├── architecture/
│   ├── decisions/
│   ├── operations/
│   └── development/
└── tools/
```

Los directorios son límites conceptuales iniciales, no una obligación de crear crates vacíos. Cada crate se crea cuando existe una responsabilidad y una frontera de dependencia verificables.

## Reglas de dependencia

1. `domain` no depende de SO, red, UI, base de datos ni framework web.
2. `application` depende del dominio y de contratos/puertos; no conoce adaptadores concretos.
3. Adaptadores implementan puertos. El ensamblado de dependencias ocurre en los puntos de entrada.
4. `server`, `cli`, `web`, `desktop` y `android` no reimplementan reglas de negocio.
5. La persistencia es un adaptador. El dominio no contiene SQL ni tipos del motor.
6. El inventario distingue **observaciones** de **hechos de dominio**. Cada observación de host incluye fuente, instante, alcance y nivel de confianza; la ausencia de permiso se informa como desconocido/no accesible, nunca como inexistencia.
7. Las capacidades se declaran explícitamente: no se asume que Android, Linux y FreeBSD puedan inspeccionar o administrar los mismos recursos.
8. No se crean capas genéricas por anticipado. Una abstracción existe cuando delimita una variación real, una frontera de seguridad o una responsabilidad estable.

## Hardware, sistema operativo y usuarios

Bittopos distinguirá al menos:

- **Producto administrado:** componentes, relaciones, versiones, dependencias y estado funcional.
- **Host de ejecución:** hardware, sistema operativo, procesos, servicios, almacenamiento y red que alojan Bittopos.
- **Agente o fuente de observación:** mecanismo con permisos y alcance definidos para recopilar datos.
- **Usuario/operador:** identidad, permisos y acciones permitidas.

La recolección de inventario será modular y de privilegio mínimo. La aplicación normal no requiere privilegios de root/administrador. Operaciones privilegiadas se separan en adaptadores o agentes estrechos, con protocolo explícito y superficie de ataque limitada.

Las métricas de infraestructura (CPU, memoria, disco, red, salud de proceso) no se mezclan con métricas de producto (usuarios/s, operaciones, solicitudes, acciones). Ambas pueden correlacionarse mediante tiempo e identificadores de instancia, manteniendo fuentes y semánticas distintas.

## Portabilidad como contrato verificable

Linux será la primera plataforma de referencia de servidor; FreeBSD será una plataforma objetivo explícita; Android será un cliente de plataforma con restricciones propias. No se declarará soporte por compilar únicamente.

Cada plataforma define capacidades comprobables y pruebas para:

- rutas de configuración, datos, caché, temporales y logs;
- identidad de servicio y permisos;
- inicio, señales, apagado limpio y recuperación;
- almacenamiento, bloqueos y concurrencia;
- red, límites de recursos y restricciones del entorno;
- recopilación de inventario disponible y permisos necesarios;
- instalación, actualización, desinstalación y restauración.

Las diferencias de systemd/rc.d/Android permanecen en adaptadores, no en el dominio.

## Persistencia y cambios incompatibles

La arquitectura está en fase experimental. Se permite reemplazar completamente los contratos y esquemas iniciales cuando el diseño lo justifique. No se implementarán capas de compatibilidad por defecto ni se preservarán APIs/CLI/esquemas experimentales por inercia.

Cada formato persistente tendrá una versión explícita y una política de creación, validación, copia/restauración y reinicialización. Durante esta fase, un cambio incompatible puede requerir crear una base nueva o ejecutar una migración deliberada documentada; no se promete compatibilidad automática con datos experimentales. Antes de una operación destructiva se debe advertir claramente y evitar pérdida silenciosa de datos.

## Contratos y seguridad

- OpenAPI para HTTP; esquemas explícitos para eventos y formatos persistentes.
- Errores tipados y estables dentro de una versión de contrato.
- Autenticación y autorización en el servidor, separadas de las reglas del dominio.
- La CLI local y la API remota llaman a los mismos casos de uso cuando ejecutan la misma operación.
- Registro de auditoría separado del panel de estado para usuarios.
- Secretos fuera del repositorio y de los logs.
- Sin ejecución de comandos arbitrarios del host desde la API.

## Verificación y releases

La integración continua deberá comprobar formato, lint, pruebas unitarias, pruebas de contratos, integración de almacenamiento y compilación por plataforma. La matriz se ampliará según soporte real, incluyendo targets Linux y FreeBSD donde el runner esté disponible, y Android en su toolchain.

No se fusionará una reestructuración masiva sin:
1. mapa de responsabilidades y dependencias;
2. contratos e invariantes definidos;
3. pruebas de comportamiento del dominio;
4. instalación limpia reproducible;
5. comprobaciones de permisos y recuperación;
6. documentación de cambios incompatibles.

## Plan de ejecución

1. Inventariar el código actual y clasificarlo por responsabilidad; detectar copias divergentes.
2. Fijar el modelo de dominio y los contratos nuevos antes de mover código.
3. Crear el workspace canónico y mover/reimplementar por responsabilidad, no por nombre de archivo.
4. Rehacer servidor, CLI y UI para consumir los mismos casos de uso/contratos.
5. Definir el ciclo de vida del proceso y las rutas de sistema como contratos de runtime.
6. Incorporar adaptadores Linux y FreeBSD y pruebas de plataforma.
7. Diseñar inventario y observabilidad con límites de permisos.
8. Incorporar Android y empaquetados de usuario cuando el núcleo/contratos estén definidos.
9. Convertir los repositorios separados en artefactos derivados, documentación histórica o repositorios archivados; nunca mantener copias editables del mismo código.

## Criterio de éxito

La arquitectura se considera adecuada cuando una nueva interfaz o plataforma se incorpora principalmente implementando sus adaptadores y consumiendo contratos existentes, sin reescribir el dominio; cuando el host revela solo las capacidades que puede observar con sus permisos; y cuando las operaciones locales, remotas y administrativas comparten la misma semántica.
