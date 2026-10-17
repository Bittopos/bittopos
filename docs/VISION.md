# Visión de Bittopos

Bittopos es un sistema de **estructuración semántica y control de ejecución**. Permite que las personas definan cómo se organiza la información y cómo se interpreta, en vez de imponerles una estructura automática.

## Núcleo conceptual

El sistema distingue recursos reutilizables, instancias organizadas en una estructura, segmentos compartidos o locales y una resolución semántica jerárquica. Las máscaras expresan diferencias locales sin convertir cada instancia en una copia independiente del recurso.

## Dirección de desarrollo

La prioridad actual es un núcleo local coherente, fácil de comprender y modificar. La CLI es la interfaz operativa principal. Las interfaces web y móviles, los servicios remotos y el despliegue en proveedores externos son extensiones futuras, no requisitos del núcleo actual.

La arquitectura debe permitir incrementos pequeños: modelo, resolución, operaciones, persistencia e interfaz deben tener responsabilidades reconocibles, sin añadir capas por anticipación.
