# ADR 0001: núcleo adaptable y adaptadores explícitos

- **Estado:** aceptado
- **Fecha:** 2026-10-13

## Contexto

Bittopos debe poder servir a distintas interfaces y, en el futuro, a adaptadores de IA y diferentes modelos de persistencia. El diseño actual es local y utiliza TOML para definiciones de recursos y JSON para el workspace. No hay que convertir esos formatos iniciales en la semántica permanente del producto ni crear abstracciones sin utilidad concreta.

## Decisión

- Las interfaces solicitan operaciones estructuradas.
- El núcleo contiene el modelo y valida sus invariantes.
- Las operaciones de aplicación coordinan cambios compuestos cuando sea necesario.
- Los adaptadores se ocupan de formatos, rutas y almacenes concretos.
- Cada frontera se introduce cuando reduce acoplamiento, aclara una responsabilidad o mejora la capacidad de prueba.
- JSON es un formato inicial de persistencia/exportación, no la definición conceptual del modelo.

## Alternativas consideradas

- Acoplar modelo, archivos y CLI en un solo lugar: simple al principio, pero hace más difícil reutilizar reglas y probarlas desde otras interfaces.
- Introducir una arquitectura genérica completa antes de tener casos de uso: aumenta complejidad sin validar sus beneficios.
- La decisión elegida mantiene límites explícitos sin imponer abstracciones preventivas.

## Consecuencias

- Las reglas del modelo deben poder probarse sin persistencia cuando sea práctico.
- Los DTOs de persistencia no deben convertirse automáticamente en tipos de dominio.
- La CLI no debe ser el único lugar donde existan las reglas de negocio.
- La arquitectura objetivo no se considerará implementada hasta que el código y sus pruebas la respalden.
