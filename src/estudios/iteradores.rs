#[test]
fn indice() {
    tipos_asociados::iterador_personalizado_basico();
    tipos_asociados::iterador_desde_contenedor();
    tipos_asociados::iterador_tipo_string();

    custom_iterators::uso_iter_inmutable();
    custom_iterators::uso_iter_mut_mutable();
    custom_iterators::uso_into_iter_consume();

    trait_intoiterator::trait_intoiterator_basico();
    trait_intoiterator::trait_intoiterator_en_for_loop();
    trait_intoiterator::trait_intoiterator_con_adaptores();

    fundamentos::que_es_un_iterador();
    fundamentos::ventaja_seguridad();
    fundamentos::ventaja_legibilidad();
    fundamentos::ventaja_composabilidad();

    tipos_iteradores::iter_prestamo_inmutable();
    tipos_iteradores::iter_mut_prestamo_mutable();
    tipos_iteradores::into_iter_toma_posesion();

    adaptores::map_transformacion();
    adaptores::filter_predicado();
    adaptores::take_skip();
    adaptores::enumerate_con_indices();
    adaptores::chain_concatenar();
    adaptores::zip_emparejar();

    consumidores::collect_crear_coleccion();
    consumidores::sum_suma_elementos();
    consumidores::fold_acumulador();
    consumidores::find_primer_elemento();
    consumidores::any_all_predicados();
    consumidores::count_cantidad_elementos();

    lazy_evaluation::lazy_no_hace_trabajo();
    lazy_evaluation::lazy_solo_computa_necesario();
    lazy_evaluation::composicion_sin_intermedios();

    println!("\n✅ Todos los tests de iteradores ejecutados\n");
}

//
// ═══════════════════════════════════════════════════════════════════════════
// TRAIT ITERATOR
// ═══════════════════════════════════════════════════════════════════════════
// Un iterador es un objeto que implementa el trait `Iterator`.
// Su función principal es `next()`, que devuelve `Option<Item>`.

//   struct Iterador { ... estado ... }
//
//   impl Iterator for Iterador {
//       type Item = T;
//
//       fn next(&mut self) -> Option<T> {
//           // 1. Calcula siguiente valor
//           // 2. &mut self: Actualiza estado interno
//           // 3. Retorna Some(valor) o None
//       }
//   }
// ─────────────────────────────────────────────────────────────────────────
// TRAIT PROVIDED/DEFAULT METHODS
// ─────────────────────────────────────────────────────────────────────────
//
// El trait `Iterator` proporciona algunos métodos por defecto.
// Solo necesitamos implementar el método `next()`, y los demás métodos
// se implementan por defecto. Algunos de estos métodos son:

// * filter()
// * skip()
// * take()
// * find()
// * collect()
// * enumerate()
// * zip()
// * any()
// * all()
// * ...

// ═══════════════════════════════════════════════════════════════════════════
// MÓDULO: TIPOS ASOCIADOS CONCRETOS
// ═══════════════════════════════════════════════════════════════════════════
// Un iterador tiene dos tipos asociados concretos:
// 1. El tipo de dato que almacena
// 2. El tipo de dato que devuelve en cada llamada a next()

#[cfg(test)]
mod tipos_asociados {
    use std::marker::PhantomData;

    // ─────────────────────────────────────────────────────────────────────
    // ITERADOR BÁSICO: MyIntoIter
    // ─────────────────────────────────────────────────────────────────────
    /// Iterador que consume un vector y devuelve elementos owned (tipo: i32)
    struct MyIntoIter {
        // tipo de dato que almacena el iterador
        items: Vec<i32>,
        index: usize,
    }

    /*
    para otros tipos usar otros struct iteradores ej: MyIntoIterI8 con su type Item = i8
    */
    impl Iterator for MyIntoIter {
        //
        // tipo de dato que devuelve el iterador
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = self.items[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    /// Test: Uso básico del iterador personalizado
    #[test]
    pub fn iterador_personalizado_basico() {
        let mut iter = MyIntoIter {
            items: vec![1, 2, 3],
            index: 0,
        };

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None); // Cuando se agota
    }

    // ─────────────────────────────────────────────────────────────────────
    // CONTENEDOR CON ITERADOR INTEGRADO
    // ─────────────────────────────────────────────────────────────────────
    /// Un contenedor que expone un iterador personalizado
    struct MyData {
        // vector owned por el contenedor
        items: Vec<i32>,
    }

    impl MyData {
        fn new(items: Vec<i32>) -> Self {
            MyData { items }
        }

        fn into_iter(self) -> MyIntoIter {
            MyIntoIter {
                items: self.items,
                index: 0,
            }
        }
    }

    /// Test: Iterador integrado en contenedor
    #[test]
    pub fn iterador_desde_contenedor() {
        let data = MyData::new(vec![1, 2, 3]);
        let mut iter = data.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    // ─────────────────────────────────────────────────────────────────────
    // ITERADOR CON TIPO DIFERENTE
    // ─────────────────────────────────────────────────────────────────────
    /// Iterador que devuelve strings (ejemplo con tipo diferente)
    struct MyStringIter {
        items: Vec<String>,
        index: usize,
    }

    impl Iterator for MyStringIter {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = self.items[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    /// Test: Iterador con tipo asociado diferente (String)
    #[test]
    pub fn iterador_tipo_string() {
        let mut iter = MyStringIter {
            items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            index: 0,
        };

        assert_eq!(iter.next(), Some("a".to_string()));
        assert_eq!(iter.next(), Some("b".to_string()));
        assert_eq!(iter.next(), Some("c".to_string()));
        assert_eq!(iter.next(), None);
    }
}
// ═══════════════════════════════════════════════════════════════════════════
// EJEMPLO COMPLETO: TRES TIPOS DE ITERADORES EN UN CONTENEDOR PERSONALIZADO
// ═══════════════════════════════════════════════════════════════════════════
//
// Ejemplo completo:
// Tres tipos de iteradores en un contenedor personalizado
// - iter()       → &T       (Referencia inmutable)
// - iter_mut()   → &mut T   (Referencia mutable)
// - into_iter()  → T        (Valor / Ownership)
//

#[cfg(test)]
mod custom_iterators {

    /// Un contenedor personalizado que implementa Iterator
    #[derive(Clone)]
    struct MyContainer {
        items: Vec<i32>,
    }

    impl MyContainer {
        fn new(items: Vec<i32>) -> Self {
            MyContainer { items }
        }

        /// Iterador inmutable: &T
        fn iter(&self) -> MyIter {
            MyIter {
                items: &self.items,
                index: 0,
            }
        }

        /// Iterador mutable: &mut T
        fn iter_mut(&mut self) -> MyIterMut {
            MyIterMut {
                items: &mut self.items,
            }
        }

        /// Iterador que consume: T (ownership)
        fn into_iter(self) -> MyIntoIter {
            MyIntoIter {
                items: self.items,
                index: 0,
            }
        }
    }

    // Referencias inmutables &T
    struct MyIter<'a> {
        // referencia inmutable al vector
        items: &'a Vec<i32>,
        index: usize,
    }

    impl<'a> Iterator for MyIter<'a> {
        // referencia inmutable de cada item del vector
        type Item = &'a i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = &self.items[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    // Referencias mutables &mut T
    struct MyIterMut<'a> {
        items: &'a mut [i32],
    }

    impl<'a> Iterator for MyIterMut<'a> {
        type Item = &'a mut i32;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.items.is_empty() {
                // advanced trick: para tomar referencias mutables cuando &mut self ya tiene tomada una referencia mutable, usamos std::mem::take
                let items = std::mem::take(&mut self.items);
                let (first, rest) = items.split_first_mut()?;
                self.items = rest;
                Some(first)
            } else {
                None
            }
        }
    }

    // Valor / Ownership T
    struct MyIntoIter {
        // vector owned por el iterador
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for MyIntoIter {
        // valor owned del vector devuelto por el iterador
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = self.items[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    // USO DE LOS TRES ITERADORES
    // ─────────────────────────────────────────────────────────────────────

    #[test]
    pub fn uso_iter_inmutable() {
        let container = MyContainer::new(vec![10, 20, 30]);

        // Lectura: x es &i32
        let result: Vec<_> = container
            .iter()
            .map(|&x| x * 2) // Desreferenciamos
            .collect();

        assert_eq!(result, vec![20, 40, 60]);
        // container aún existe
        assert_eq!(container.items, vec![10, 20, 30]);
    }

    #[test]
    pub fn uso_iter_mut_mutable() {
        let mut container = MyContainer::new(vec![10, 20, 30]);

        // Modificación: x es &mut i32
        for x in container.iter_mut() {
            *x *= 2; // Modificamos en place
        }

        assert_eq!(container.items, vec![20, 40, 60]);
        // container aún existe y está modificado
    }

    #[test]
    pub fn uso_into_iter_consume() {
        let container = MyContainer::new(vec![10, 20, 30]);

        // Consumo: x es i32 (ownership)
        let result: Vec<_> = container
            .into_iter()
            .map(|x| x * 2) // Sin desreferenciar
            .collect();

        assert_eq!(result, vec![20, 40, 60]);
        // container NO EXISTE más - fue consumido
        // println!("{:?}", container);  // ERROR
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MÓDULO: TRAIT IntoIterator - .into_iter()
// ═══════════════════════════════════════════════════════════════════════════
// IntoIterator es el trait oficial de Rust para implementar iteradores.
// Solo .into_iter() tiene trait oficial; .iter() e .iter_mut() son convenciones.

#[cfg(test)]
mod trait_intoiterator {
    /// Tipo de datos que implementa IntoIterator
    struct MyData {
        items: Vec<i32>,
    }

    impl MyData {
        fn new(items: Vec<i32>) -> Self {
            MyData { items }
        }
    }

    /// Iterador personalizado que devuelve elementos owned
    struct MyIntoIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for MyIntoIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = self.items[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    /// Implementación del trait IntoIterator
    impl std::iter::IntoIterator for MyData {
        type Item = i32;
        type IntoIter = MyIntoIterator;

        fn into_iter(self) -> MyIntoIterator {
            MyIntoIterator {
                items: self.items,
                index: 0,
            }
        }
    }

    /// Test: Uso del trait IntoIterator con .into_iter()
    #[test]
    pub fn trait_intoiterator_basico() {
        let data = MyData::new(vec![1, 2, 3]);
        let mut iter = data.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    /// Test: IntoIterator en bucles for (azúcar sintáctico)
    #[test]
    pub fn trait_intoiterator_en_for_loop() {
        let data = MyData::new(vec![10, 20, 30]);
        let mut sum = 0;

        // El compilador convierte `for x in data` en `data.into_iter()`
        for x in data {
            sum += x;
        }

        assert_eq!(sum, 60);
    }

    /// Test: Consumo total con IntoIterator y adaptores
    #[test]
    pub fn trait_intoiterator_con_adaptores() {
        let data = MyData::new(vec![1, 2, 3, 4, 5]);

        let result: Vec<_> = data
            .into_iter()
            .filter(|&x| x % 2 == 0)
            .map(|x| x * 10)
            .collect();

        assert_eq!(result, vec![20, 40]);
    }
}

// ─────────────────────────────────────────────────────────────────────────
// MÓDULO: FUNDAMENTOS DE ITERADORES
// ─────────────────────────────────────────────────────────────────────────
// Conceptos básicos: qué son, ventajas y características principales
// de los iteradores en Rust.

#[cfg(test)]
mod fundamentos {
    /// Un iterador es un objeto que implementa la trait Iterator
    /// El corazón es el método next() que devuelve Option<Item>
    #[test]
    pub fn que_es_un_iterador() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None); // Cuando se agota
    }

    /// Con iteradores: NO hay índices fuera de rango
    #[test]
    pub fn ventaja_seguridad() {
        let vec = vec![1, 2, 3];

        // Con for tradicional (índice):
        // for i in 0..10 { vec[i] }  // ← Paniquearía si i >= 3

        // Con iterador:
        let count = vec.iter().count();
        assert_eq!(count, 3); // Seguro, sin panics
    }

    /// Forma declarativa vs imperativa
    #[test]
    pub fn ventaja_legibilidad() {
        let numbers = vec![1, 2, 3, 4, 5];

        // DECLARATIVA (idiomática Rust):
        let resultado: Vec<_> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0) // impl Iterator
            .map(|&x| x * 2) // impl Iterator
            .collect();

        assert_eq!(resultado, vec![4, 8]); // Código claro: "qué" no "cómo"
    }

    /// Se pueden encadenar múltiples operaciones fácilmente
    #[test]
    pub fn ventaja_composabilidad() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let result: Vec<_> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0) // Pares
            .map(|&x| x * x) // Elevar al cuadrado
            .take(2) // Primeros 2
            .collect();

        assert_eq!(result, vec![4, 16]); // [2², 4²]
    }
}

// ─────────────────────────────────────────────────────────────────────────
// CONVENCIÓN DE NOMBRES
// ─────────────────────────────────────────────────────────────────────────
//
//   1. .iter()        → &T       (Referencia inmutable)
//      Solo lectura. El contenedor original sigue intacto.
//
//   2. .iter_mut()    → &mut T   (Referencia mutable)
//      Lectura/Escritura. Puedes modificar los elementos in-place.
//
//   3. .into_iter()   → T        (Valor / Ownership)
//      Consumo. El contenedor original se destruye/mueve.
//
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tipos_iteradores {
    /// .iter() presta los elementos inmutablemente
    #[test]
    pub fn iter_prestamo_inmutable() {
        let numbers = vec![1, 2, 3];
        let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();

        assert_eq!(doubled, vec![2, 4, 6]);
        assert_eq!(numbers, vec![1, 2, 3]); // numbers aún existe
    }

    /// .iter_mut() presta mutablemente - podemos modificar
    #[test]
    pub fn iter_mut_prestamo_mutable() {
        let mut numbers = vec![1, 2, 3];

        for n in numbers.iter_mut() {
            *n *= 2; // Modificamos cada elemento
        }

        assert_eq!(numbers, vec![2, 4, 6]);
        // numbers aún existe y está modificado
    }

    /// .into_iter() CONSUME el vector - toma posesión
    #[test]
    pub fn into_iter_toma_posesion() {
        let numbers = vec![1, 2, 3];
        let doubled: Vec<_> = numbers.into_iter().map(|x| x * 2).collect();

        assert_eq!(doubled, vec![2, 4, 6]);
        // numbers NO EXISTE más - fue consumido
        // println!("{:?}", numbers);  // ← ERROR: value borrowed after move
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MÓDULO 3: ADAPTORES - Transformaciones Lazy
// ══════════════════════════════════════════════════════════════════════════════
//
// Los adaptores toman un iterador y devuelven OTRO iterador modificado.
// Son LAZY: no hacen nada hasta que se llama a un consumidor.
//
// ─────────────────────────────────────────────────────────────────────────
// PIPELINE DE TRANSFORMACIÓN
// ─────────────────────────────────────────────────────────────────────────
//
//   Datos     Iterador      Adaptor(Map)   Adaptor(Filter)   Consumidor
//   [1,2,3] ──► iter() ───► map(x*2) ────► filter(>2) ─────► collect()
//                                                                  │
//                                                                  ▼
//                                                                [4, 6]
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod adaptores {
    /// map() aplica una función a cada elemento
    #[test]
    pub fn map_transformacion() {
        let numbers = vec![1, 2, 3];

        let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);

        // map() se puede encadenar
        let result: Vec<_> = numbers.iter().map(|&x| x * 2).map(|x| x + 1).collect();
        assert_eq!(result, vec![3, 5, 7]);
    }

    /// filter() mantiene solo los que cumplen la condición
    #[test]
    pub fn filter_predicado() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
        assert_eq!(evens, vec![&2, &4, &6]);
    }

    /// take(n) y skip(n) para slicing
    #[test]
    pub fn take_skip() {
        let numbers = vec![1, 2, 3, 4, 5];

        // take(2): [1, 2]
        let first_two: Vec<_> = numbers.iter().take(2).collect();
        assert_eq!(first_two, vec![&1, &2]);

        // skip(2): [3, 4, 5]
        let skip_two: Vec<_> = numbers.iter().skip(2).collect();
        assert_eq!(skip_two, vec![&3, &4, &5]);

        // skip(2) + take(3): [3, 4, 5]
        let range: Vec<_> = (1..=10).skip(2).take(3).collect();
        assert_eq!(range, vec![3, 4, 5]);
    }

    /// enumerate() añade índice a cada elemento: (i, val)
    #[test]
    pub fn enumerate_con_indices() {
        let letters = vec!["a", "b", "c"];

        let with_index: Vec<_> = letters
            .iter()
            .enumerate()
            .map(|(i, &letter)| (i, letter))
            .collect();

        assert_eq!(with_index, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    /// chain() concatena dos iteradores
    #[test]
    pub fn chain_concatenar() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];

        let combined: Vec<_> = vec1.iter().chain(vec2.iter()).collect();
        assert_eq!(combined, vec![&1, &2, &3, &4, &5, &6]);
    }

    /// zip() empareja elementos de dos iteradores
    #[test]
    pub fn zip_emparejar() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec!["a", "b", "c"];

        let pairs: Vec<_> = vec1.iter().zip(vec2.iter()).collect();
        assert_eq!(pairs, vec![(&1, &"a"), (&2, &"b"), (&3, &"c")]);
    }
}

//
// ─────────────────────────────────────────────────────────────────────────
// CONSUMIDORES
// ─────────────────────────────────────────────────────────────────────────

// Los consumidores "tiran" del iterador para procesar los elementos.
// Son operaciones terminales.
//
//   • collect()  → Transforma iterador en colección (Vec, HashMap...)
//   • sum()      → Suma todos los elementos
//   • fold()     → Reduce a un solo valor (acumulador)
//   • for_each() → Ejecuta efecto secundario por elemento
//   • find()     → Busca un elemento (retorna Option)
//

// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod consumidores {
    /// collect() reúne los elementos en una colección
    #[test]
    pub fn collect_crear_coleccion() {
        let numbers = vec![1, 2, 3];
        let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    /// sum() suma todos los elementos
    #[test]
    pub fn sum_suma_elementos() {
        let numbers = vec![1, 2, 3, 4, 5];
        let total: i32 = numbers.iter().sum();
        assert_eq!(total, 15);
    }

    /// fold(init, fn) reduce a un valor acumulando
    #[test]
    pub fn fold_acumulador() {
        let numbers = vec![1, 2, 3, 4];

        // Producto: 1 * 1 * 2 * 3 * 4 = 24
        let product = numbers.iter().fold(1, |acc, &x| acc * x);
        assert_eq!(product, 24);

        // Concatenación
        let sum_str = numbers
            .iter()
            .fold(String::new(), |acc, x| format!("{}{}", acc, x));
        assert_eq!(sum_str, "1234");
    }

    /// find(pred) encuentra el primer elemento que cumple
    #[test]
    pub fn find_primer_elemento() {
        let numbers = vec![1, 2, 3, 4, 5];

        let first_even = numbers.iter().find(|&&x| x % 2 == 0);
        assert_eq!(first_even, Some(&2));

        let not_found = numbers.iter().find(|&&x| x > 100);
        assert_eq!(not_found, None);
    }

    /// any() y all() verifican predicados
    #[test]
    pub fn any_all_predicados() {
        let numbers = vec![1, 3, 5, 7];
        assert!(!numbers.iter().any(|x| x % 2 == 0)); // ¿Algún par? No

        let evens = vec![2, 4, 6];
        assert!(evens.iter().all(|x| x % 2 == 0)); // ¿Todos pares? Sí
    }

    /// count() cuenta los elementos
    #[test]
    pub fn count_cantidad_elementos() {
        let numbers = vec![1, 2, 3, 4, 5];
        let count = numbers.iter().count();
        assert_eq!(count, 5);
    }
}

// ══════════════════════════════════════════════════════════════════════════════
//LAZY EVALUATION - El principio fundamental
// ══════════════════════════════════════════════════════════════════════════════
//
// Los iteradores en Rust son perezosos (lazy). No hacen nada hasta que se les
// pide. Esto permite optimizaciones masivas y trabajar con secuencias infinitas.
//
// ─────────────────────────────────────────────────────────────────────────
// LAZINESS EN ACCIÓN
// ─────────────────────────────────────────────────────────────────────────
//
//   let iter = (1..).map(|x| x * 2);  // Rango infinito, map infinito
//                                     // ¡Costo CERO aquí!
//
//   iter.take(3).collect();           // Solo aquí se calculan 3 valores
//
//   1 ──(*2)──► 2
//   2 ──(*2)──► 4
//   3 ──(*2)──► 6
//   ... (el resto del infinito nunca se toca)
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod lazy_evaluation {
    /// Los adaptores SIN consumidor NO hacen nada
    #[test]
    pub fn lazy_no_hace_trabajo() {
        let numbers = vec![1, 2, 3];

        // map() es lazy - solo describe qué hacer
        let _lazy_map = numbers.iter().map(|x| x * 2);
        // Nada sucedió aún. No se recorrió el vector.

        // Para que funcione, necesitamos consumidor
        let result: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    /// take(n) es muy eficiente con rangos grandes/infinitos
    #[test]
    pub fn lazy_solo_computa_necesario() {
        let big_range = 1..1_000_000;

        // Con laziness: solo genera 5 números
        let first_five: Vec<_> = big_range.take(5).collect();
        assert_eq!(first_five, vec![1, 2, 3, 4, 5]);

        // El resto (999995 números) nunca se generaron ni ocuparon memoria
    }

    /// Composición eficiente sin asignaciones intermedias
    #[test]
    pub fn composicion_sin_intermedios() {
        let numbers = vec![1, 2, 3, 4, 5];

        // Con iteradores (composición lazy):
        // Se compila a un solo loop eficiente, sin vectores intermedios.
        let result: Vec<_> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .collect();

        assert_eq!(result, vec![4, 8]);
    }
}

// ============================================================================
// 5. Syntax Sugar: for x in
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
        let v = vec![1, 2, 3];
        // let x = v.into_iter();

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
