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
    static_vs_scope::thread_spawn_static_lifetime_capture();
    static_vs_scope::thread_spawn_non_static_lifetime_capture_error();
    static_vs_scope::thread_scope_capture();

    send::info();
    sync::info();
    semantic::info();
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

/*
========================================================================
'STATIC vs 'SCOPE with Send:
========================================================================

'static
-------------------------------------------
    thread::spawn() and task::spawn()
    spawn the closure, which may outlive the current scope.
    They require the closure to be 'static, and therefore its captures to be 'static.

        pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,  // F: 'static
            T: Send,

'scope
-------------------------------------------
    thread::scope/spawn() and task::scope/spawn()
    The scope ends only after all child threads have finished.
    spawn captures may be 'local because the scope will not end before they do.

        pub fn scope<F, R>(f: F) -> R
        where
            F: FnOnce(&Scope<'_>) -> R,
            R: Send,

        pub fn spawn<F, T>(&self, f: F) -> ScopedJoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'scope, // F: Send + 'scope
            T: Send + 'scope,
*/
#[cfg(test)]
mod static_vs_scope {
    use super::*;

    #[test]
    pub fn thread_spawn_static_lifetime_capture() {
        static VALUE: i32 = 10;
        let ref_value: &'static i32 = &VALUE;
        thread::spawn(move || {
            println!("value: {}", ref_value); // OK: because &VALUE is 'static
        });
        thread::spawn(|| {
            println!("value: {}", VALUE); // OK: captures &VALUE because it is 'static
        });
        assert_eq!(VALUE, 10); // OK: because VALUE is 'static
    }

    #[test]
    pub fn thread_spawn_non_static_lifetime_capture_error() {
        let _value: i32 = 10;
        // let ref_value: &i32 = &_value; // ERROR: _value doesnt live long enough
        // thread::spawn(move || {
        //     thread::sleep(core::time::Duration::from_secs(1));
        //     println!("_value: {}", ref__value); // ERROR: _value is not 'static
        //     // therefore captures must be 'static
        // });
        // drop(_value);
    }

    #[test]
    pub fn thread_scope_capture() {
        let value: i32 = 10;
        thread::scope(|s| {
            s.spawn(|| {
                println!("value: {}", value); // OK: value is 'local
            });
        }); // value will live after any spawn threads finish.
    }
}

/*
========================================================================
SEND
========================================================================

Concept:
-------------------------------------------
    T can transfer ownership between threads safely, i.e. without causing
    race conditions or memory corruption.

    Non-Send case:
        Rc<T> is not Send: calling rc::clone() from different threads can corrupt
        the reference count because it is not atomic; synchronization is not guaranteed.

Notes:
-------------------------------------------
    Every type T has Send implemented either manually or automatically.
    Internal types (refs, pointers, ...) are marked internally by the compiler as Send or !Send.

    Send and lifetimes:
    Send does not verify lifetimes; ensuring the reference lives long enough is the closure's responsibility.

Marker trait Send:
-------------------------------------------

    pub unsafe auto trait Send {}

    Send resolution order:
    -------------------------------------------
    If there is a manual impl, it takes precedence over auto Send.

    Is T: Send?
        |
        |-- manual Send? --> (e.g. Arc<T> with unsafe impl and bounds)
        |
        |-- auto Send?   --> (e.g. struct where all fields are Send, primitives)

    Auto Send:
    -------------------------------------------
    - auto trait:
        The compiler implements it for struct|enum only if all fields are Send.

    Manual Send:
    -------------------------------------------
    Allows defining manually when a type is Send, overriding auto Send, e.g. when the
    pointer is managed correctly to be safe across threads.

    - unsafe trait:
        If you implement it manually, you must use unsafe impl; you tell the compiler
        it is Send and under what conditions.
    - trait bounds:
        Some types use trait bounds to determine when the trait holds.

    // without trait bounds:
        unsafe impl Send for T {}

    // with trait bounds:
        unsafe impl<T: Send> Send for Mutex<T> {}
        unsafe impl<T: Send + Sync> Send for Arc<T> {}

        Mutex<T> requires T: Send, because the thread accesses &mut T (exclusive access),
        and using T from another thread requires T: Send for safety.

        Arc<T> requires T: Send + Sync,
        - T: Send -> because the last Arc<T> runs drop(&mut self) on T and needs ownership to do so.
        - T: Sync -> because it allows sharing multiple read-only references (&T) across threads safely.

Case-by-case analysis:
-------------------------------------------

    Pointers: !Send
        No guarantee they point to valid data; they may be invalid or already freed.
        - *const T
        - *mut T

    Local references: depends on scope/spawn usage.
        Send if the reference is guaranteed to live long enough on the receiving thread,
        e.g. using thread::scope/spawn().
        - &'a mut T
        - &'a T

    Static references:
        The reference lives for the entire thread (or program for 'static).
        - &'static T <--> T: Sync
        - &mut 'static T <--> T: Send

    Constants: Send
        Limited to primitive types; stored inline in the code.
        - const T

    Primitives and compound primitives: Send
        The whole value is copied; no references or pointers involved.
        - i32, f64, bool, char, tuples, arrays [T; N], structs, enums, etc.

    String: Send

    Closures|Futures (FnOnce|FnMut|Fn): Send if captures are Send.
        A closure|Future is a struct, so the same auto rules for Sync and Send apply.
        Arguments (function parameters) do NOT affect Send.

    Known smart pointers<T>:
        Move ownership to another thread.
        They implement Send (often manually); e.g. Vec has an internal pointer so it is not auto Send, but is manually Send.
        - Box<T>, Mutex<T>, RwLock<T>, Vec<T>, HashMap<K: Send, V: Send> <--> T: Send
        - Arc<T> <--> T: Send + Sync

    Struct|Enum: Send if all fields are Send
        The auto Send trait determines this.
*/
#[cfg(test)]
mod send {
    #[test]
    pub fn info() {}
}
/*
========================================================================
SYNC
========================================================================

Concept:
-------------------------------------------
    T: Sync <-> &T: Send
    T is Sync if multiple threads can access &T safely, without data races.
    Here &T may be a plain reference or, for smart pointers, the result of Deref.

    Non-Sync case:
    -------------------------------------------
        &Cell<T>: !Sync
        - Allows concurrent mutation without synchronization of an internal resource.

        Arc<T> where T: !Sync: !Sync
        - Sharing data that is not safe for concurrent access across threads is not Sync.

Marker trait Sync:
-------------------------------------------
    pub unsafe auto trait Sync {}

    Sync resolution order:
    -------------------------------------------
    Same as Send resolution order, but for Sync. See Send resolution order.

    Auto Sync:
    -------------------------------------------
    Same as auto Send, but for Sync. See Auto Send.

    Manual Sync:
    -------------------------------------------
    Same as manual Send, but for Sync. See Manual Send.

Implementations
-------------------------------------------

    T: Sync <-> &T: Send
    -------------------------------------------
    unsafe impl<T: Sync> Send for &T {}   // (std lib blanket impl)

    // T: Sync -> &T: Send
    // &T: Send -> T: Sync (by definition)
    // T: Sync <-> &T: Send (conclusion)
    // where &T also stands for smart-pointer Deref.


    T: !Sync -> &T: !Sync
    -------------------------------------------
    unsafe impl<T> Sync for &T {}   // (std lib blanket impl)

    // Technically references as a type are always Sync because they are immutable
    // and can be shared safely between threads — so &&T: Send always.
    // But it is useless if T is not Sync because you cannot safely access its value.
    // Therefore if T: !Sync -> &T: !Sync

    Others
    -------------------------------------------
    // Vec
    unsafe impl<T: Sync> Sync for Vec<T> {}

    // Arc
    unsafe impl<T: Send + Sync> Sync for Arc<T> {}

Case analysis:
-------------------------------------------

    Pointers: !Sync
        Sharing &(*const T) or &(*mut T) across threads would allow concurrent
        access without synchronization to the pointed-to data.
        - *const T
        - *mut T

    Shared references &T
        If T: Sync -> &T: Sync
        Sharing the reference itself is safe; what matters for threads is whether
        the data T allows concurrent access (T: Sync).
        - &'a T
        - &'static T

    Mutable references &mut T: !Sync
        Exclusive access; &mut T cannot be shared across threads.
        - &'a mut T: not Sync.

    Constants: Sync
        Immutable embedded data, no interior mutability.
        - const T: is Sync (allowed primitive types).

    Copy types: Sync
        Primitive and compound Copy types without interior mutability.
        - i32, f64, bool, char, Copy tuples, Copy arrays, Copy structs, enums, etc.

    String: Sync

    Closures|Futures (Fn): Sync if captures are Sync.
        A closure|Future is a struct, so the same auto rules for Sync and Send apply.
        Arguments (function parameters) do NOT affect Sync.

    Cell/RefCell: !Sync
        Multiple semantic &T (Cell/RefCell) would allow concurrent mutation
        without synchronization → data race.
        - Cell<T>: not Sync.
        - RefCell<T>: not Sync.

    Known wrappers<T>:
        Sharing &T across threads requires internal synchronization or immutable T.
        - Arc<T>: Sync <--> T: Send + Sync
            (atomic refcount + data safe for shared access).
        - Mutex<T>, RwLock<T>: Sync <--> T: Send
            (the lock serializes access; T must be Send to move the guard across threads).
        - Rc<T>: not Sync (non-atomic refcount, single-thread only).
        - Box<T>: Sync <--> T: Sync.

    Static variables:
        - static T (immutable): Sync (safe concurrent reads).
        - static mut T: not Sync (concurrent access without synchronization → data race).

    Struct|Enum: Sync if all fields are Sync
        The auto Sync trait determines this.

*/
#[cfg(test)]
mod sync {
    #[test]
    pub fn info() {}
}

/*
========================================================================
&T SEMANTIC:
========================================================================

&T in syntax:
-------------------------------------------
    &T is a reference to the concrete type T.

&T in semantics:
-------------------------------------------
    &T: Entity that allows shared access to the same value,
    without exclusive access (mut). multiple aliasing.

    Representations:
    - &T (classic reference)
    - Smart-pointer Deref (Deref coercion)

    Characteristics:
    -------------------------------------------
    - many aliasing.
    - immutable access.
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
mod semantic {
    #[test]
    pub fn info() {}
}
