/*
 ============================================================================
 INDEX - Run all demos
 ============================================================================
*/
#[test]
fn index() {
    type_copy::duplicate_and_original_valid();
    type_copy::copy_types();
    type_move::owned_move();
    references::references();
    raw_pointers::raw_pointers();
    auto_ref::auto_ref();
    deref::deref_trait();
    deref::deref_coercion();
    deref::deref_coercion_depth();
    deref::derefmut_coercion();
    dereference::ref_copy();
    dereference::deref_copy();
    dereference::deref_move();
    dereference::deref_multiple_manual();
    dereference::ref_mut_assignment();
    dereference::derefmut_assignment();
    deref_access_inner_methods_and_fields::inheritance();
    auto_deref::ref_methods();
    auto_deref::ref_fields();
    auto_deref::tderef_methods();
    auto_deref::tderef_fields();
    auto_deref::tderef_move();
    auto_deref::indexing();
    raw_pointers_fat_pointers_smart_pointers::info();
    smart_pointers::smart_pointer_box();
    smart_pointers::rc();
    smart_pointers::refcell();
    references_pointers::coercion_reference_to_raw_pointer();
    references_pointers::pointer_to_pointer();
    references_pointers::reference_from_raw_pointer();
}

/*
============================================================================
1. OWNED T - Value ownership
============================================================================

COPY TYPE:
-----------------------------------------------------------

    Diagram:
        let a: i32 = 42;
        let b = a;

        STACK:
        ┌───────┐  ┌───────┐
        │ a: 42 │  │ b: 42 │  ← Two independent values
        └───────┘  └───────┘

    Characteristics:
        * value on stack
        * On assignment/pass: value is duplicated (memcpy bit by bit)
        * Original remains valid
        * Cheap: only copies bytes on stack
        * No special cleanup needed (no Drop)
        * method(self) -> send a copy of self

    Copy Types:
        * References: &T and &mut T
        * Raw pointers: *const T and *mut T
        * Primitive types: u32, i32, f64, bool, char
        * Tuples of Copy types: ((), (i32, bool), etc.)
        * Arrays of Copy types: ([i32; 3], etc.)
        * Structs/Enums of Copy types

    Trait:
    trait Copy : Clone {}
        Is a marker trait (without methods)
        Implemented on structs/enums to make them Copy. (easier with derive)
        Cannot mark a type as Copy if it has Drop (or is Move).

MOVE TYPE: !Copy
-----------------------------------------------------------

    Diagram:
        let s1 = String::from("hello");
        let s2 = s1;

        STACK:                      HEAP:
        ┌──────────────┐           ┌─────────┐
        │ s1 (invalid) │           │ "hello" │
        │ ptr ──────────────┐      │         │
        └──────────────┘    │      └─────────┘
                            │           ▲
        ┌──────────────┐    │           │
        │ s2           │    │           │
        │ ptr ──────────────┴───────────┘
        │ len: 5       │
        │ cap: 5       │
        └──────────────┘

    Characteristics:
        * value on heap, metadata on stack
        * On assignment/pass: ownership is moved (shallow copy stack)
        * Original becomes invalid
        * Cheap: only copies metadata on stack
        * Requires special cleanup (Drop) at scope end

    Move Types:
        * Smart pointer = heap + (ptr + Metadata stack) + Deref + Drop + !Copy
            (String, Vec, Box, HashMap, Rc, Arc, etc.)
        * Structs/Enums containing smart pointers

    Why smart pointers are not copied:
        If a smart pointer were copied, only the stack metadata would be copied,
        both pointing to the same heap content, which is dangerous (double free
        when both are dropped). So only moving ownership is allowed. Same for
        structs with smart pointers.

*/

#[cfg(test)]
mod type_copy {
    use std::fmt::Debug;
    use std::ptr::{addr_of};

    fn assert_is_copy<T: Copy + PartialEq + Debug>(x: T, y: T) {
        assert_eq!(x, y);
        assert_ne!(addr_of!(x), addr_of!(y));
    }

    // duplicate and original are valid
    #[test]
    pub fn duplicate_and_original_valid() {
        let a: i32 = 42;
        let b = a; // copy the value
        assert_is_copy(a, b); // a and b valid and independent
    }

    // method(self) -> send a copy of self
    #[test]
    pub fn method_copy() {
        let a: i32 = 42;
        assert_eq!(a.abs(), 42); // send a copy to abs(self)
        assert_eq!(a.abs(), 42); // a is not consumed
    }

    // different Copy types:
    #[test]
    pub fn copy_types() {
        // references
        let value = 10;
        let ref_1 = &value;
        let ref_2 = ref_1; // copy the reference
        assert_is_copy(ref_1, ref_2);

        // raw pointers
        let raw_ptr_1: *const i32 = &value;
        let raw_ptr_2 = raw_ptr_1; // copy the raw pointer
        assert_is_copy(raw_ptr_1, raw_ptr_2);

        let x: u32 = 10;
        let y = x;
        assert_is_copy(x, y);

        let flag: bool = true;
        let flag2 = flag;
        assert_is_copy(flag, flag2);

        let tup: (i32, bool) = (5, false);
        let tup2 = tup;
        assert_is_copy(tup, tup2);

        let arr: [i32; 3] = [1, 2, 3];
        let arr2 = arr; // copy
        assert_is_copy(arr, arr2);

        // Struct requires implementing Copy
        #[derive(Copy, Clone, PartialEq, Debug)]
        struct MyStruct {
            a: i32,
            b: bool,
        }
        let my_struct = MyStruct { a: 7, b: true };
        let my_struct2 = my_struct;
        assert_is_copy(my_struct, my_struct2);
    }
}

#[cfg(test)]
mod type_move {

    // MOVE TYPE: 
    // metadata on stack, content on heap
    // value is invalid after move
    #[test]
    pub fn owned_move() {
        let original: String = String::from("hello");

        // String has 24 bytes on stack: ptr + len + cap
        assert_eq!(std::mem::size_of::<String>(), 24);
        assert_eq!(original.len(), 5);
        assert_eq!(original.capacity(), 5);

        let ptr_before = original.as_ptr();
        let after_moved = original; // move: copy 24 bytes on stack
        let ptr_after_moved = after_moved.as_ptr();

        // The pointer to heap is the same (heap content was not copied)
        assert_eq!(ptr_before, ptr_after_moved);
        // owned_move is no longer valid here
    }
}

/*
 ============================================================================
 2. REFERENCES &T 
 ============================================================================

    References are a way to borrow without ownership
    Similar to pointers, but with safety guarantees by the compiler.

    Diagram:
        let val: i32 = 42;
        let ref_val: &i32 = &val;

        STACK:
        ┌────────────────────────┐
        │ val: 42                │ ◄─────────┐
        │ @ 0x7fff1234           │           │
        ├────────────────────────┤           │
        │ ref_val: &i32          │           │
        │ @ 0x7fff1238           │           │
        │ value: 0x7fff1234 ─────────────────┘
        └────────────────────────┘

    &T is just a memory address (8 bytes on 64-bit)
    ref_val is another variable on stack that CONTAINS the address of val
    it is Copy

    Special case: Reference to value without variable
    --------------------------------
    let x = &10;
    Create a reference to a temporary value, which lives on the stack, 
    and dies when stack exits
*/

#[cfg(test)]
mod references {

    // References are a way to borrow without ownership
    #[test]
    pub fn references() {
        let val: i32 = 42;
        let ref_val: &i32 = &val; // &val = 0x7fff1234 (address of val) 

        // &T is 8 bytes (pointer)
        assert_eq!(std::mem::size_of::<&i32>(), 8);

        // ref_val points to val
        assert_eq!(*ref_val, 42);
        assert_eq!(ref_val as *const i32, &val as *const i32);

        // Multiple immutable references allowed
        let ref2: &i32 = &val;
        let ref3: &i32 = &val;
        assert_eq!(*ref2, *ref3);

        // Mutable reference
        let mut val_mut: i32 = 10;
        let ref_mut: &mut i32 = &mut val_mut;
        *ref_mut = 20;
        assert_eq!(val_mut, 20);

        // reference to value without variable.
        let x = &10;
        assert_eq!(*x, 10);
    }
}

/*
============================================================================
RAW POINTERS *const T / *mut T
============================================================================

    Similar to &T / &mut T, but without safety guarantees.
    Memory address that contains another memory address as value.

     let val: i32 = 100;
     let ptr: *const i32 = &val;

     DIFFERENCES &T vs *const T:
     ┌─────────────────────┬──────────────────┬────────────────────┐
     │ Characteristic      │ &T               │ *const T           │
     ├─────────────────────┼──────────────────┼────────────────────┤
     │ Can be null         │ ❌ Never         │ ✅ Yes             │
     │ Always valid        │ ✅ Guaranteed    │ ❌ Not guaranteed  │
     │ Lifetime checking   │ ✅ Compiler      │ ❌ Manual          │
     │ Dereference         │ Safe             │ unsafe             │
     └─────────────────────┴──────────────────┴────────────────────┘
*/
#[cfg(test)]
mod raw_pointers {
    #[test]
    pub fn raw_pointers() {
        let val: i32 = 100;
        let ptr: *const i32 = &val;

        // The raw pointer contains the address
        assert!(!ptr.is_null());

        // Dereferencing requires unsafe
        unsafe {
            assert_eq!(*ptr, 100);
        }

        // *mut T for mutability
        let mut val_mut: i32 = 50;
        let ptr_mut: *mut i32 = &mut val_mut;

        unsafe {
            *ptr_mut = 75;
        }
        assert_eq!(val_mut, 75);

        // Null pointer (not possible with &T)
        let null_ptr: *const i32 = std::ptr::null();
        assert!(null_ptr.is_null());
    }
}

/*
============================================================================
AUTO-REF in Methods
============================================================================

    Rust automatically adds & / &mut in method calls, if the method requires it.

    Idea:
        struct Data { value: i32 }
        impl Data {
            fn by_ref(&self) -> i32 { self.value }
            fn by_ref_mut(&mut self, new_val: i32) { self.value = new_val; }
        }

        let mut d = Data { value: 42 };

        d.by_ref()        → Rust converts to: (&d).by_ref()
        d.by_ref_mut(15)  → Rust converts to: (&mut d).by_ref_mut(15)

    AUTO-REF only works with . operator (method calls)
    Does NOT work with free functions: fn foo(x: &T) requires foo(&val)
*/
#[cfg(test)]
mod auto_ref {
    struct Data {
        value: i32,
    }

    impl Data {
        fn by_ref(&self) -> i32 {
            self.value
        }
        fn by_ref_mut(&mut self, new_val: i32) {
            self.value = new_val;
        }
    }

    fn free_function(d: &Data) -> i32 {
        d.value
    }

    // Auto-ref in methods: d.method() → (&d).method()
    #[test]
    pub fn auto_ref() {
        let mut d = Data { value: 42 };

        // Auto-ref in methods: d.method() → (&d).method()
        assert_eq!(d.by_ref(), 42); // Rust adds & automatically

        // Auto-ref with &mut
        d.by_ref_mut(100); // Rust adds &mut automatically
        assert_eq!(d.value, 100);

        // In free functions there is NO auto-ref
        let result = free_function(&d); // WE MUST put &
        assert_eq!(result, 100);
    }
}

/*
============================================================================
Deref and DerefMut
============================================================================

    Obtains a reference to where the smart pointer points
    a reference of the Target, through the deref() method

    trait Deref {
        type Target: ?Sized;
        fn deref(&self) -> &Self::Target;
    }

    DerefMut
    ------------------------
    Similar to Deref, but for mutable references &mut T

    Auto-Deref Coercion
    ------------------------
    Whenever a &Target is needed and you have a reference &T where T:Deref,
    Rust automatically applies T.deref() to obtain the required reference.

        let a: Box<i32> = Box::new(10);
        let ref1: &i32 = &a;  // implicit
        let ref2: &i32 = a.deref(); // explicit

    Auto-Deref Coercion in depth
    ------------------------
    Applied recursively .deref() until reaching the required &Target

        let b: Box<Box<i32>> = Box::new(Box::new(20));
        let ref1: &i32 = &b;   // implicit
        let ref2: &i32 = b.deref().deref(); // explicit
*/
#[cfg(test)]
mod deref {
    use std::ops::Deref;

    // Auto-Deref Coercion: obtains a reference to the Target
    #[test]
    pub fn deref_trait() {
        struct MyPointer<T>(T);

        impl<T> Deref for MyPointer<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let my_ptr = MyPointer(42);
        let ref1: &i32 = &my_ptr; // implicit
        let ref2: &i32 = my_ptr.deref(); // explicit
        assert_eq!(ref1, &42);
        assert_eq!(ref2, &42);
    }

    #[test]
    pub fn deref_coercion() {
        let a: Box<i32> = Box::new(10);
        let ref1: &i32 = &a; // implicit
        let ref2: &i32 = a.deref(); // explicit
        assert_eq!(ref1, &10);
        assert_eq!(ref2, &10);
    }

    #[test]
    pub fn deref_coercion_depth() {
        let b: Box<Box<i32>> = Box::new(Box::new(20));
        let ref1: &i32 = &b; // implicit
        let ref2: &i32 = b.deref().deref(); // explicit
        assert_eq!(ref1, &20);
        assert_eq!(ref2, &20);
    }

    #[test]
    pub fn derefmut_coercion() {
        let mut c: Box<i32> = Box::new(30);
        let ref_mut1: &mut i32 = &mut c; // implicit
        // let ref_mut1: &mut i32 = c.deref_mut(); // explicit
        *ref_mut1 += 5;
        assert_eq!(*c, 35);
    }
}


/*
============================================================================
Dereference *
============================================================================

    Obtain the value behind the reference or smart pointer.

    * references: Obtain the value of a reference
    * T:Deref / DerefMut: get *T = Target, value where T points with deref() -> 
        (takes the reference with deref(), then obtains the value).

    References: *ref -> content
    T:Deref:     *T -> *(T.deref()) -> *&Target -> Target
                **T -> *(T.deref().deref()) -> *&Target2 -> Target2

    No AutoDeref in depth:
    ----------------------------
    Does not search in depth of deref() only uses the first, for multiple depth use multiple **T.

    Content assignment:
    ----------------------------
    *ref = value;
    If ref is &mut T or T:DerefMut, assigns value to pointed content.
*/
#[cfg(test)]
mod dereference {

    #[test]
    pub fn ref_copy() {
        let a: i32 = 10;
        let ref1: &i32 = &a;
        let _val1: i32 = *ref1; // a is Copy so it copies
        assert_eq!(a, 10);
        assert_eq!(_val1, 10);
    }

    #[test]
    pub fn deref_copy() {
        let x: Box<i32> = Box::new(42);
        let y: i32 = *x; // copy
        assert_eq!(y, 42);
        assert_eq!(*x, 42); // x remains valid
    }

    #[test]
    pub fn deref_move() {
        let x: Box<String> = Box::new(String::from("hello"));
        let s: String = *x; // moves the String out of the Box
        assert_eq!(s, "hello");
        // assert_eq!(*x, "hello"); // x is now consumed and cannot be used
    }

    #[test]
    pub fn deref_multiple_manual() {
        let x: Box<Box<i32>> = Box::new(Box::new(100));
        let y: i32 = **x; // copy the i32 out of the inner Box
        assert_eq!(y, 100);
        assert_eq!(**x, 100);
    }

    #[test]
    pub fn ref_mut_assignment() {
        let mut a: i32 = 20;
        let ref_mut: &mut i32 = &mut a;
        *ref_mut = 30; // assign to pointed content
        assert_eq!(a, 30);
    }

    #[test]
    pub fn derefmut_assignment() {
        let mut b: Box<i32> = Box::new(30);
        *b = 40; // assign to pointed content
        assert_eq!(*b, 40);
    }
}

/*
============================================================================
6. AUTO-DEREF: * 
============================================================================

Similar to Auto-ref, but for the value instead of a reference.

it performs auto-deref to reach the value that has the method or field.

Methods through references
--------------------------------
    // abs(self) -> Self

    let n: i32 = -5;
    let m: &i32 = &n;
    let o: &&i32 = &m;
    n.abs()    // direct
    m.abs()    // Rust does: (*m).abs()
    o.abs()    // Rust does: (**o).abs()

Fields and references
--------------------------------
    let p: &Point = &point;
    p.x      // Rust does: (*p).x

T:Deref and methods
--------------------------------
    let b: Box<String> = Box::new(String::from("hello"));
    b.len() // where len(&self) -> usize
            // method auto-deref: (*b).len()  = String.len()
            // method auto-ref:  (&*b).len() -> &String.len()
    &*b     // &String, does not move *b the value and then takes the ref, 
            // only takes the final value/type to do the operation


T:Deref and fields
--------------------------------
    let box_point: Box<Point> = Box::new(Point { x: 5, y: 15 });
    box_point.x    // Rust does: (*box_point).x


Move type when dereferencing moves the content:
--------------------------------
    let c: Box<String> = Box::new(String::from("world"));
    let d = *c; // moves the String out of the Box
    // let x = c;   // error: use of moved value: `c`

Indexing through references
--------------------------------
    let v: &Vec<i32> = &vec![1,2,3];
    v[0];            // Rust does: (*v)[0]

*/
#[cfg(test)]
mod auto_deref {
    #[test]
    pub fn ref_methods() {
        let n: i32 = -5;
        let m: &i32 = &n;
        let o: &&i32 = &m;

        assert_eq!(n.abs(), 5); // direct
        assert_eq!(m.abs(), 5); // Rust does: (*m).abs()
        assert_eq!(o.abs(), 5); // Rust does: (**o).abs()
    }

    #[test]
    pub fn ref_fields() {
        struct Point {
            x: i32,
            _y: i32,
        }
        let point = Point { x: 10, _y: 20 };
        let p: &Point = &point;
        assert_eq!(p.x, 10); // Rust does: (*p).x
    }

    #[test]
    pub fn tderef_methods() {
        let b: Box<String> = Box::new(String::from("hello"));
        assert_eq!(b.len(), 5); // Rust does: (&*b).len()
    }

    #[test]
    pub fn tderef_fields() {
        struct Point {
            x: i32,
            _y: i32,
        }
        let box_point: Box<Point> = Box::new(Point { x: 5, _y: 15 });
        assert_eq!(box_point.x, 5); // Rust does: (*box_point).x
    }

    #[test]
    pub fn tderef_move() {
        let c: Box<String> = Box::new(String::from("world"));
        let d = *c; // moves the String out of the Box
        assert_eq!(d, "world");
        // let x = c;   // error: use of moved value: `c`
    }

    #[test]
    pub fn indexing() {
        let v: &Vec<i32> = &vec![1, 2, 3];
        assert_eq!(v[0], 1); // Rust does: (*v)[0]
    }
}


/*
============================================================================
DEREF: ACCESS TO INNER METHODS AND FIELDS
============================================================================

    The compiler performs multiple deref coercions in sequence 
    to find the member/method you're accessing

    in case of methods:
        * method(&self)  // is ok
        * method(self)   // only works if self is Copy.

    // B -> A -> value: i32 -> abs()
    b.abs()    // b.a.value.abs()
*/

#[cfg(test)]
mod deref_access_inner_methods_and_fields {

    #[test]
    pub fn inheritance() {
        use std::ops::Deref;

        struct A {
            value: i32,
        }

        impl Deref for A {
            type Target = i32;
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        struct B {
            a: A,
            _y: i32,
        }

        impl Deref for B {
            type Target = A;
            fn deref(&self) -> &Self::Target {
                &self.a
            }
        }
        
        let b = B {
            a: A { value: 123 },
            _y: 2,
        };

        assert_eq!(b.value, 123); // B -> A -> value
                                  // (&*b).value
        assert_eq!(b.abs(), 123); // B -> A -> i32 -> abs(), searchs for an abs()
                                  // (**b).abs() = 123
        assert_eq!(b.abs(), 123); // before the value was copied, so it works!
    }
}

/*
============================================================================
RAW/THIN POINTERS, FAT POINTER, SMART POINTERS
============================================================================

    1. RAW/THIN POINTERS (*const T, *mut T) - REFERENCES
    ────────────────────────────────────────────────────
    Raw pointers are pointers to a memory address.
    Size: 8 bytes (thin pointer)
    
    Raw pointers can point to EITHER stack OR heap (compiler doesn't care!)
    
    ┌─────────────────┐              ┌──────────────┐
    │ raw_ptr: 0x1000 │ ───────────→ │ value: 42    │
    │ (8 bytes)       │              └──────────────┘
    └─────────────────┘


    2. FAT POINTERS (&[T], &dyn Trait)
    ────────────────────────────────────────────────────
    Fat pointers are pointers to a memory address that contains metadata.
    Size: 16 bytes (8 bytes ptr + 8 bytes metadata)
    No owns the data.
    No cleanup, no Drop. just a pointer to the data with metadata.
    
    SLICE &[T] - Points to HEAP data with length metadata:
    ┌──────────────────────────┐  ┌─────────────┐
    │ data_ptr: 0x1008 ────────┼──→ arr[1]: 2   │
    │ (8 bytes)                │  ├─────────────┤
    ├──────────────────────────┤  │ arr[2]: 3   │
    │ len: 2 (metadata)        │  └─────────────┘
    │ (8 bytes)                │   (HEAP)
    └──────────────────────────┘
    
    TRAIT OBJECT &dyn Trait - Points to HEAP with vtable:
    ┌──────────────────────────┐  ┌───────────────────┐
    │ data_ptr: 0x2000 ────────┼──→ [object instance] │
    │ (8 bytes)                │  └───────────────────┘
    ├──────────────────────────┤  ┌───────────────────┐
    │ vtable_ptr: 0x3000 ──────┼──→ [method pointers] │
    │ (8 bytes)                │  └───────────────────┘
    └──────────────────────────┘   (both on HEAP)


    3. SMART POINTERS (Box<T>, Rc<T>, Arc<T>, String, Vec<T>)
    ────────────────────────────────────────────────────────────

    Smart pointers are pointers that contain metadata.
    They are used to store data on the heap and manage its lifetime.
    Owns the data.
    Clean up with Drop trait.

    Box<T>
    ----------------------------
    Unique ownership on heap (8 bytes stack pointer)
    ┌─────────────────────┐    ┌────────────┐
    │ STACK (8 bytes)     │    │ HEAP       │
    ├─────────────────────┤    ├────────────┤
    │ box_ptr: 0x4000  ───┼───→│ value: T   │
    └─────────────────────┘    └────────────┘
    
    When box drops → automatically frees heap data
    
    String
    ----------------------------
    Owned UTF-8 text (24 bytes on stack)
    
    STACK (24 bytes)                    HEAP
    ┌──────────────────────────────┐   ┌─────────────────────┐
    │ ptr: 0x4000 (8 bytes) ───────┼──→│ 'h' 'e' 'l' 'l' 'o' │
    ├──────────────────────────────┤   └─────────────────────┘
    │ len: 5 (8 bytes)             │   (5 bytes of data)
    ├──────────────────────────────┤
    │ capacity: 8 (8 bytes)        │
    └──────────────────────────────┘

    Vec<T>
    ----------------------------
    Dynamic array (24 bytes on stack)

    STACK (24 bytes)                    HEAP
    ┌──────────────────────────────┐   ┌──────────────┐
    │ ptr: 0x5000 (8 bytes) ───────┼──→│ 1 │ 2 │ 3  | ... |
    ├──────────────────────────────┤   │  │   │   │    │
    │ len: 3 (8 bytes)             │   └──────────────┘
    ├──────────────────────────────┤   (3 elements)
    │ capacity: 8 (8 bytes)        │
    └──────────────────────────────┘
*/
#[cfg(test)]
mod raw_pointers_fat_pointers_smart_pointers {
    #[test]
    pub fn info() { }
}

/*
============================================================================
8. SMART POINTERS - Pointers with extra functionality
============================================================================

    SMART POINTERS SUMMARY:
    ┌─────────────────┬─────────────────────────────────────────────────────┐
    │ Type            │ Use                                                 │
    ├─────────────────┼─────────────────────────────────────────────────────┤
    │ Box<T>          │ Value on heap, unique ownership                     │
    │ Rc<T>           │ Multiple owners, single-thread                      │
    │ Arc<T>          │ Multiple owners, multi-thread                       │
    │ RefCell<T>      │ Interior mutability (borrow check at runtime)       │
    │ Cell<T>         │ Interior mutability for Copy types                  │
    └─────────────────┴─────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod smart_pointers {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    pub fn smart_pointer_box() {
        // Box<T> - Unique ownership on heap
        let box_val: Box<i32> = Box::new(42);
        assert_eq!(*box_val, 42);

        // Useful for recursive types
        #[allow(dead_code)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        let _list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

        println!("  ✅ smart_pointers::box");
    }

    #[test]
    pub fn rc() {
        // Rc<T> - Multiple owners (single-thread)
        let rc1: Rc<i32> = Rc::new(42);
        let rc2 = Rc::clone(&rc1);
        let rc3 = Rc::clone(&rc1);

        assert_eq!(Rc::strong_count(&rc1), 3);
        assert_eq!(*rc1, 42);
        assert_eq!(*rc2, 42);
        assert_eq!(*rc3, 42);

        // All point to the same value
        assert!(Rc::ptr_eq(&rc1, &rc2));

        drop(rc3);
        assert_eq!(Rc::strong_count(&rc1), 2);

        println!("  ✅ smart_pointers::rc");
    }

    #[test]
    pub fn refcell() {
        // RefCell<T> - Borrow checking at runtime
        let cell: RefCell<i32> = RefCell::new(42);

        // Immutable borrow
        {
            let borrowed = cell.borrow();
            assert_eq!(*borrowed, 42);
        }

        // Mutable borrow
        {
            let mut borrowed_mut = cell.borrow_mut();
            *borrowed_mut = 100;
        }

        assert_eq!(*cell.borrow(), 100);

        // Arc<T> for multi-thread
        let arc1: Arc<i32> = Arc::new(42);
        let arc2 = Arc::clone(&arc1);
        assert_eq!(Arc::strong_count(&arc1), 2);
        assert_eq!(*arc1, *arc2);

        println!("  ✅ smart_pointers::refcell");
    }
}
/*
============================================================================
10. REFERENCES - POINTERS
============================================================================

CONVERSION CASES between references and raw pointers:

1. COERCION: &T → *const T (automatic)
2. POINTER TO POINTER: *const *const T
3. SAFE DEREFERENCE: *const T → &T (inside unsafe)
*/
#[cfg(test)]
mod references_pointers {
    #[test]
    pub fn coercion_reference_to_raw_pointer() {
        // ─────────────────────────────────────────────────────────────────
        // Coercion to raw pointers from references
        // ─────────────────────────────────────────────────────────────────
        //
        // &T can be automatically converted to *const T
        // This conversion is safe because the reference guarantees validity

        let a = 42;
        let x: &i32 = &a; // x: &i32

        // FORM 1: Automatic conversion (coercion)
        let y: *const i32 = x;
        // Valid - Rust automatically infers that &i32 → *const i32

        // FORM 2: Explicit cast
        let z = x as *const i32;
        // Valid - Manual and explicit cast

        // FORM 3: Conversion with type annotation (if needed)
        let w: *const i32 = x as *const i32;
        // Valid - Redundant but explicit

        // All point to the same address
        assert_eq!(y, z);
        assert_eq!(z, w);
        assert_eq!(y as usize, &a as *const i32 as usize);

        unsafe {
            assert_eq!(*y, 42);
            assert_eq!(*z, 42);
            assert_eq!(*w, 42);
        }

        println!("  ✅ references_pointers::coercion_reference_to_raw_pointer");
    }

    #[test]
    pub fn pointer_to_pointer() {
        // ─────────────────────────────────────────────────────────────────
        // A pointer can point to another pointer
        // ─────────────────────────────────────────────────────────────────
        //
        // *const *const T = pointer to pointer
        // Access: ** dereferences two levels

        let x = 42;
        let ptr_x: *const i32 = &x as *const i32; // Pointer to i32

        // Pointer to pointer
        let ptr_to_ptr: *const *const i32 = &ptr_x as *const *const i32;

        // Visualized:
        // ptr_to_ptr → address of ptr_x → address of x → value 42
        //   0x3000      0x2000              0x1000          42

        // Access:
        unsafe {
            assert_eq!(*ptr_to_ptr as *const i32, ptr_x);
            assert_eq!(**ptr_to_ptr, 42); // Double dereference
        }

        // Verify addresses
        assert_eq!(ptr_to_ptr as *const *const i32, &ptr_x as *const *const i32);

        println!("  ✅ references_pointers::pointer_to_pointer");
    }

    #[test]
    pub fn reference_from_raw_pointer() {
        // ─────────────────────────────────────────────────────────────────
        // A reference can be borrowed from a pointer
        // ─────────────────────────────────────────────────────────────────
        //
        // *const T → &T requires:
        // - Safe dereference (inside unsafe)
        // - Pointer validity

        let x = 42;
        let ptr_x: *const i32 = &x as *const i32;

        // ❌ You cannot do this:
        // let ref_x: &i32 = ptr_x;  // Error: no automatic conversion

        // ✅ You must dereference the pointer inside unsafe:
        unsafe {
            let ref_x: &i32 = &*ptr_x; // Dereference + take reference
            assert_eq!(*ref_x, 42);
            assert_eq!(ref_x, &42);
        }

        // Safe pattern with validation
        if !ptr_x.is_null() {
            unsafe {
                let ref_x: &i32 = &*ptr_x;
                assert_eq!(*ref_x, 42);
            }
        }

        println!("  ✅ references_pointers::reference_from_raw_pointer");
    }
}
