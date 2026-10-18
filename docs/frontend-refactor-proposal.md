# Propuesta de refactorización del frontend

## Estado observado

- `web/index.html`, `web/styles.css` y `web/app.js` forman una SPA estática sin proceso de compilación.
- La interfaz tiene árbol de instancias, vista de estructura/mapa, inspector, búsqueda, creación de recursos, importación/exportación y persistencia del navegador.
- La persistencia actual usa `localStorage` con la clave `bittopos.web.prototype.v1`.
- El núcleo operativo es Rust (`bittopos-core`) y la CLI `bitto`; la SPA no invoca ese núcleo ni lee directamente sus archivos.
- El modelo distingue recursos TOML, estructura JSON, instancias, máscaras, relaciones, referencias y resolución semántica. La web no debe reinterpretar esas reglas por su cuenta.

## Patrones de producto que conviene preservar

1. **Definiciones reutilizables:** una definición de recurso puede tener múltiples instancias.
2. **Organización explícita:** la estructura organiza instancias; no crear una segunda entidad paralela llamada workspace.
3. **Identidades separadas:** ID del recurso e ID de la instancia no son intercambiables.
4. **Resolución semántica centralizada:** las diferencias locales y la herencia pertenecen al núcleo, no a componentes visuales.
5. **Relaciones y referencias explícitas:** no inventar componentes por defecto ni convertirlos en contención.
6. **Flujo local y exportable:** la SPA actual puede seguir siendo útil como prototipo offline, pero debe identificar claramente que sus datos son locales y no están sincronizados con Rust.

## Riesgos y deuda técnica

- `web/app.js` combina estado, persistencia, normalización, parsing TOML, renderizado y operaciones de interfaz en un único archivo.
- La normalización de instancias reconstruye objetos con un conjunto reducido de campos. Antes de conectar la web al núcleo, hay que comprobar con fixtures que máscaras, referencias y otros campos semánticos no se pierdan durante importación/exportación.
- El parser TOML del navegador es deliberadamente parcial; no debe presentarse como parser general ni como validación canónica del formato TOML.
- Los datos de `localStorage` están aislados por navegador y origen. No son almacenamiento compartido, sincronización ni copia de seguridad.
- No existe actualmente una compilación WASM del núcleo ni un contrato de API web.

## Arquitectura propuesta

### Fase 1 — Base estática verificable (esta rama)

- Conservar HTML, CSS y JavaScript nativo: no hay una necesidad demostrada que justifique añadir un framework o bundler.
- Definir ejecución local y despliegue de Cloudflare Pages como sitio estático con directorio de salida `web/` y sin comando de build.
- Añadir comprobaciones de sintaxis y smoke checks en CI.
- Declarar explícitamente el límite entre prototipo local y núcleo Rust.

### Fase 2 — Separación interna, sin cambiar el modelo

Separar el código por responsabilidad en módulos ES pequeños:

- `domain/`: tipos/validaciones de datos que reflejan contratos documentados; no duplicar resolución semántica.
- `application/`: casos de uso de la interfaz.
- `adapters/local-storage/`: persistencia local versionada.
- `adapters/files/`: importación/exportación y validación de formatos.
- `ui/`: renderizado y eventos.
- `main.js`: composición e inicio.

Hacerlo en incrementos pequeños con pruebas de caracterización sobre las funciones actuales. No cambiar a la vez el formato de datos y la organización del código.

### Fase 3 — Contrato con el núcleo

Elegir explícitamente una vía cuando exista un caso de uso real:

1. **WASM:** compilar una API pública y pequeña de Rust, exponer operaciones tipadas y mantener la resolución en el núcleo. Requiere definir serialización, errores y estrategia de build.
2. **API remota:** solo cuando se necesite compartir datos entre usuarios/dispositivos. Requiere autenticación, autorización, persistencia y un modelo de sincronización.

No implementar ambas vías preventivamente. Cloudflare Pages puede servir los archivos estáticos; el hecho de alojar la SPA allí no conecta por sí mismo el núcleo Rust.

## Reglas de implementación

- El modelo vigente en `docs/model.md` y la API pública de Rust son la autoridad.
- No descartar campos desconocidos o semánticos silenciosamente durante importación/exportación.
- Validar archivos antes de sustituir el estado activo; mostrar errores accionables.
- Mantener las identidades de recurso e instancia separadas.
- Las operaciones que cambien varias representaciones deben evitar estados parciales en la interfaz; no afirmar que esto resuelve la limitación transaccional del almacenamiento del núcleo.
- Usar DOM APIs y `textContent` para texto importado; no insertar metadatos externos como HTML.
- Toda funcionalidad nueva debe tener una comprobación automatizada o pasos de verificación reproducibles.

## Criterios de aceptación

- La web se sirve como archivos estáticos sin credenciales ni variables secretas.
- CI comprueba sintaxis JavaScript y estructura mínima de los assets.
- La interfaz anuncia claramente que el almacenamiento es local y no sincronizado.
- Fixtures de importación/exportación preservan todos los campos admitidos antes de conectar la SPA al núcleo.
- La integración con Rust solo se considera terminada cuando las operaciones semánticas se ejecutan en el núcleo y existen pruebas de contrato.
