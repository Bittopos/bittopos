# Visión de Bittopos

Bittopos es un sistema de **estructuración semántica, navegación y control de ejecución**. Permite que las personas describan cómo se organiza la información y cómo se interpreta, sin imponer a todos los dominios una plantilla rígida.

## Núcleo conceptual

El sistema distingue recursos reutilizables, instancias organizadas en una estructura, segmentos compartidos o locales, resolución semántica y relaciones explícitas. Las máscaras expresan diferencias locales sin convertir cada instancia en una copia independiente del recurso.

La estructura es una jerarquía navegable que expresa pertenencia. Las relaciones forman una red semántica transversal que expresa vínculos entre recursos o instancias. «Se alimenta de», «acicala a», «utilizado por» y «depende de» son ejemplos de etiquetas posibles, no tipos codificados exclusivamente para un dominio. Las relaciones no sustituyen ni alteran la contención.

Esta combinación hace que el modelo sea general: puede describir software, planes de estudio, sistemas ecológicos y actividades cotidianas. Los dominios aportan sus conceptos; Bittopos aporta identidad, organización, herencia, resolución y vínculos.

## Principios de producto

- La estructura y la red de relaciones son vistas complementarias de un mismo sistema.
- Las definiciones compartidas, las instancias, las diferencias locales y los datos efectivos son dimensiones distintas.
- La interfaz debe revelar estas distinciones y no crear una semántica paralela.
- El estado visible de trabajo y el registro de auditoría son mecanismos separados, con públicos y propósitos distintos.
- Las preferencias de notificación pertenecen al usuario y deben poder gestionarse por la interfaz normal o por API.
- La interfaz de trabajo con estructuras y la interfaz de cuentas se separan visualmente y en sus responsabilidades de seguridad.
- Bittopos no es un tablero Kanban; puede aprender de patrones de interfaz de otras aplicaciones, adaptándolos a su modelo jerárquico y semántico.

## Una estructura, muchas vistas

Un árbol de directorios, un diagrama, un mapa conceptual y un árbol radial pueden mostrar distintos aspectos de la misma estructura. La imagen de especies en blanco y negro con zoom circular se entiende como una jerarquía tipo filesystem Linux cuya disposición visual se ha doblado en forma de círculo: la geometría cambia, pero la estructura lógica no.

Bittopos debe tratar estas representaciones como vistas o proyecciones de un modelo semántico compartido, no como modelos independientes. Cambiar de vista no debe duplicar elementos ni cambiar su identidad. El árbol hace visibles la contención y los niveles; el mapa o diagrama puede enfatizar vecindad, relaciones o caminos. Cada arista debe comunicar si representa contención, referencia o relación tipada. El zoom y el enfoque son operaciones de visualización, no cambios en la jerarquía.

## Dirección de desarrollo

El núcleo semántico Rust es la autoridad de las reglas. La dirección elegida para la siguiente etapa es integrar la SPA y el núcleo mediante una API versionada, completar la refactorización del frontend por responsabilidades y establecer persistencia remota. La API experimental Python/SQLite actual todavía no integra la resolución Rust ni está preparada para un servicio multiusuario público.

La arquitectura debe permitir que la interfaz web y la CLI actúen sobre reglas coherentes. No se debe duplicar la semántica en clientes o adaptadores. La seguridad, las cuentas, el aislamiento de datos, la concurrencia, la auditoría y las migraciones deben definirse antes de abrir el servicio remoto a usuarios reales.

El trabajo avanza en incrementos revisables, con contratos compartidos, pruebas y documentación que distingue explícitamente entre comportamiento implementado y objetivo. La explicación conectada de producto y arquitectura está en [PRODUCT-CONTEXT.md](PRODUCT-CONTEXT.md); las reglas normativas permanecen en [model.md](model.md).
