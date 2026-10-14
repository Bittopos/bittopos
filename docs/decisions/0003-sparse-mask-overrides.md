# ADR 0003: máscaras dispersas y herencia dinámica

- **Estado:** aceptado conceptualmente; detalles operativos pendientes
- **Fecha:** 2026-10-14
- **Sustituye:** [ADR 0002](0002-segment-masks.md)

## Contexto

La primera formulación de las máscaras trataba la máscara como una copia del estado de un segmento en el momento de su creación. Esa interpretación congela valores que no han sido modificados localmente y hace que una instancia deje de reflejar cambios posteriores en la definición incluso cuando no existe una razón local para hacerlo.

La regla acordada más reciente es mantener la definición compartida como origen vigente y registrar en la instancia únicamente las diferencias locales.

## Decisión

- Una máscara representa diferencias locales de un segmento definido; no es una instantánea completa.
- La creación de una máscara no copia todos los valores heredados como estado local. Solo se registran las diferencias locales.
- Un valor sin modificación local se resuelve desde la definición actual del recurso.
- Los cambios posteriores en la definición se reflejan automáticamente en los valores no modificados localmente.
- Editar un valor heredado sin máscara puede modificar directamente la definición compartida y afectar a otras instancias.
- Editar un valor mediante una máscara crea o modifica una diferencia local de esa instancia y no muta la definición compartida.
- Las máscaras se aplican por segmento y no convierten toda la instancia en una copia privada.
- Los permisos se mantienen conceptualmente separados del contenido heredado. Este ADR no define la autorización efectiva ni su precedencia.

## Consecuencias

- La resolución debe combinar la definición vigente con las diferencias locales de la instancia.
- El modelo debe distinguir de manera estable los elementos de la definición y sus posibles modificaciones locales.
- Los cambios de definición deben preservar las modificaciones locales, sin congelar por accidente los valores heredados.
- La semántica de máscaras debe implementarse en un único lugar compartido por las interfaces.
- La implementación necesita pruebas para herencia dinámica, edición compartida, edición local y cambios de definición.
- La persistencia no debe decidir la semántica por el mero hecho de copiar o no copiar campos.

## Cuestiones que siguen abiertas

Antes de implementar el mecanismo se deben resolver con ejemplos y pruebas:

- identidad y correspondencia de conceptos/componentes entre definición y máscara;
- tratamiento de una modificación local que coincide con el valor definido;
- representación de la eliminación local de un elemento heredado;
- operación de restablecimiento y pérdida de datos locales;
- validación cuando cambia la estructura del segmento de origen;
- precedencia y aplicación efectiva de permisos.

Este ADR establece la dirección conceptual, pero no afirma que las máscaras estén implementadas.
