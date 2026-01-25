#[allow(dead_code)]
#[allow(unused_variables)]
#[test]
fn index() {
    intro::basic();

    implicit_lifetimes::no_return_value();
    implicit_lifetimes::return_owned();
    implicit_lifetimes::only_modifies();

    why_explicit::why_explicit();
    
    in_structs::in_structs();

    bounds::bounds();

    elision_rules::rule_1_multi_params();
    elision_rules::rule_2_single_input();
    elision_rules::rule_3_self();
    elision_rules::elision_fails();

    static_lifetime::static_intro();
    static_lifetime::static_literals();
    static_lifetime::const_inline();
    static_lifetime::static_basic();
    static_lifetime::static_return_test();
    static_lifetime::static_mutable();

    static_lifetime::static_box_leak();
    static_lifetime::leak_mut();
    static_lifetime::static_trait_bounds();

    placeholder::placeholder_intro();
    placeholder::placeholder_obvious();
    placeholder::placeholder_impl_blocks();
    placeholder::placeholder_nested_types();
    placeholder::placeholder_ambiguity();
}

/*
========================================================================
INTRO: WHAT ARE LIFETIMES?
========================================================================

    KEY CONCEPT:
    --------------------------------------------
        Lifetimes ALWAYS exist for every reference (&T).
        The compiler automatically infers them in most cases.
        You only write them when there's ambiguity.

    EVERY REFERENCE has an associated LIFETIME:
    --------------------------------------------
        let value: i32 = 123;        // value lives in this scope
        let ref_value: &i32 = &value; // ref has implicit lifetime
        Internally the compiler sees something like:
        let ref_value: &'a i32 = &value;   // 'a = lifetime of value

    'static:
    --------------------------------------------
        The lifetime of T is 'static, doesn't depend on other local data, 
        has the ability to live throughout the entire program
            
            * If they are 'static references, they live as long as 
            the reference exists, independent of any variable.
            Ex: &'static str, const, static, Arc<T>

            * Owner types (non-reference types) can be said to have 
            implicit 'static lifetime

    'a:
    --------------------------------------------
        Lifetime conditioned by the 'static lifetime of the owner.

    '_:
    --------------------------------------------
        Anonymous lifetime (placeholder) that the compiler automatically infers.

    FUNDAMENTAL RULE:
    --------------------------------------------
        The lifetime of the reference MUST be ≤ than the owner's lifetime
        (the reference cannot live longer than the referenced object)
        value:     ├──────────────────────┤  (lives lines 1-10)
        ref_value: │     ├────────┤       │  (lives lines 3-7)
                         └────────┘  ✓ OK: ref within owner

    IMPLICIT LIFETIMES:
    --------------------------------------------
        fn foo(s: &str)              // lifetime inferred
        fn foo<'a>(s: &'a str)       // explicit lifetime (redundant)

    SIGNATURES WITH LIFETIMES:
    --------------------------------------------
            fn foo<'a>(s: &'a str)
            struct Parser<'a> { ...; input: &'a str }
            impl<'a> Parser<'a> { ... }
            fn foo<'a, T>(s: &'a str, t: T) -> &'a str

        In signatures lifetimes are added when there are references 
        involved and generic types. 
        So the compiler knows how to relate the durations of the references 
        and generic types used.
        If they're omitted from the signature because they're obvious, 
        elision is used = syntax sugar, internally they're added to the signature anyway.

*/

#[cfg(test)]
mod intro {
    #[test]
    pub fn basic() {
        {
            // A lifetime is "how long a reference lives"
            let owner = String::from("hello");
            let reference = &owner;
            assert!(reference.is_empty() == false);
        }
        // line 5: owner dies, reference can no longer be used
    }
}

/*
========================================================================
IMPLICIT LIFETIMES
========================================================================

    KEY RULE:
    --------------------------------------------
        If the function DOESN'T return a reference, you DON'T need explicit lifetimes.

        Lifetimes relate the duration of the OUTPUT to the duration
        of the INPUTS. Without reference output → nothing to relate.

    COMPARISON: WITH vs WITHOUT LIFETIMES
    --------------------------------------------

        WITHOUT lifetimes (returns owned/void/primitive):
          fn process(a: &str, b: &str)                   // void
          fn process(a: &str, b: &str) -> String         // owned
          fn process(a: &str, b: &str) -> usize          // Copy

        WITH lifetimes (returns reference):
          fn process<'a>(a: &'a str, b: &str) -> &'a str // ref!
          fn process<'a>(a: &'a str) -> &'a [u8]         // ref!

        Elision: with 1 input, you don't write 'a)
          fn process(a: &str) -> &str              // ref! elision

    SPECIAL CASES:
    --------------------------------------------

        STRUCTS that contain &T always need 'a:

          struct Parser<'a> {
              input: &'a str,  // The struct "borrows" the string
          }

          impl<'a> Parser<'a> {
              fn new(input: &'a str) -> Self {
                  Parser { input }  // returns struct with ref
              }

              fn len(&self) -> usize {
                  self.input.len()  // ✓ NO lifetime - returns usize
              }

              // Elision in methods: references without lifetimes in returns
              // use the lifetime of &self (note: 'a is greater than &self)
              fn input(&self) -> &str {
                  self.input
              }
          }
*/

#[cfg(test)]
mod implicit_lifetimes {

    // No return value
    #[test]
    pub fn no_return_value() {


        fn do_nothing(_a: &str, _b    : &str) {
            // ✓ NO need for lifetimes - returns nothing
        }
        do_nothing("hello", "world");
    }

    // Returns owned value
    #[test]
    pub fn return_owned() {
        fn join_strings(a: &str, b: &str) -> String {
            // ✓ NO need for lifetimes - returns String (owned)
            format!("{} {}", a, b)
        }
        let joined = join_strings("hello", "world");
        assert_eq!(joined, "hello world");
    }

    // only modifies the reference
    #[test]
    pub fn only_modifies() {
        fn append_suffix(s: &mut String, suffix: &str) {
            // ✓ NO need for lifetimes - doesn't return reference
            s.push_str(suffix);
        }
        let mut text = String::from("Hello");
        append_suffix(&mut text, "!");
        assert_eq!(text, "Hello!");
    }
}

/*
========================================================================
EXPLICIT
========================================================================

    When there are TWO reference inputs and an output from them → AMBIGUITY

        fn return_ref<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { ... }
*/

#[cfg(test)]
mod why_explicit {
    #[test]
    pub fn why_explicit() {
        // Case 1: ONE input → compiler infers
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        // The compiler knows: output lives as long as input

        // Case 2: TWO inputs → AMBIGUITY
        // fn longest(x: &str, y: &str) -> &str { ... }
        // ERROR: Does the result live like x? Or like y?

        // Solution: specify with 'a
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        let s1 = String::from("short");
        let s2 = String::from("much longer");

        assert_eq!(first_word(&s1), "short");
        assert_eq!(longest(&s1, &s2), "much longer");
    }
}

/*
========================================================================
LIFETIMES IN STRUCTS
========================================================================

    STRUCT WITH REFERENCE:
    --------------------------------------------
        The struct can live as long as the reference it contains.
        If a struct contains references, it MUST have a lifetime.

        let text = String::from("Hello world");
            │
            ▼
        ┌─────────┐     ┌────────────────┐
        │ text    │───▶│ "Hello world"  │  (heap)
        └─────────┘     └────────────────┘
                               ▲
                               │
        ┌─────────────────┐    │
        │ Excerpt<'a>     │    │
        │   part: &'a str─┼────┘  (points to text)
        └─────────────────┘

        → Excerpt<'a> CANNOT live longer than text
*/

#[cfg(test)]
mod in_structs {
    #[test]
    pub fn in_structs() {
        #[derive(Debug)]
        struct Excerpt<'a> {
            part: &'a str,
        }
        
        impl<'a> Excerpt<'a> {
            fn get_part(&self) -> &str {
                self.part
            }
        }

        let text = String::from("Hello cruel world");
        let excerpt = Excerpt { part: &text[0..4] };
        assert_eq!(excerpt.get_part(), "Hell");
    }
}

/*
========================================
BOUNDS - LIFETIME RESTRICTIONS
========================================

    Bounds are used to restrict the lifetime of a type or another lifetime.

    T: 'a
    --------------------------
    The type T lives at least as long as the lifetime 'a

    'a: 'b
    --------------------------
    The lifetime 'a lives at least as long as the lifetime 'b

    'a: 'static
    --------------------------
    The lifetime 'a is static

    T: 'a + 'b
    --------------------------
    The type T lives at least as long as the lifetime 'a and 'b

*/
mod bounds {
    #[test]
    pub fn bounds() {
        struct Container<'a, T: 'a>
        {
            reference: &'a T,
        }

        impl<'a, T: 'a> Container<'a, T> {
            fn get_reference(&self) -> &'a T {
                self.reference
            }
        }

        let container = Container { reference: &1 };
        let reference = container.get_reference();
        assert_eq!(*reference, 1);
    }
}


/*
========================================================================
ELISION_RULES: (AUTOMATIC INFERENCE)
========================================================================

    "ELISION" = The compiler OMITS/INFERS lifetimes for you.

    When you write:    fn foo(s: &str) -> &str
    The compiler sees: fn foo<'a>(s: &'a str) -> &'a str
*/

#[cfg(test)]
mod elision_rules {

    /*
    Each parameter receives its own lifetime, no return ref
    --------------------------------------------
        You write:
          fn foo(x: &str, y: &str, z: &i32)

        Compiler infers:
          fn foo<'a, 'b, 'c>(x: &'a str, y: &'b str, z: &'c i32)
                 ↑   ↑   ↑
                 └───┴───┴── each one receives unique lifetime
    */
    #[test]
    pub fn rule_1_multi_params() {
        fn _foo(x: &str, y: &str) {
            println!("  x: {}, y: {}", x, y);
        }
        // The compiler sees: fn foo<'a, 'b>(x: &'a str, y: &'b str)
    }

    /*
    return ref input
    --------------------------------------------
        You write:
          fn first(s: &str) -> &str

        Compiler infers:
          fn first<'a>(s: &'a str) -> &'a str
                          ↑              ↑
                          └──────────────┘  same lifetime
    */
    #[test]
    pub fn rule_2_single_input() {
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        let s = String::from("hello world");
        let word = first_word(&s);
        println!("  First word: {}", word);
        assert_eq!(word, "hello");
    }

    /*
    methods with &self → lifetime of self
    --------------------------------------------
        You write:
          fn method(&self, other: &str) -> &str

        Compiler infers:
          fn method<'a, 'b>(&'a self, other: &'b str) -> &'a str
                             ↑                            ↑
                             └────────────────────────────┘
                             when not specified uses the one from &self

        Because methods usually return data from the struct.
    */
    #[test]
    pub fn rule_3_self() {
        struct Reader<'a> {
            content: &'a str,
        }

        impl<'a> Reader<'a> {
            // Elision: the return inherits the lifetime of &self
            fn get_content(&self) -> &str {
                self.content
            }
        }

        let text = String::from("content");
        let reader = Reader { content: &text };
        assert_eq!(reader.get_content(), "content");
    }

    /*
    WHEN RULES FAIL → You must write lifetimes
    --------------------------------------------
        fn longest(x: &str, y: &str) -> &str

        Rule 1: assigns 'a to x, 'b to y
        Rule 2: ✗ there are 2 lifetimes, not 1
        Rule 3: ✗ no &self

        → ERROR: "missing lifetime specifier"
        → You must write: fn longest<'a>(x: &'a str, y: &'a str)
    */
    #[test]
    pub fn elision_fails() {}
}

/*
========================================================================
'STATIC LIFETIME
========================================================================

    'static = doesn't depend on another local data, 
        has the ability to live throughout the entire program, 
        depends only on its own lifetime.

    owners also have implicit 'static lifetime.

        let x = 100; // i32 has implicit 'static lifetime

    'static references live as long as the reference exists, independent of any variable.
*/

#[cfg(test)]
mod static_lifetime {

    /*
    OWNERS AND IMPLICIT 'static
    --------------------------------------------
        Types that are NOT references (&T) have implicit 'static lifetime.
    */
    #[test]
    pub fn owners() {
        fn test_static<T: 'static>(_owner: T) {
            //...
        }
        let owner = 100; // i32 has implicit 'static lifetime
        test_static(owner);
    }

    /*
    LIFETIME 'static:
    --------------------------------------------
    */
    #[test]
    pub fn static_intro() {
        // 'static is a special lifetime that depends only on its own lifetime
        {
            let s: &'static str = "I am static";
            println!("{s}");
        }
        // dropped: s // out of block scope
        // println!("{s}"); // ERROR: s doesn't live beyond this scope, 
        // not because another variable is using it, but because it's out of scope.
    }

    /*
    String Literals
    --------------------------------------------
        let s: &'static str = "hello";

        Lifetime throughout the entire program, its owner is the executable binary.
        Can be returned from functions without problems.
    */
    #[test]
    pub fn static_literals() {
        fn get_static_str() -> &'static str {
            "string literal"
        }

        let _s1: &str = get_static_str(); // The actual type is &'static str
        let _s2: &'static str = "explicit literal";
    }

    /*
    CONST
    --------------------------------------------
        const PI: f64 = 3.14159;

        Ex: transpiles: return PI * radius * radius -> 3.14159 * radius * radius (inlining)

        Value copied where it's used (inlining) the variable is replaced by its value 
        during compilation.
        Types it accepts:
            * Copy
            * 'static references
        It's accessible according to its visibility and location.
            * If defined at module level, it's global.
            * If inside a function, it's local to that function.
        It's immutable by default.
        It's thread-safe
     */
    #[test]
    pub fn const_inline() {
        const PI: f64 = 3.14159;
        fn area_circle(radius: f64) -> f64 {
            PI * radius * radius
            // 3.14159 * radius * radius  ← PI is replaced directly by its value
        }
        let area = area_circle(2.0);
        assert_eq!(area, PI * 4.0);
    }

    /*
    STATIC var
    --------------------------------------------
        static ID: i32 = 100;           // immutable, 'static
        static NAME: &str = "Rust";     // immutable, 'static
        static mut COUNT: i32 = 0;      // mutable, requires unsafe

        Fixed address in memory throughout the entire program.
        Types it accepts:
            * Copy
            * 'static references
        It's accessible according to its visibility and location.
            * If defined at module level, it's global.
            * If inside a function, it's local to that function, but persists between calls. Can also be returned as a 'static reference from the function.
        If it's static mut:
            * unsafe: Requires unsafe to read or write, other threads have access to the variable.
            * +Sync if used with Mutex/RwLock/... for threads
        */

    // Access to static variable within another stack
    #[test]
    pub fn static_basic() {
        static COUNT: i32 = 42;
        fn print_count() {
            // Access to static variable
            println!("  Static COUNT inside fn: {}", COUNT);
        }
        print_count();
    }

    // return of 'static reference from function with static local
    #[test]
    pub fn static_return_test() {
        fn return_static() -> &'static i32 {
            static GREETING: i32 = 1;
            // returns 'static reference
            &GREETING
        }
        // valid, 'static depends on itself, in this case fixed address in memory.
        let _greeting_ref: &'static i32 = return_static();
    }

    // Mutable access with UNSAFE
    // other threads can access the variable, so it doesn't guarantee safety.
    #[test]
    pub fn static_mutable() {
        static mut COUNTER: i32 = 0;
        unsafe {
            let _ = COUNTER; // unsafe read
        }
        unsafe {
            COUNTER += 1; // unsafe write
        }
    }

    /*
    Box::leak
    --------------------------------------------
        Convert heap to 'static

        let s = String::from("runtime");
        let leaked: &'static str = Box::leak(s.into_boxed_str());

        Takes ownership of the Box, puts it in memory that NEVER gets freed and returns
        a 'static reference to that data. So it has 'static lifetime.
        Memory is freed when the program terminates.

        Can be mutable as well.

        Types it accepts: T: 'static
    */
    #[test]
    pub fn static_box_leak() {
        let x = Box::new(42);
        let _static_1: &'static i32 = Box::leak(x);

        let s = String::from("created at runtime");
        let _static_2: &'static str = Box::leak(s.into_boxed_str());
    }

    #[test]
    pub fn leak_mut() {
        let v = 1;
        let static_vec: &'static mut i32 = Box::leak(Box::new(v));
        *static_vec += 1;
        assert_eq!(*static_vec, 2);
    }

    /*
    'static in Trait Bounds
    --------------------------------------------
        fn spawn<F>(f: F)
        where
            F: FnOnce() + Send + 'static
                                ^^^^^^^^

        Why does tokio::spawn require 'static?

        The Future can execute at any future moment.
        If it contained references to local data, that data
        could die before the Future completes.

        'static guarantees that the Future doesn't depend on data
        that might die (only owned data or 'static refs).
    */
    #[test]
    pub fn static_trait_bounds() {}
}

/*
========================================================================
PLACEHOLDER: '_
========================================================================

    '_ = There's a lifetime here, the compiler deduces it automatically.
*/

#[cfg(test)]
mod placeholder {

    /*
    ANONYMOUS LIFETIME '_ (Placeholder)
    --------------------------------------------
    */
    #[test]
    pub fn placeholder_intro() {}

    /*
    USAGE 1: In parameters where the lifetime is obvious
    --------------------------------------------
        struct Excerpt<'a> { ... }

        // With placeholder (recommended):
        fn print(e: &Excerpt<'_>) { ... }

        // Equivalent explicit:
        fn print<'a>(e: &Excerpt<'a>) { ... }

        Why use '_? When you don't need to relate the lifetime with other
        parameters or with the return.
    */
    #[test]
    pub fn placeholder_obvious() {
        #[derive(Debug)]
        struct Excerpt<'a> {
            _part: &'a str,
        }

        // WITH placeholder (cleaner):
        fn print_excerpt(_e: &Excerpt<'_>) {}

        // EQUIVALENT with explicit lifetime (more verbose):
        fn _print_excerpt2<'a>(_e: &Excerpt<'a>) {}

        let text = String::from("Hello world");
        let exc = Excerpt { _part: &text };
        print_excerpt(&exc);
    }

    /*
    USAGE 2: In impl blocks
    --------------------------------------------
        // With placeholder:
        impl Excerpt<'_> {
            fn len(&self) -> usize { ... }
        }

        // Equivalent explicit:
        impl<'a> Excerpt<'a> {
            fn len(&self) -> usize { ... }
        }

        Use '_ when the method doesn't need to reference 'a.
    */
    #[test]
    pub fn placeholder_impl_blocks() {
        #[derive(Debug)]
        struct Excerpt<'a> {
            part: &'a str,
        }

        // When you implement for a type with lifetime but don't need to name it:
        impl Excerpt<'_> {
            fn len(&self) -> usize {
                self.part.len()
            }
        }

        let text = String::from("Hello world");
        let exc = Excerpt { part: &text };
        assert_eq!(exc.len(), 11);
    }

    /*
    USAGE 3: In nested types
    --------------------------------------------
        fn process(data: &[&'_ str]) { ... }
                           ^^
                           placeholder for the internal lifetime

        Equivalent: fn process<'a>(data: &[&'a str])
    */
    #[test]
    pub fn placeholder_nested_types() {
        fn process_refs(data: &[&'_ str]) {
            for s in data {
                println!("  Item: {}", s);
            }
        }

        let items = ["one", "two", "three"];
        process_refs(&items);
    }

    /*
    WHEN '_ DOESN'T WORK:
    --------------------------------------------
        // This DOESN'T compile:
        fn longest(x: &str, y: &str) -> &'_ str { ... }

        Error: "missing lifetime specifier"

        Why? There are 2 input lifetimes, the compiler doesn't know which one to use
        for the output. You must be explicit:

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    */
    #[test]
    pub fn placeholder_ambiguity() {}
}

/*
========================================
FOR<'a>
========================================
    for<'a> = "For every lifetime 'a"

    A way to indicate that a lifetime can be any, it's independent, not added to the signature T<'a>

    Used in:
        * Bounds in closures as parameters (mostly)
        * Definition of generic Traits

    Ex:
        fn apply_to_str<F>(s: &str, f: F) -> &str
        where
            F: for<'a> Fn(&'a str) -> &'a str,
                     ^^^^^^^^^^^^^^^^
                     for every lifetime 'a

        The closure F must work for any lifetime 'a passed to it.


        Version without for<'a>:
          * more restrictive, only works with a specific lifetime
          * signature with more dependencies
        fn apply_to_str2<'a, F>(s: &'a str, f: F) -> String
        where
            F: Fn(&'a str) -> String,
        {
            f(s)
        }
*/

#[cfg(test)]
mod for_lifetimes {
    #[test]
    pub fn for_lifetimes_intro() {
        // for every lifetime 'a independent of apply_to_str
        fn apply_to_str<F>(s: &str, f: F) -> String
        where
            F: for<'a> Fn(&'a str) -> String,
        {
            f(s)
        }

        let f = |input: &str| -> String { input.to_uppercase() };
        let _result = apply_to_str("hello", f);

        // Option 2: without for<'a>
        //  * more restrictive, only works with a specific lifetime
        //  * signature with more dependencies
        fn apply_to_str2<'a, F>(s: &'a str, f: F) -> String
        where
            F: Fn(&'a str) -> String,
        {
            f(s)
        }

        let f2 = |input: &str| -> String { input.to_lowercase() };
        let _result2 = apply_to_str2("HELLO", f2);
    }
}

