# Política de releases de Bittopos

Este documento define los requisitos de coordinación de releases. No implica que ya exista un workflow de publicación automática.

## Fuente de la release

Cada release debe construirse desde commits inmutables de los repositorios de componentes. La rama predeterminada o un tag mutable no sustituyen a un SHA verificado.

El manifiesto de producto debe fijar:
- versión y canal;
- repositorio y SHA de cada componente;
- versión de cada componente, si está declarada;
- targets, toolchain y opciones de build;
- pruebas ejecutadas y resultado;
- artefactos, formato, tamaño y SHA-256;
- firma y procedencia, cuando estén configuradas;
- limitaciones conocidas y combinaciones no verificadas.

## Artefactos distintos

Identificar por separado:
1. **Código fuente:** archivo de fuentes del producto o fuentes de cada componente.
2. **Binarios:** ejecutables compilados para un target concreto.
3. **Paquetes:** archivos instalables producidos por `bittopos-packaging`.

No describir un archivo fuente como un binario portable. Cada artefacto debe indicar su sistema operativo, arquitectura y requisitos relevantes.

## Reglas de publicación

- No publicar desde ramas móviles.
- No sobrescribir una release ni reemplazar silenciosamente sus artefactos.
- Generar checksums SHA-256 y conservar el manifiesto junto a los artefactos.
- Registrar toolchain, configuración y resultados de pruebas.
- Marcar explícitamente targets no probados.
- No automatizar publicación firmada ni despliegue hasta que credenciales, permisos y flujo de aprobación estén configurados y revisados.

## Secuencia de implementación

1. Definir el esquema del manifiesto de producto.
2. Añadir validación que compruebe SHAs y repositorios existentes.
3. Comprobar compatibilidad entre versiones de componentes.
4. Construir artefactos reproducibles en CI para Linux.
5. Añadir FreeBSD cuando los runners o el entorno de prueba estén disponibles.
6. Incorporar Windows tras verificar su build y pruebas.
7. Automatizar la publicación como último paso, con permisos mínimos y aprobación explícita.
