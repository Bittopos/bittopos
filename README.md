# Bittopos

**Estructura la información. Comprende sus relaciones. Da contexto a tus proyectos.**

Bittopos es una plataforma de software libre para organizar información y proyectos mediante estructuras jerárquicas, recursos reutilizables y relaciones semánticas explícitas. Su objetivo es que las personas puedan construir espacios de trabajo adaptados a sus necesidades: desde proyectos de software y conocimiento hasta planes, procesos y sistemas complejos.

Este repositorio es el **punto de entrada del proyecto**: presenta Bittopos, su arquitectura, documentación y enlaces a los componentes mantenidos por separado.

## Componentes

El proyecto se divide en repositorios independientes para mantener límites claros entre responsabilidades:

- **[bittopos-core](https://github.com/jasperalsain/bittopos-core)** — modelo y operaciones compartidas.
- **[bittopos-cli](https://github.com/jasperalsain/bittopos-cli)** — interfaz de línea de comandos.
- **[bittopos-server](https://github.com/jasperalsain/bittopos-server)** — servicio HTTP para acceder a las operaciones desde otras interfaces.
- **[bittopos-web](https://github.com/jasperalsain/bittopos-web)** — interfaz web.
- **bittopos-desktop** — aplicación de escritorio para Windows, Linux y macOS; su separación está en preparación.

Los nombres `bittopos-app` (Android) y `bittopos-ios` (iPhone y iPad) quedan reservados para posibles clientes futuros; esos repositorios aún no existen.

## Ideas centrales

- **Recursos reutilizables:** una definición puede utilizarse en distintas instancias.
- **Estructuras jerárquicas:** los proyectos organizan sus instancias en un espacio de trabajo navegable.
- **Relaciones explícitas:** los elementos pueden relacionarse sin perder su identidad.
- **Resolución semántica:** el sistema combina las definiciones compartidas con las diferencias locales de cada instancia.
- **Interfaces separadas del núcleo:** distintos clientes pueden utilizar las mismas operaciones compartidas.

Bittopos no se limita a una categoría de uso. El mismo modelo puede ayudar a estructurar un proyecto de software, un plan de estudios, un inventario, un conjunto de fichas de conocimiento o un sistema de relaciones entre entidades.

## Documentación

- [Visión del proyecto](docs/VISION.md)
- [Modelo conceptual](docs/model.md)
- [Arquitectura](docs/architecture.md)
- [Persistencia estructurada](docs/database-storage.md)
- [Guía de desarrollo](docs/DEVELOPMENT-GUIDE.md)
- [Contribuir al proyecto](CONTRIBUTING.md)

## Estado del proyecto

Bittopos está en desarrollo activo. Los componentes tienen distintos grados de madurez; consulta el README y los issues de cada repositorio antes de depender de una función en producción. La web pública presenta el proyecto y no implica que todos los clientes o flujos estén terminados.

## Contribuciones y licencia

Las contribuciones se gestionan mediante issues y pull requests en el repositorio del componente correspondiente. Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para las pautas generales.

Bittopos contiene componentes con licencias distintas. Revisa la licencia incluida en cada repositorio antes de reutilizar o redistribuir un componente.

## Sitio del proyecto

La página estática del proyecto se encuentra en [docs/](docs/). Cuando se configure GitHub Pages, esta carpeta podrá publicarse como sitio del repositorio.
