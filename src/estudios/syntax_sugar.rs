//! # Syntax Sugar en Rust
//!
//! Syntax sugar = sintaxis más corta/legible que el compilador expande
//! a código más explícito.
//!
//! ## Ejecutar tests
//! ```bash
//! cargo test --bin 0_syntax_sugar -- --nocapture
//! cargo test --bin 0_syntax_sugar indice -- --nocapture
//! ```

// ============================================================================
// ÍNDICE - Ejecuta todas las demos
// ============================================================================

#[test]
fn indice() {
    deref_coercion::deref_coercion();
    method_call::method_call();
    operators::operators();
    loops::loops();
    question_mark::question_mark();
    range_syntax::range_syntax();
    indexing::indexing();
    closures::closures();
    format_macros::format_macros();
    struct_init::struct_init();
    pattern_matching::pattern_matching();
    impl_trait::impl_trait();
    async_await::async_await();
    derive_macros::derive_macros();

    println!("\n✅ Todos los tests ejecutados\n");
}

// ============================================================================
// 1. DEREF COERCION (Auto-dereferenciación)
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ DEREF COERCION                                               │
//     ├──────────────────────────────────────────────────────────────┤
//     │ Syntax Sugar:                                                │
//     │                                                              │
//     │ ref.method()       →  (*ref).method()      // ref = &T       │
//     │ ref_ref.method()   →  (**ref_ref).method() // ref_ref = &&T  │
//     └──────────────────────────────────────────────────────────────┘
//

#[cfg(test)]
mod deref_coercion {
    #[test]
    pub fn deref_coercion() {
        let s = String::from("hello");
        let r: &String = &s;
        let rr: &&String = &&s;

        // Tú escribes:           El compilador hace:
        // r.len()                (*r).len()
        // rr.len()               (**rr).len()
        assert_eq!(r.len(), 5);
        assert_eq!(rr.len(), 5);
    }
}

// ============================================================================
// 2. METHOD CALL (Llamada a métodos)
// ============================================================================
//
//  METHOD CALL SYNTAX
//  ══════════════════════════════════════════════════════════════════
//  .method(&self):
//    ref.method()     -> Type::method(&*ref)              (auto-deref)
//    value.method()   -> Type::method(&value)             (auto-borrow)
//
//  .method(&mut self):
//    mut_ref.method() -> Type::method(&mut *mut_ref)      (auto-deref)
//    value.method()   -> Type::method(&mut value)         (auto-borrow mut)
//
//  .method(self):
//    value.method()   -> Type::method(value)              (sin cambios)
//  ══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod method_call {
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn distance(&self) -> f64 {
            ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
        }

        fn move_by(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y += dy;
        }

        fn consume(self) -> String {
            format!("Point({}, {})", self.x, self.y)
        }
    }

    #[test]
    pub fn method_call() {
        let p = Point { x: 3, y: 4 };
        let r = &p;
        let rr = &r;

        // &self: auto-deref y auto-borrow
        assert_eq!(p.distance(), 5.0);
        assert_eq!(r.distance(), 5.0);
        assert_eq!(rr.distance(), 5.0);

        // &mut self: modificación in-place
        let mut p2 = Point { x: 1, y: 1 };
        p2.move_by(2, 3);
        assert_eq!(p2.x, 3);
        assert_eq!(p2.y, 4);

        // self: consumo del valor
        let p3 = Point { x: 5, y: 12 };
        let result = p3.consume();
        assert_eq!(result, "Point(5, 12)");
        // p3 ya no existe

        println!("  ✅ method_call::method_call");
    }
}

// ============================================================================
// 4. OPERATORS (Operadores)
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ OPERATORS                                                    │
//     ├──────────────────────────────────────────────────────────────┤
//     │ a + b     →  Add::add(a, b)                                  │
//     │ a - b     →  Sub::sub(a, b)                                  │
//     │ a * b     →  Mul::mul(a, b)                                  │
//     │ a / b     →  Div::div(a, b)                                  │
//     │ a % b     →  Rem::rem(a, b)                                  │
//     │ a == b    →  PartialEq::eq(&a, &b)                           │
//     │ a != b    →  PartialEq::ne(&a, &b)                           │
//     │ a < b     →  PartialOrd::lt(&a, &b)                          │
//     │ a > b     →  PartialOrd::gt(&a, &b)                          │
//     │ a <= b    →  PartialOrd::le(&a, &b)                          │
//     │ a >= b    →  PartialOrd::ge(&a, &b)                          │
//     │ -a        →  Neg::neg(a)                                     │
//     │ !a        →  Not::not(a)                                     │
//     │ *a        →  Deref::deref(&a)                                │
//     │ a[i]      →  Index::index(&a, i)                             │
//     │ a[i] = v  →  IndexMut::index_mut(&mut a, i) = v              │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod operators {
    use std::ops::Add;

    #[test]
    pub fn operators() {
        let a = 5;
        let b = 3;

        // Aritméticos → traits en std::ops
        assert_eq!(a + b, Add::add(a, b));
        assert_eq!(a + b, 8);
        assert_eq!(a - b, 2);
        assert_eq!(a * b, 15);
        assert_eq!(a / b, 1);
        assert_eq!(a % b, 2);

        // Comparación → PartialEq, PartialOrd
        assert_eq!(a == b, std::cmp::PartialEq::eq(&a, &b));
        assert!(!std::cmp::PartialEq::eq(&a, &b));
        assert!(a > b);
        assert!(b < a);

        // Negación
        assert_eq!(-a, -5);
        assert!(!false);

        println!("  ✅ operators::operators");
    }
}

// ============================================================================
// 5. LOOPS (Bucles)
// ============================================================================
//
// formas abreviadas de llamar a metodos .iter(), .iter_mut(), .into_iter()
// collection puede ser cualquier tipo que implemente esos metodos o alguno
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ FOR LOOP                                                     │
//     ├──────────────────────────────────────────────────────────────┤
//     │                                                              │
//     │ for x in &collection    →  for x in collection.iter()        │
//     │                            (Iterator<Item = &T>)             │
//     │                                                              │
//     │ for x in &mut coll      →  for x in collection.iter_mut()    │
//     │                            (Iterator<Item = &mut T>)         │
//     │                                                              │
//     │ for x in collection     →  for x in collection.into_iter()   │
//     │                                                              │
//     │ for x in iter           →  { let mut it = iter.into_iter();  │
//     │                              while let Some(x) = it.next()   │
//     │                              { ... } }                       │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod loops {
    #[test]
    pub fn loops() {
        let mut v = vec![1, 2, 3];

        // for x in &v equivale a for x in v.iter()
        let mut sum1 = 0;
        for x in &v {
            sum1 += x;
        }

        let mut sum2 = 0;
        for x in v.iter() {
            sum2 += x;
        }

        assert_eq!(sum1, 6);
        assert_eq!(sum2, 6);

        // for x in v consume el vector (into_iter)
        let v2 = vec![1, 2, 3];
        let mut sum3 = 0;
        for x in v2 {
            // v2.into_iter()
            sum3 += x;
        }
        // v2 ya no es válido
        assert_eq!(sum3, 6);

        // for x in &mut v permite modificar
        let mut v3 = vec![1, 2, 3];
        for x in &mut v3 {
            *x *= 2;
        }
        assert_eq!(v3, vec![2, 4, 6]);

        println!("  ✅ loops::loops");
    }
}

// ============================================================================
// 6. QUESTION MARK OPERATOR (?)
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ QUESTION MARK OPERATOR                                       │
//     ├──────────────────────────────────────────────────────────────┤
//     │ let x = expr?;                                               │
//     │                                                              │
//     │ →  let x = match expr {                                      │
//     │        Ok(v) => v,                                           │
//     │        Err(e) => return Err(e.into()),                       │
//     │    };                                                        │
//     │                                                              │
//     │ Para Option:                                                 │
//     │ →  let x = match expr {                                      │
//     │        Some(v) => v,                                         │
//     │        None => return None,                                  │
//     │    };                                                        │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod question_mark {
    fn might_fail(ok: bool) -> Result<i32, &'static str> {
        if ok { Ok(42) } else { Err("failed") }
    }

    fn with_sugar(ok: bool) -> Result<i32, &'static str> {
        let x = might_fail(ok)?; // Sugar
        Ok(x * 2)
    }

    #[allow(clippy::question_mark)]
    fn without_sugar(ok: bool) -> Result<i32, &'static str> {
        let x = match might_fail(ok) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(x * 2)
    }

    #[test]
    pub fn question_mark() {
        assert_eq!(with_sugar(true), Ok(84));
        assert_eq!(without_sugar(true), Ok(84));
        assert_eq!(with_sugar(false), Err("failed"));
        assert_eq!(without_sugar(false), Err("failed"));

        // También funciona con Option
        fn option_chain(x: Option<i32>) -> Option<i32> {
            let v = x?;
            Some(v * 2)
        }
        assert_eq!(option_chain(Some(5)), Some(10));
        assert_eq!(option_chain(None), None);

        println!("  ✅ question_mark::question_mark");
    }
}

// ============================================================================
// 7. RANGE SYNTAX
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ RANGE SYNTAX                                                 │
//     ├──────────────────────────────────────────────────────────────┤
//     │ 0..5      →  Range { start: 0, end: 5 }       [0,1,2,3,4]   │
//     │ 0..=5     →  RangeInclusive::new(0, 5)       [0,1,2,3,4,5] │
//     │ ..5       →  RangeTo { end: 5 }                             │
//     │ 5..       →  RangeFrom { start: 5 }                         │
//     │ ..        →  RangeFull                                      │
//     │ arr[1..3] →  Index::index(&arr, Range{1,3})                 │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod range_syntax {
    #[test]
    pub fn range_syntax() {
        // 0..5 = Range { start: 0, end: 5 } (exclusivo)
        let r1: Vec<i32> = (0..5).collect();
        assert_eq!(r1, vec![0, 1, 2, 3, 4]);

        // 0..=5 = RangeInclusive (inclusivo)
        let r2: Vec<i32> = (0..=5).collect();
        assert_eq!(r2, vec![0, 1, 2, 3, 4, 5]);

        // Slicing con ranges
        let arr = [0, 1, 2, 3, 4, 5];
        assert_eq!(&arr[1..4], &[1, 2, 3]);
        assert_eq!(&arr[..3], &[0, 1, 2]);
        assert_eq!(&arr[3..], &[3, 4, 5]);
        assert_eq!(&arr[..], &[0, 1, 2, 3, 4, 5]);

        println!("  ✅ range_syntax::range_syntax");
    }
}

// ============================================================================
// 8. INDEXING
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ INDEXING                                                     │
//     ├──────────────────────────────────────────────────────────────┤
//     │ arr[i]       →  *<[T] as Index<usize>>::index(&arr, i)       │
//     │              →  Retorna &T, luego copia si T: Copy           │
//     │                                                              │
//     │ arr[i] = v   →  *IndexMut::index_mut(&mut arr, i) = v        │
//     │                                                              │
//     │ arr[1..3]    →  Index::index(&arr, 1..3)  → &[T]             │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod indexing {
    use std::ops::Index;

    #[test]
    pub fn indexing() {
        let arr = [1, 2, 3, 4, 5];
        let mut v = vec![1, 2, 3];

        // arr[i] = *Index::index(&arr, i)
        assert_eq!(arr[0], 1);
        assert_eq!(<[i32] as Index<usize>>::index(&arr, 0), &1);

        // arr[i] = v usa IndexMut
        v[0] = 10;
        assert_eq!(v[0], 10);

        // Slicing
        assert_eq!(&arr[1..3], &[2, 3]);

        println!("  ✅ indexing::indexing");
    }
}

// ============================================================================
// 9. CLOSURES
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ CLOSURES                                                     │
//     ├──────────────────────────────────────────────────────────────┤
//     │ |args| expr                                                  │
//     │                                                              │
//     │ →  struct __AnonymousClosure {                               │
//     │        captured_var: &T / &mut T / T  (según uso)            │
//     │    }                                                         │
//     │    impl Fn/FnMut/FnOnce for __AnonymousClosure { ... }      │
//     │                                                              │
//     │ move |args| expr  →  captura por valor (ownership)           │
//     └──────────────────────────────────────────────────────────────┘
//
//     El compilador genera un struct anónimo que captura las variables
//     del entorno e implementa Fn, FnMut, o FnOnce según cómo las use.

#[cfg(test)]
mod closures {
    #[test]
    pub fn closures() {
        let x = 5;

        // Captura x por referencia (impl Fn)
        let add = |a| a + x;
        assert_eq!(add(3), 8);
        assert_eq!(x, 5); // x sigue válido

        // Captura mutable (impl FnMut)
        let mut count = 0;
        let mut increment = || count += 1;
        increment();
        increment();
        assert_eq!(count, 2);

        // move: captura por valor (impl FnOnce o Fn si Copy)
        let s = String::from("hello");
        let consume = move || s.len();
        assert_eq!(consume(), 5);
        // s ya no es válido

        println!("  ✅ closures::closures");
    }
}

// ============================================================================
// 10. FORMAT MACROS
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ FORMAT MACROS                                                │
//     ├──────────────────────────────────────────────────────────────┤
//     │ println!("{}", x)  →  print + newline                        │
//     │ print!("{}", x)    →  print sin newline                      │
//     │ format!("{}", x)   →  retorna String                         │
//     │ write!(w, "{}", x) →  escribe a writer                       │
//     │ panic!("{}", x)    →  panic con mensaje                      │
//     │                                                              │
//     │ Placeholders:                                                │
//     │ {}      →  Display                                           │
//     │ {:?}    →  Debug                                             │
//     │ {:#?}   →  Debug pretty-printed                              │
//     │ {:p}    →  Pointer                                           │
//     │ {:b}    →  Binary                                            │
//     │ {:x}    →  Hex lowercase                                     │
//     │ {:X}    →  Hex uppercase                                     │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod format_macros {
    #[test]
    pub fn format_macros() {
        let name = "world";
        let num = 42;

        // format! retorna String
        let s = format!("Hello, {}!", name);
        assert_eq!(s, "Hello, world!");

        // Diferentes formatos
        let debug = format!("{:?}", vec![1, 2, 3]);
        assert_eq!(debug, "[1, 2, 3]");

        let binary = format!("{:b}", num);
        assert_eq!(binary, "101010");

        let hex = format!("{:x}", num);
        assert_eq!(hex, "2a");

        let hex_upper = format!("{:X}", num);
        assert_eq!(hex_upper, "2A");

        // Padding y alineación
        let padded = format!("{:>5}", 42);
        assert_eq!(padded, "   42");

        let zero_padded = format!("{:05}", 42);
        assert_eq!(zero_padded, "00042");

        println!("  ✅ format_macros::format_macros");
    }
}

// ============================================================================
// 11. STRUCT INITIALIZATION
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ STRUCT INITIALIZATION                                        │
//     ├──────────────────────────────────────────────────────────────┤
//     │ Config { name, value }  →  Config { name: name, value: v }  │
//     │                             (field init shorthand)           │
//     │                                                              │
//     │ Config { x: 1, ..other } →  copia campos restantes de other │
//     │                             (struct update syntax)           │
//     │                                                              │
//     │ let Config { name, .. }  →  destructure, ignora otros        │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod struct_init {
    #[derive(Debug, Clone, PartialEq)]
    struct Config {
        name: String,
        value: i32,
        enabled: bool,
    }

    #[test]
    pub fn struct_init() {
        let name = String::from("test");
        let value = 42;

        // Field init shorthand: name en vez de name: name
        let c1 = Config {
            name,  // = name: name
            value, // = value: value
            enabled: true,
        };
        assert_eq!(c1.name, "test");
        assert_eq!(c1.value, 42);

        // Struct update syntax: ..other copia campos restantes
        let c2 = Config {
            value: 100,
            ..c1.clone()
        };
        assert_eq!(c2.name, "test");
        assert_eq!(c2.value, 100);
        assert!(c2.enabled);

        // Destructuring con ..
        let Config { name, .. } = c1;
        assert_eq!(name, "test");

        println!("  ✅ struct_init::struct_init");
    }
}

// ============================================================================
// 12. PATTERN MATCHING
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ PATTERN MATCHING SUGAR                                       │
//     ├──────────────────────────────────────────────────────────────┤
//     │ if let Pat = expr { }                                        │
//     │ →  match expr { Pat => { }, _ => () }                        │
//     │                                                              │
//     │ while let Pat = expr { }                                     │
//     │ →  loop { match expr { Pat => { }, _ => break } }            │
//     │                                                              │
//     │ let Pat = expr else { diverge };                             │
//     │ →  match expr { Pat => ..., _ => diverge }                   │
//     │                                                              │
//     │ matches!(expr, Pat)                                          │
//     │ →  match expr { Pat => true, _ => false }                    │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod pattern_matching {
    #[test]
    pub fn pattern_matching() {
        let opt: Option<i32> = Some(42);

        // if let es sugar para match con un brazo
        let result = if let Some(x) = opt { x } else { 0 };
        assert_eq!(result, 42);

        // while let
        let mut stack = vec![1, 2, 3];
        let mut items = Vec::new();
        while let Some(x) = stack.pop() {
            items.push(x);
        }
        assert_eq!(items, vec![3, 2, 1]);

        // let else (Rust 1.65+)
        fn get_value() -> Option<i32> {
            Some(5)
        }
        let Some(v) = get_value() else {
            panic!("expected Some");
        };
        assert_eq!(v, 5);

        // matches! macro
        assert!(matches!(opt, Some(42)));
        assert!(matches!(opt, Some(_)));
        assert!(!matches!(opt, None));

        println!("  ✅ pattern_matching::pattern_matching");
    }
}

// ============================================================================
// 13. IMPL TRAIT
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ IMPL TRAIT                                                   │
//     ├──────────────────────────────────────────────────────────────┤
//     │ fn foo(x: impl Trait)       →  fn foo<T: Trait>(x: T)        │
//     │                                (sugar para generics)         │
//     │                                                              │
//     │ fn foo() -> impl Trait      →  retorna tipo concreto anónimo │
//     │                                que implementa Trait          │
//     │                                (no es dyn, es estático)      │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod impl_trait {
    // En parámetros: sugar para generics
    fn print_iter(iter: impl Iterator<Item = i32>) -> Vec<i32> {
        iter.collect()
    }

    // Equivalente explícito con generics
    fn _print_iter_generic<I: Iterator<Item = i32>>(iter: I) -> Vec<i32> {
        iter.collect()
    }

    // En retorno: oculta el tipo concreto
    fn make_iter() -> impl Iterator<Item = i32> {
        vec![1, 2, 3].into_iter()
    }

    #[test]
    pub fn impl_trait() {
        let result = print_iter(make_iter());
        assert_eq!(result, vec![1, 2, 3]);

        // impl Trait en retorno es útil para closures
        fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
            move |y| x + y
        }
        let add5 = make_adder(5);
        assert_eq!(add5(3), 8);

        println!("  ✅ impl_trait::impl_trait");
    }
}

// ============================================================================
// 14. ASYNC/AWAIT
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ ASYNC/AWAIT                                                  │
//     ├──────────────────────────────────────────────────────────────┤
//     │ async fn foo() -> T                                          │
//     │ →  fn foo() -> impl Future<Output = T>                       │
//     │                                                              │
//     │ async { expr }                                               │
//     │ →  genera struct anónimo que impl Future                     │
//     │                                                              │
//     │ future.await                                                 │
//     │ →  loop { match future.poll(cx) {                            │
//     │        Poll::Ready(v) => break v,                            │
//     │        Poll::Pending => suspend/yield                        │
//     │    }}                                                        │
//     └──────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod async_await {
    use std::future::Future;

    // async fn es sugar para retornar impl Future
    async fn fetch_data() -> i32 {
        42
    }

    // Equivalente conceptual:
    fn _fetch_data_expanded() -> impl Future<Output = i32> {
        async { 42 }
    }

    #[test]
    pub fn async_await() {
        // async fn retorna un Future, no ejecuta nada hasta .await
        let future = fetch_data();

        // El Future existe como un tipo
        let _ = std::mem::size_of_val(&future);

        // .await es sugar para poll loop
        // No ejecutamos porque necesitaríamos un runtime

        drop(future);

        println!("  ✅ async_await::async_await");
    }
}

// ============================================================================
// 15. DERIVE MACROS
// ============================================================================
//
//     ┌──────────────────────────────────────────────────────────────┐
//     │ DERIVE MACROS                                                │
//     ├──────────────────────────────────────────────────────────────┤
//     │ #[derive(Debug)]       →  impl Debug for Type { ... }       │
//     │ #[derive(Clone)]       →  impl Clone for Type { ... }       │
//     │ #[derive(Copy)]        →  impl Copy for Type (marker)        │
//     │ #[derive(PartialEq)]   →  impl PartialEq for Type { ... }   │
//     │ #[derive(Eq)]          →  impl Eq for Type (marker)          │
//     │ #[derive(Hash)]        →  impl Hash for Type { ... }        │
//     │ #[derive(Default)]     →  impl Default for Type { ... }     │
//     │ #[derive(PartialOrd)]  →  impl PartialOrd for Type { ... }  │
//     │ #[derive(Ord)]         →  impl Ord for Type { ... }         │
//     └──────────────────────────────────────────────────────────────┘
//
//     Los derives generan implementaciones automáticas de traits.

#[cfg(test)]
mod derive_macros {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    pub fn derive_macros() {
        let p = Point { x: 1, y: 2 };

        // Debug
        let debug = format!("{:?}", p);
        assert!(debug.contains("Point"));
        assert!(debug.contains("x: 1"));

        // Clone y Copy
        let p2 = p.clone();
        let p3 = p; // Copy, no move
        assert_eq!(p, p2);
        assert_eq!(p, p3);

        // PartialEq
        assert_eq!(p, Point { x: 1, y: 2 });
        assert_ne!(p, Point { x: 0, y: 0 });

        // Default
        let default = Point::default();
        assert_eq!(default, Point { x: 0, y: 0 });

        // Hash (permite usar en HashSet/HashMap)
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(p);
        assert!(set.contains(&p));

        println!("  ✅ derive_macros::derive_macros");
    }
}

// ============================================================================
// TABLA RESUMEN
// ============================================================================
//
//     ┌───────────────────┬──────────────────────────────────────────────────────┐
//     │ SUGAR             │ SE EXPANDE A                                         │
//     ├───────────────────┼──────────────────────────────────────────────────────┤
//     │ ref.field         │ (*ref).field                                         │
//     │ ref.method()      │ Type::method(&ref) / (&*ref).method()                │
//     │ a + b             │ Add::add(a, b)                                       │
//     │ a == b            │ PartialEq::eq(&a, &b)                                │
//     │ arr[i]            │ *Index::index(&arr, i)                               │
//     │ for x in coll     │ for x in coll.into_iter()                            │
//     │ expr?             │ match expr { Ok(v)=>v, Err(e)=>return Err(e) }      │
//     │ 0..5              │ Range { start: 0, end: 5 }                          │
//     │ |x| expr          │ struct anónimo + impl Fn/FnMut/FnOnce                │
//     │ S { field }       │ S { field: field }                                  │
//     │ S { ..other }     │ copia campos restantes de other                      │
//     │ if let P = e {}   │ match e { P => {}, _ => () }                        │
//     │ impl Trait        │ <T: Trait>                                           │
//     │ async fn -> T     │ fn -> impl Future<Output=T>                          │
//     │ fut.await         │ poll loop hasta Ready                                │
//     │ #[derive(X)]      │ impl X for Type { ... }                             │
//     └───────────────────┴──────────────────────────────────────────────────────┘
