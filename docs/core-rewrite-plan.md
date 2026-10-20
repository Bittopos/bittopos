# Reescritura del núcleo: perfil, proyecto y fichas

## Objetivo

El núcleo debe representar explícitamente el dominio de Bittopos y no confundirlo con los formatos de persistencia. La jerarquía mínima es:

`Perfil -> Proyectos -> Fichas raíz -> Fichas descendientes`

Una ficha tiene identidad propia. Su posición jerárquica expresa contención; las relaciones semánticas entre fichas son independientes de esa contención.

## Contrato mínimo

- Un perfil tiene ID estable y contiene cero o más proyectos.
- Un proyecto tiene ID estable y contiene una estructura de fichas.
- Un proyecto puede tener cero o más fichas raíz.
- Cada ficha tiene un ID único dentro del ámbito definido por el modelo y puede tener cero o más fichas hijas.
- Cada ficha conserva identidad al moverse dentro de la estructura.
- Una ficha puede tener relaciones con otras fichas sin alterar su padre ni su contención.
- Las validaciones del núcleo deben rechazar IDs duplicados, referencias rotas y estados de contención incoherentes.
- JSON es una representación serializada del documento, no el modelo de dominio ni el validador.
- La deserialización debe ir seguida de validación semántica.
- Web, CLI y servidor deben consumir las mismas operaciones y reglas del núcleo.

## Forma JSON objetivo ilustrativa

```json
{
  "profile": {
    "id": "profile-001",
    "name": "Ana García",
    "projects": [
      {
        "id": "project-001",
        "name": "Huerta familiar",
        "objects": [
          {
            "id": "obj-001",
            "title": "Cultivos",
            "children": [
              {
                "id": "obj-004",
                "title": "Tomates",
                "children": []
              }
            ]
          },
          {
            "id": "obj-002",
            "title": "Riego",
            "children": [
              {
                "id": "obj-005",
                "title": "Riego por goteo",
                "children": []
              }
            ]
          },
          {
            "id": "obj-003",
            "title": "Herramientas",
            "children": []
          }
        ]
      }
    ]
  }
}
```

Este ejemplo ilustra la jerarquía, no define todavía todos los campos finales de una ficha ni sustituye el contenido semántico de los recursos actuales.

## Estrategia de reescritura

1. Definir tipos de dominio explícitos para Perfil, Proyecto y Ficha/Instancia, con invariantes verificables.
2. Definir un DTO de persistencia JSON separado de los tipos de dominio.
3. Implementar conversiones y validación al cargar; guardar solo documentos válidos.
4. Añadir pruebas de ida y vuelta JSON, tres fichas raíz, descendientes, IDs duplicados, referencias inválidas y profundidad adicional.
5. Migrar CLI y las interfaces a las operaciones del núcleo, evitando que cada interfaz replique reglas.
6. Documentar compatibilidad o migración para los archivos JSON existentes antes de cambiar su formato.

## Restricciones

- No fusionar estos cambios en `main` sin revisión.
- No borrar el modelo actual hasta que los casos de uso existentes estén cubiertos por pruebas.
- No asumir que una relación semántica equivale a contención.
- No tratar la validez sintáctica de JSON como garantía de validez del dominio.
