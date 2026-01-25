#[test]
fn index() {
    associated_types::custom_basic_iterator();
    associated_types::iterator_from_container();
    associated_types::iterator_string_type();

    custom_iterators::use_iter_immutable();
    custom_iterators::use_iter_mut_mutable();
    custom_iterators::use_into_iter_consume();

    trait_intoiterator::trait_intoiterator_basic();
    trait_intoiterator::trait_intoiterator_in_for_loop();
    trait_intoiterator::trait_intoiterator_with_adapters();

    fundamentals::what_is_an_iterator();
    fundamentals::safety_advantage();
    fundamentals::readability_advantage();
    fundamentals::composability_advantage();

    iterator_types::iter_immutable_borrow();
    iterator_types::iter_mut_mutable_borrow();
    iterator_types::into_iter_takes_ownership();

    adapters::map_transformation();
    adapters::filter_predicate();
    adapters::take_skip();
    adapters::enumerate_with_indices();
    adapters::chain_concatenate();
    adapters::zip_pair();

    consumers::collect_create_collection();
    consumers::sum_sum_elements();
    consumers::fold_accumulator();
    consumers::find_first_element();
    consumers::any_all_predicates();
    consumers::count_quantity_elements();

    lazy_evaluation::lazy_does_no_work();
    lazy_evaluation::lazy_computes_only_necessary();
    lazy_evaluation::composition_without_intermediates();

    println!("\n✅ All iterator tests executed\n");
}

//
// ═══════════════════════════════════════════════════════════════════════════
// ITERATOR TRAIT
// ═══════════════════════════════════════════════════════════════════════════
// An iterator is an object that implements the `Iterator` trait.
// Its main function is `next()`, which returns `Option<Item>`.

//   struct Iterator { ... state ... }
//
//   impl Iterator for Iterator {
//       type Item = T;
//
//       fn next(&mut self) -> Option<T> {
//           // 1. Calculate next value
//           // 2. &mut self: Update internal state
//           // 3. Return Some(value) or None
//       }
//   }
// ─────────────────────────────────────────────────────────────────────────
// PROVIDED/DEFAULT METHODS ON TRAIT
// ─────────────────────────────────────────────────────────────────────────
//
// The `Iterator` trait provides some default methods.
// We only need to implement the `next()` method, and the other methods
// are implemented by default. Some of these methods are:

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
// MODULE: CONCRETE ASSOCIATED TYPES
// ═══════════════════════════════════════════════════════════════════════════
// An iterator has two concrete associated types:
// 1. The type of data it stores
// 2. The type of data it returns on each call to next()

#[cfg(test)]
mod associated_types {

    // ─────────────────────────────────────────────────────────────────────
    // BASIC ITERATOR: MyIntoIter
    // ─────────────────────────────────────────────────────────────────────
    /// Iterator that consumes a vector and returns owned elements (type: i32)
    struct MyIntoIter {
        // type of data the iterator stores
        items: Vec<i32>,
        index: usize,
    }

    /*
    for other types use other iterator structs e.g.: MyIntoIterI8 with its type Item = i8
    */
    impl Iterator for MyIntoIter {
        //
        // type of data the iterator returns
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

    /// Test: Basic usage of custom iterator
    #[test]
    pub fn custom_basic_iterator() {
        let mut iter = MyIntoIter {
            items: vec![1, 2, 3],
            index: 0,
        };

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None); // When exhausted
    }

    // ─────────────────────────────────────────────────────────────────────
    // CONTAINER WITH INTEGRATED ITERATOR
    // ─────────────────────────────────────────────────────────────────────
    /// A container that exposes a custom iterator
    struct MyData {
        // vector owned by the container
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

    /// Test: Iterator integrated in container
    #[test]
    pub fn iterator_from_container() {
        let data = MyData::new(vec![1, 2, 3]);
        let mut iter = data.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    // ─────────────────────────────────────────────────────────────────────
    // ITERATOR WITH DIFFERENT TYPE
    // ─────────────────────────────────────────────────────────────────────
    /// Iterator that returns strings (example with different type)
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

    /// Test: Iterator with different associated type (String)
    #[test]
    pub fn iterator_string_type() {
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
// COMPLETE EXAMPLE: THREE ITERATOR TYPES IN A CUSTOM CONTAINER
// ═══════════════════════════════════════════════════════════════════════════
//
// Complete example:
// Three types of iterators in a custom container
// - iter()       → &T       (Immutable reference)
// - iter_mut()   → &mut T   (Mutable reference)
// - into_iter()  → T        (Value / Ownership)
//

#[cfg(test)]
mod custom_iterators {

    /// A custom container that implements Iterator
    #[derive(Clone)]
    struct MyContainer {
        items: Vec<i32>,
    }

    impl MyContainer {
        fn new(items: Vec<i32>) -> Self {
            MyContainer { items }
        }

        /// Immutable iterator: &T
        fn iter(&self) -> MyIter<'_> {
            MyIter {
                items: &self.items,
                index: 0,
            }
        }

        /// Mutable iterator: &mut T
        fn iter_mut(&'_ mut self) -> MyIterMut<'_> {
            MyIterMut {
                items: &mut self.items,
            }
        }

        /// Consuming iterator: T (ownership)
        fn into_iter(self) -> MyIntoIter {
            MyIntoIter {
                items: self.items,
                index: 0,
            }
        }
    }

    // Immutable references &T
    struct MyIter<'a> {
        // immutable reference to the vector
        items: &'a Vec<i32>,
        index: usize,
    }

    impl<'a> Iterator for MyIter<'a> {
        // immutable reference of each item in the vector
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

    // Mutable references &mut T
    struct MyIterMut<'a> {
        items: &'a mut [i32],
    }

    impl<'a> Iterator for MyIterMut<'a> {
        type Item = &'a mut i32;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.items.is_empty() {
                // advanced trick: to take mutable references when &mut self 
                // already has one, we use std::mem::take
                let items = std::mem::take(&mut self.items);
                let (first, rest) = items.split_first_mut()?;
                self.items = rest;
                Some(first)
            } else {
                None
            }
        }
    }

    // Value / Ownership T
    struct MyIntoIter {
        // vector owned by the iterator
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for MyIntoIter {
        // owned value of the vector returned by the iterator
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
    // USAGE OF THE THREE ITERATORS
    // ─────────────────────────────────────────────────────────────────────

    #[test]
    pub fn use_iter_immutable() {
        let container = MyContainer::new(vec![10, 20, 30]);

        // Reading: x is &i32
        let result: Vec<_> = container
            .iter()
            .map(|&x| x * 2) // Dereference
            .collect();

        assert_eq!(result, vec![20, 40, 60]);
        // container still exists
        assert_eq!(container.items, vec![10, 20, 30]);
    }

    #[test]
    pub fn use_iter_mut_mutable() {
        let mut container = MyContainer::new(vec![10, 20, 30]);

        // Modification: x is &mut i32
        for x in container.iter_mut() {
            *x *= 2; // Modify in place
        }

        assert_eq!(container.items, vec![20, 40, 60]);
        // container still exists and is modified
    }

    #[test]
    pub fn use_into_iter_consume() {
        let container = MyContainer::new(vec![10, 20, 30]);

        // Consumption: x is i32 (ownership)
        let result: Vec<_> = container
            .into_iter()
            .map(|x| x * 2) // Without dereferencing
            .collect();

        assert_eq!(result, vec![20, 40, 60]);
        // container NO LONGER EXISTS - it was consumed
        // println!("{:?}", container);  // ERROR
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MODULE: IntoIterator TRAIT - .into_iter()
// ═══════════════════════════════════════════════════════════════════════════
// IntoIterator is the official Rust trait for implementing iterators.
// Only .into_iter() has an official trait; .iter() and .iter_mut() are conventions.

#[cfg(test)]
mod trait_intoiterator {
    /// Data type that implements IntoIterator
    struct MyData {
        items: Vec<i32>,
    }

    impl MyData {
        fn new(items: Vec<i32>) -> Self {
            MyData { items }
        }
    }

    /// Custom iterator that returns owned elements
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

    /// Implementation of the IntoIterator trait
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

    /// Test: Usage of IntoIterator trait with .into_iter()
    #[test]
    pub fn trait_intoiterator_basic() {
        let data = MyData::new(vec![1, 2, 3]);
        let mut iter = data.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    /// Test: IntoIterator in for loops (syntactic sugar)
    #[test]
    pub fn trait_intoiterator_in_for_loop() {
        let data = MyData::new(vec![10, 20, 30]);
        let mut sum = 0;

        // The compiler converts `for x in data` to `data.into_iter()`
        for x in data {
            sum += x;
        }

        assert_eq!(sum, 60);
    }

    /// Test: Full consumption with IntoIterator and adapters
    #[test]
    pub fn trait_intoiterator_with_adapters() {
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
// MODULE: FUNDAMENTALS OF ITERATORS
// ─────────────────────────────────────────────────────────────────────────
// Basic concepts: what they are, advantages, and main characteristics
// of iterators in Rust.

#[cfg(test)]
mod fundamentals {
    /// An iterator is an object that implements the Iterator trait
    /// The heart of it is the next() method which returns Option<Item>
    #[test]
    pub fn what_is_an_iterator() {
        let vec = vec![1, 2, 3];
        let mut iter = vec.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None); // When exhausted
    }

    /// With iterators: NO index out of bounds
    #[test]
    pub fn safety_advantage() {
        let vec = vec![1, 2, 3];

        // With traditional for (index):
        // for i in 0..10 { vec[i] }  // ← Would panic if i >= 3

        // With iterator:
        let count = vec.iter().count();
        assert_eq!(count, 3); // Safe, no panics
    }

    /// Declarative vs imperative form
    #[test]
    pub fn readability_advantage() {
        let numbers = vec![1, 2, 3, 4, 5];

        // DECLARATIVE (idiomatic Rust):
        let result: Vec<_> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0) // impl Iterator
            .map(|&x| x * 2) // impl Iterator
            .collect();

        assert_eq!(result, vec![4, 8]); // Clear code: "what" not "how"
    }

    /// Multiple operations can be chained easily
    #[test]
    pub fn composability_advantage() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let result: Vec<_> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0) // Even numbers
            .map(|&x| x * x) // Square
            .take(2) // First 2
            .collect();

        assert_eq!(result, vec![4, 16]); // [2², 4²]
    }
}

// ─────────────────────────────────────────────────────────────────────────
// NAMING CONVENTION
// ─────────────────────────────────────────────────────────────────────────
//
//   1. .iter()        → &T       (Immutable reference)
//      Read-only. The original container remains intact.
//
//   2. .iter_mut()    → &mut T   (Mutable reference)
//      Read/Write. You can modify elements in-place.
//
//   3. .into_iter()   → T        (Value / Ownership)
//      Consumption. The original container is destroyed/moved.
//
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod iterator_types {
    /// .iter() borrows elements immutably
    #[test]
    pub fn iter_immutable_borrow() {
        let numbers = vec![1, 2, 3];
        let doubled: Vec<_> = numbers.iter().map(|&x| x * 2).collect();

        assert_eq!(doubled, vec![2, 4, 6]);
        assert_eq!(numbers, vec![1, 2, 3]); // numbers still exists
    }

    /// .iter_mut() borrows mutably - we can modify
    #[test]
    pub fn iter_mut_mutable_borrow() {
        let mut numbers = vec![1, 2, 3];

        for n in numbers.iter_mut() {
            *n *= 2; // Modify each element
        }

        assert_eq!(numbers, vec![2, 4, 6]);
        // numbers still exists and is modified
    }

    /// .into_iter() CONSUMES the vector - takes ownership
    #[test]
    pub fn into_iter_takes_ownership() {
        let numbers = vec![1, 2, 3];
        let doubled: Vec<_> = numbers.into_iter().map(|x| x * 2).collect();

        assert_eq!(doubled, vec![2, 4, 6]);
        // numbers NO LONGER EXISTS - it was consumed
        // println!("{:?}", numbers);  // ← ERROR: value borrowed after move
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE 3: ADAPTERS - Lazy Transformations
// ══════════════════════════════════════════════════════════════════════════════
//
// Adapters take an iterator and return ANOTHER modified iterator.
// They are LAZY: they do nothing until a consumer is called.
//
// ─────────────────────────────────────────────────────────────────────────
// TRANSFORMATION PIPELINE
// ─────────────────────────────────────────────────────────────────────────
//
//   Data      Iterator    Adapter(Map)   Adapter(Filter)   Consumer
//   [1,2,3] ──► iter() ───► map(x*2) ────► filter(>2) ─────► collect()
//                                                                  │
//                                                                  ▼
//                                                                [4, 6]
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod adapters {
    /// map() applies a function to each element
    #[test]
    pub fn map_transformation() {
        let numbers = vec![1, 2, 3];

        let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);

        // map() can be chained
        let result: Vec<_> = numbers.iter()
            .map(|&x| x * 2)
            .map(|x| x + 1)
            .collect();
        assert_eq!(result, vec![3, 5, 7]);
    }

    /// filter() keeps only those that meet the condition
    #[test]
    pub fn filter_predicate() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
        assert_eq!(evens, vec![&2, &4, &6]);
    }

    /// take(n) and skip(n) for slicing
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

    /// enumerate() adds index to each element: (i, val)
    #[test]
    pub fn enumerate_with_indices() {
        let letters = vec!["a", "b", "c"];

        let with_index: Vec<_> = letters
            .iter()
            .enumerate()
            .map(|(i, &letter)| (i, letter))
            .collect();

        assert_eq!(with_index, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    /// chain() concatenates two iterators
    #[test]
    pub fn chain_concatenate() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];

        let combined: Vec<_> = vec1.iter().chain(vec2.iter()).collect();
        assert_eq!(combined, vec![&1, &2, &3, &4, &5, &6]);
    }

    /// zip() pairs elements from two iterators
    #[test]
    pub fn zip_pair() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec!["a", "b", "c"];

        let pairs: Vec<_> = vec1.iter().zip(vec2.iter()).collect();
        assert_eq!(pairs, vec![(&1, &"a"), (&2, &"b"), (&3, &"c")]);
    }
}

//
// ─────────────────────────────────────────────────────────────────────────
// CONSUMERS
// ─────────────────────────────────────────────────────────────────────────

// Consumers "pull" from the iterator to process elements.
// They are terminal operations.
//
//   • collect()  → Transform iterator into collection (Vec, HashMap...)
//   • sum()      → Sum all elements
//   • fold()     → Reduce to single value (accumulator)
//   • for_each() → Execute side effect per element
//   • find()     → Search for element (returns Option)
//

// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod consumers {
    /// collect() gathers elements into a collection
    #[test]
    pub fn collect_create_collection() {
        let numbers = vec![1, 2, 3];
        let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    /// sum() sums all elements
    #[test]
    pub fn sum_sum_elements() {
        let numbers = vec![1, 2, 3, 4, 5];
        let total: i32 = numbers.iter().sum();
        assert_eq!(total, 15);
    }

    /// fold(init, fn) reduce to a value by accumulating
    #[test]
    pub fn fold_accumulator() {
        let numbers = vec![1, 2, 3, 4];

        // Product: 1 * 1 * 2 * 3 * 4 = 24
        let product = numbers.iter().fold(1, |acc, &x| acc * x);
        assert_eq!(product, 24);

        // Concatenation
        let sum_str = numbers
            .iter()
            .fold(String::new(), |acc, x| format!("{}{}", acc, x));
        assert_eq!(sum_str, "1234");
    }

    /// find(pred) finds the first element that meets condition
    #[test]
    pub fn find_first_element() {
        let numbers = vec![1, 2, 3, 4, 5];

        let first_even = numbers.iter().find(|&&x| x % 2 == 0);
        assert_eq!(first_even, Some(&2));

        let not_found = numbers.iter().find(|&&x| x > 100);
        assert_eq!(not_found, None);
    }

    /// any() and all() check predicates
    #[test]
    pub fn any_all_predicates() {
        let numbers = vec![1, 3, 5, 7];
        assert!(!numbers.iter().any(|x| x % 2 == 0)); // Any even? No

        let evens = vec![2, 4, 6];
        assert!(evens.iter().all(|x| x % 2 == 0)); // All even? Yes
    }

    /// count() counts elements
    #[test]
    pub fn count_quantity_elements() {
        let numbers = vec![1, 2, 3, 4, 5];
        let count = numbers.iter().count();
        assert_eq!(count, 5);
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// LAZY EVALUATION - The fundamental principle
// ══════════════════════════════════════════════════════════════════════════════
//
// Iterators in Rust are lazy. They do nothing until asked.
// This allows massive optimizations and working with infinite sequences.
//
// ─────────────────────────────────────────────────────────────────────────
// LAZINESS IN ACTION
// ─────────────────────────────────────────────────────────────────────────
//
//   let iter = (1..).map(|x| x * 2);  // Infinite range, infinite map
//                                     // ZERO cost here!
//
//   iter.take(3).collect();           // Only 3 values are calculated here
//
//   1 ──(*2)──► 2
//   2 ──(*2)──► 4
//   3 ──(*2)──► 6
//   ... (the rest of infinity is never touched)
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod lazy_evaluation {
    /// Adapters WITHOUT a consumer do NOTHING
    #[test]
    pub fn lazy_does_no_work() {
        let numbers = vec![1, 2, 3];

        // map() is lazy - only describes what to do
        let _lazy_map = numbers.iter().map(|x| x * 2);
        // Nothing happened yet. The vector wasn't traversed.

        // To make it work, we need a consumer
        let result: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    /// take(n) is very efficient with large/infinite ranges
    #[test]
    pub fn lazy_computes_only_necessary() {
        let big_range = 1..1_000_000;

        // With laziness: only generates 5 numbers
        let first_five: Vec<_> = big_range.take(5).collect();
        assert_eq!(first_five, vec![1, 2, 3, 4, 5]);

        // The rest (999995 numbers) were never generated or stored in memory
    }

    /// Efficient composition without intermediate allocations
    #[test]
    pub fn composition_without_intermediates() {
        let numbers = vec![1, 2, 3, 4, 5];

        // With iterators (lazy composition):
        // Compiles to a single efficient loop, without intermediate vectors.
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
// Shorthand forms for calling .iter(), .iter_mut(), .into_iter() methods
// collection can be any type that implements those methods or any of them
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

        // for x in &v is equivalent to for x in v.iter()
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

        // for x in v consumes the vector (into_iter)
        let v2 = vec![1, 2, 3];
        let mut sum3 = 0;
        for x in v2 {
            // v2.into_iter()
            sum3 += x;
        }
        // v2 is no longer valid
        assert_eq!(sum3, 6);

        // for x in &mut v allows modification
        let mut v3 = vec![1, 2, 3];
        for x in &mut v3 {
            *x *= 2;
        }
        assert_eq!(v3, vec![2, 4, 6]);

        println!("  ✅ loops::loops");
    }
}
