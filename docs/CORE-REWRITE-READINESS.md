# Preparación para la reestructuración del núcleo

**Estado:** propuesta de preparación; no autoriza por sí sola la implementación posterior.  
**Alcance de este documento:** ordenar el inicio de un cambio arquitectónico amplio sin modificar todavía el comportamiento ejecutable.

## 1. Propósito

Bittopos va a someter su núcleo a una reestructuración importante para construir el modelo semántico y el motor de resolución de forma coherente. Antes de empezar esa implementación, el repositorio debe dejar claro qué se conserva, qué se rediseña, qué reglas ya están acordadas y qué casos siguen necesitando una decisión.

Este documento prepara ese trabajo. No implementa máscaras, no cambia el flujo de `bitto create`, no modifica formatos persistidos y no decide por adelantado la forma final de las APIs.

## 2. Fuentes de autoridad

Las fuentes vigentes tienen responsabilidades diferentes:

- [Visión](VISION.md): propósito del producto y dirección estratégica.
- [Modelo conceptual](model.md): vocabulario y semántica prevista.
- [ADR 0004](decisions/0004-semantic-mask-resolution.md): resolución semántica y modos de máscara.
- [ADR 0005](decisions/0005-mask-contract-boundaries.md): reglas acordadas y seis grupos de casos de borde pendientes.
- [Arquitectura](architecture.md): límites entre núcleo, aplicación, interfaces y persistencia.
- [Estado del proyecto](status.md): capacidades implementadas y trabajo pendiente.
- [Estado de continuidad](WORKING-STATE.md): contexto técnico y siguiente paso autorizado.

Si aparece una contradicción, no se debe resolver silenciosamente copiando una regla a otro documento. Identificar la fuente competente, describir un ejemplo concreto y registrar la decisión antes de implementar el comportamiento afectado.

## 3. Dirección arquitectónica que debe conservarse

La reestructuración debe mantener estas responsabilidades:

1. **Núcleo:** tipos del modelo, invariantes, topología, direcciones semánticas, resolución y reglas de dominio.
2. **Aplicación:** coordinación de casos de uso y de operaciones que implican varios pasos.
3. **Adaptadores:** carga, validación de formatos externos, serialización, archivos y futuros mecanismos de persistencia.
4. **Interfaces:** traducen acciones del usuario a operaciones explícitas y presentan resultados; no duplican reglas semánticas.

Estas fronteras son una guía funcional, no una exigencia de crear un módulo o una capa por cada concepto. La estructura concreta debe seguir las dependencias y facilitar pruebas independientes.

## 4. Alcance previsto del trabajo sobre el núcleo

El rediseño deberá evaluar de manera integrada, sin asumir que todo se implementará en un único PR:

- tipos de dominio para Resource, Instance, Segment, Concept y Component;
- relación entre definición, instancia, estructura local y recurso de origen;
- topología del modelo y direcciones semánticas independientes de rutas físicas y formato de almacenamiento;
- resolución de contenido heredado y representación de diferencias locales;
- modos de máscara puntual y Snapshot;
- combinación específica por tipo de Component y expansión de referencias;
- resolución de autorización efectiva, separada del contenido semántico.

La lista expresa el área que debe quedar arquitectónicamente bien fundada; no prescribe nombres de structs, módulos, traits ni una API pública antes de examinar el código actual.

## 5. Límites y decisiones que no se deben inventar

Las reglas centrales de máscaras ya están acordadas. Deben conservarse como punto de partida:

- una máscara es un Segment local de una Instance y contiene al menos un Concept o Component;
- una máscara puntual guarda solo diferencias locales;
- lo no modificado localmente sigue heredando y recibe cambios posteriores de la definición;
- editar un elemento heredado sin máscara puede modificar el Resource compartido;
- retirar una máscara reactiva la herencia correspondiente;
- Snapshot congela solo las partes incluidas explícitamente;
- la semántica pertenece al núcleo compartido, no a cada interfaz.

Los casos de borde del ADR 0005 siguen pendientes y no deben resolverse por inferencia: cambios estructurales del origen; ocultación/eliminación local de elementos heredados; identidad tras renombrar o mover; eliminación de recursos referenciados; precedencia exacta de permisos; y cambios de estructura que afecten a Snapshot. Cada caso debe bloquear solo la operación que dependa de él, salvo que una dependencia real demuestre un bloqueo más amplio.

## 6. Compatibilidad y datos persistidos

El cambio del modelo interno no autoriza a cambiar silenciosamente los datos existentes.

Antes de modificar una estructura persistida o su interpretación, el PR que lo proponga debe indicar explícitamente una de estas opciones:

- se conserva la compatibilidad con el formato y los datos actuales;
- se introduce una conversión/migración identificada, con pruebas y manejo de fallos;
- se declara incompatible el formato de prototipo y se explica cómo se detectan o rechazan los datos antiguos.

No declarar una migración necesaria ni innecesaria antes de comparar los tipos y contratos reales del código actual con el diseño propuesto. Preservar las garantías actuales de identidad, contención, coherencia entre el registro de instancias y el workspace, journal durable y recuperación salvo cambio deliberado, documentado y probado.

## 7. Secuencia de ingeniería recomendada

La implementación debe poder revisarse en incrementos pequeños:

1. **Establecer la línea base:** inspeccionar código y pruebas actuales, ejecutar las comprobaciones disponibles y registrar el resultado real de CI para el commit de partida. No duplicar pruebas existentes sin demostrar una brecha.
2. **Presentar el diseño concreto del núcleo:** mostrar los tipos actuales que cambian, las responsabilidades propuestas, las invariantes y el tratamiento de datos persistidos. Separar decisiones confirmadas de propuestas nuevas.
3. **Construir y probar el modelo puro:** validar invariantes, topología y direcciones semánticas sin depender de TOML, JSON, la CLI ni la web.
4. **Implementar la resolución incrementalmente:** cubrir primero la herencia base y las diferencias locales; después Snapshot, comportamiento por tipo, referencias y autorización según sus dependencias y decisiones pendientes.
5. **Integrar adaptadores y casos de uso:** mantener persistencia y coordinación fuera de las reglas puras; añadir pruebas de integración y recuperación cuando una operación cruce varias escrituras.
6. **Conectar interfaces:** hacer que la CLI consuma operaciones comunes; no implementar una segunda resolución semántica en la CLI o en la web.

Esta secuencia es una guía de revisión, no una autorización automática para iniciar cada fase después de fusionar este documento.

## 8. Criterios de preparación y aceptación

Antes de considerar listo un PR que inicie la reestructuración del núcleo, su descripción debe precisar:

- problema y alcance de ese incremento, con exclusiones explícitas;
- fuentes conceptuales que aplica y decisiones nuevas que propone;
- invariantes actuales que conserva o cambia;
- estrategia de compatibilidad o migración de datos, si corresponde;
- pruebas de dominio y regresión, más pruebas de persistencia/recuperación si afectan esos límites;
- comandos realmente ejecutados y enlace al resultado de CI;
- documentación que debe actualizarse.

El cambio se acepta solo si el diff permite revisar la responsabilidad de cada módulo, no duplica semántica en interfaces, no oculta cambios de contrato y no afirma garantías sin pruebas.

## 9. Operación del repositorio

- Trabajar en una rama dedicada basada en el `main` vigente y abrir un PR por incremento lógico.
- No desarrollar directamente sobre `main`.
- No borrar ni reutilizar ramas antiguas sin revisar antes si contienen trabajo pendiente.
- No cambiar reglas de protección de GitHub ni configuración del repositorio desde este PR documental. Esas opciones requieren una acción separada en la configuración del repositorio.
- Revisar CI y el diff antes de fusionar.
- Después de la fusión, volver a consultar el estado real de `main`, las ramas, los PR y CI; esta guía no sustituye esa verificación.

## 10. Qué queda fuera de esta preparación

Este documento no implementa el motor de máscaras, no cierra los casos pendientes del ADR 0005, no cambia la CLI, no añade servidor web, sincronización remota ni adaptadores de bases de datos, y no modifica los formatos TOML/JSON. El alcance del siguiente trabajo debe ser confirmado después de revisar y fusionar este PR.
