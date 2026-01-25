
#[test]
fn index() {
    // ===== BASICS =====
    what_is_a_closure::what_is_a_closure();
    capture_vs_parameter::capture_vs_parameter();
    
    // ===== CLOSURE TRAITS =====
    fn_trait::fn_trait();
    fn_mut_trait::fn_mut_trait();
    fn_once_trait::fn_once_trait();
    closure_and_stackframe::closure_and_stackframe();
    closure_as_struct::closure_as_struct();
    trait_inheritance_and_families::inheritance();
    trait_inheritance_and_families::traits_matching();
    trait_inheritance_and_families::different_traits();
    
    // ===== MOVE SEMANTICS =====
    move_fn_fn_mut_fn_once::no_move_fn();
    move_fn_fn_mut_fn_once::no_move_fn_mut();
    move_fn_fn_mut_fn_once::move_closure_fn();
    move_fn_fn_mut_fn_once::move_closure_fn_mut();
    move_fn_fn_mut_fn_once::move_closure_fn_once();
    move_fn_fn_mut_fn_once::move_thread();
    move_fn_fn_mut_fn_once::move_return_closure();
    move_fn_fn_mut_fn_once::move_copy_type();
    move_references::with_move_references();
    move_references::without_move_references();
    move_implicit::move_implicit_consumption();
    
    // ===== IMPL vs DYN =====
    impl_dyn::impl_param();
    impl_dyn::impl_return();
    impl_dyn::dyn_param();
    impl_dyn::dyn_return();
    impl_dyn::dyn_collection();
    impl_dyn::dyn_storage();
    
    // ===== PATTERNS =====
    patterns::closure_factories::closure_factories();
    patterns::strategy_pattern::strategy_pattern();
    patterns::middleware_pattern::middleware_pattern();
    patterns::lazy_evaluation::lazy_evaluation();
    patterns::event_listeners::event_listeners();
    patterns::stateful_closures::stateful_closures();
    patterns::currying_partial_application::currying_partial_application();
}

/*
========================================================================
WHAT_IS_A_CLOSURE
========================================================================

    WHAT IS A CLOSURE?
    --------------------------------------------
        CLOSURE = Anonymous function that can CAPTURE its environment

        DIAGRAM:
        ┌─────────────────────────────────────────────────────────────────────────┐
        │  NORMAL FUNCTION:                    CLOSURE:                           │
        │  fn add(a: i32, b: i32) -> i32       let add = |a, b| a + b;            │
        │      a + b                                                              │
        │                                                                         │
        │  KEY DIFFERENCE: Environment capture                                    │
        │                                                                         │
        │      let factor = 10;                                                   │
        │          │                                                              │
        │          ▼                                                              │
        │      ┌───────┐                                                          │
        │      │  10   │  ◄── variable in scope                                   │
        │      └───────┘                                                          │
        │          ▲                                                              │
        │          │ automatic capture                                            │
        │      ┌───┴─────────────────┐                                            │
        │      │ let scale = |x| x * factor;                                      │
        │      │             ▲                                                    │
        │      │             └── uses factor without passing it as parameter      │
        │      └─────────────────────┘                                            │
        │                                                                         │
        │      scale(5)  →  50                                                    │
        └─────────────────────────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod what_is_a_closure {
    #[test]
    pub fn what_is_a_closure() {
        let add = |a, b| a + b;
        assert_eq!(add(2, 3), 5);

        let factor = 10;
        let scale = |x| x * factor;
        assert_eq!(scale(5), 50);
    }
}

/*
========================================================================
CAPTURE_VS_PARAMETER
========================================================================

    CAPTURE vs PARAMETER - They are different things!
    --------------------------------------------
        DIAGRAM:
        ┌─────────────────────────────────────────────────────────────────────────┐
        │   CAPTURE (from environment):                                           │
        │   ┌─────────────────────────────────────────────────────────────────┐   │
        │   │  let multiplier = 2;                                            │   │
        │   │  let f = |x| x * multiplier;                                    │   │
        │   │              ▲       ▲                                          │   │
        │   │              │       └── CAPTURED (comes from environment)      │   │
        │   │              └────────── PARAMETER (passed when calling)        │   │
        │   └─────────────────────────────────────────────────────────────────┘   │
        │                                                                         │
        │   PARAMETER ONLY (without capture):                                     │
        │   ┌─────────────────────────────────────────────────────────────────┐   │
        │   │  let add = |a, b| a + b;   ← Captures nothing                   │   │
        │   │                                                                 │   │
        │   │  // This closure is basically a function!                       │   │
        │   │  // Can be coerced to: fn(i32, i32) -> i32                      │   │
        │   └─────────────────────────────────────────────────────────────────┘   │
        └─────────────────────────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod capture_vs_parameter {
    #[test]
    pub fn capture_vs_parameter() {
        let multiplier = 2;
        let multiply = |x| x * multiplier;
        assert_eq!(multiply(5), 10);

        let add = |a: i32, b: i32| a + b;
        assert_eq!(add(3, 4), 7);
    }
}

/*
========================================================================
CLOSURE AND FUNCTIONS
========================================================================

    Closure: 
    --------------------------------------------
    * es un Fn|FnMut|FnOnce(Args) -> Output
    * puede capturar variables del entorno

    Function Pointer: 
    --------------------------------------------
    * es un fn(Args) -> Output
    * no puede capturar variables del entorno 
    y implementa internamente Fn, entonces puede ser usado como Fn, FnMut o FnOnce
    * no usa impl, es un tipo concreto, no es un trait
*/
#[cfg(test)]
pub mod closure_and_functions {
    #[test]
    pub fn functions_as_parameters() {
        fn greet(f: fn() -> String) -> String { // no usa impl 
            f()
        }

        fn say_hello() -> String {
            return String::from("Hello, world!");
        }

        assert_eq!(greet(say_hello), String::from("Hello, world!"));
    }

    #[test]
    pub fn fn_compatible_with_trait() {
        fn greet(f: impl Fn() -> String) -> String { // usa impl 
            f()
        }

        fn say_hello() -> String {
            return String::from("Hello, world!");
        }

        let say_hi = || {
            return String::from("Hi, world!");
        };

        assert_eq!(greet(say_hello), String::from("Hello, world!"));  // works with fn
        assert_eq!(greet(say_hi), String::from("Hi, world!"));  // works with closure
    }

}

/*
========================================================================
CLOSURE AND STACKFRAME
========================================================================

Cuando se llama a una closure, se crea un stackframe temporal (como cualquier función)

*/
#[cfg(test)]
pub mod closure_and_stackframe {
    #[test]
    pub fn closure_and_stackframe() { }
}

/*
========================================================================
CLASIFICATION AND ANALOGIES WITH REFERENCES
========================================================================
    Fn ~ &T: READ ONLY
        • Captures: 
            * immutable reference, copy, 
            * move: owned value with *READ ONLY* access   (using move)
        • Does not mutate or consume what is captured
        • Can be called multiple times

    FnMut ~ &mut T: MUTABLE 
        • Captures: 
           * mutable reference
           * move: owned value with *MUTABLE* access 
        • Mutates what is captured -> mut keyword required
        • Can be called multiple times

    FnOnce ~ T (ownership): CONSUMES 
        • Captures: ownership of a variable (using move)
        • *CONSUMES* a captured variable
        • Can only be called once
*/

/*
FN TRAIT - READ ONLY
--------------------------------------------
*/
#[cfg(test)]
mod fn_trait {
    #[test]
    pub fn fn_trait() {
        let value = 10;

        let print_value = || {
            assert_eq!(value, 10);
        };

        print_value();
        print_value();
        assert_eq!(value, 10);
    }

    #[test]
    pub fn fn_trait_with_mutable_reference() {
        let mut value = 10;

        let _print_value = || {
            value += 1;
            assert_eq!(value, 11);
        };
    }

    #[test]
    pub fn fn_trait_with_owned_value() {
        let value = 10;

        let _print_value = move|| {
            assert_eq!(value, 10);
        };
    }
}

/*
FN_MUT TRAIT - MUTABLE
--------------------------------------------
*/
#[cfg(test)]
mod fn_mut_trait {
    #[test]
    pub fn fn_mut_trait() {
        let mut counter = 0;

        let mut increment = || {
            counter += 1;
            counter
        };

        assert_eq!(increment(), 1);
        assert_eq!(increment(), 2);
        assert_eq!(increment(), 3);
        assert_eq!(counter, 3);
    }

    #[test]
    pub fn fn_mut_trait_with_owned_value() {
        let mut value = 10;

        let mut _increment = || {
            value += 1;
            value
        };
    }
}

/*
FN_ONCE TRAIT - CONSUMES
--------------------------------------------
*/
#[cfg(test)]
mod fn_once_trait {
    #[test]
    pub fn fn_once_trait() {
        let data = vec![1, 2, 3];

        let consume = || {
            assert_eq!(data, vec![1, 2, 3]);
            drop(data);
        };

        consume();
        // data is dropped, so it cannot be used again
        // consume(); // ✗ ERROR: cannot be called twice
    }
}

/*
=========================================================================
CLOSURE AS STRUCT
=========================================================================

    A closure is an anonymous struct 
        * its fields are the captured variables
        * implements one or more of the traits:
            * Fn: call(&self, args...) -> Output
            * FnMut: call_mut(&mut self, args...) -> Output
            * FnOnce: call_once(self, args...) -> Output
        * running the closure is executing the method of the trait that implements it

    Example:
        let x = String::from("hello");
        let callback = |y: i32| -> String { format!("{} {}", x, y) }

        Conceptually it is:

            struct __AnonymousClosure {
                x: &String,
            }

            impl Fn(i32) -> String for __AnonymousClosure {
                fn call(&self, y: i32) -> String {
                    format!("{} {}", *self.x, y)
                }
            }

    by implementing Fn the compiler also implements FnMut and FnOnce, so it
    can be used in any context that requires one of the traits.
    Implemented Traits: Fn|FnMut|FnOnce(i32) -> String
            
            // internally the compiler generates this code:
            impl FnMut(i32) -> String for __AnonymousClosure {
                fn call_mut(&mut self, y: i32) -> String {
                    self.call(y)    
                }
            }

            // internally the compiler generates this code:
            impl FnOnce(i32) -> String for __AnonymousClosure {
                fn call_once(self, y: i32) -> String {
                    self.call_mut(y)    
                }
            }
        }

*/
#[cfg(test)]
pub mod closure_as_struct {
    #[test]
    pub fn closure_as_struct() { }
}

/*
========================================================================
TRAIT INHERITANCE AND FAMILIES
========================================================================

    Trait Inheritance:
    --------------------------------------------
        Fn : FnMut : FnOnce

    Trait Families:
    --------------------------------------------
        Each closure implements one or more traits according to its signature:
            Fn(Args...) -> Output
            FnMut(Args...) -> Output
            FnOnce(Args...) -> Output
        this is what allows using them as generics, instead of a concrete type.

        Note: 
        - Fn(i32)->i32 and Fn(String)->bool are different traits.
        - You cannot mix them.

            trait Fn(i32)->i32 {
                fn call(&self, arg: i32) -> i32;
            }
            trait Fn(String)->bool {
                fn call(&self, arg: String) -> bool;
            }
*/
#[cfg(test)]
pub mod trait_inheritance_and_families {

    // Fn : FnMut : FnOnce inheritance
    #[test]
    pub fn inheritance() {
        // Accepts any trait Fn, FnMut or FnOnce
        fn fn_param(callback: impl FnOnce(i32) -> i32) {
            callback(1);
            assert!(true);
        }
        
        // Fn : FnMut
        let callback_fn = |x: i32| x + 1;
        fn_param(callback_fn);

        // FnMut : FnOnce
        let mut data = String::from("hello");
        let callback_fn_mut = | x: i32| { 
            data.push('a'); 
            x
        };
        fn_param(callback_fn_mut);

        // FnOnce
        let data2 = String::from("hello");
        let callback_fn_once = move |x: i32| {
            drop(data2);
            x
        };
        fn_param(callback_fn_once);
    }

    // Fn(i32) -> i32 same signature
    pub fn traits_matching() {
        fn fn_param(callback: impl Fn(i32) -> i32) {
            callback(1);
            assert!(true);
        }
        
        // Fn(i32) -> i32
        let callback_fn = |x: i32| x + 1;
        fn_param(callback_fn);
    }

    // different signatures, not implement same traits
    pub fn different_traits() {
        fn _fn_param(callback: impl Fn(i32) -> bool) {
            callback(1);
            assert!(true);
        }
        
        // Fn(i32) -> i32 -> bool different signature
        let _callback_fn = |x: i32| x > 1;
        // _fn_param(callback_fn);   // ✗ ERROR: different signature
    }
}

/*
========================================================================
MOVE
========================================================================

    - Sin move: captura por referencia (& o &mut) según uso
    - Con move: captura ownership de todo lo capturado
    - move implicito: cuando se consume algo dentro del closure, 
    el compilador aplica move automáticamente.

    Move and references:
    --------------------------------------------
        References are Copy, so it creates another reference inside the closure,

    implicit move
    --------------------------------------------
        there are cases where the compiler applies implicit move:
            * When detecting a drop / consumption inside the closure
            * When using closures in threads
            * When returning closures from functions

        Example:
        let x = String::from("hello");
        let c = || { drop (x); }
        let c = move || { drop(x); } // compiler converts it to move


    move and 'static
    --------------------------------------------
        If everything captured with move is 'static then the closure is 'static
        and lives as long as the variable that contains it.

        Useful for:
            * Threads / Async
            * Return from functions / closures
            * Storage in long-lived data structures
*/
#[cfg(test)]
mod move_fn_fn_mut_fn_once {

    #[test]
    pub fn no_move_fn() {
        let data = vec![1, 2, 3];
        let print_len = || data.len();
        assert_eq!(print_len(), 3);
        assert_eq!(print_len(), 3);
    }

    #[test]
    pub fn no_move_fn_mut() {
        let mut data = vec![1, 2, 3];
        let mut add_value = |v| {
            data.push(v);
            return data.len();
        };
        add_value(4);
        let value = add_value(5);
        assert_eq!(value, 5);
    }

    #[test]
    pub fn move_closure_fn() {
        let data = vec![1, 2, 3];
        let print_len = move || data.len();
        assert_eq!(print_len(), 3);
        assert_eq!(print_len(), 3);
    }

    #[test]
    pub fn move_closure_fn_mut() {
        let mut data = vec![1, 2, 3];
        let mut add_value = move |v| {
            data.push(v);
            return data.len();
        };
        add_value(4);
        let value = add_value(5);
        assert_eq!(value, 5);
    }

    #[test]
    pub fn move_closure_fn_once() {
        let data = vec![1, 2, 3];
        let consume_data = move || {
            drop(data);
        };
        consume_data();
        // consume_data(); // ✗ ERROR: cannot be called twice
    }

    #[test]
    pub fn move_thread() {
        let data = vec![1, 2, 3];
        let handle = std::thread::spawn(move || {
            assert_eq!(data, vec![1, 2, 3]);
        });
        handle.join().unwrap();
    }

    #[test]
    pub fn move_return_closure() {
        fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
            move |x| x * factor
        }
        let times_5 = make_multiplier(5);
        assert_eq!(times_5(10), 50);
    }

    #[test]
    pub fn move_copy_type() {
        let number = 42;
        let print_num = move || {
            assert_eq!(number, 42);
        };
        print_num();
        assert_eq!(number, 42);
    }
}

#[cfg(test)]
mod move_references {
    #[test]
    pub fn with_move_references() {
        let text = String::from("Hello, world!");
        let text_ref = &text;
        let print_text = move || {
            // text_ref is a copied reference
            assert_eq!(text_ref, "Hello, world!");
        };
        print_text();
    }

    #[test]
    pub fn without_move_references() {
        let text = String::from("Hello, world!");
        let text_ref = &text;
        let print_text = || {
            // text_ref is a copied reference
            assert_eq!(text_ref, "Hello, world!");
        };
        print_text();
    }
}

#[cfg(test)]
mod move_implicit {
    #[test]
    pub fn move_implicit_consumption() {
        let message = String::from("Goodbye!");
        let consume_message = || {
            drop(message); // The compiler applies implicit move here
        };
        consume_message();
        // consume_message(); // ✗ ERROR: cannot be called twice
    }
}

/*
========================================================================
IMPL AND DYN
========================================================================

Impl and Dyn:
--------------------------------------------

Since Fn/FnMut/FnOnce are traits, so there are some ways to implement them:
    - impl Fn... : monomorphization: unique parameters and return for functions
    - Box<dyn Fn...> : heterogeneous collections of the same trait
    - &dyn Fn... : dynamic dispatch (reference to a trait object)
*/
#[cfg(test)]
mod impl_dyn {
    /*
    Impl: (Monomorphization)
    --------------------------------------------
        impl Fn(Args) -> Output

        Params:
            fn fn_param_impl<F: Fn(i32) -> i32>(callback: F) {  // monomorphization
                callback(3);
            }
        Return:
            fn fn_return_impl() -> impl Fn(i32) -> i32 { 
                |x: i32| x + 1 // returns a clone of the same type of the closure
        } 
     */
    #[test]
    pub fn impl_param() {
        fn fn_param_impl<F: Fn(i32) -> i32>(callback: F) {
            assert_eq!(callback(3), 6);
        }
        let callback = |x: i32| x * 2;
        fn_param_impl(callback);
    }

    #[test]
    pub fn impl_return() {
        fn fn_return_impl() -> impl Fn(i32) -> i32 {
            |x: i32| x + 1
        }
        let callback = fn_return_impl();
        assert_eq!(callback(5), 6);
    }

    /*
    Box Dyn: (For heterogeneous collections)
    --------------------------------------------
        Box<dyn Fn(Args) -> Output>

        You can mix different closures
        Useful for storing in Vec<Box<dyn Fn...>>

        Params:
            fn fn_param_dyn(callback: Box<dyn Fn(i32) -> i32>) {  // dynamic dispatch
                callback(3); 
            }
        Return:
            fn fn_return_dyn() -> Box<dyn Fn(i32) -> i32> {  // dynamic dispatch
                if true {
                    Box::new(|x| x + 1)
                } else {
                    Box::new(|x| x * 2)
                }
            }
     */
    #[test]
    pub fn dyn_param() {
        fn fn_param_dyn(callback: Box<dyn Fn(i32) -> i32>) {
            assert_eq!(callback(3), 6);
        }
        let callback = |x: i32| x * 2;
        fn_param_dyn(Box::new(callback));
    }

    #[test]
    pub fn dyn_return() {
        fn fn_return_dyn() -> Box<dyn Fn(i32) -> i32> {
            if true {
                Box::new(|x| x + 1)
            } else {
                Box::new(|x| x * 2)
            }
        }
        let callback = fn_return_dyn();
        assert_eq!(callback(5), 6);
    }

    #[test]
    pub fn dyn_collection() {
        let mut callbacks: Vec<Box<dyn Fn(i32) -> i32>> = Vec::new();
        callbacks.push(Box::new(|x| x + 1));
        callbacks.push(Box::new(|x| x * 2));

        assert_eq!(callbacks[0](3), 4);
        assert_eq!(callbacks[1](3), 6);
    }

    #[test]
    pub fn dyn_storage() {
        struct CallbackStorage {
            callback: Box<dyn Fn(i32) -> i32>,
        }

        let storage = CallbackStorage {
            callback: Box::new(|x| x + 10),
        };

        assert_eq!((storage.callback)(5), 15);
    }

    /*
    &dyn: (For heterogeneous collections)
    --------------------------------------------
        &dyn Fn(Args) -> Output

        it is a reference to a trait object, has lifetime, it is faster than box<dyn>
        it is no owned, so it is not possible to store it in a variable, you need to borrow it.

        Params:
            fn fn_param_dyn(callback: &dyn Fn(i32) -> i32) {  // dynamic dispatch
                callback(3); 
            }
        Return:
            fn fn_return_dyn() -> &dyn Fn(i32) -> i32 {  // dynamic dispatch
                if true {
                    &|x| x + 1
                } else {
                    &|x| x * 2
                }
            }
     */
}


/*
    CLOSURES NOT VARIADIC (FIXED NUMBER OF ARGUMENTS)
    --------------------------------------------
        Closures in Rust are NOT VARIADIC:
        - They always have a fixed signature of arguments and return type
        - They cannot have optional or default arguments
        - They cannot have a variable number of arguments (variadic)

        Unlike C++, Rust prioritizes type and memory safety.
        If you want to pass a variable number of arguments, you can use slices (&[T]),
        tuples, vectors (Vec<T>), or use macros.
*/
mod novariadic_closures {}

#[cfg(test)]
mod patterns {

    /*
    CLOSURE FACTORIES
    --------------------------------------------
        Functions that return closures.
        - impl Fn: Static return (single closure type).
        - Box<dyn Fn>: Dynamic return (allows conditional logic to choose the closure).
    */
    pub mod closure_factories {
        #[test]
        pub fn closure_factories() {
            // Static factory (impl Fn)
            fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
                move |y| x + y
            }
            let add_5 = create_adder(5);
            assert_eq!(add_5(10), 15);

            // Dynamic factory (Box<dyn Fn>)
            fn create_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
                match op {
                    "add" => Box::new(|a, b| a + b),
                    "mul" => Box::new(|a, b| a * b),
                    _ => Box::new(|_, _| 0),
                }
            }
            let op = create_operation("mul");
            assert_eq!(op(3, 4), 12);
        }
    }

    /*
    STRATEGY PATTERN
    --------------------------------------------
        Allows injecting behavior (algorithms) into a struct.
        The struct defines the interface (the trait bound) and the closure provides the implementation.
    */
    pub mod strategy_pattern {
        struct Validator<F>
        where
            F: Fn(&str) -> bool,
        {
            validate: F,
        }

        impl<F> Validator<F>
        where
            F: Fn(&str) -> bool,
        {
            fn new(validate: F) -> Self {
                Self { validate }
            }

            fn is_valid(&self, s: &str) -> bool {
                (self.validate)(s)
            }
        }

        #[test]
        pub fn strategy_pattern() {
            println!("  ✅ strategy_pattern::strategy_pattern");

            let numeric_validator = Validator::new(|s| s.chars().all(|c| c.is_numeric()));
            let length_validator = Validator::new(|s| s.len() > 5);

            assert!(numeric_validator.is_valid("12345"));
            assert!(!numeric_validator.is_valid("123a5"));
            assert!(length_validator.is_valid("123456"));
        }
    }

    /*
    MIDDLEWARE / DECORATORS
    --------------------------------------------
        Wrap a closure with another to add cross-cutting functionality
        (logging, metrics, error handling) without modifying the original logic.
    */
    // Middleware that adds logging to any function/closure
    pub mod middleware_pattern {
        fn with_logging<F, T, R>(func: F) -> impl Fn(T) -> R
        where
            F: Fn(T) -> R,
        {
            move |arg| {
                println!("LOG: Calling with argument...");
                let result = func(arg);
                println!("LOG: Call finished.");
                result
            }
        }

        #[test]
        pub fn middleware_pattern() {
            println!("  ✅ middleware_pattern::middleware_pattern");

            let square = |x: i32| x * x;
            let square_with_log = with_logging(square);

            assert_eq!(square_with_log(5), 25);
        }
    }

    /*
    LAZY EVALUATION (Thunks)
    --------------------------------------------
        Defer the execution of expensive code until the result is needed.
        A closure is used as a "recipe" to generate the value on demand.
    */
    pub mod lazy_evaluation {
        struct Lazy<T, F>
        where
            F: Fn() -> T,
        {
            value: Option<T>,
            initializer: F,
        }

        impl<T, F> Lazy<T, F>
        where
            F: Fn() -> T,
        {
            fn new(initializer: F) -> Self {
                Self {
                    value: None,
                    initializer,
                }
            }

            fn get(&mut self) -> &T {
                if self.value.is_none() {
                    self.value = Some((self.initializer)());
                }
                self.value.as_ref().unwrap()
            }
        }

        #[test]
        pub fn lazy_evaluation() {
            println!("  ✅ lazy_evaluation::lazy_evaluation");

            let mut expensive_value = Lazy::new(|| {
                println!("Calculating expensive value...");
                42
            });

            // First access triggers the calculation
            assert_eq!(*expensive_value.get(), 42);
            // Second access uses the cached value
            assert_eq!(*expensive_value.get(), 42);
        }
    }

    /*
    EVENT LISTENERS / OBSERVER PATTERN
    --------------------------------------------
        Store multiple callbacks to be executed on an event.
        Requires `Box<dyn FnMut>` to allow listeners to modify their own state.
    */
    pub mod event_listeners {
        use std::rc::Rc;
        use std::cell::RefCell;

        struct EventEmitter {
            listeners: Vec<Box<dyn FnMut(&str)>>,
        }

        impl EventEmitter {
            fn new() -> Self {
                Self {
                    listeners: Vec::new(),
                }
            }

            fn on<F>(&mut self, callback: F)
            where
                F: FnMut(&str) + 'static,
            {
                self.listeners.push(Box::new(callback));
            }

            fn emit(&mut self, event: &str) {
                for listener in self.listeners.iter_mut() {
                    listener(event);
                }
            }
        }

        #[test]
        pub fn event_listeners() {
            println!("  ✅ event_listeners::event_listeners");
            let mut emitter = EventEmitter::new();
            
            // Use Rc<RefCell<i32>> to share mutable state across closures
            let count = Rc::new(RefCell::new(0));

            // First listener: prints the event
            emitter.on(move |ev| println!("Event received: {}", ev));
            
            // Second listener: captures and modifies count
            let count_clone = count.clone();
            emitter.on(move |_| {
                *count_clone.borrow_mut() += 1;
            });

            emitter.emit("click");
            emitter.emit("hover");
            
            // Print final count
            println!("Total events: {}", count.borrow());
        }
    }

    /*
    STATEFUL CLOSURES (State encapsulation)
    --------------------------------------------
        Use a closure to create an object with private state without defining a struct.
    */
    pub mod stateful_closures {
        fn create_counter(start: i32) -> impl FnMut() -> i32 {
            let mut count = start;
            move || {
                count += 1;
                count
            }
        }

        #[test]
        pub fn stateful_closures() {
            println!("  ✅ stateful_closures::stateful_closures");

            let mut counter_a = create_counter(0);
            let mut counter_b = create_counter(100);

            assert_eq!(counter_a(), 1);
            assert_eq!(counter_a(), 2);
            assert_eq!(counter_b(), 101);
            assert_eq!(counter_a(), 3);
        }
    }

    /*
    CURRYING / PARTIAL APPLICATION
    --------------------------------------------
        Transform a function of N arguments into a chain of functions of 1 argument.
    */
    pub mod currying_partial_application {
        #[test]
        pub fn currying_partial_application() {
            println!("  ✅ currying_partial_application::currying_partial_application");

            // Curried function
            let add = |x| move |y| x + y;

            let add_ten = add(10); // Partial application
            let result = add_ten(5);

            assert_eq!(result, 15);
            assert_eq!(add(20)(30), 50);
        }
    }
}
