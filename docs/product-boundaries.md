# Límites entre Workspace, Cloud y la administración multi-tenant

## Visión de producto

Bittopos debe poder instalarse y utilizarse sin depender de Bittopos Cloud. La oferta alojada debe eliminar el trabajo operativo de desplegar un servidor, configurar redes y conectar dispositivos. El valor comercial es la operación gestionada, la disponibilidad, las copias de seguridad, el acceso entre dispositivos y las funciones de equipo, no el bloqueo artificial del núcleo.

Hay tres conceptos distintos:

1. **Bittopos Workspace**: la interfaz con la que una persona utiliza Bittopos.
2. **Bittopos Server**: el servicio que ejecuta los casos de uso, expone la API y aplica las reglas de acceso.
3. **Bittopos Cloud**: el plano de control comercial que gestiona identidades, organizaciones, suscripciones y aprovisionamiento de espacios alojados.

Los nombres son nombres de trabajo de arquitectura y producto; pueden revisarse antes de publicar la marca.

## Experiencias de uso

### Instalación propia (self-hosted)

Una persona clona o descarga el repositorio público, compila Workspace y/o la CLI, y ejecuta Bittopos Server en su propio equipo o infraestructura. Puede crear un espacio de trabajo local y, cuando estén implementadas, conectar varios dispositivos mediante un servidor que administre.

Esta modalidad no debe exigir una cuenta en Bittopos Cloud ni una suscripción. Debe documentar sus requisitos reales de red, persistencia, copias de seguridad y seguridad.

### Bittopos Cloud

Una persona visita el portal alojado, inicia sesión, crea una organización o acepta una invitación y solicita un espacio de trabajo alojado. Cloud aprovisiona los recursos y asocia el espacio con la organización. Workspace consume la API del servidor correspondiente.

La interfaz Workspace puede servirse desde Cloudflare Pages, pero la API y la persistencia remota son servicios separados. El navegador no debe conectarse directamente a la base de datos.

### Varios dispositivos

El usuario puede abrir Workspace desde distintos dispositivos y conectarse al mismo espacio remoto. Para esto, el servidor mantiene los datos compartidos y aplica identidad, autorización y control de concurrencia. No debe requerirse que el usuario abra puertos en su hogar ni configure manualmente una VPN para la modalidad alojada.

La modalidad autogestionada puede necesitar configuración de red si se accede desde fuera de la red local; no debe prometerse conectividad remota automática sin implementar un mecanismo específico.

## Repositorios propuestos

### Repositorio público: `bittopos`

Contiene el software instalable y las interfaces abiertas:

- Núcleo Rust y reglas semánticas canónicas.
- CLI con modo local y, cuando se implemente, modo remoto.
- Workspace web y sus clientes de API.
- Bittopos Server genérico, portable y autogestionable.
- Contratos de API, migraciones públicas de esquema, pruebas, documentación y archivos de contenedor.
- Automatización CI para construir y probar los componentes públicos.

No debe contener credenciales, configuración secreta de producción ni lógica exclusiva de administración comercial.

### Repositorio privado: `bittopos-cloud`

Contiene el plano de control comercial y las interfaces de administración alojada:

- Portal de autenticación y selección de organizaciones.
- Gestión de membresías y roles de la plataforma.
- Planes, suscripciones, límites y facturación.
- Aprovisionamiento y suspensión de espacios alojados.
- Administración operativa privilegiada y auditoría de acciones administrativas.
- Integraciones privadas con proveedores de pago, correo y servicios de infraestructura.

El portal no debe ser la única capa que impide el acceso no autorizado. Los servicios de datos deben validar credenciales, membresía, permisos y alcance en cada solicitud protegida.

### Repositorio privado opcional: `bittopos-deploy`

Puede incorporarse cuando la operación lo justifique para alojar configuración de despliegue de producción, infraestructura como código y pipelines operativos. Los secretos se guardan en un gestor de secretos o en el almacén de secretos de CI, nunca en el repositorio.

La visibilidad privada de un repositorio no es una frontera de seguridad en tiempo de ejecución. La protección real depende de credenciales, permisos, aislamiento de datos y controles del servidor.

## Modelo de identidad y multi-tenancy

Mantener separados estos conceptos en el modelo:

- **Usuario**: identidad autenticada de una persona.
- **Organización**: grupo de usuarios, membresías y políticas.
- **Espacio de trabajo**: contenedor lógico de datos de Bittopos.
- **Membresía**: relación entre usuario y organización, con un rol.
- **Instancia alojada**: recurso operativo donde se ejecuta un servidor o se almacenan los datos de uno o más espacios.
- **Suscripción/plan**: relación comercial con cuotas y derechos de uso.

No asumir que una organización, un espacio y una instancia alojada son la misma entidad. Una organización puede tener varios espacios, y la infraestructura puede alojar múltiples organizaciones con aislamiento lógico o físico según el plan y el riesgo.

El contexto de organización/espacio debe derivarse de una identidad verificada y de permisos comprobados por el servidor. Un identificador de tenant enviado por el cliente solo selecciona el contexto solicitado; no concede acceso. Todas las rutas de lectura y escritura deben comprobar autorización y aislamiento.

## Autenticación y acceso

- Preferir un proveedor de identidad o un protocolo estándar como OpenID Connect cuando sea apropiado, en vez de crear un sistema casero de contraseñas y tokens.
- El portal de Cloud gestiona el flujo de inicio de sesión, organizaciones y planes.
- Workspace utiliza un flujo de autenticación adecuado al tipo de cliente y obtiene acceso limitado al API.
- El servidor verifica credenciales, audiencia, expiración, permisos y membresía antes de ejecutar operaciones protegidas.
- La CLI remota utiliza un flujo adecuado para dispositivos de línea de comandos; no debe pedir ni almacenar la contraseña principal del usuario.
- La instalación local puede funcionar sin identidad Cloud. Si necesita colaboración remota, el propietario configura sus propias identidades, invitaciones y políticas locales.
- Las funciones administrativas de la plataforma requieren privilegios separados, auditoría y controles más estrictos que una membresía normal de organización.

## Límites de responsabilidad

### Workspace

Responsable de presentación, accesibilidad, navegación, edición y validaciones de experiencia. No es autoridad para conceder permisos ni decidir el estado comercial de una cuenta.

### Núcleo

Responsable de las reglas semánticas de Bittopos. No depende de Cloud, de un proveedor de identidad ni de la facturación.

### Server

Responsable de aplicar las reglas del núcleo, validar entradas, autenticar solicitudes remotas, autorizar operaciones, aplicar el alcance de organización/espacio, coordinar persistencia y proteger los datos.

### Cloud Control Plane

Responsable de las cuentas comerciales, organizaciones, suscripciones, cuotas y ciclo de vida de los servicios alojados. Debe comunicarse con los servidores de datos mediante interfaces autenticadas y privilegiadas de forma explícita.

## Separación de datos y seguridad

El aislamiento multi-tenant debe formar parte del diseño de persistencia y autorización, no ser un filtro visual de la interfaz. Cada operación tenant-scoped debe comprobar el tenant del recurso y los permisos del usuario. Los trabajos asíncronos, cachés, archivos, logs y copias de seguridad también deben respetar ese alcance.

El modelo de despliegue inicial puede ser un monolito modular con una base de datos compartida y límites de tenant verificados. No hace falta microservicios para empezar. Si el producto crece, se pueden ofrecer niveles de aislamiento más fuertes sin cambiar la semántica del núcleo.

## Orden recomendado de implementación

1. Definir el modelo de usuario, organización, espacio de trabajo y permisos en un documento ADR.
2. Establecer el servidor público como API portable que invoca el núcleo Rust.
3. Definir autenticación/autorización en la API y probar explícitamente que un tenant no puede acceder a los datos de otro.
4. Mantener Workspace desacoplado del portal y capaz de conectar a un servidor local o remoto.
5. Implementar el plano de control privado para creación de organizaciones, aprovisionamiento y suscripciones.
6. Añadir sincronización CLI y experiencias móviles/escritorio una vez definidos identidad, revisiones y conflictos.

Esta arquitectura describe el destino deseado, no funcionalidades ya implementadas. El servidor Python/SQLite actual sigue siendo experimental y no debe exponerse como servicio multiusuario de producción.
