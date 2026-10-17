# Contribuir a Bittopos

El desarrollo debe avanzar en incrementos pequeños que puedan explicarse, compilarse y revisarse.

## Flujo

1. Parte del `main` actualizado.
2. Crea una rama por cambio lógico: `feat/`, `fix/`, `refactor/`, `docs/` o `test/`.
3. Mantén el cambio limitado a una responsabilidad.
4. Actualiza la documentación solo cuando cambie el modelo, el comportamiento o la forma de construir.
5. Ejecuta las comprobaciones de la guía de desarrollo y comunica cuáles no pudiste ejecutar.
6. Abre un Pull Request con motivo, alcance, pruebas y limitaciones.

Usa mensajes de commit claros, preferiblemente Conventional Commits, por ejemplo `refactor(core): simplify instance lifecycle`.

## Comprobaciones

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
git diff --check
```

No declares pruebas como exitosas si no se ejecutaron. No añadas capas, abstracciones, recuperación automática ni preparación para plataformas futuras sin una necesidad actual demostrable.

## Documentos

- [Visión](docs/VISION.md)
- [Modelo](docs/model.md)
- [Arquitectura](docs/architecture.md)
- [Guía de desarrollo](docs/DEVELOPMENT-GUIDE.md)

El README es la entrada al proyecto; el modelo define las reglas conceptuales y la arquitectura explica cómo están repartidas en el código.
