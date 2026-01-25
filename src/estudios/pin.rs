#[test]
fn index() {
    the_problem::movement::changes_direction();
    the_problem::movement::to_function();

    the_problem::tmp_solve_with_box::does_not_break_when_moving_box();
    
    the_problem::move_content::changing_content_breaks_self_reference();
    the_problem::move_content::dereference_box_to_move_content();

    unpin_and_not_unpin::unpin_usage();
    unpin_and_not_unpin::not_unpin_usage();

    pin_stack_and_heap::pin_stack_not_unpin_usage();

    pin_project::some_fields_pinned_strong();
}

/*
========================================================================
THE PROBLEM WITH SELF-REFERENTIAL STRUCTS
========================================================================

    Problem: move ownership on stack:
    ------------------------------------------------
        A self-referential struct has a field that points to another field of the same
        struct. This is DANGEROUS because when the struct is moved, the internal pointer
        ends up pointing to invalid memory (dangling pointer).


        CREATION:                     AFTER MOVING:
        ┌────────────────────┐        ┌────────────────────┐
        │ sr @ 0x1000        │        │ sr_moved @ 0x2000  │
        │ data: "Hello"      │        │ data: "Hello"      │
        │ pointer: 0x1000 ───┼──┐     │ pointer: 0x1000 ───┼──→ ??? DANGLING
        └────────────────────┘  │     └────────────────────┘
                    ▲            │
                    └────────────┘     The pointer still points to 0x1000
                                        but data is now at 0x2000!

        → Using `pointer` is UNDEFINED BEHAVIOR



    Temporary Solution: Box<T> solves the problem
    ------------------------------------------------

        Box<T> puts the value on the heap. 
        When moving a BOX<T> variable, the self-reference does NOT break 
        because the heap doesn't move, only the pointer location 
        on the stack changes (8 bytes).


    Problem with Box<T>: Moving content breaks self-reference
    ------------------------------------------------

        Content can be moved directly from the stack or heap, 
        breaking the self-reference:

        std::mem::swap(&mut *var1, &mut *var2)  // swap contents
        std::mem::replace(&mut *var1, new_value) // replace content

        equivalent using dereferencing (on heap):
        (*var1, *var2) = (*var2, *var1) // swap
        *var1 = *var2;  // replace

        It cancels the solution of using Box<T> to solve the problem.
        
            let boxed = Box::new(var_self_ref);
            let moved_stack: T = *boxed // dereference and move to stack, breaks self-reference
            let moved_heap = Box::new(*boxed); // move to another heap, breaks self-reference        
    
*/
mod the_problem {
    #[cfg(test)]
    pub mod movement {
    
        #[derive(Debug, Clone)]
        pub struct Movable {
            pub value: i32,
            pub ptr: *const i32,
        }
    
        /// Values change address when moved
        #[test]
        pub fn changes_direction() {
            let mut val = Movable {
                value: 42,
                ptr: std::ptr::null(),
            };
            val.ptr = &val.value as *const i32;
    
            let val2 = val;
            let ptr2 = &val2.value as *const i32;
    
            assert_ne!(val2.ptr, ptr2, "The value moved to another address");
        }
    
        /// When passing to functions it also moves
        #[test]
        pub fn to_function() {
            fn take_and_return_address(m: Movable) -> *const i32 {
                &m.value as *const i32
            }
    
            let mut val = Movable {
                value: 42,
                ptr: std::ptr::null(),
            };
            val.ptr = &val.value as *const i32;
            let ptr1 = val.ptr;
    
            let ptr2 = take_and_return_address(val);
    
            assert_ne!(ptr1, ptr2, "The value moved");
        }
    }
        

    #[cfg(test)]
    pub mod tmp_solve_with_box {
    
        #[test]
        pub fn does_not_break_when_moving_box() {
            #[derive(Debug)]
            pub struct UnsafeSelfRef {
                pub data: i32,
                pub pointer: *const i32,
            }
    
            let mut var = Box::new(UnsafeSelfRef {
                data: 42,
                pointer: std::ptr::null(),
            });
            var.pointer = &var.data as *const i32;
            let ptr1 = var.pointer;
    
            let var_moved = var;
    
            assert_eq!(
                ptr1, var_moved.pointer,
                "Both point to the same heap address"
            );
        }
    }
    #[cfg(test)]
    pub mod move_content {
    
        /*
        CHANGING CONTENT BREAKS SELF-REFERENCE
        ------------------------------------------------
        */
        #[test]
        pub fn changing_content_breaks_self_reference() {
            struct AutoRef {
                value: bool,
                ptr: *const bool,
            }
    
            let mut var1 = AutoRef {
                value: true,
                ptr: std::ptr::null(),
            };
            var1.ptr = &var1.value as *const bool;
    
            let mut var2 = AutoRef {
                value: true,
                ptr: std::ptr::null(),
            };
            var2.ptr = &var2.value as *const bool;
    
            let _ = std::mem::replace(&mut var1, var2);
            assert_ne!(var1.ptr, &var1.value);
        }
    
        /*
        (DEREFERENCE BOX<T> to move content)
        ------------------------------------------------
        */
        #[test]
        pub fn dereference_box_to_move_content() {
            #[derive(Debug, Clone)]
            struct AutoRef {
                value: i32,
                ptr: *const i32,
            }
    
            let mut var1 = Box::new(AutoRef {
                value: 1,
                ptr: std::ptr::null(),
            });
            var1.ptr = &var1.value as *const i32;
    
            let var2 = Box::new(*var1);
    
            assert_ne!(var2.ptr, &var2.value);
        }
    }
}


/*
========================================================================
CONCEPTS AND DEFINITIONS
========================================================================

    Pin:
        When a value is pinned, it means that it cannot be moved to another memory address
        in case it's !Unpin.

    Unpin:
        A type that can be safely moved (no self-references).
        All structs are Unpin by default.
        Except those marked with PhantomPinned.

    !Unpin:
        A type that cannot be safely moved.
        Structs marked with PhantomPinned are !Unpin.
        Pin tells the compiler that if the wrapped value T is !Unpin.
        - Requiring unsafe methods (get_unchecked_mut) to access the content
        - Not implementing DerefMut (prevents safe mutable access that could move the value)

    PhantomPinned:
        By adding a marker type field PhantomPinned to a struct,
        the struct is marked as !Unpin.


    WHAT PIN IS
    ------------------------------------------------
        Pin is a wrapper with constraints:
        Offers access to the content of the wrapped value T only if T is Unpin.
        If T is !unpin, the only way to access the content is using unsafe methods.

            pub struct Pin<P> {
                pointer: P,  // The wrapped pointer (Box, &mut T) 
                             // idea, in reality it doesnt occupy space
            }
            
            impl Deref for Pin<P> where P: Unpin {
                type Target = P::Target;
            }

            impl DerefMut for Pin<P> where P: Unpin {
                fn deref_mut(&mut self) ...
            }

            // Not declared Deref/DerefMut for !Unpin
            // so hide access to the content of !unpin

            impl Pin<P> {
                
                // get pinned references to the content
                pub fn as_ref(&self) -> Pin<&P::Target> { ... }
                pub fn as_mut(&mut self) -> Pin<&mut P::Target> { ... }
                
                // safe methods: only for Unpin types
                pub fn get_ref(&self) -> &P::Target { ... }
                pub fn get_mut(&mut self) -> &mut P::Target { ... }
                
                // unsafe methods: for Unpin and !Unpin types
                pub unsafe fn get_unchecked_ref(&self) -> &P::Target { ... }
                pub unsafe fn get_unchecked_mut(&mut self) -> &mut P::Target { ... }
            }
        }

    MARK STRUCT AS !UNPIN
    ------------------------------------------------
        By adding a marker type field PhantomPinned to a struct,
        the struct is marked as !Unpin.

        struct NoMovable {
            value: i32,
            ptr: *const i32,
            _pin: std::marker::PhantomPinned, // marks as !Unpin
        }


*/
mod unpin_and_not_unpin {
    /*
    Unpin usage:
     */
    #[test]
    pub fn unpin_usage() {
        #[derive(Debug, Clone)]
        pub struct NoMovable {
            pub value: i32,
            pub ptr: *const i32,
        }

        let value_to_pin = NoMovable {
            value: 10,
            ptr: std::ptr::null(),
        };
        let mut pinned = Box::pin(value_to_pin);
        // value_to_pin is moved to the heap and pinned, so you can't use it anymore.


        // Equivalent ways in Unpin:
        
        // 1. DerefMut
        pinned.ptr = &pinned.value as *const i32; // works in unpin, 

        // 2. explicitly access with pin methods:
        // better because it checks Unpin at compile time, specific error for !Unpin
        {
            // fails in !unpin: uses deref but since it's not implemented in 
            // !unpin gives deref error
            let var = pinned.as_mut().get_mut();
            var.ptr = &var.value as *const i32;
        }

        // 3. same but compact form:
        pinned.as_mut().get_mut().ptr = &pinned.value as *const i32; // compact form

        assert_eq!(&pinned.value as *const i32, pinned.ptr);
    }


    /*
    !Unpin usage:
     */
    #[test]
    pub fn not_unpin_usage() {
        #[derive(Debug, Clone)]
        pub struct NoMovable {
            pub value: i32,
            pub ptr: *const i32,
            _pin: std::marker::PhantomPinned,
        }

        let mut var2 = Box::pin(NoMovable {
            value: 10,
            ptr: std::ptr::null(),
            _pin: std::marker::PhantomPinned,
        });
        // require unsafe
        unsafe {
            // !Unpin doesn't implement DerefMut
            let var = var2.as_mut().get_unchecked_mut();
            
            var.ptr = &var.value as *const i32;
        }

        assert_eq!(&var2.value as *const i32, var2.ptr);
    }
}

/*
========================================================================
PIN ON STACK AND HEAP
========================================================================

    PIN ON STACK
    ---------------------------------------------

        pin!(T) -> Pin<&mut T>

            // same as:
            let mut __tmp = var1; // you can't use var1 anymore, it moved to __tmp
            let var2 = unsafe { Pin::new_unchecked(&mut __tmp) };  

        Syntactic sugar to pin on stack and obtain a Pin<&mut T> without writing unsafe by hand
        and "hides" the temporary variable that moves the value so it doesn't lose ownership.

    PIN ON HEAP
    ---------------------------------------------

        Box::pin(T) : Pin<Box<T>>

        - Moves the value to the heap Box<T>, pins it, returns Pin<Box<T>>.

        Pin<Box<>> is a single memory address, same as Box, 
        because Pin for Box is a wrapper that doesn't change
        the memory address of Box. That is, Pin would take 0 bytes.
*/    
#[cfg(test)]
mod pin_stack_and_heap {
    use std::pin::pin;

    #[test]
    pub fn pin_stack_not_unpin_usage() {
        #[derive(Debug, Clone)]
        pub struct NoMovable {
            pub value: i32,
            pub ptr: *const i32,
            _pin: std::marker::PhantomPinned,
        }

        let var1 = NoMovable {
            value: 10,
            ptr: std::ptr::null(),
            _pin: std::marker::PhantomPinned,
        };
        let mut var2 = pin!(var1);

        // this works for both Unpin and !Unpin:
        unsafe {
            let var = var2.as_mut().get_unchecked_mut();
            var.ptr = &var.value as *const i32;
        }
        // var2.ptr = &var2.value as *const i32; // impossible in !unpin because there's no deref for unpin

        // the only way to access !pin content is using get_unchecked_mut or get_unchecked_ref inside an unsafe block, because deref is not implemented for !Unpin, therefore content cannot be modified directly.
        // replace(
        //     &mut *var2,  // error: no deref implementation in !Unpin
        //     NoMovable {
        //         value: var2.value,
        //         ptr: &var2.value as *const i32,
        //         _pin: std::marker::PhantomPinned,
        //     },
        // );

        assert_eq!(&var2.value as *const i32, var2.ptr);
    }


}



/*
PIN PROJECT
----------------------------------------------------
    pin-project is a crate that facilitates the creation of structs with pinned fields.
    It allows defining which fields are pinned and which are not, automatically generating
    the necessary code to project the fields correctly.


    Benefits:
    - You don't need to write unsafe code to access the mut pinned struct.
    you can use the unpin fields without unsafe, and the !unpin fields with unsafe.
    reducing the use of unsafe code.

    - #[pin_project]: Macro to mark the struct.
    - #[pin]: Attribute to mark specific fields as pinned.
    - .project(): Generated method to get projected references to the fields.
*/
#[cfg(test)]
mod pin_project {
    #[test]
    pub fn some_fields_pinned_strong() {
        use pin_project::pin_project;
        use std::pin::Pin;

        #[derive(Debug)]
        #[pin_project]
        struct MyStruct {
            #[pin] // This field will be projected as Pin<&mut T>
            field1: i32,
            #[pin] // This field will be projected as Pin<&T>
            prt_field1: *const i32,

            // Without #[pin]: direct access (&mut T)
            field2: i32,
        }

        impl MyStruct {
            fn modify(self: Pin<&mut Self>) {
                // macro generates this method
                let this = self.project(); 

                // *this.field1 = 23; /// Error: no DerefMut
                unsafe {
                    *this.field1.get_unchecked_mut() = 100; // ok
                }
                *this.field2 += 1; // ok, no unsafe
            }
        }

        let mut instance = MyStruct {
            field1: 10,
            prt_field1: std::ptr::null(),
            field2: 20,
        };
        instance.prt_field1 = &instance.field1 as *const i32;
        let mut pinned = Pin::new(&mut instance);
        pinned.as_mut().modify();
        println!("field2: {:?}", &pinned);
    }
}
