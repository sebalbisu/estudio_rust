#[allow(unused_variables)]
#[allow(dead_code)]
#[test]
fn index() {
    move_semantics::key_concept();
    when_ownership_moves::when_ownership_moves();
    avoid_move::avoid_move();
    pass_by_value::pass_by_value();
    pass_by_reference::pass_by_reference();
    no_ref_to_own_stack::no_ref_to_own_stack();
    return_solutions::solutions();
    nrvo::nrvo();
    drop_patterns::drop_patterns();
}

// ============================================================================
// 1. MOVE SEMANTICS - Key Concept
// ============================================================================
//
//     Moving ownership is ALWAYS:
//       • Copying BYTES from the stack (metadata: ptr, len, cap, etc.)
//       • Keeping the HEAP intact (the actual data doesn't move)
//
//     Example with String::from("hello"):
//
//     BEFORE the move:
//     ┌──────────────┐                  ┌───────────┐
//     │ s1 (stack)   │                  │   HEAP    │
//     │ ptr ─────────┼─────────────────►│ "hello"   │
//     │ len: 5       │                  └───────────┘
//     │ cap: 5       │
//     └──────────────┘
//
//     AFTER the move (let s2 = s1):
//     ┌──────────────┐  ┌──────────────┐   ┌───────────┐
//     │ s1 (invalid) │  │ s2 (stack)   │   │   HEAP    │
//     │ ██████████   │  │ ptr ─────────┼──►│ "hello"   │
//     │ ██████████   │  │ len: 5       │   │ (same!)   │
//     │ ██████████   │  │ cap: 5       │   └───────────┘
//     └──────────────┘  └──────────────┘
//
//     ✓ Stack: 24 bytes copied (ptr + len + cap)
//     ✓ Heap: 0 bytes copied ("hello" stays in the same place)
//     ✓ s1 becomes invalid (ownership transferred)
//
//     This applies to: String, Vec, Box, and any type with heap.
//     For Copy types (i32, f64, bool): they are copied completely.

#[cfg(test)]
mod move_semantics {
    #[test]
    pub fn key_concept() {
        let s1 = String::from("hello");
        let ptr_before = s1.as_ptr();

        let s2 = s1; // move: s1 invalid
        let ptr_after = s2.as_ptr();

        // The heap pointer is the same
        assert_eq!(ptr_before, ptr_after);
        // s1 is no longer valid, s2 is the new owner
        assert_eq!(s2, "hello");
    }
}

// ============================================================================
// 2. WHEN OWNERSHIP MOVES
// ============================================================================
//
//     Situations that cause move: (in move types)
//
//     1. ASSIGNMENT TO ANOTHER VARIABLE
//        let s2 = s1;  // s1 invalid
//
//     2. PASSING TO FUNCTION / METHOD BY VALUE
//        consume(s1);  // s1 invalid
//        x.method(s1);  // s invalid
//
//     3. RETURNING FROM A FUNCTION
//        fn create() -> String { String::from("x") }  // ownership to caller
//
//     5. DESTRUCTURING / PATTERN MATCHING
//        let (x, y) = tuple;  // tuple invalid
//
//     6. CLOSURE WITH `move`
//        let c = move || println!("{}", s);  // s moved to closure

#[cfg(test)]
mod when_ownership_moves {
    #[test]
    pub fn when_ownership_moves() {
        // 1. Assignment
        let s1 = String::from("hello");
        let s2 = s1;
        // s1 is no longer valid
        assert_eq!(s2, "hello");

        // 2. Passing to function by value
        fn consume(s: String) -> usize {
            s.len()
        }
        let s3 = String::from("world");
        let len = consume(s3);
        // s3 is no longer valid
        assert_eq!(len, 5);

        // 3. Returning from function
        fn create() -> String {
            String::from("created")
        }
        let s4 = create();
        assert_eq!(s4, "created");

        // 4. Inserting into collection
        let s5 = String::from("item");
        let mut vec = Vec::new();
        vec.push(s5);
        // s5 is no longer valid
        assert_eq!(vec[0], "item");

        // 5. Destructuring
        let tuple = (String::from("a"), String::from("b"));
        let (x, y) = tuple;
        // tuple is no longer valid
        assert_eq!(x, "a");
        assert_eq!(y, "b");

        // 6. Closure with move
        let s6 = String::from("closure");
        let closure = move || s6.len();
        // s6 is no longer valid
        assert_eq!(closure(), 7);
    }
}

// ============================================================================
// 3. HOW TO AVOID THE MOVE (keep ownership)
// ============================================================================
//
//     OPTION 1: Pass reference (borrow, no move)
//         fn borrow(s: &String) { ... }
//         borrow(&s);  // s remains valid
//
//     OPTION 2: Clone (deep copy, new heap)
//         let s2 = s1.clone();  // both valid
//
//     OPTION 3: Rc/Arc for multiple owners
//         let s = Rc::new(String::from("hello"));
//         let s2 = Rc::clone(&s);  // increments counter

#[cfg(test)]
mod avoid_move {

    #[test]
    pub fn avoid_move() {
        use std::rc::Rc;
        // Option 1: Reference
        fn borrow(s: &String) -> usize {
            s.len()
        }
        let s = String::from("hello");
        let len = borrow(&s);
        assert_eq!(s, "hello"); // s remains valid
        assert_eq!(len, 5);

        // Option 2: Clone
        let s1 = String::from("world");
        let s2 = s1.clone();
        assert_eq!(s1, s2); // both valid

        // Option 3: Rc
        let rc1 = Rc::new(String::from("shared"));
        let rc2 = Rc::clone(&rc1);
        let rc3 = Rc::clone(&rc1);
        assert_eq!(Rc::strong_count(&rc1), 3);
        assert_eq!(*rc1, *rc2);
        assert_eq!(*rc2, *rc3);
    }
}

// ============================================================================
// 4. PASS BY VALUE (Move between stacks)
// ============================================================================
//
//     Passing a variable by value (passing ownership) to a function:
//     The bytes of one stack are COPIED to the new stack frame,
//     and the original is considered 'moved' (cannot be used).
//
//     BEFORE calling consume(user):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ user: User                              │        HEAP             │
//     │ │   _id: 1                                │      ┌────────────┐     │
//     │ │   _name: (ptr, len:5, cap:5) ───────────┼────▶│ "Alice"    │     │
//     │ │   age: 30                               │      └────────────┘     │
//     │ └─────────────────────────────────────────┘                         │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     AFTER calling consume(user):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ user: ████ MOVED ████                   │                         │
//     │ │   (bytes still exist but inaccessible)  │                         │
//     │ └─────────────────────────────────────────┘                         │
//     ├─────────────────────────────────────────────────────────────────────┤
//     │ consume() stack frame (NEW)                                         │
//     │ ┌─────────────────────────────────────────┐        HEAP             │
//     │ │ user: User (COPIED from caller)         │      ┌────────────┐     │
//     │ │   _id: 1                                │      │ "Alice"    │     │
//     │ │   _name: (ptr, len:5, cap:5) ───────────┼────▶│ (same!)    │     │
//     │ │   age: 30                               │      └────────────┘     │
//     │ └─────────────────────────────────────────┘                         │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     WHEN consume() finishes:
//     - consume() stack frame destroyed
//     - user.drop() frees the heap
//
//     ✓ Move = copy bytes from stack + transfer of ownership
//     ✓ Only ONE owner can do Drop (prevents double-free)
//     ✓ The heap is NOT copied, only the pointer

#[cfg(test)]
mod pass_by_value {
    #[test]
    pub fn pass_by_value() {
        #[derive(Debug)]
        struct User {
            _id: u64,
            name: String,
            _age: u32,
        }
        let user = User {
            _id: 1,
            name: String::from("Alice"),
            _age: 30,
        };
        let ptr_before = user.name.as_ptr();

        fn consume(u: User) -> *const u8 {
            // The heap pointer is the same
            u.name.as_ptr()
        }

        let ptr_after = consume(user);
        // user is no longer valid

        // The heap pointer was the same inside consume
        assert_eq!(ptr_before, ptr_after);
    }
}

// ============================================================================
// 5. PASS BY REFERENCE (Pointer to another stack)
// ============================================================================
//
//     Passing a variable by reference to another function:
//     The reference is a pointer that points to the caller's stack.
//
//     DURING the call to borrow(&user):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ user: User                              │        HEAP             │
//     │ │   _id: 1                      ◄─────────┼──┐   ┌────────────┐     │
//     │ │   _name: (ptr, len, cap) ───────────────┼──┼─▶│ "Alice"    │     │
//     │ │   age: 30                               │  │   └────────────┘     │
//     │ └─────────────────────────────────────────┘  │                      │
//     ├──────────────────────────────────────────────┼──────────────────────┤
//     │ borrow() stack frame                         │                      │
//     │ ┌────────────────────────────────────────┐   │                      │
//     │ │ user: &User (8 bytes)                  │   │                      │
//     │ │   ptr ─────────────────────────────────┼───┘                      │
//     │ │   (points to main's stack!)            │                          │
//     │ └────────────────────────────────────────┘                          │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     WHEN borrow() finishes:
//     - borrow() does NOT drop user (only had a reference)
//     - user remains valid in main()
//
//     ✓ Reference = 8-byte pointer to the caller's stack
//     ✓ Does NOT transfer ownership
//     ✓ The original owner remains responsible for Drop

#[cfg(test)]
mod pass_by_reference {

    #[test]
    pub fn pass_by_reference() {
        #[derive(Debug)]
        struct User {
            _id: u64,
            name: String,
            age: u32,
        }
        let user = User {
            _id: 1,
            name: String::from("Bob"),
            age: 25,
        };
        let user_addr = &user as *const User;

        fn borrow(u: &User) -> *const User {
            u as *const User
        }

        let borrowed_addr = borrow(&user);

        // The reference points to the same place on stack
        assert_eq!(user_addr, borrowed_addr);

        // user remains valid after the borrow
        assert_eq!(user.age, 25);
        assert_eq!(user.name, "Bob");
    }
}

// ============================================================================
// 6. CANNOT RETURN REFERENCE TO OWN STACK
// ============================================================================
//
//     You cannot return a reference to a local variable
//     Rust prevents this at compile time.
//
//     CODE THAT DOESN'T COMPILE:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn dangling() -> &String {                                       │
//     │     let s = String::from("hello");                               │
//     │     &s  // ✗ ERROR: returns reference to local variable          │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘
//
//     WHY IT WOULD FAIL (if Rust allowed it):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ result: &String                         │        HEAP             │
//     │ │   ptr ──────────────────────────────────┼──┐   ┌────────────┐     │
//     │ └─────────────────────────────────────────┘  │   │ FREED!     │     │
//     │                                              │   └────────────┘     │
//     │ dangling() ← DESTROYED                       │                      │
//     │ ┌─────────────────────────────────────────┐  │                      │
//     │ │ ████████████████████████████████████████│◄─┘                      │
//     │ │ s: NO LONGER EXISTS (stack frame destroyed) │   ← DANGLING POINTER!│
//     │ └─────────────────────────────────────────┘                         │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     RULE: References can only point to:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ 1. PARENT stack frames (callers)  ← live longer than the function │
//     │ 2. Heap (via Box, Vec, String)    ← lives until owner's Drop      │
//     │ 3. 'static data (.rodata)         ← lives for the entire program  │
//     │                                                                  │
//     │ NEVER to own stack frame (dies when returning)                  │
//     └──────────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod no_ref_to_own_stack {
    #[test]
    pub fn no_ref_to_own_stack() {
        // This code does NOT compile:
        // fn dangling() -> &String {
        //     let s = String::from("hello");
        //     &s  // ERROR: `s` does not live long enough
        // }

        // The rule is: references can only point to
        // data that lives longer than the reference

        // This DOES work: return reference to parameter
        fn first<'a>(x: &'a str, _y: &str) -> &'a str {
            x
        }

        let s1 = String::from("hello");
        let s2 = String::from("world");
        let result = first(&s1, &s2);
        assert_eq!(result, "hello");
    }
}

// ============================================================================
// 7. VALID SOLUTIONS FOR RETURNING DATA
// ============================================================================
//
//     SOLUTION 1: Return ownership (move out)
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn create() -> String {                                          │
//     │     let s = String::from("hello");                               │
//     │     s  // ✓ Moves ownership to caller                            │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘
//
//     SOLUTION 2: Return reference to external parameter with lifetime
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {              │
//     │     if x.len() > y.len() { x } else { y }                        │
//     │ }  // ✓ Returns reference to parameter (lives in caller)         │
//     └──────────────────────────────────────────────────────────────────┘
//
//     SOLUTION 3: Return 'static
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn get_message() -> &'static str {                               │
//     │     "hello"  // ✓ String literal lives forever                   │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod return_solutions {
    #[test]
    pub fn solutions() {
        // Solution 1: Return ownership
        fn create() -> String {
            String::from("created")
        }
        let s = create();
        assert_eq!(s, "created");

        // Solution 2: Return reference to parameter
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        let s1 = String::from("short");
        let s2 = String::from("much longer");
        let result = longest(&s1, &s2);
        assert_eq!(result, "much longer");

        // Solution 3: Return 'static
        fn get_message() -> &'static str {
            "hello forever"
        }
        let msg = get_message();
        assert_eq!(msg, "hello forever");
    }
}


// ============================================================================
// 12. NRVO - Named Return Value Optimization
// ============================================================================
//
//     The compiler avoids copies when returning large values.
//
//     WITHOUT OPTIMIZATION (conceptual):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ create_array() stack:                                               │
//     │ ┌─────────────────────────────────┐                                 │
//     │ │ arr: [i64; 1000] (8000 bytes)   │                                 │
//     │ └─────────────────────────────────┘                                 │
//     │                 ↓ COPY 8000 bytes                                   │
//     │ main() stack:                                                       │
//     │ ┌─────────────────────────────────┐                                 │
//     │ │ result: [i64; 1000] (8000 bytes)│                                 │
//     │ └─────────────────────────────────┘                                 │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     WITH NRVO (what actually happens):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack:                                                       │
//     │ ┌─────────────────────────────────┐                                 │
//     │ │ result: [i64; 1000]             │ ← create_array WRITES           │
//     │ │ (8000 bytes)                    │   DIRECTLY HERE                 │
//     │ └─────────────────────────────────┘                                 │
//     │                                                                     │
//     │ The compiler passes &mut result to create_array() internally.      │
//     │ create_array() constructs the array IN-PLACE, without copy.        │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     CASES WHERE NRVO WORKS:
//     ✓ Direct return of local variable
//     ✓ Return of direct expression
//     ⚠ Multiple returns (may or may not optimize)
//
//     CASES WHERE THERE IS A COPY:
//     ✗ Passing by value to a function
//     ✗ Assigning to another variable (Copy types)
//     ✗ Explicit Clone

#[cfg(test)]
mod nrvo {

    #[test]
    pub fn nrvo() {
        const SIZE: usize = 10000;

        // NRVO: the compiler can optimize
        fn create_array() -> [i64; SIZE] {
            [42; SIZE]
        }

        let arr = create_array();
        assert_eq!(arr[0], 42);
        assert_eq!(arr[SIZE - 1], 42);

        println!("  ✅ nrvo::nrvo");
    }

    #[test]
    pub fn value_vs_reference() {
        use std::hint::black_box;
        use std::time::Instant;
        const SIZE: usize = 10000;
        let heavy_array: [i64; SIZE] = [42; SIZE];

        // By value (potential copy)
        fn sum_by_value(arr: [i64; 10000]) -> i64 {
            arr.iter().sum()
        }

        // By reference (no copy)
        fn sum_by_ref(arr: &[i64; 10000]) -> i64 {
            arr.iter().sum()
        }

        let iterations = 100;

        let start = Instant::now();
        for _ in 0..iterations {
            black_box(sum_by_value(black_box(heavy_array)));
        }
        let duration_value = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            black_box(sum_by_ref(black_box(&heavy_array)));
        }
        let duration_ref = start.elapsed();

        // Reference should be faster (or equal if inlined)
        assert!(duration_value.as_nanos() > 0);
        assert!(duration_ref.as_nanos() > 0);

        println!("  ✅ nrvo::value_vs_reference");
    }
}

// ============================================================================
// 13. DROP PATTERNS - Order and patterns of Drop
// ============================================================================
//
//     DROP PATTERNS:
//     • drop(x)           → Drops x immediately, you control when
//     • { let x; }        → Drops when exiting the block, LIFO order
//     • drop((a,b,c))     → Drops tuple and its fields in order
//     • option.take()     → Extracts and drops Option content
//     • vec with elements → Drops elements 0, 1, 2... then the Vec
//     • x = new_value     → Drops old value of x
//     • mem::forget(x)    → LEAK! Doesn't drop, avoid except FFI
//     • ManuallyDrop      → Full control, requires unsafe to drop
//
//     DROP ORDER:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ {                                                                │
//     │     let s1 = ...;  // declared first                            │
//     │     let s2 = ...;  // declared second                           │
//     │     let s3 = ...;  // declared third                            │
//     │ }                                                                 │
//     │ // Drop order: s3, s2, s1 (LIFO - last declared dropped first) │
//     └──────────────────────────────────────────────────────────────────┘
//
//     STRUCT DROP ORDER:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ struct Container {                                               │
//     │     first: T,   // dropped first                                │
//     │     second: T,  // dropped second                               │
//     │     third: T,   // dropped third                                │
//     │ }                                                                 │
//     │ // If Container implements Drop, it's called BEFORE fields     │
//     └──────────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod drop_patterns {
    use std::cell::Cell;

    #[allow(dead_code)]
    struct Droppable<'a> {
        #[allow(dead_code)]
        name: &'static str,
        counter: &'a Cell<usize>,
    }

    impl Drop for Droppable<'_> {
        fn drop(&mut self) {
            let count = self.counter.get();
            self.counter.set(count + 1);
            // println!("  → Dropping: {} (order #{})", self.name, count + 1);
        }
    }

    #[test]
    pub fn drop_patterns() {
        let counter = Cell::new(0);

        // Test LIFO order
        {
            let _s1 = Droppable {
                name: "s1",
                counter: &counter,
            };
            let _s2 = Droppable {
                name: "s2",
                counter: &counter,
            };
            let _s3 = Droppable {
                name: "s3",
                counter: &counter,
            };
        }
        assert_eq!(counter.get(), 3);

        // Test explicit drop
        counter.set(0);
        let s = Droppable {
            name: "explicit",
            counter: &counter,
        };
        drop(s);
        assert_eq!(counter.get(), 1);

        // Test Vec drop order
        counter.set(0);
        {
            let _v = vec![
                Droppable {
                    name: "v0",
                    counter: &counter,
                },
                Droppable {
                    name: "v1",
                    counter: &counter,
                },
                Droppable {
                    name: "v2",
                    counter: &counter,
                },
            ];
        }
        assert_eq!(counter.get(), 3);

        println!("  ✅ drop_patterns::drop_patterns");
    }

    #[test]
    pub fn option_take() {
        let counter = Cell::new(0);

        let mut maybe = Some(Droppable {
            name: "option_content",
            counter: &counter,
        });

        // take() extracts the value
        let _taken = maybe.take();
        assert!(maybe.is_none());
        assert_eq!(counter.get(), 0); // not dropped yet

        drop(_taken);
        assert_eq!(counter.get(), 1); // now it is

        println!("  ✅ drop_patterns::option_take");
    }

    #[test]
    #[allow(unused_assignments)]
    pub fn replace_drops_old() {
        let counter = Cell::new(0);

        let mut value = Droppable {
            name: "original",
            counter: &counter,
        };
        assert_eq!(counter.get(), 0);

        // Assigning new value drops the old one
        value = Droppable {
            name: "replacement",
            counter: &counter,
        };
        assert_eq!(counter.get(), 1); // original dropped

        drop(value);
        assert_eq!(counter.get(), 2); // replacement dropped

        println!("  ✅ drop_patterns::replace_drops_old");
    }

    #[test]
    pub fn manually_drop() {
        use std::mem::ManuallyDrop;
        let counter = Cell::new(0);

        let mut manual = ManuallyDrop::new(Droppable {
            name: "manual",
            counter: &counter,
        });

        // Does not drop automatically
        // We need to drop it explicitly with unsafe
        assert_eq!(counter.get(), 0);

        unsafe {
            ManuallyDrop::drop(&mut manual);
        }
        assert_eq!(counter.get(), 1);

        println!("  ✅ drop_patterns::manually_drop");
    }

    #[test]
    pub fn forget_leaks() {
        let counter = Cell::new(0);

        let leaked = Droppable {
            name: "leaked",
            counter: &counter,
        };

        // forget prevents Drop from being called - LEAK!
        std::mem::forget(leaked);
        assert_eq!(counter.get(), 0); // never dropped

        println!("  ✅ drop_patterns::forget_leaks");
    }
}
