# Persistencia SQLite

## Propósito

El núcleo Rust expone `Database` como almacenamiento estructurado opcional. Puede ser abierto por un servidor nativo o por una herramienta local; no cambia la autoridad semántica del núcleo ni elimina los archivos que el usuario administra.

## Esquema inicial

- `profiles`: identidad del perfil y documento JSON de metadatos.
- `projects`: metadatos, perfil propietario, estructura JSON y revisión.
- `project_resources`: recursos ligados a un proyecto, con columnas consultables para título, descripción y etiquetas, documento JSON y texto TOML original cuando se importa.
- `project_instances`: identidad de instancia, instancia padre, ID del recurso fuente, datos semánticos, máscaras y documento JSON de cada instancia.

Las claves foráneas evitan proyectos huérfanos y las instancias se proyectan dentro del alcance de un proyecto. Las escrituras de estructura se ejecutan en una transacción y exigen una revisión esperada para rechazar modificaciones obsoletas.

## Formatos y compatibilidad

TOML sigue siendo un formato de definición editable y transportable; JSON sigue representando estructuras y documentos de intercambio. La base de datos guarda documentos completos además de columnas relacionales para consultas. La importación conserva el texto TOML original; no se reconstruye ni se sobrescribe automáticamente un archivo fuente a partir de columnas normalizadas.

La base de datos no importa automáticamente el almacén local existente. La migración de perfiles, proyectos, recursos y estructuras debe ser un flujo explícito con validación y respaldo, para no convertir una apertura de base de datos en una operación destructiva.

## Uso

El núcleo exporta `Database` y `DatabaseError`. La aplicación puede abrir una base con `Database::open(ruta)`, crear perfiles/proyectos, guardar recursos JSON con TOML original opcional y persistir estructuras por revisión. Cada conexión debe mantenerse dentro de una tarea o protegerse con sincronización si se comparte entre hilos.

La implementación enlaza con la biblioteca nativa SQLite (`libsqlite3`). El entorno de compilación y el de ejecución deben disponer de esa biblioteca. La ubicación de la base y los permisos de acceso los determina el proceso que la abre.

## Límites de esta iteración

- No se han migrado los archivos locales existentes.
- El servidor HTTP Rust todavía debe invocar esta capa de persistencia; el adaptador Python experimental no se convierte en el núcleo.
- No se implementan todavía autenticación, autorización multiusuario, sincronización CLI ni una política de exportación automática a archivos.
- El llamador debe aplicar las reglas de autenticación/autorización antes de consultar o modificar datos privados.
