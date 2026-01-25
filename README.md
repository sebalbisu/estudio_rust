# Rust: Fundamental and Advanced Concepts

Reference repository for fundamental and advanced concepts of Rust. Each module in [`src/estudios/`](src/estudios/) contains detailed explanations, executable examples, and tests that demonstrate specific language features.

## Content

- [`memory_and_types.rs`](src/estudios/memory_and_types.rs) - Types, sizes, alignment, layouts
- [`stack_vs_heap_performance.rs`](src/estudios/stack_vs_heap_performance.rs) - Stack vs heap benchmarks
- [`lifetimes.rs`](src/estudios/lifetimes.rs) - Implicit/explicit lifetimes, bounds, 'static lifetimes 
- [`references_vs_values.rs`](src/estudios/references_vs_values.rs) - References vs values, borrowing, auto-ref in methods, Deref/DerefMut, auto-deref coercion in structs, benchmark
- [`references_vs_values_performance.rs`](src/estudios/references_vs_values_performance.rs) - References vs values benchmarks
- [`stackframes.rs`](src/estudios/stackframes.rs) - Stack frames, passing variables
- [`array_slice_vec_string.rs`](src/estudios/array_slice_vec_string.rs) - Arrays, slices, vectors, strings, and UTF-8 handling
- [`traits_concepts.rs`](src/estudios/traits_concepts.rs) - Traits, implementations, impl/dyn, object safety
- [`clousures.rs`](src/estudios/clousures.rs) - Closures: Fn, FnMut, FnOnce, captures, trait objects
- [`sync_send.rs`](src/estudios/sync_send.rs) - Concurrency: Send, Sync, thread safety
- [`pin.rs`](src/estudios/pin.rs) - Pin, Unpin, self-referential structs
- [`futures.rs`](src/estudios/futures_async.rs) - Futures, async, await, async closures.
- [`iterators.rs`](src/estudios/iterators.rs) - Iterators, adapters, lazy evaluation
- [`syntax_sugar.rs`](src/estudios/syntax_sugar.rs) - Rust syntactic sugar
- [`comparisons.rs`](src/estudios/comparisons.rs) - Eq, PartialEq, Ord, PartialOrd
- [`error_result.rs`](src/estudios/error_result.rs) - Error handling, Result, Option, ? operator
- [`modules_demo.rs`](src/estudios/modules_demo.rs) - Module organization strategies
- [`testing_demo.rs`](src/estudios/testing_demo.rs) - Unit tests, property-based testing (proptest)
- [`estructuras_ids.rs`](src/estudios/estructuras_ids.rs) - Structures with IDs, newtype pattern


## Quick Guide: Running and Debugging Tests

This project contains multiple demos and tests in [`src/estudios/`](src/estudios/). Below is an explanation of how to run and debug them from both VS Code and the console.

## VS Code: Rust Analyzer

- Recommended extension: [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
- For debugging, also install [`CodeLLDB`](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).

How to use it:
- Open any file with tests (for example [`src/estudios/array_slice_vec_string.rs`](src/estudios/array_slice_vec_string.rs)).
- Above each `#[test]` you will see CodeLens: "Run Test" and "Debug Test".
	- "Run Test" runs that individual test.
	- "Debug Test" launches it in the debugger (CodeLLDB).
- Testing Panel: open the "Testing" panel in VS Code to see all discovered tests and run them in batch or individually.

## Console: `cargo test`

Useful commands:
- List all tests:
	```bash
	cargo test -- --list
	```

- Run an individual test (for example, invalid UTF-8 slicing):
	```bash
	cargo test --lib -- estudios::array_slice_vec_string::utf8_slicing::invalid_slice_panics -- --nocapture
	```

- Disable backtrace on panics when running tests:
	```bash
	RUST_BACKTRACE=0 cargo test --lib -- estudios
	```
