# Guía rápida: Ejecutar y depurar tests en Rust

Este proyecto contiene múltiples demos y tests en `src/estudios/`. A continuación se explica cómo ejecutarlos y depurarlos, tanto desde VS Code como desde la consola.

## VS Code: Rust Analyzer

- Extensión recomendada: `rust-analyzer`.
- Para depurar, instala también `CodeLLDB`.

Cómo usarlo:
- Abre cualquier archivo con tests (por ejemplo `src/estudios/array_slice_vec_string.rs`).
- Encima de cada `#[test]` verás CodeLens: "Run Test" y "Debug Test".
	- "Run Test" ejecuta ese test individual.
	- "Debug Test" lo lanza en el depurador (CodeLLDB).
- Panel de Testing: abre el panel "Testing" de VS Code para ver todos los tests descubiertos y ejecutarlos en lote o individualmente.

Consejos:
- Si un test usa `#[should_panic]`, puedes agregar `expected = "..."` para evitar backtrace verboso cuando el pánico es esperado.

## Consola: `cargo test`

Comandos útiles:
- Listar todos los tests:
	```bash
	cargo test -- --list
	```

- Ejecutar un test individual (por ejemplo, slicing UTF-8 inválido):
	```bash
	cargo test --lib -- estudios::array_slice_vec_string::utf8_slicing::invalid_slice_panics -- --nocapture
	```

- Desactivar el backtrace en pánicos al ejecutar tests:
	```bash
	RUST_BACKTRACE=0 cargo test --lib -- estudios
	```

Notas:
- Puedes añadir `--package estudio-rust` si trabajas en un workspace con múltiples paquetes:
	```bash
	cargo test --package estudio-rust --lib -- estudios::testing_demo -- --nocapture
	```
