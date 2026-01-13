use std::hint::black_box;
use std::time::Instant;

/*
 ============================================================================
 ÍNDICE - Ejecuta todas las demos
 ============================================================================
*/
#[test]
fn indice() {
    type_copy::duplica_y_original_valido();
    type_copy::tipos_copy();
    type_move::owned_move();
    referencias::referencias();
    punteros_raw::raw_pointers();
    auto_ref::auto_ref();
    deref::deref_trait();
    deref::deref_coercion();
    deref::deref_coercion_profundidad();
    deref::derefmut_coercion();
    derefenciar::ref_copy();
    derefenciar::deref_copy();
    derefenciar::deref_move();
    derefenciar::deref_multiple_manual();
    derefenciar::ref_mut_asignacion();
    derefenciar::derefmut_asignacion();
    auto_deref::ref_metodos();
    auto_deref::ref_campos();
    auto_deref::tderef_metodos();
    auto_deref::tderef_campos();
    auto_deref::tderef_move();
    auto_deref::indexacion();
    smart_pointers::smart_pointer_box();
    smart_pointers::rc();
    smart_pointers::refcell();
    referencias_punteros::coercion_referencia_a_puntero_raw();
    referencias_punteros::puntero_a_puntero();
    referencias_punteros::referencia_desde_puntero_raw();
    benchmarks::benchmark_strings();
    benchmarks::benchmark_structs();
    benchmarks::benchmark_vectors();
}

/*
============================================================================
1. OWNED T - Propiedad del valor
============================================================================

TIPO COPY:
-----------------------------------------------------------

    Esquema:
        let a: i32 = 42;
        let b = a;

        STACK:
        ┌───────┐  ┌───────┐
        │ a: 42 │  │ b: 42 │  ← Dos valores independientes
        └───────┘  └───────┘

    Caracteristicas:
        * valor en stack
        * Al asignar/pasar: se duplica el valor (memcpy bit a bit)
        * El original permanece válido
        * Barato: solo copia bytes en stack
        * No necesita limpieza especial (no Drop)

    Tipos Copy:
        * Refencias: &T y &mut T
        * Punteros raw: *const T y *mut T
        * Tipos primitivos: u32, i32, f64, bool, char
        * Tuplas de tipos Copy: ((), (i32, bool), etc.)
        * Arrays de tipos Copy: ([i32; 3], etc.)
        * Structs/Enums de tipos Copy

    Trait:
    trait Copy : Clone {}
        Es un marker trait (sin métodos)
        Se implementa en structs/enums para hacerlos Copy. (mas facil con derive)
        No se puede marcar un tipo como Copy si tiene Drop (o si es Move).

TIPO MOVE: !Copy
-----------------------------------------------------------

    Esquema:
        let s1 = String::from("hola");
        let s2 = s1;

        STACK:                      HEAP:
        ┌──────────────┐           ┌─────────┐
        │ s1 (inválido)│           │ "hola"  │
        │ ptr ──────────────┐      │         │
        └──────────────┘    │      └─────────┘
                            │           ▲
        ┌──────────────┐    │           │
        │ s2           │    │           │
        │ ptr ──────────────┴───────────┘
        │ len: 4       │
        │ cap: 4       │
        └──────────────┘

    Caracteristicas:
        * valor en heap, metadata en stack
        * Al asignar/pasar: se mueve ownership (shallow copy stack)
        * El original queda inválido
        * Barato: solo copia metadatos en stack
        * Requiere limpieza especial (Drop) al finalizar el scope

    Tipos !Copy:
        * Smart pointer = heap + (ptr + Metadata stack) + Deref + Drop + !Copy
            (String, Vec, Box, HashMap, Rc, Arc, etc.)
        * Structs/Enums que contienen smart pointers

    Porque no se copian smart pointers:
        Si se hiciera copy de un puntero inteligente, solo se copiarían
        los metadatos en stack, apuntando ambos al mismo contenido
        en heap, lo cual es peligroso (doble free al liberar ambos).
        Entonces solo se permite mover ownership. Idem en struct con
        smart pointers.

*/

#[cfg(test)]
mod type_copy {
    use std::fmt::Debug;
    use std::ptr::{addr_eq, addr_of};

    fn assert_is_copy<T: Copy + PartialEq + Debug>(x: T, y: T) {
        assert_eq!(x, y);
        assert_ne!(addr_of!(x), addr_of!(y));
    }

    #[test]
    pub fn duplica_y_original_valido() {
        let a: i32 = 42;
        let b = a; // copia el valor
        assert_is_copy(a, b); // a y b validos e independientes
    }

    #[test]
    pub fn tipos_copy() {
        // references
        let value = 10;
        let ref_1 = &value;
        let ref_2 = ref_1; // copia la referencia
        assert_is_copy(ref_1, ref_2);

        // punteros raw
        let raw_ptr_1: *const i32 = &value;
        let raw_ptr_2 = raw_ptr_1; // copia el puntero raw
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
        let arr2 = arr; // copia
        assert_is_copy(arr, arr2);

        // Struct requiere implementar Copy
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

    #[test]
    pub fn owned_move() {
        // TIPO MOVE: metadatos en stack, contenido en heap
        let owned_move: String = String::from("hola");

        // String tiene 24 bytes en stack: ptr + len + cap
        assert_eq!(std::mem::size_of::<String>(), 24);
        assert_eq!(owned_move.len(), 4);
        assert_eq!(owned_move.capacity(), 4);

        let ptr_before = owned_move.as_ptr();
        let moved = owned_move; // move: copia 24 bytes en stack
        let ptr_after = moved.as_ptr();

        // El puntero al heap es el mismo (no se copió el contenido)
        assert_eq!(ptr_before, ptr_after);
        // owned_move ya no es válido aquí
    }
}

/*
 ============================================================================
 2. REFERENCIAS &T - Borrow sin ownership
 ============================================================================

    Diagrama:
        let val: i32 = 42;
        let ref_val: &i32 = &val;

        STACK:
        ┌────────────────────────┐
        │ val: 42                │ ◄─────────┐
        │ @ 0x7fff1234           │           │
        ├────────────────────────┤           │
        │ ref_val: &i32          │           │
        │ @ 0x7fff1238           │           │
        │ valor: 0x7fff1234 ─────────────────┘
        └────────────────────────┘

    &T es solo una dirección de memoria (8 bytes en 64-bit)
    ref_val es otra variable en stack que CONTIENE la dirección de val
    es Copy

    Caso especial: Referencia a valor sin variable
    --------------------------------
    let x = &10;
    Crear una referencia a un valor temporal, que vive en el stack, y muere cuando finaliza el stack:
*/

#[cfg(test)]
mod referencias {
    #[test]
    pub fn referencias() {
        let val: i32 = 42;
        let ref_val: &i32 = &val; // &val = 0x7fff1234 (dirección de val) 

        // &T es 8 bytes (puntero)
        assert_eq!(std::mem::size_of::<&i32>(), 8);

        // ref_val apunta a val
        assert_eq!(*ref_val, 42);
        assert_eq!(ref_val as *const i32, &val as *const i32);

        // Múltiples referencias inmutables permitidas
        let ref2: &i32 = &val;
        let ref3: &i32 = &val;
        assert_eq!(*ref2, *ref3);

        // Referencia mutable
        let mut val_mut: i32 = 10;
        let ref_mut: &mut i32 = &mut val_mut;
        *ref_mut = 20;
        assert_eq!(val_mut, 20);

        // referencia a valor sin variable.
        let x = &10;
        assert_eq!(*x, 10);
    }
}

/*
============================================================================
PUNTEROS RAW *const T / *mut T
============================================================================

    Similar a &T / &mut T, pero sin garantías de seguridad.
    Direccion de memoria que contiene como valor otra direccion de memoria.

     let val: i32 = 100;
     let ptr: *const i32 = &val;

     DIFERENCIAS &T vs *const T:
     ┌─────────────────────┬──────────────────┬────────────────────┐
     │ Característica      │ &T               │ *const T           │
     ├─────────────────────┼──────────────────┼────────────────────┤
     │ Puede ser null      │ ❌ Nunca         │ ✅ Sí              │
     │ Siempre válido      │ ✅ Garantizado   │ ❌ No garantizado  │
     │ Lifetime checking   │ ✅ Compilador    │ ❌ Manual          │
     │ Dereferenciar       │ Safe             │ unsafe             │
     └─────────────────────┴──────────────────┴────────────────────┘
*/
#[cfg(test)]
mod punteros_raw {
    #[test]
    pub fn raw_pointers() {
        let val: i32 = 100;
        let ptr: *const i32 = &val;

        // El puntero raw contiene la dirección
        assert!(!ptr.is_null());

        // Dereferenciar requiere unsafe
        unsafe {
            assert_eq!(*ptr, 100);
        }

        // *mut T para mutabilidad
        let mut val_mut: i32 = 50;
        let ptr_mut: *mut i32 = &mut val_mut;

        unsafe {
            *ptr_mut = 75;
        }
        assert_eq!(val_mut, 75);

        // Null pointer (no posible con &T)
        let null_ptr: *const i32 = std::ptr::null();
        assert!(null_ptr.is_null());
    }
}

/*
============================================================================
AUTO-REF en Metodos
============================================================================

    Rust añade & / &mut automáticamente en llamadas a métodos,
    si el metodo lo requiere.

    Idea:
        struct Data { value: i32 }
        impl Data {
            fn by_ref(&self) -> i32 { self.value }
            fn by_ref_mut(&mut self, new_val: i32) { self.value = new_val; }
        }

        let mut d = Data { value: 42 };

        d.by_ref()        → Rust convierte a: (&d).by_ref()
        d.by_ref_mut(15)  → Rust convierte a: (&mut d).by_ref_mut(15)

    AUTO-REF solo funciona con el operador . (llamadas a métodos)
    NO funciona con funciones libres: fn foo(x: &T) requiere foo(&val)
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

    #[test]
    pub fn auto_ref() {
        let mut d = Data { value: 42 };

        // Auto-ref en métodos: d.method() → (&d).method()
        assert_eq!(d.by_ref(), 42); // Rust añade & automáticamente

        // Auto-ref con &mut
        d.by_ref_mut(100); // Rust añade &mut automáticamente
        assert_eq!(d.value, 100);

        // En funciones libres NO hay auto-ref
        let result = free_function(&d); // DEBEMOS poner &
        assert_eq!(result, 100);
    }
}

/*
============================================================================
Deref y DerefMut
============================================================================

    Obtiene una referencia a donde apunta el smart pointer
    una referencia de &Target, atraves del metodo deref()

    trait Deref {
        type Target: ?Sized;
        fn deref(&self) -> &Self::Target;
    }

    Coercion Deref
    ------------------------
    Cada vez que se necesita una &Target y se tiene una referencia &T donde T:Deref,
    Rust aplica automaticamente T.deref() para obtener la referencia requerida.

        let a: Box<i32> = Box::new(10);
        let ref1: &i32 = &a;  // implicito
        let ref2: &i32 = a.deref(); // explicito

    Coercion Deref en profundidad
    ------------------------
    Se aplica recursivamente .deref() hasta llegar a &Target requerido

        let b: Box<Box<i32>> = Box::new(Box::new(20));
        let ref1: &i32 = &b;   // implicito
        let ref2: &i32 = b.deref().deref(); // explicito

    DerefMut
    ------------------------
    Similar a Deref, pero para referencias mutables &mut T
    * Coercion DerefMut: obtiene ref mutables
    * Coercion DerefMut en profundidad
*/
#[cfg(test)]
mod deref {
    use std::ops::Deref;

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
        let ref1: &i32 = &my_ptr; // implicito
        let ref2: &i32 = my_ptr.deref(); // explicito
        assert_eq!(ref1, &42);
        assert_eq!(ref2, &42);
    }

    #[test]
    pub fn deref_coercion() {
        let a: Box<i32> = Box::new(10);
        let ref1: &i32 = &a; // implicito
        let ref2: &i32 = a.deref(); // explicito
        assert_eq!(ref1, &10);
        assert_eq!(ref2, &10);
    }

    #[test]
    pub fn deref_coercion_profundidad() {
        let b: Box<Box<i32>> = Box::new(Box::new(20));
        let ref1: &i32 = &b; // implicito
        let ref2: &i32 = b.deref().deref(); // explicito
        assert_eq!(ref1, &20);
        assert_eq!(ref2, &20);
    }

    #[test]
    pub fn derefmut_coercion() {
        let mut c: Box<i32> = Box::new(30);
        let ref_mut1: &mut i32 = &mut c; // implicito
        // let ref_mut1: &mut i32 = c.deref_mut(); // explicito
        *ref_mut1 += 5;
        assert_eq!(*c, 35);
    }
}

/*
============================================================================
Dereferenciar *
============================================================================

    Obtener el valor de una referencia
    Si T es T:Deref / DerefMut, *T = Target, valor a donde apunta T con deref()
    (toma la referencia con deref(), para despues obtener el valor).

    Referencias: *ref -> contenido
    T:Deref:     *T -> *(T.deref()) -> *&Target -> Target
                **T -> *(T.deref().deref()) -> *&Target2 -> Target2

    No AutoDeref en profundidad:
    ----------------------------
    No busca en profundidad de deref() solo usa el primero, para multiple profundidad usar multiples **T.

    Asignacion en contenido:
    ----------------------------
    *ref = valor;
    Si ref es &mut T o T:DerefMut, asigna valor al contenido apuntado.
*/
#[cfg(test)]
mod derefenciar {

    #[test]
    pub fn ref_copy() {
        let a: i32 = 10;
        let ref1: &i32 = &a;
        let _val1: i32 = *ref1; // a es Copy entonces copia
        assert_eq!(a, 10);
        assert_eq!(_val1, 10);
    }

    #[test]
    pub fn deref_copy() {
        let x: Box<i32> = Box::new(42);
        let y: i32 = *x; // copy
        assert_eq!(y, 42);
        assert_eq!(*x, 42); // x sigue válido
    }

    #[test]
    pub fn deref_move() {
        let x: Box<String> = Box::new(String::from("hola"));
        let s: String = *x; // mueve el String fuera del Box
        assert_eq!(s, "hola");
        // assert_eq!(*x, "hola"); // x ya está consumido y no puede usarse
    }

    #[test]
    pub fn deref_multiple_manual() {
        let x: Box<Box<i32>> = Box::new(Box::new(100));
        let y: i32 = **x; // copia el i32 fuera del Box interno
        assert_eq!(y, 100);
        assert_eq!(**x, 100);
    }

    #[test]
    pub fn ref_mut_asignacion() {
        let mut a: i32 = 20;
        let ref_mut: &mut i32 = &mut a;
        *ref_mut = 30; // asigna al contenido apuntado
        assert_eq!(a, 30);
    }

    #[test]
    pub fn derefmut_asignacion() {
        let mut b: Box<i32> = Box::new(30);
        *b = 40; // asigna al contenido apuntado
        assert_eq!(*b, 40);
    }
}

/*
============================================================================
6. AUTO-DEREF: * en métodos, campos, indexacion
============================================================================

Cuando se usan metodos, campos o indexacion sobre referencias o tipos T:Deref, realiza auto-deref para llegar al tipo que tiene el metodo o campo.

Metodos a través de referencias
--------------------------------
    // abs(self) -> Self

    let n: i32 = -5;
    let m: &i32 = &n;
    let o: &&i32 = &m;
    n.abs()    // directo
    m.abs()    // Rust hace: (*m).abs()
    o.abs()    // Rust hace: (**o).abs()

Campos y referencias
--------------------------------
    let p: &Point = &point;
    p.x      // Rust hace: (*p).x

T:Deref y métodos
--------------------------------
    let b: Box<String> = Box::new(String::from("hola"));
    b.len()    // donde len(self) -> usize
        // method auto-deref: (*b).len()  = String.len()
        // method auto-ref:  (&*b).len() -> &String.len()
    &*b     // &String, no es mueve *b el valor y luego toma la ref, solo toma el valor/tipo final para hacer la operacion


T:Deref y campos
--------------------------------
    let box_point: Box<Point> = Box::new(Point { x: 5, y: 15 });
    box_point.x    // Rust hace: (*box_point).x


Tipo Move al dereferenciar mueve el contenido:
--------------------------------
    let c: Box<String> = Box::new(String::from("mundo"));
    let d = *c; // mueve el String fuera del Box
    // let x = c;   // error: use of moved value: `c`

Indexacion a través de referencias
--------------------------------
    let v: &Vec<i32> = &vec![1,2,3];
    v[0];            // Rust hace: (*v)[0]

*/
#[cfg(test)]
mod auto_deref {
    #[test]
    pub fn ref_metodos() {
        let n: i32 = -5;
        let m: &i32 = &n;
        let o: &&i32 = &m;

        assert_eq!(n.abs(), 5); // directo
        assert_eq!(m.abs(), 5); // Rust hace: (*m).abs()
        assert_eq!(o.abs(), 5); // Rust hace: (**o).abs()
    }

    #[test]
    pub fn ref_campos() {
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 10, y: 20 };
        let p: &Point = &point;
        assert_eq!(p.x, 10); // Rust hace: (*p).x
    }

    #[test]
    pub fn tderef_metodos() {
        let b: Box<String> = Box::new(String::from("hola"));
        assert_eq!(b.len(), 4); // Rust hace: (&*b).len()
    }

    #[test]
    pub fn tderef_campos() {
        struct Point {
            x: i32,
            y: i32,
        }
        let box_point: Box<Point> = Box::new(Point { x: 5, y: 15 });
        assert_eq!(box_point.x, 5); // Rust hace: (*box_point).x
    }

    #[test]
    pub fn tderef_move() {
        let c: Box<String> = Box::new(String::from("mundo"));
        let d = *c; // mueve el String fuera del Box
        assert_eq!(d, "mundo");
        // let x = c;   // error: use of moved value: `c`
    }

    #[test]
    pub fn indexacion() {
        let v: &Vec<i32> = &vec![1, 2, 3];
        assert_eq!(v[0], 1); // Rust hace: (*v)[0]
    }
}

/*
============================================================================
8. SMART POINTERS - Punteros con funcionalidad extra
============================================================================

    RESUMEN SMART POINTERS:
    ┌─────────────────┬─────────────────────────────────────────────────────┐
    │ Tipo            │ Uso                                                 │
    ├─────────────────┼─────────────────────────────────────────────────────┤
    │ Box<T>          │ Valor en heap, ownership único                      │
    │ Rc<T>           │ Múltiples owners, single-thread                     │
    │ Arc<T>          │ Múltiples owners, multi-thread                      │
    │ RefCell<T>      │ Mutabilidad interior (borrow check en runtime)      │
    │ Cell<T>         │ Mutabilidad interior para Copy types                │
    └─────────────────┴─────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod smart_pointers {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    pub fn smart_pointer_box() {
        // Box<T> - Ownership único en heap
        let box_val: Box<i32> = Box::new(42);
        assert_eq!(*box_val, 42);

        // Útil para tipos recursivos
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
        // Rc<T> - Múltiples owners (single-thread)
        let rc1: Rc<i32> = Rc::new(42);
        let rc2 = Rc::clone(&rc1);
        let rc3 = Rc::clone(&rc1);

        assert_eq!(Rc::strong_count(&rc1), 3);
        assert_eq!(*rc1, 42);
        assert_eq!(*rc2, 42);
        assert_eq!(*rc3, 42);

        // Todos apuntan al mismo valor
        assert!(Rc::ptr_eq(&rc1, &rc2));

        drop(rc3);
        assert_eq!(Rc::strong_count(&rc1), 2);

        println!("  ✅ smart_pointers::rc");
    }

    #[test]
    pub fn refcell() {
        // RefCell<T> - Borrow checking en runtime
        let cell: RefCell<i32> = RefCell::new(42);

        // Borrow inmutable
        {
            let borrowed = cell.borrow();
            assert_eq!(*borrowed, 42);
        }

        // Borrow mutable
        {
            let mut borrowed_mut = cell.borrow_mut();
            *borrowed_mut = 100;
        }

        assert_eq!(*cell.borrow(), 100);

        // Arc<T> para multi-thread
        let arc1: Arc<i32> = Arc::new(42);
        let arc2 = Arc::clone(&arc1);
        assert_eq!(Arc::strong_count(&arc1), 2);
        assert_eq!(*arc1, *arc2);

        println!("  ✅ smart_pointers::refcell");
    }
}
/*
============================================================================
10. REFERENCIAS - PUNTEROS
============================================================================

CASOS DE CONVERSIÓN entre referencias y punteros raw:

1. COERCIÓN: &T → *const T (automático)
2. PUNTERO A PUNTERO: *const *const T
3. DEREFERENCIA SEGURA: *const T → &T (dentro de unsafe)
*/
#[cfg(test)]
mod referencias_punteros {
    #[test]
    pub fn coercion_referencia_a_puntero_raw() {
        // ─────────────────────────────────────────────────────────────────
        // Coerción a punteros raw desde referencias
        // ─────────────────────────────────────────────────────────────────
        //
        // &T puede convertirse automáticamente a *const T
        // Esta conversión es segura porque la referencia garantiza validez

        let a = 42;
        let x: &i32 = &a; // x: &i32

        // FORMA 1: Conversión automática (coerción)
        let y: *const i32 = x;
        // Válida - Rust infiere automáticamente que &i32 → *const i32

        // FORMA 2: Cast explícito
        let z = x as *const i32;
        // Válida - Cast manual y explícito

        // FORMA 3: Conversión con tipo anotado (si fuera necesario)
        let w: *const i32 = x as *const i32;
        // Válida - Redundante pero explícito

        // Todos apuntan a la misma dirección
        assert_eq!(y, z);
        assert_eq!(z, w);
        assert_eq!(y as usize, &a as *const i32 as usize);

        unsafe {
            assert_eq!(*y, 42);
            assert_eq!(*z, 42);
            assert_eq!(*w, 42);
        }

        println!("  ✅ referencias_punteros::coercion_referencia_a_puntero_raw");
    }

    #[test]
    pub fn puntero_a_puntero() {
        // ─────────────────────────────────────────────────────────────────
        // Un puntero puede apuntar a otro puntero
        // ─────────────────────────────────────────────────────────────────
        //
        // *const *const T = puntero a puntero
        // Acceso: ** desreferencia dos niveles

        let x = 42;
        let ptr_x: *const i32 = &x as *const i32; // Puntero a i32

        // Puntero a puntero
        let ptr_to_ptr: *const *const i32 = &ptr_x as *const *const i32;

        // Visualizado:
        // ptr_to_ptr → dirección de ptr_x → dirección de x → valor 42
        //   0x3000      0x2000              0x1000          42

        // Acceso:
        unsafe {
            assert_eq!(*ptr_to_ptr as *const i32, ptr_x);
            assert_eq!(**ptr_to_ptr, 42); // Doble desreferencia
        }

        // Verificar direcciones
        assert_eq!(ptr_to_ptr as *const *const i32, &ptr_x as *const *const i32);

        println!("  ✅ referencias_punteros::puntero_a_puntero");
    }

    #[test]
    pub fn referencia_desde_puntero_raw() {
        // ─────────────────────────────────────────────────────────────────
        // Una referencia puede prestarse de un puntero
        // ─────────────────────────────────────────────────────────────────
        //
        // *const T → &T requiere:
        // - Dereferencia segura (dentro de unsafe)
        // - Validez del puntero

        let x = 42;
        let ptr_x: *const i32 = &x as *const i32;

        // ❌ No puedes hacer esto:
        // let ref_x: &i32 = ptr_x;  // Error: no conversión automática

        // ✅ Debes dereferenciar el puntero dentro de unsafe:
        unsafe {
            let ref_x: &i32 = &*ptr_x; // Dereferencia + toma referencia
            assert_eq!(*ref_x, 42);
            assert_eq!(ref_x, &42);
        }

        // Patrón seguro con validación
        if !ptr_x.is_null() {
            unsafe {
                let ref_x: &i32 = &*ptr_x;
                assert_eq!(*ref_x, 42);
            }
        }

        println!("  ✅ referencias_punteros::referencia_desde_puntero_raw");
    }
}
/*
============================================================================
10. BENCHMARKS - Comparaciones de rendimiento
============================================================================

REGLA GENERAL:
    Tipos Copy pequeños (≤16 bytes) → pasar por valor
    Tipos grandes o no-Copy → pasar por referencia

RESULTADOS TÍPICOS:
    String (heap):    clone es ~50-100x más caro que referencia
    Point (16 bytes): copy es casi igual que referencia
    User (con Strings): clone es MUY caro
    Vec<i32>:         clone es proporcional al tamaño
*/
#[cfg(test)]
mod benchmarks {
    use super::*;

    #[derive(Clone, Copy)]
    struct Point {
        _x: f64,
        _y: f64,
    }

    #[derive(Clone)]
    struct User {
        _id: u64,
        _name: String,
        _email: String,
        age: u32,
    }

    fn process_string_by_ref(s: &String) -> usize {
        s.len()
    }

    fn process_string_by_value(s: String) -> usize {
        s.len()
    }

    fn process_point_by_ref(point: &Point) -> f64 {
        point._x + point._y
    }

    fn process_point_by_value(point: Point) -> f64 {
        point._x + point._y
    }

    fn process_user_by_ref(user: &User) -> bool {
        user.age > 18
    }

    fn process_user_by_value(user: User) -> bool {
        user.age > 18
    }

    fn process_vec_by_ref(vec: &Vec<i32>) -> usize {
        vec.len()
    }

    fn process_vec_by_value(vec: Vec<i32>) -> usize {
        vec.len()
    }

    #[test]
    pub fn benchmark_strings() {
        let iterations = 10_000;
        let s = "x".repeat(1000);

        // Por referencia
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = black_box(process_string_by_ref(black_box(&s)));
        }
        let duration_ref = start.elapsed();

        // Por valor (con clone)
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = black_box(process_string_by_value(black_box(s.clone())));
        }
        let duration_value = start.elapsed();

        let ratio = duration_value.as_nanos() as f64 / duration_ref.as_nanos().max(1) as f64;

        println!("  Benchmark String 1000 chars ({} iter):", iterations);
        println!("    Por referencia: {:?}", duration_ref);
        println!("    Por valor (clone): {:?}", duration_value);
        println!("    Factor: {:.1}x", ratio);
        assert!(ratio > 1.0, "clone debería ser más lento");

        println!("  ✅ benchmarks::benchmark_strings");
    }

    #[test]
    pub fn benchmark_structs() {
        let iterations = 10_000;

        // Point (Copy, 16 bytes)
        let points: Vec<Point> = (0..iterations)
            .map(|i| Point {
                _x: i as f64,
                _y: (i as f64) * 2.0,
            })
            .collect();

        let start = Instant::now();
        for p in &points {
            let _ = black_box(process_point_by_ref(black_box(p)));
        }
        let duration_ref_point = start.elapsed();

        let start = Instant::now();
        for p in &points {
            let _ = black_box(process_point_by_value(black_box(*p)));
        }
        let duration_value_point = start.elapsed();

        println!("  Benchmark Point (16 bytes, Copy) {} iter:", iterations);
        println!("    Por referencia: {:?}", duration_ref_point);
        println!("    Por valor (copy): {:?}", duration_value_point);

        // User (Clone, con Strings)
        let users: Vec<User> = (0..iterations)
            .map(|i| User {
                _id: i,
                _name: format!("User{}", i),
                _email: format!("user{}@test.com", i),
                age: 30,
            })
            .collect();

        let start = Instant::now();
        for user in &users {
            let _ = black_box(process_user_by_ref(black_box(user)));
        }
        let duration_ref_user = start.elapsed();

        let start = Instant::now();
        for user in &users {
            let _ = black_box(process_user_by_value(black_box(user.clone())));
        }
        let duration_value_user = start.elapsed();

        let ratio_user =
            duration_value_user.as_nanos() as f64 / duration_ref_user.as_nanos().max(1) as f64;

        println!("  Benchmark User (con Strings) {} iter:", iterations);
        println!("    Por referencia: {:?}", duration_ref_user);
        println!("    Por valor (clone): {:?}", duration_value_user);
        println!("    Factor: {:.1}x", ratio_user);

        println!("  ✅ benchmarks::benchmark_structs");
    }

    #[test]
    pub fn benchmark_vectors() {
        let iterations = 1_000;
        let vecs: Vec<Vec<i32>> = (0..iterations).map(|_| (0..1000).collect()).collect();

        let start = Instant::now();
        for vec in &vecs {
            let _ = black_box(process_vec_by_ref(black_box(vec)));
        }
        let duration_ref = start.elapsed();

        let start = Instant::now();
        for vec in &vecs {
            let _ = black_box(process_vec_by_value(black_box(vec.clone())));
        }
        let duration_value = start.elapsed();

        let ratio = duration_value.as_nanos() as f64 / duration_ref.as_nanos().max(1) as f64;

        println!("  Benchmark Vec<i32> 1000 elements ({} iter):", iterations);
        println!("    Por referencia: {:?}", duration_ref);
        println!("    Por valor (clone): {:?}", duration_value);
        println!("    Factor: {:.1}x", ratio);

        println!("  ✅ benchmarks::benchmark_vectors");
    }
}
