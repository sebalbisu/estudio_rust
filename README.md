# Rust: Aprendizaje de conceptos fundamentales y avanzados 

Repositorio de aprendizaje y referencia sobre conceptos fundamentales y avanzados de Rust. Cada módulo en [`src/estudios/`](src/estudios/) contiene explicaciones detalladas, ejemplos ejecutables y tests que demuestran características específicas del lenguaje.

## Contenido

- [`memory_and_types.rs`](src/estudios/memory_and_types.rs) - Tipos, tamaños, alignment, layouts
- [`stack_vs_heap.rs`](src/estudios/stack_vs_heap.rs) - Diferencias entre stack y heap, Box, Rc, Arc
- [`array_slice_vec_string.rs`](src/estudios/array_slice_vec_string.rs) - Arrays, slices, vectores, strings y manejo de UTF-8
- [`stacks_func.rs`](src/estudios/stacks_func.rs) - Stack frames, call stack
- [`referencias_vs_valor.rs`](src/estudios/referencias_vs_valor.rs) - Referencias vs valores, borrowing, deref coercion
- [`lifetimes.rs`](src/estudios/lifetimes.rs) - Lifetimes explícitos, elision rules, static
- [`comparisons_equal_partial_greater.rs`](src/estudios/comparisons_equal_partial_greater.rs) - Eq, PartialEq, Ord, PartialOrd
- [`syntax_sugar.rs`](src/estudios/syntax_sugar.rs) - Azúcar sintáctica de Rust
- [`estructuras_ids.rs`](src/estudios/estructuras_ids.rs) - Estructuras con IDs, newtype pattern
- [`clousures.rs`](src/estudios/clousures.rs) - Closures: Fn, FnMut, FnOnce, capturas, trait objects
- [`modules_demo.rs`](src/estudios/modules_demo.rs) - Estrategias de organización de módulos
- [`iteradores.rs`](src/estudios/iteradores.rs) - Iteradores, adaptadores, lazy evaluation
- [`traits_concepts.rs`](src/estudios/traits_concepts.rs) - Traits, implementaciones, object safety, polimorfismo
- [`testing_demo.rs`](src/estudios/testing_demo.rs) - Unit tests, property-based testing (proptest)
- [`error_result.rs`](src/estudios/error_result.rs) - Error handling, Result, Option, operador ?
- [`sync_send_demo.rs`](src/estudios/sync_send_demo.rs) - Concurrencia: Send, Sync, thread safety
- [`pin_demo.rs`](src/estudios/pin_demo.rs) - Pin, Unpin, self-referential structs
- [`futures_demo.rs`](src/estudios/futures_demo.rs) - Async/await, futures, executors


## Guía rápida: Ejecutar y depurar tests

Este proyecto contiene múltiples demos y tests en [`src/estudios/`](src/estudios/). A continuación se explica cómo ejecutarlos y depurarlos, tanto desde VS Code como desde la consola.

## VS Code: Rust Analyzer

- Extensión recomendada: [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
- Para depurar, instala también [`CodeLLDB`](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).

Cómo usarlo:
- Abre cualquier archivo con tests (por ejemplo [`src/estudios/array_slice_vec_string.rs`](src/estudios/array_slice_vec_string.rs)).
- Encima de cada `#[test]` verás CodeLens: "Run Test" y "Debug Test".
	- "Run Test" ejecuta ese test individual.
	- "Debug Test" lo lanza en el depurador (CodeLLDB).
- Panel de Testing: abre el panel "Testing" de VS Code para ver todos los tests descubiertos y ejecutarlos en lote o individualmente.

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
