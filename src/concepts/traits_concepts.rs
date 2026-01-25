#[test]
fn index() {
    utilities::single_responsibility();
    utilities::polymorphism();
    utilities::dependency_inversion();
    utilities::mocking();

    concepts::what_is_a_trait();
    concepts::manual();
    concepts::derive();
    concepts::blanket_impl();

    impl_dyn::traits_as_parameters();
    impl_dyn::traits_as_return();
    dyn_vtable::info();
    dyn_object_safety::object_safety();

    trait_with_associated_types::info();
    
    testing::testing_with_traits();
    
    other_concepts::multiple_trait_bounds();
    other_concepts::associated_types();
    other_concepts::supertraits();
    other_concepts::extension_traits();
    other_concepts::blanket_implementations();
    other_concepts::impl_for_concrete_types_from_generics();

    ufcs::ufcs();
    same_method_multiple_traits::multiple_traits();

}

/*
============================================================================
UTILITIES
============================================================================

    SINGLE RESPONSIBILITY PRINCIPLE (SRP)
    ----------------------------------------------
        Each implementation/trait defines an isolated aspect of
        struct behavior

            impl Validatable for User { ... } // implementation kept separate

    POLYMORPHISM AND EXTENSIBILITY
    ----------------------------------------------
        Allow writing code that operates on "capabilities" instead of
        concrete types.

            fn process(data: impl SerializableTrait) // for any T : SerializableTrait

    DEPENDENCY INVERSION (IoC)
    ----------------------------------------------
        Instead of consuming dependencies as concrete types T that couple code,
        we consume trait implementations that respect contracts.
        Now dependencies must adapt to the contract (trait),

    TESTABILITY AND MOCKING
    ----------------------------------------------
        Facilitate creation of "test doubles" (mocks/stubs).
        You can substitute heavy or external services with lightweight
        in-memory implementations during tests.
*/
#[cfg(test)]
mod utilities {
    /*
    SINGLE RESPONSIBILITY PRINCIPLE (SRP)
    ----------------------------------------------
        Each implementation/trait defines an isolated aspect of
        struct behavior

            impl Validatable for User { ... } // implementation kept separate
    */
    #[test]
    pub fn single_responsibility() {
        trait Validatable {
            fn validate(&self) -> Result<(), Vec<String>>;
        }
        struct User {
            #[allow(dead_code)]
            id: u32,
            #[allow(dead_code)]
            email: String,
        }

        // base implementation
        impl User {
            fn new(id: u32, email: String) -> Self {
                Self { id, email }
            }
            #[allow(dead_code)]
            fn get_email(&self) -> &str {
                &self.email
            }
        }
        // trait implementation kept separate
        impl Validatable for User {
            fn validate(&self) -> Result<(), Vec<String>> {
                Ok(())
            }
        }

        let user = User::new(1, "test@example.com".into());
        assert!(user.validate().is_ok());
    }

    /*
    POLYMORPHISM AND EXTENSIBILITY
    ----------------------------------------------
        Allow writing code that operates on "capabilities" instead of
        concrete types.

            fn process(data: impl SerializableTrait) // for any T : SerializableTrait

        This makes the system extensible: you can add new
        types in the future and existing functions will work
        for types that implement the trait.
    */
    #[test]
    pub fn polymorphism() {
        trait Serializable {
            fn serialize(&self) -> String;
        }

        struct User {
            id: u32,
            name: String,
        }

        struct Product {
            id: u32,
            title: String,
        }

        impl Serializable for User {
            fn serialize(&self) -> String {
                format!("User {{ id: {}, name: '{}' }}", self.id, self.name)
            }
        }

        impl Serializable for Product {
            fn serialize(&self) -> String {
                format!("Product {{ id: {}, title: '{}' }}", self.id, self.title)
            }
        }

        fn save_serializable(item: &impl Serializable) -> String {
            item.serialize()
        }

        let user = User {
            id: 1,
            name: "Alice".into(),
        };
        let product = Product {
            id: 101,
            title: "Gadget".into(),
        };

        assert_eq!(save_serializable(&user), "User { id: 1, name: 'Alice' }");
        assert_eq!(
            save_serializable(&product),
            "Product { id: 101, title: 'Gadget' }"
        );
    }

    /*
    DEPENDENCY INVERSION (IoC)
    ----------------------------------------------
        Instead of consuming dependencies as concrete types T that couple code,
        we consume trait implementations that respect contracts.
        Now dependencies must adapt to the contract (trait),
        This facilitates changing implementations without affecting
        the consuming code.
    */
    #[test]
    pub fn dependency_inversion() {
        // 1. THE ABSTRACTION (The contract that defines the upper level)
        trait Logger {
            fn log(&self, message: &str) -> String;
        }

        // 2. UPPER LEVEL (Business logic)
        // It doesn't know HOW to log, it just knows it can do it through the Trait.
        struct AppLogic<L: Logger> {
            logger: L,
        }

        impl<L: Logger> AppLogic<L> {
            fn do_work(&self) -> String {
                self.logger.log("Performing important operation...")
            }
        }

        // 3. LOWER LEVEL (Implementation details)
        struct ConsoleLogger;
        impl Logger for ConsoleLogger {
            fn log(&self, message: &str) -> String {
                format!("[Console]: {}", message)
            }
        }

        struct FileLogger;
        impl Logger for FileLogger {
            fn log(&self, message: &str) -> String {
                format!("[File]: {}", message)
            }
        }

        // 4. INJECTION (We decide the dependency at creation time)
        let app_with_console = AppLogic {
            logger: ConsoleLogger,
        };
        assert_eq!(
            app_with_console.do_work(),
            "[Console]: Performing important operation..."
        );

        let app_with_file = AppLogic { logger: FileLogger };
        assert_eq!(
            app_with_file.do_work(),
            "[File]: Performing important operation..."
        );
    }

    /*
    TESTABILITY AND MOCKING
    ----------------------------------------------
        Facilitate creation of "test doubles" (mocks/stubs).
        You can substitute heavy or external services with lightweight
        in-memory implementations during tests,
        ensuring tests are fast and deterministic.
    */
    #[test]
    pub fn mocking() {
        trait DataService {
            fn get_data(&self, id: u32) -> String;
        }

        struct RealDataService;
        impl DataService for RealDataService {
            fn get_data(&self, id: u32) -> String {
                format!("Real data for id {}", id) // Simulation
            }
        }

        struct MockDataService;
        impl DataService for MockDataService {
            fn get_data(&self, id: u32) -> String {
                format!("Mock data for id {}", id)
            }
        }

        struct DataProcessor<D: DataService> {
            service: D,
        }

        impl<D: DataService> DataProcessor<D> {
            fn process(&self, id: u32) -> String {
                self.service.get_data(id)
            }
        }

        // In production
        let real_processor = DataProcessor {
            service: RealDataService,
        };
        assert_eq!(real_processor.process(1), "Real data for id 1");

        // In tests
        let mock_processor = DataProcessor {
            service: MockDataService,
        };
        assert_eq!(mock_processor.process(1), "Mock data for id 1");
    }
}

/*
============================================================================
CONCEPTS
============================================================================

    CONTRACT:
    ----------------------------------------------
        A trait defines a behavior contract:
        Any type that implements the trait must have those public methods

    Ways to implement traits in structs:
    ----------------------------------------------
        TRAIT IMPLEMENTATION
        1. impl Trait for Type
        └── Manual implementation

        2. #[derive(Trait)]
        └── The compiler generates the implementation

        3. impl<T: AnotherTrait> MyTrait for T
        └── Blanket implementations (for any T: AnotherTrait)

    ORPHAN RULE:
    ----------------------------------------------
        At least ONE must be local to your crate.
        (Without this rule, two crates could implement the
        same external trait for the same external type and have conflicts)

        impl Display for MyStruct    (MyStruct is local)
        impl MyTrait for String      (MyTrait is local)
        impl Display for String      (both are external)    // error
*/
#[cfg(test)]
mod concepts {
    /*
    CONTRACT:
    ----------------------------------------------
    A trait defines a behavior contract:
    Any type that implements the trait must have those public methods
    */
    #[test]
    pub fn what_is_a_trait() {
        trait Greeting {
            fn greet(&self) -> String;

            // Traits can have methods with default implementation
            fn formal_greeting(&self) -> String {
                format!("Formally: {}", self.greet())
            }
        }

        // We implement for different types
        struct Person {
            name: String,
        }
        impl Greeting for Person {
            // public: inherits visibility from trait
            fn greet(&self) -> String {
                format!("Hello, I am {}", self.name)
            }
        }

        struct Robot {
            model: String,
        }
        impl Greeting for Robot {
            fn greet(&self) -> String {
                format!("BEEP BOOP. Unit {}", self.model)
            }
        }

        let person = Person {
            name: "Alice".into(),
        };
        let robot = Robot {
            model: "R2D2".into(),
        };

        assert_eq!(person.greet(), "Hello, I am Alice");
        assert_eq!(person.formal_greeting(), "Formally: Hello, I am Alice");
        assert_eq!(robot.greet(), "BEEP BOOP. Unit R2D2");
    }

    /*
    Manual implementation:
    ----------------------------------------------
    */
    #[test]
    pub fn manual() {
        trait Operation {
            fn apply(&self, x: i64) -> i64;
        }

        // Manual implementation
        struct Add(i64);

        impl Operation for Add {
            fn apply(&self, x: i64) -> i64 {
                x + self.0
            }
        }

        struct Multiply(i64);

        impl Operation for Multiply {
            fn apply(&self, x: i64) -> i64 {
                x * self.0
            }
        }

        let add = Add(10);
        let mult = Multiply(3);

        assert_eq!(add.apply(5), 15);
        assert_eq!(mult.apply(5), 15);
    }

    /*
    Derive implementation:
    ----------------------------------------------
    */
    #[test]
    pub fn derive() {
        #[derive(Debug, Clone, PartialEq, Default)]
        struct _Point {
            x: i32,
            y: i32,
        }
    }

    /*
    Blanket implementation: (Bounds clause)
    ----------------------------------------------
    covers many cases, used with generic T

        impl<T: TraitB> TraitA for T
        // all T that implement TraitB now have TraitA implemented
    */
    #[test]
    pub fn blanket_impl() {
        use std::fmt::Display;

        // 1. We define a new trait
        trait Logger {
            fn log_error(&self);
        }

        // 2. BLANKET IMPL: "Any T that implements Display now implements Logger"
        impl<T: Display> Logger for T {
            fn log_error(&self) {
                println!("❌ ERROR: {}", self); // We use Display of T
            }
        }

        // i32 implements Display, therefore now also Logger
        404.log_error();
    }

    /*
    Orphan Rule:
    ----------------------------------------------
    Apply my trait to an external type (String) and vice versa

        impl Display for MyStruct    (MyStruct is local)
        impl MyTrait for String      (MyTrait is local)
        impl Display for String      (both are external)    // error
    */
    #[test]
    pub fn orphan_rule() {
        trait _Greeting {
            fn greet(&self) -> String;
        }

        impl _Greeting for String {
            fn greet(&self) -> String {
                format!("Hello, {}", self)
            }
        }
    }
}

/*
============================================================================
IMPL AND DYN
============================================================================

    Monomorphization: (impl Trait, <T: Trait>)
    ----------------------------------------------

        Monomorphization means the compiler generates a
        separate copy of the code for each concrete type you use.

        Same ways to represent trait bounds for monomorphization:
            * impl Trait          (syntax sugar)
            * <T: Trait>          (explicit generic)
            * where T: Trait      (generic with where clause)

        Ejemplo:
            fn procesar<T: Op>(x: T) { x.aplicar() }
            procesar(Sumar(1));    → El compilador genera: procesar_Sumar()
            procesar(Mult(2));     → El compilador genera: procesar_Mult()

    Dynamic dispatch: (&dyn Trait, Box<dyn Trait>)
    ----------------------------------------------

        Dynamic dispatch means the compiler generates a vtable for the trait
        and uses it to call the appropriate method at runtime.

        &dyn Trait
        ---------------
            ┌──────────────┐
            │ data_ptr ────┼──→ object
            ├──────────────┤
            │ vtable_ptr ──┼──→ (BINARY) vtable
            └──────────────┘

            - is a fat pointer
            - does not own the data


            Auto-Coercion
            ---------------
            &T  →  (coerced to)  →  &dyn Trait
            Se usa con referencias, internamente se convierte a &dyn Trait,
            cuando espera un &dyn Trait, se puede pasar un &T

        Box<dyn Trait>
        ---------------
            ┌──────────────┐
            │ box_ptr ─────┼────────────┐
            └──────────────┘            │
                                        ▼
                                    (HEAP - 16 bytes)
                                    ┌──────────────┐
                                    │ data_ptr ────┼──→ actual object data
                                    ├──────────────┤
                                    │ vtable_ptr ──┼──→ (BINARY) vtable
                                    └──────────────┘
            - is a smart pointer
            - owns the data
*/
#[cfg(test)]
pub mod impl_dyn {

    #[test]
    pub fn traits_as_parameters() {
        trait Operation {
            fn apply(&self, x: i64) -> i64;
        }

        struct Add(i64);

        impl Operation for Add {
            fn apply(&self, x: i64) -> i64 {
                x + self.0
            }
        }

        struct Multiply(i64);

        impl Operation for Multiply {
            fn apply(&self, x: i64) -> i64 {
                x * self.0
            }
        }

        /*
        ─────────────────────────────────────────────────────────────
        impl Trait
        ─────────────────────────────────────────────────────────────
            impl Trait in parameters is syntax sugar for generics <T: Trait>

            Syntax sugar	           Expands to
            fn f(x: impl A)	           // fn f<T: A>(x: T)
            fn f(x: impl A + B)        // fn f<T: A + B>(x: T)
            fn f(x: impl A, y: impl B) // fn f<T: A, U: B>(x: T, y: U)
        */
        fn with_impl_trait(op: impl Operation, x: i64) -> i64 {
            op.apply(x)
        }

        assert_eq!(with_impl_trait(Add(10), 5), 15);

        /*
        ─────────────────────────────────────────────────────────────
        Generic <T: Trait>
        ─────────────────────────────────────────────────────────────
        */
        fn with_generic<T: Operation>(op: T, x: i64) -> i64 {
            op.apply(x)
        }

        assert_eq!(with_generic(Add(10), 5), 15);

        /*
        ─────────────────────────────────────────────────────────────
        where clause: is more readable for complex bounds
        ─────────────────────────────────────────────────────────────
        */
        fn with_where<T>(op: T, x: i64) -> i64
        where
            T: Operation,
        {
            op.apply(x)
        }

        assert_eq!(with_where(Multiply(2), 5), 10);

        /*
        ─────────────────────────────────────────────────────────────
        &dyn Trait
        ─────────────────────────────────────────────────────────────
        */
        // &T → (coerced to) → &dyn Trait
        fn with_dyn(op: &dyn Operation, x: i64) -> i64 {
            op.apply(x)
        }

        let ref_add = &Add(10);

        assert_eq!(with_dyn(ref_add, 5), 15);

        /*
        ─────────────────────────────────────────────────────────────
        Box<dyn Trait>
        ─────────────────────────────────────────────────────────────
        */
        fn with_box_dyn(op: Box<dyn Operation>, x: i64) -> i64 {
            op.apply(x)
        }

        let box_add = Box::new(Add(10));

        assert_eq!(with_box_dyn(box_add, 5), 15);
    }

    /*
    TRAITS AS RETURN VALUES:
    ----------------------------------------------

       ┌────────────────────────────────────────────────────────────────┐
       │ 1. -> impl Trait                                               │
       │    ✓ Single concrete type (decided at compile time)            │
       │    ✓ Zero-cost abstraction                                     │
       │    ✗ Cannot return different types based on condition          │
       └────────────────────────────────────────────────────────────────┘
       ┌────────────────────────────────────────────────────────────────┐
       │ 2. -> Box<dyn Trait>                                           │
       │    ✓ Can return any type that implements the trait             │
       │    ✓ Decision at runtime                                       │
       │    ✗ Heap allocation + indirection                             │
       └────────────────────────────────────────────────────────────────┘

       EXAMPLE:
       ─────────
       fn create_op(type_: &str) -> Box<dyn Op> {
           match type_ {
               "add" => Box::new(Add(1)),    // type A
               _     => Box::new(Mult(2)),   // type B
           }
       }
       // ^ This REQUIRES dyn because it returns different types
     */
    #[test]
    pub fn traits_as_return() {
        trait Operation {
            fn apply(&self, x: i64) -> i64;
        }

        struct Add(i64);
        impl Operation for Add {
            fn apply(&self, x: i64) -> i64 {
                x + self.0
            }
        }

        struct Multiply(i64);
        impl Operation for Multiply {
            fn apply(&self, x: i64) -> i64 {
                x * self.0
            }
        }

        // ─────────────────────────────────────────────────────────────
        // FORM 1: -> impl Trait (single concrete type)
        // ─────────────────────────────────────────────────────────────
        fn create_adder() -> impl Operation {
            Add(100)
        }

        // ❌ This does NOT compile (two different types):
        // fn create_by_flag(flag: bool) -> impl Operation {
        //     if flag { Add(1) } else { Multiply(2) }
        // }

        let op = create_adder();
        assert_eq!(op.apply(5), 105);

        // ─────────────────────────────────────────────────────────────
        // FORM 2: -> Box<dyn Trait> (multiple possible types)
        // ─────────────────────────────────────────────────────────────
        fn create_by_type(type_: &str) -> Box<dyn Operation> {
            match type_ {
                "add" => Box::new(Add(10)),
                "mult" => Box::new(Multiply(5)),
                _ => Box::new(Add(0)), // identity for addition
            }
        }

        let add = create_by_type("add");
        let mult = create_by_type("mult");
        assert_eq!(add.apply(5), 15);
        assert_eq!(mult.apply(5), 25);

        // ─────────────────────────────────────────────────────────────
        // Special case: Closures
        // ─────────────────────────────────────────────────────────────
        // Each closure is a unique concrete type, anonymous, with different size.
        // Another closure with same trait is a different type and doesn't respect impl Trait
        // because it's not the same concrete type and doesn't have the same size.
        fn create_closure() -> impl Fn(i64) -> i64 {
            |x| x * 2
        }

        let f = create_closure();
        assert_eq!(f(10), 20);
    }
}

/*
============================================================================
DYN VTABLE
============================================================================

    Cada tipo del dyn Trait tiene su propia VTABLE generada en compile time.


    FAT Pointer to vtable
    --------------------------------------------------------

    &dyn Trait = FAT POINTER (16 bytes en 64-bit)

        ┌───────────────────────────────────────┐
        │ data_ptr   │ vtable_ptr               │
        │ (8 bytes)  │ (8 bytes)                │
        └─────┬──────┴─────────┬────────────────┘
            │                │
            ▼                ▼
        ┌──────────┐     ┌─────────────────────────────────────┐
        │ DATOS    │     │ VTABLE                              │
        │ del tipo │     │                                     │
        │ concreto │     │  ┌───────────────┬────────────────┐ │
        │          │     │  │ drop_fn       │ → destructor   │ │
        │ Sumar(10)│     │  │ size          │ → 8 bytes      │ │
        │          │     │  │ align         │ → 8            │ │
        └──────────┘     │  │ aplicar_fn    │ → Sumar::aplicar│ │
                         │  │ otro_metodo_fn│ → ...          │ │
                         │  └───────────────┴────────────────┘ │
                         └─────────────────────────────────────┘

    VTABLE (Virtual Table) - Tabla de punteros a funciones
    --------------------------------------------------------
    Cada combinación (Tipo, Trait) genera UNA vtable en compile time:

    VTABLE para Sumar                     VTABLE para Multiplicar
    ───────────────────────────────────   ─────────────────────────────────
    drop_fn:       Sumar::drop            drop_fn:       Multiplicar::drop
    aplicar_fn:    Sumar::aplicar         aplicar_fn:    Multiplicar::aplicar
    otro_metodo_fn: Sumar::otro_metodo    otro_metodo_fn: Mult::otro_metodo
    size:          8 bytes                size:          8 bytes


    CÓMO FUNCIONA UNA LLAMADA A MÉTODO:
    --------------------------------------

    op.aplicar(x)   donde op: &dyn Operacion

    1. Leer vtable_ptr del fat pointer
    2. Buscar aplicar_fn en la vtable (offset conocido en compile time)
    3. Llamar: (vtable.aplicar_fn)(data_ptr, x)

*/
mod dyn_vtable {
    #[test]
    pub fn info() {}
}


/*
============================================================================
OBJECT SAFE in dyn TRAITS
============================================================================

    To use an implementation of a trait with dyn, the trait must be object-safe

    Not all traits can be used with dyn:
    --------------------------------------
        The compiler needs to know the size of all arguments and returns
        at compile time to build the vtable.

                trait Clone {
                    fn clone(&self) -> Self;  // ❌ Self = unknown type
                }
                Clone cant be used with dyn because what size does the return have?
                dyn Trait uses a vtable (table of function pointers).

    A trait is "object safe" if:
    --------------------------------------

        PROHIBITED:
        --------------------
        • Methods with Self in parameters or return
        • Generic methods
        • Associated functions (without self)
        • where Self: Sized

        ALLOWED:
        --------------------
        • Methods with &self, &mut self, self: Box<Self>
        • Methods that return concrete types
        • Associated constants


    WORKAROUND: where Self: Sized
    --------------------------------------
        This method will NOT be in the vtable, but the trait remains object-safe
        This method will be available only if the compiler knows the size of the concrete type
        only direct implementations of the trait can use this method

            trait MixedTrait {
                fn normal_method(&self) -> i32;

                fn method_with_self(&self) -> Self // not in the vtable
                where
                    Self: Sized;
            }

    WHY -> Self IS NOT OBJECT-SAFE?
    --------------------------------------
        The problem is NOT finding the method (that works via vtable).
        The problem is: WHERE DO I PUT THE RESULT?

        trait Clonable {
            fn clone(&self) -> Self;  // ← How many bytes does the return have?
        }

        fn problem(x: &dyn Clonable) {
            let copy = x.clone();  // ← Reserve 1 byte? 1000 bytes?
        }

        STACK FRAME (built at COMPILE TIME):
        ┌──────────────────────────────────┐
        │ x: &dyn Clonable (16 bytes)      │  ← This we know
        ├──────────────────────────────────┤
        │ copy: ???                        │  ← 1 byte? 1000 bytes?
        │        ↑                         │     WE DON'T KNOW
        │   How much space to reserve?     │
        └──────────────────────────────────┘

        The concrete type is known at RUNTIME, but the stack frame
        is built at COMPILE TIME. An unsolvable contradiction.

        SOLUTION: -> Box<Self> (always 8 bytes, data goes to heap)
*/
#[cfg(test)]
pub mod dyn_object_safety {
    #[test]
    pub fn object_safety() {
        // ─────────────────────────────────────────────────────────────
        // OBJECT-SAFE TRAIT
        // ─────────────────────────────────────────────────────────────
        trait Drawable {
            fn draw(&self) -> String;
        }

        struct Circle;

        impl Drawable for Circle {
            fn draw(&self) -> String {
                "○".into()
            }
        }

        struct Square;

        impl Drawable for Square {
            fn draw(&self) -> String {
                "□".into()
            }
        }

        let shapes: Vec<Box<dyn Drawable>> = vec![
            Box::new(Circle), 
            Box::new(Square)
        ];

        let _: Vec<String> = shapes
            .iter()
            .map(|shape| shape.draw())
            .collect();

        // ─────────────────────────────────────────────────────────────
        // NON-OBJECT-SAFE TRAIT (conceptual example)
        // ─────────────────────────────────────────────────────────────
        
          trait _Cloneable {
              fn clone(&self) -> Self;  // Self in return
          }
          // Vec<Box<dyn Cloneable>> ← ERROR

          trait _Comparable {
              fn compare(self, other: bool) -> bool;  // self in param
          }
          // &dyn Comparable ← ERROR

          trait _Generic {
              fn process<T>(&self, x: T);  // Generic method
          }
          // &dyn Generic ← ERROR
        
        // ─────────────────────────────────────────────────────────────
        // WORKAROUND: where Self: Sized
        // ─────────────────────────────────────────────────────────────
        #[allow(dead_code)]
        trait MixedTrait {  // object-safe trait
            fn normal_method(&self) -> i32;
            fn method_with_self(&self) -> Self      // excluded from the vtable
            where
                Self: Sized;
        }
    }
}

/*
============================================================================
TRAITS WITH ASSOCIATED TYPES
============================================================================

    Each implementation configures the associated types 
    instead of the trait defining them generically

      trait TripleTransform {
          type Input;
          type Output;
          type Error;
          fn transform(&self, input: Self::Input)
              -> Result<Self::Output, Self::Error>;
      }

      fn use_impl<T>(t: T, val: i32) 
      where T: TripleTransform<Input = i32, Output = String, Error = String>
      {
        //...
      }
    
*/
#[cfg(test)]
mod trait_with_associated_types {
    trait _TripleTransform {
        type Input;
        type Output;
        type Error;

        fn transform(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
    }

    struct _SafeConverter;

    impl _TripleTransform for _SafeConverter {
        type Input = i32;
        type Output = String;
        type Error = String;

        fn transform(&self, input: i32) -> Result<String, String> {
            if input < 0 {
                Err("Negatives not allowed".to_string())
            } else {
                Ok(format!("Safe value: {}", input))
            }
        }
    }

    struct _MathConverter;

    impl _TripleTransform for _MathConverter {
        type Input = &'static str;
        type Output = f64;
        type Error = String;

        fn transform(&self, input: &'static str) -> Result<f64, String> {
            input.parse::<f64>()
                .map_err(|_| format!("Failed to parse: {}", input))
        }
    }

    #[test]
    pub fn info() {
    }
}

/*
============================================================================
TESTING
============================================================================
 */
#[cfg(test)]
mod testing {
    use std::collections::HashMap;

    // ============================================================================
    //              DEMO 7: TESTING WITH TRAITS
    // ============================================================================

    /// Traits allow dependency injection and mocking:
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────────────────────────────┐
    /// │  TESTING WITH TRAITS - Dependency Injection                             │
    /// │                                                                         │
    /// │  PRODUCTION                          TESTING                            │
    /// │  ──────────                          ───────                            │
    /// │  ┌─────────────────┐                ┌─────────────────┐                 │
    /// │  │   PostgresDB    │                │    MockDB       │                 │
    /// │  │   impl DB       │                │    impl DB      │                 │
    /// │  └────────┬────────┘                └────────┬────────┘                 │
    /// │           │                                  │                          │
    /// │           └──────────────┬───────────────────┘                          │
    /// │                          │                                              │
    /// │                          ▼                                              │
    /// │                  trait Database {                                       │
    /// │                      fn query(&self, sql: &str) -> Vec<Row>;            │
    /// │                  }                                                      │
    /// │                          │                                              │
    /// │                          ▼                                              │
    /// │                  ┌───────────────┐                                      │
    /// │                  │   Service<D>  │                                      │
    /// │                  │   D: Database │                                      │
    /// │                  └───────────────┘                                      │
    /// │                                                                         │
    /// │  The Service doesn't know or care if it's PostgresDB or MockDB.         │
    /// │  It just knows it has something that implements Database.               │
    /// └─────────────────────────────────────────────────────────────────────────┘
    /// ```
    #[test]
    pub fn testing_with_traits() {
        // ─────────────────────────────────────────────────────────────
        // We define the trait (contract)
        // ─────────────────────────────────────────────────────────────
        #[derive(Clone, Debug, PartialEq)]
        struct User {
            id: u64,
            name: String,
        }

        #[derive(Debug, PartialEq, Clone)]
        enum RepoError {
            NotFound,
        }

        trait UserRepo {
            fn find(&self, id: u64) -> Result<User, RepoError>;
            fn save(&mut self, user: User) -> Result<(), RepoError>;
        }

        // ─────────────────────────────────────────────────────────────
        // REAL Implementation (production)
        // ─────────────────────────────────────────────────────────────
        struct MemoryRepo {
            data: HashMap<u64, User>,
        }

        impl MemoryRepo {
            fn new() -> Self {
                Self {
                    data: HashMap::new(),
                }
            }
        }

        impl UserRepo for MemoryRepo {
            fn find(&self, id: u64) -> Result<User, RepoError> {
                self.data.get(&id).cloned().ok_or(RepoError::NotFound)
            }

            fn save(&mut self, user: User) -> Result<(), RepoError> {
                self.data.insert(user.id, user);
                Ok(())
            }
        }

        // ─────────────────────────────────────────────────────────────
        // MOCK Implementation (testing)
        // ─────────────────────────────────────────────────────────────
        struct MockRepo {
            responses: HashMap<u64, Result<User, RepoError>>,
            #[allow(dead_code)]
            save_calls: Vec<User>,
        }

        impl MockRepo {
            fn new() -> Self {
                Self {
                    responses: HashMap::new(),
                    save_calls: Vec::new(),
                }
            }

            fn when_find(&mut self, id: u64, result: Result<User, RepoError>) {
                self.responses.insert(id, result);
            }
        }

        impl UserRepo for MockRepo {
            fn find(&self, id: u64) -> Result<User, RepoError> {
                self.responses
                    .get(&id)
                    .cloned()
                    .unwrap_or(Err(RepoError::NotFound))
            }

            fn save(&mut self, user: User) -> Result<(), RepoError> {
                self.save_calls.push(user);
                Ok(())
            }
        }

        // ─────────────────────────────────────────────────────────────
        // Generic Service over the trait
        // ─────────────────────────────────────────────────────────────
        struct UserService<R: UserRepo> {
            repo: R,
        }

        impl<R: UserRepo> UserService<R> {
            fn new(repo: R) -> Self {
                Self { repo }
            }

            fn get_name(&self, id: u64) -> Result<String, RepoError> {
                let user = self.repo.find(id)?;
                Ok(user.name)
            }
        }

        // ─────────────────────────────────────────────────────────────
        // Usage in "production"
        // ─────────────────────────────────────────────────────────────
        println!("USAGE WITH REAL IMPLEMENTATION:");
        let mut real_repo = MemoryRepo::new();
        real_repo
            .save(User {
                id: 1,
                name: "Alice".into(),
            })
            .unwrap();

        let service = UserService::new(real_repo);
        println!("  service.get_name(1) = {:?}", service.get_name(1));
        println!("  service.get_name(99) = {:?}", service.get_name(99));
        println!();

        // ─────────────────────────────────────────────────────────────
        // Usage in testing with mock
        // ─────────────────────────────────────────────────────────────
        println!("USAGE WITH MOCK (testing):");
        let mut mock = MockRepo::new();
        mock.when_find(
            42,
            Ok(User {
                id: 42,
                name: "Test User".into(),
            }),
        );
        mock.when_find(99, Err(RepoError::NotFound));

        let test_service = UserService::new(mock);
        println!(
            "  test_service.get_name(42) = {:?}",
            test_service.get_name(42)
        );
        println!(
            "  test_service.get_name(99) = {:?}",
            test_service.get_name(99)
        );
        println!();

        // ─────────────────────────────────────────────────────────────
        // Alternative: Box<dyn Trait> for late binding
        // ─────────────────────────────────────────────────────────────
        println!("ALTERNATIVE WITH Box<dyn Trait>:");

        struct DynUserService {
            repo: Box<dyn UserRepo>,
        }

        impl DynUserService {
            fn new(repo: Box<dyn UserRepo>) -> Self {
                Self { repo }
            }

            #[allow(dead_code)]
            fn get_name(&self, id: u64) -> Result<String, RepoError> {
                Ok(self.repo.find(id)?.name)
            }
        }

        let dyn_service = DynUserService::new(Box::new(MemoryRepo::new()));
        println!("  DynUserService created with Box<dyn UserRepo>");
        println!("  Useful when we don't know the type at compile time");
        let _ = dyn_service;
        println!();
    }
}

// ============================================================================
// OTHER CONCEPTS
// ============================================================================
/*

    1. MULTIPLE TRAIT BOUNDS
        fn f<T: Clone + Debug + Send>(x: T)
        fn f<T>(x: T) where T: Clone + Debug + Send

    2. ASSOCIATED TYPES vs GENERICS
        trait Iterator { type Item; }     ← Single Item per impl
        trait Add<Rhs> { ... }            ← Multiple Rhs per impl
                                                                            
    3. SUPERTRAITS                                                         
        trait Drawable: Clone + Debug { }                                   
        └── Whoever implements Drawable MUST implement Clone and Debug      
                                                                            
    4. EXTENSION TRAITS                                                    
        trait StringExt { fn shout(&self) -> String; }                      
        impl StringExt for str { ... }                                      
        └── Add methods to existing types                                   
                                                                            
    5. MARKER TRAITS                                                       
        trait Marker {}                                                     
        └── No methods, only for "tagging" types (Send, Sync, Copy)         
                                                                            
    6. IMPL FOR CONCRETE TYPES FROM GENERICS                               
        struct Wrapper<T> { value: T }                                      
        impl<T> Wrapper<T> { ... }      ← For all T                         
        impl Wrapper<i32> { ... }       ← Only for Wrapper<i32>             
        impl Wrapper<String> { ... }    ← Only for Wrapper<String>          
        └── Manual specialization of behavior by type                       
*/
#[cfg(test)]
mod other_concepts {

        
    #[test]
    pub fn multiple_trait_bounds() {
        // ─────────────────────────────────────────────────────────────
        // 1. MULTIPLE TRAIT BOUNDS
        // ─────────────────────────────────────────────────────────────
        use std::fmt::Debug;

        fn process_and_show<T: Clone + Debug>(x: T) {
            let copy = x.clone();
            println!("  Original: {:?}, Copy: {:?}", x, copy);
        }

        println!("1. MULTIPLE TRAIT BOUNDS (T: Clone + Debug):");
        process_and_show(vec![1, 2, 3]);
        println!();
    }

    // ─────────────────────────────────────────────────────────────
    // 2. ASSOCIATED TYPES
    // ─────────────────────────────────────────────────────────────
    #[test]
    pub fn associated_types() {
        trait Container {
            type Item;

            fn get(&self) -> Option<&Self::Item>;
            fn put(&mut self, item: Self::Item);
        }

        struct Stack<T> {
            items: Vec<T>,
        }

        impl<T> Container for Stack<T> {
            type Item = T;

            fn get(&self) -> Option<&T> {
                self.items.last()
            }

            fn put(&mut self, item: T) {
                self.items.push(item);
            }
        }

        println!("2. ASSOCIATED TYPES:");
        let mut stack: Stack<i32> = Stack { items: vec![] };
        stack.put(10);
        stack.put(20);
        println!("  Stack top: {:?}", stack.get());
        println!();
    }

    // ─────────────────────────────────────────────────────────────
    // 3. SUPERTRAITS
    // ─────────────────────────────────────────────────────────────
    #[test]
    pub fn supertraits() {
        use std::fmt::Debug;

        trait Printable: Debug {
            fn print(&self) {
                println!("  Printable: {:?}", self);
            }
        }

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Document {
            title: String,
        }

        impl Printable for Document {}

        println!("3. SUPERTRAITS (Printable: Debug):");
        let doc = Document {
            title: "My Doc".into(),
        };
        doc.print();
        println!();
    }
    // ─────────────────────────────────────────────────────────────
    // 4. EXTENSION TRAITS
    // ─────────────────────────────────────────────────────────────
    #[test]
    pub fn extension_traits() {
        trait _StringExt {
            fn shout(&self) -> String;
            fn is_question(&self) -> bool;
        }

        impl _StringExt for str {
            fn shout(&self) -> String {
                self.to_uppercase() + "!"
            }

            fn is_question(&self) -> bool {
                self.ends_with('?')
            }
        }


    }
    // ─────────────────────────────────────────────────────────────
    // 5. BLANKET IMPLEMENTATIONS
    // ─────────────────────────────────────────────────────────────
    #[test]
    pub fn blanket_implementations() {
        use std::fmt::Debug;

        trait _Describe {
            fn describe(&self) -> String;
        }

        // "Blanket" implementation: for ALL T that are Debug
        impl<T: Debug> _Describe for T {
            fn describe(&self) -> String {
                format!("I am: {:?}", self)
            }
        }
    }

    // ─────────────────────────────────────────────────────────────
    // 6. IMPL FOR CONCRETE TYPES FROM GENERICS
    // ─────────────────────────────────────────────────────────────
    // You can define a generic struct/trait and then implement
    // methods ONLY for certain concrete types.
    #[test]
    pub fn impl_for_concrete_types_from_generics() {
        use std::fmt::Debug;
        
        // Generic struct
        struct Wrapper<T> {
            _value: T,
        }

        // Generic implementation (available for ALL T)
        impl<T> Wrapper<T> {
            fn new(value: T) -> Self {
                Wrapper { _value: value }
            }

            fn _get(&self) -> &T {
                &self._value
            }
        }

        // Implementation ONLY for Wrapper<i32>
        impl Wrapper<i32> {
            fn _double(&self) -> i32 {
                self._value * 2
            }

            fn _is_positive(&self) -> bool {
                self._value > 0
            }
        }

        // Implementation ONLY for Wrapper<String>
        impl Wrapper<String> {
            fn _shout(&self) -> String {
                self._value.to_uppercase() + "!"
            }

            fn _len(&self) -> usize {
                self._value.len()
            }
        }

        // Implementation for Wrapper<T> where T: Clone + Debug
        impl<T: Clone + Debug> Wrapper<T> {
            fn _clone_and_describe(&self) -> String {
                format!("Cloned: {:?}", self._value.clone())
            }
        }

        let _w_int = Wrapper::new(42);
        let _w_str = Wrapper::new(String::from("hello"));
        let _w_vec = Wrapper::new(vec![1, 2, 3]);
    }
    
}


/*
============================================================================
LLAMAR MÉTODO COMO FUNCIÓN (UFCS)
============================================================================

    Universal Function Call Syntax (UFCS) allows calling a method as a function.
    Especially useful when the method doesn't take `self` or you need to pass
    arguments that aren't the instance.
*/
pub mod ufcs {

    #[test]
    pub fn ufcs() {

        // Trait with methods of different types
        trait Calculator {
            // Takes &self
            fn value(&self) -> i32;

            // Static method (doesn't take self)
            fn add(a: i32, b: i32) -> i32;
        }

        struct Calc {
            value: i32,
        }

        impl Calculator for Calc {
            fn value(&self) -> i32 {
                self.value
            }

            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        }

        let calc = Calc { value: 42 };

        /*
        Method that takes &self
        --------------------------------
        */

        // Normal form (method call syntax)
        let v1 = calc.value();
        println!("    calc.value() = {}", v1);

        // Function form (UFCS)
        let v2 = Calculator::value(&calc);
        println!("    Calculator::value(&calc) = {}", v2);

        
        /*
        Static method (doesn't take self)
        ----------------------------------
         */

        // Can only be called as a function
        let result = <Calc as Calculator>::add(10, 20);
        println!("    Calculator::add(10, 20) = {}", result);

        // Method syntax doesn't work for static methods:
        // calc.add(10, 20);  // ❌ ERROR: no `self` parameter

        /*
        When UFCS is particularly useful
        --------------------------------
        */

        // Pass a function as a parameter
        trait Mapper {
            fn map(x: i32) -> i32;
        }

        struct Doubler;
        impl Mapper for Doubler {
            fn map(x: i32) -> i32 {
                x * 2
            }
        }

        // UFCS allows passing the method as a function to another function
        fn apply<T: Mapper>(nums: Vec<i32>) -> Vec<i32> {
            // Here we use UFCS to pass T::map as a function
            nums.iter().map(|&x| T::map(x)).collect()
        }

        let values = vec![1, 2, 3, 4, 5];
        let _doubled = apply::<Doubler>(values);
    }
}

/*
============================================================================
SAME METHOD IN MULTIPLE TRAITS (CLARIFICATION)
============================================================================

    When a struct implements two traits with the same method name,
    you need to use the explicit function syntax to disambiguate which one to call.

        s.foo() is ambiguous - which foo?

        TraitA::foo(&s) = "I come from TraitA"
        TraitB::foo(&s) = "I come from TraitB"

*/
pub mod same_method_multiple_traits {

    #[test]
    pub fn multiple_traits() {
        // We define two traits with the same method
        trait TraitA {
            fn foo(&self) -> &'static str;
        }

        trait TraitB {
            fn foo(&self) -> &'static str;
        }

        struct MyStruct;

        // We implement both traits on the same struct
        impl TraitA for MyStruct {
            fn foo(&self) -> &'static str {
                "I come from TraitA"
            }
        }

        impl TraitB for MyStruct {
            fn foo(&self) -> &'static str {
                "I come from TraitB"
            }
        }

        let s = MyStruct;

        // ⚠️ s.foo() is ambiguous - which foo?
        // s.foo();  // ❌ ERROR: multiple `foo` found

        // ✅ SOLUTION: Use fully qualified syntax
        let _a = TraitA::foo(&s);
        let _b = TraitB::foo(&s);

        // If you wanted to call it as a method:
        // let a = <MyStruct as TraitA>::foo(&s);  // Also works
        // let b = <MyStruct as TraitB>::foo(&s);

    }
}