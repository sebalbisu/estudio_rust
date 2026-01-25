#[allow(unused_imports)]
use std::cell::{Cell, RefCell};
#[allow(unused_imports)]
use std::rc::Rc;
#[allow(unused_imports)]
use std::sync::{Arc, Mutex, RwLock};
#[allow(unused_imports)]
use std::thread;

// Index: match section name with module name, and test function with section topics
#[test]
fn index() {
    unsafe_concurrent_access_patterns::info();
    
    _t_semantic::info();

    send::not_send::reference_not_send();
    send::not_send::raw_pointer_not_send();
    send::not_send::rc_not_send();

    send::moving_variables_without_move_keyword::assigning_to_variable();
    send::moving_variables_without_move_keyword::dropping_variable();
    send::moving_variables_without_move_keyword::method_that_consumes_variable();
    
    send::copy_values_dont_required_move();
    send::move_keyword();

    sync::static_inmutable_is_sync();
    sync::static_mutable_is_sync();

    synchronization_primitives::arc();
    synchronization_primitives::mutex();
    synchronization_primitives::rwlock();
}

/*
========================================================================
Unsafe Concurrent Access Patterns
========================================================================

    * Basic concepts about concurrency and race conditions.

    Race Condition 
    -------------------------------------------

        (Single or Multi-thread) Logical error due to execution order or use of stale data.
        The program is "safe" (does not crash), but the result is incorrect.

        Visualization of Race Condition (Single Thread - Stale Data):

          Variable (X)          Local Copy (stale)            Operation
             [ 5 ]
               |
               |-------------------> [ 5 ]                  (1. Save copy)
               |
             [ 10 ] <-----------------------------------    (2. X changes: e.g. event)
               |
               |                     [ 5 ] ---------------> (3. Use old copy)
               |                                               5 + 1 = 6
             [ 6 ] <--------------------------------------- (4. Overwrite X)
               |
            ERROR!
        (X should be 11, but we went back in time because we used old data)


    Data Race:
    -------------------------------------------

        (Multi-thread) Concurrent memory access without synchronization. 
        UB (Undefined Behavior).

        The read or write does not necessarily have to be simultaneous, 
        it is enough that there is no synchronization.
        That is, by not knowing the actions of other threads, 
        a race condition can occur.


       Thread A                Memory (Counter)               Thread B
                                     [ 5 ]
                                       |
    (t1) Read (5) <--------------------|
          |                            |
    (t2) Increment (5+1)               |
          |                            |----------------> Read (5) (t3)
    (t4) Write (6) -----------------> |                     |
                                     [ 6 ]             (t5) Increment (5+1)
                                       |                     |
                                       |<--------------- Write (6) (t6)
                                     [ 6 ]
                                       |
                                    ERROR!
                 (Increment was lost, should be 7)

    Use-After-Free
    -------------------------------------------

        A pointer that points to a memory address that has been freed or deallocated.
        It is a pointer that is no longer valid to use.

    Deadlocks
    -------------------------------------------

        Two or more threads are waiting for each other to release a resource.
*/
#[cfg(test)]
pub mod unsafe_concurrent_access_patterns {

    #[test]
    pub fn info() {}
}
/*
========================================================================
&T SEMANTIC:
========================================================================

&T in syntaxis:
-------------------------------------------
    &T is a reference to the concrete type T.

&T in semantics:
-------------------------------------------   
    &T: Entity that allows shared access to the same value, 
    without exclusive access (mut). multiple aliasing.

    Representations:
    - &T (referencia clásica)
    - Arc<T>
    - Rc<T>
    - *const T
    - any wrapper that can produce multiple observers

    Characteristics:
    -------------------------------------------
    - many aliasing. 
    - inmutable access. 
      (allows interior mutability, with unsafe code that the compiler trusts). 

&mut T
-------------------------------------------
    &mut T, is the only representation that allows exclusive access to the value.
        single exclusive aliasing.

    Characteristics:
    -------------------------------------------
    - only one access (exclusive)
    - mutable access.


    Exclusive:
    -------------------------------------------
        &mut T invalidates all the other &T. 
        &mut T is the only representation that allows exclusive access to the value.
*/
#[cfg(test)]
pub mod _t_semantic {

    #[test]
    pub fn info() {}
}

/*
========================================================================
SEND AND SYNC MARKERS
========================================================================

Concept:
---------------
    Send: send the ownership of the variable (T) to another thread safely.
    Sync: send the &T (semantic representation) to multiple threads 
    for read or write safely.

    A type T is marked with Send/Sync marker trait.

Send:
-------------------------------------------
    Safe to TRANSFER OWNERSHIP to another thread.
    
    Guarantees:
    - Lifetime safety: T will live long enough in the receiving thread
    - Memory safety: No dangling pointers, use-after-free
    - Thread safety: Moving T won't enable unsafe concurrent access patterns
    
    Only one thread owns T at any given time.
    Once transferred, the previous owner cannot access it.

    Not Send:
        * local references &'a T: the thread can live longer than the original owner.
        * raw pointers: they don't guarantee validity.
        * Rc: it uses a non-atomic counter, moving the original rc allows to 
        create &T not safe thread in different threads. So not thread-safe pattern.

Sync:
-------------------------------------------
    Safe for MULTIPLE threads to ACCESS data concurrently.
    
    T is Sync ⟺ &T is Send

    Sync can share multiple &T (semantic representation) vars
    to multiple threads safely. Because each &T is Send.

    &T (semantic representation) are:
    - Owned values that Deref to &T (Arc<T>)
    - Direct &T references (&'static T, static data)

    The &T reference is immutable, but T may provide:
        - Read access (immutable types)
        - Write access (interior mutability: Mutex, RwLock)

    Not Sync:
        * Cell: not atomic interior mutation
        * RefCell: not atomic interior mutation with &T, not Sync
        * static mut where T:'static + !Sync (not Sync)

    'static T:
        - 'static inmutable is Sync
        - 'static mutable is Sync if T:'static + Sync

Compound Send|Sync:
-------------------------------------------
    T: Struct | Enum
        T is Send|Sync if all its fields are Send|Sync.

    T: Callbacks | Futures
        T is Send|Sync if all its captured variables are Send|Sync.
        Arguments (function parameters) do NOT affect Send|Sync.
            (Arguments are passed at call time, not stored in the type.)

*/
#[cfg(test)]
pub mod send {
    use super::*;

    /*
    Not Send examples
     */
    pub mod not_send {
        use super::*;
        // reference not living long enough in the receiving thread, not Send
        #[test]
        pub fn reference_not_send() {
            let data = 10;
            let _ref_data = &data;
            // let handle = thread::spawn(move || {
            //     println!("_ref_data: {}", _ref_data);
            // });
        }
    
    
        // raw pointer not living long enough in the receiving thread, 
        // and not guaranteed to be valid, not Send
        #[test]
        pub fn raw_pointer_not_send() {
            let data = 10;
            let _ptr_data = &data as *const i32;
            // let handle = thread::spawn(move || {
            //     println!("_ptr_data: {:?}", _ptr_data);
            // });
        }
    
        // Rc not Send, because it uses a non-atomic counter, 
        // moving the original Rc allows to create more Rc::clones not Sync.
        #[test]
        pub fn rc_not_send() {
            let _rc = Rc::new(10);
            // let handle = thread::spawn(move || {
            //     println!("rc_data: {}", _rc);
            // });
        }
    }

    /*
    Moving variables without using the move keyword
     */
    pub mod moving_variables_without_move_keyword {
        use super::*;

        #[test]
        pub fn assigning_to_variable() {
            let data = String::from("hello");
            thread::spawn(|| {
                let _data_moved = data;
            }).join().unwrap();
            // assert_eq!(data, "hello");  // ✗ ERROR: data was moved
        }

        #[test]
        pub fn dropping_variable() {
            let data = String::from("hello");
            thread::spawn(|| {
                drop(data);
            }).join().unwrap();
            // assert_eq!(data, "hello");  // ✗ ERROR: data was moved
        }

        #[test]
        pub fn method_that_consumes_variable() {
            let data = String::from("hello");
            thread::spawn(|| {
                let _ = data.into_bytes();
            }).join().unwrap();
            // assert_eq!(data, "hello");  // ✗ ERROR: data was moved
        }
    }

    /*
    Move keyword, move all captured variables
     */
    #[test]
    pub fn move_keyword() {
        let data = String::from("hello");
        let data2 = String::from("world");
        thread::spawn(move || {
            assert_eq!(data, "hello");
            assert_eq!(data2, "world");
        }).join().unwrap();
        // assert_eq!(data, "hello");  // ✗ ERROR: data was moved
        // assert_eq!(data2, "world");  // ✗ ERROR: data2 was moved
    }

    /*
    Copy values dont required move
     */
    #[test]
    pub fn copy_values_dont_required_move() {
        let data = 10;
        thread::spawn(move || {
            assert_eq!(data, 10);
        }).join().unwrap();
        assert_eq!(data, 10);  // works because data is Copy
    }


}

#[cfg(test)]
pub mod sync {
    use super::*;

    /*
    static inmutable is sync:
        static accepts Copy and 'static references,and wont change the value
        so it is safe to share between threads
        */
    #[test]
    pub fn static_inmutable_is_sync() {
        static DATA: i32 = 0;
        thread::spawn(|| {
            assert_eq!(DATA, 0);
        }).join().unwrap();
        assert_eq!(DATA, 0);
    }

    /*
    static mutable is Sync if T:'static + Sync
        condition:requires unsafe to write to it
    */
    #[test]
    pub fn static_mutable_is_sync() {
        static mut DATA: i32 = 0;
        thread::spawn(|| {
            unsafe {  // requires unsafe to write to it
                DATA = 10;
            }
        }).join().unwrap();
    }

    /*
    Cell is not Sync
    because it allows interior non-atomic mutation with &T, not Sync
    */
    #[test]
    pub fn not_sync() {
        let _cell = Cell::new(42);
        thread::spawn(|| {
            // _cell.set(43);
        }).join().unwrap();
    }


}


#[cfg(test)]
pub mod compound_send_sync {
    use super::*;

    /*
    struct not send:
        * raw pointer: does not guarantee validity or synchronization
    */
    #[test]
    pub fn struct_not_send() {
        struct NotSend {
            _ptr: *const i32,
        }
        let _not_send = NotSend {  _ptr: std::ptr::null() as *const i32 };
        thread::spawn(|| {
            // ✗ ERROR: _not_send is not Send
            // assert_ne!(_not_send._ptr, std::ptr::null());  
        }).join().unwrap();
    }

    /*
    callback not send:
        * captures raw pointer: does not guarantee validity or synchronization
    */
    pub fn _callback_not_send() {

        let ptr = std::ptr::null() as *const i32;
        let _callback = || {
            assert_ne!(ptr, std::ptr::null());
            // ✗ ERROR: captured ptr is not Send
        };
        thread::spawn(|| {
            // _callback();
        }).join().unwrap();
    }

    /*
    callback send, and params not send, works!:
        * parameters are passed at call time, not stored in the type.
     */
    #[test]
    pub fn callback_send_params_not_send() {
        let callback = |ptr: *const i32| {
            assert_eq!(ptr, std::ptr::null());
        };
        thread::spawn(move || {
            callback(std::ptr::null() as *const i32);
        }).join().unwrap();
    }
}




/*
========================================================================
SYNCHRONIZATION PRIMITIVES
========================================================================

    Arc<T>: Atomic Reference Counting
    -------------------------------------------
        * Atomic Reference Counting: Multiple thread-safe owners of shared data
        * Thread-Safe Sharing: Allows the data to live as long as any owner exists
        * Similar to Rc<T>, but uses atomic operations for thread safety
        * Provides only immutable access (&T) to the data
        * Arc<T> is Send if T is Send
        * Arc<T> is Sync if T is Sync

    Mutex<T>: Mutual Exclusion
    -------------------------------------------
        * Allow multiple threads to access the same data safely, guaranteeing that 
        only one can write or read at a time.
        * T must be Send for Mutex<T> to be Sync.

    RwLock<T>:
    -------------------------------------------
        * Multiple readers or one writer.
*/
#[cfg(test)]
mod synchronization_primitives {
    use super::*;

    /*
    Arc<T>: Atomic Reference Counting
     */
    #[test]
    pub fn arc() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            assert!(data_clone.len() == 5);
        });
        assert!(data.len() == 5);
        handle.join().unwrap();
    }

    /*
    Mutex<T>: Mutual Exclusion
     */
    #[test]
    pub fn mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 0..3 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                println!("  Thread {}: incremented to {}", i, *num);
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    pub fn rwlock() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let mut handles = vec![];

        // Multiple readers
        for i in 0..2 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let read_guard = data_clone.read().unwrap();
                println!("  Reader {}: {:?}", i, *read_guard);
            });
            handles.push(handle);
        }

        // One writer
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_guard = data_clone.write().unwrap();
            write_guard.push(4);
            println!("  Writer: added 4");
        });
        handles.push(handle);

        for h in handles {
            h.join().unwrap();
        }
    }
}
