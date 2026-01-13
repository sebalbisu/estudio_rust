// Instructions for documenting code in Rust:
// - Use this file as a guide for writing documentation.
// - Follow the structure and formatting shown here.
// - Maintain text original content, only edit the style and formatting, unless specified otherwise.

#[test]
fn indice() {
    movimiento::cambia_direccion();
    movimiento::a_funcion();
    autorreferencial_move2::no_rompe_al_mover_box();
    mover_contenido::cambiar_contenido_rompe_autorreferencia();
    mover_contenido::deferenciar_box_para_mover_contenido();
    pin_stack::pin_after_ref_wrong();
    pin_stack::pin_before_ref_ok();
    pin_stack::pin_stack_antes_de_pointer_ok_unpin_strong();
    pin_heap::pin_box_unpin();
    pin_heap::pin_box_unpin_strong();
    pin_struct::pin_struct_no_movable_ok();
    pin_project::some_fields_pinned_strong();
}

/*
========================================================================
EL PROBLEMA DE STRUCTS AUTORREFERENCIALES
========================================================================

    CASO 1: mover ownership en stack:
    --------------------------------------------
    Un struct autorreferencial tiene un campo que apunta a otro campo del mismo
    struct. Esto es PELIGROSO porque al mover el struct, el puntero
    interno queda apuntando a memoria inválida (dangling pointer).

    ┌─────────────────────────────────────────────────────────────────────────┐
    │ EL PROBLEMA: STRUCT AUTORREFERENCIAL                                    │
    ├─────────────────────────────────────────────────────────────────────────┤

      CREACIÓN:                     DESPUÉS DE MOVER:
      ┌────────────────────┐        ┌────────────────────┐
      │ sr @ 0x1000        │        │ sr_moved @ 0x2000  │
      │ data: "Hola"       │        │ data: "Hola"       │
      │ pointer: 0x1000 ───┼──┐     │ pointer: 0x1000 ───┼──→ ??? DANGLING
      └────────────────────┘  │     └────────────────────┘
                 ▲            │
                 └────────────┘     El pointer sigue apuntando a 0x1000
                                    pero data ahora está en 0x2000!

      → Usar `pointer` es UNDEFINED BEHAVIOR
    └─────────────────────────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod movimiento {

    #[derive(Debug, Clone)]
    pub struct Movable {
        pub value: i32,
        pub ptr: *const i32,
    }

    /// Los valores cambian de dirección al moverse
    #[test]
    pub fn cambia_direccion() {
        let mut val = Movable {
            value: 42,
            ptr: std::ptr::null(),
        };
        val.ptr = &val.value as *const i32;

        let val2 = val;
        let ptr2 = &val2.value as *const i32;

        assert_ne!(val2.ptr, ptr2, "El valor se movió a otra dirección");
    }

    /// Al pasar a funciones también se mueve
    #[test]
    pub fn a_funcion() {
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

        assert_ne!(ptr1, ptr2, "El valor se movió");
    }
}

/*
========================================================================
CASO 2 MOVER ownership de heap
========================================================================

    Box<T> pone el valor en heap, al mover variable BOX<T> NO se rompe la autoreferencia porque el heap no se mueve, solo cambia la ubicación del puntero en stack (8 bytes).
*/
#[cfg(test)]
mod autorreferencial_move2 {

    #[test]
    pub fn no_rompe_al_mover_box() {
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
            "Ambos apuntan a la misma dirección de heap"
        );
    }
}

/*
========================================================================
CASO 3 MOVER CONTENIDO
========================================================================

    Se puede mover el contenido directamente del stack o heap, rompiendo la autoreferencia:

    std::mem::swap(&mut *var1, &mut *var2)  // intercambiar contenidos, stack o heap, los ptrs quedan apuntando a la otra direccion

    std::mem::replace(&mut *var1, new_value) // reemplazar contenido, stack o heap, el ptr de var1 ahora apunta al de new_value

    equivalente usando dereferenciacion (en heap):
    (*var1, *var2) = (*var2, *var1) // swap
    *var1 = *var2;  // replace

    // tambien se puede mover el contenido del heap dereferenciando el Box<T>: (en stack no tiene sentido porque no hay puntero externo)
    // let boxed = Box::new(value);
    // let moved_stack: T = *boxed // dereferenciar y mover a stack
    // let moved_heap = Box::new(*boxed); // mover a otro heap
*/
#[cfg(test)]
mod mover_contenido {

    /*
    CAMBIAR CONTENIDO ROMPE AUTORREFERENCIA
    --------------------------------------------
    */
    #[test]
    pub fn cambiar_contenido_rompe_autorreferencia() {
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
    (DEFERENCIAR BOX<T> para mover contenido)
    --------------------------------------------
    */
    #[test]
    pub fn deferenciar_box_para_mover_contenido() {
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

/*
========================================================================
CONCEPTOS Y DEFINICIONES
========================================================================

    Pin:
        Es un Wrapper o Tipo de dato de marcacion (Pin<T>) util para el compilador.
        Ocupa 0 bytes en tiempo de ejecucion.
    !Unpin:
        Pin Indica al compilador que si el valor envuelto T es !Unpin, y T es movido a otra direccion de memoria entonces no se permita acceder a él, porque podría causar comportamientos indefinidos.
    PhantonPinned:
        agregando un campo de tipo de dato de marcacion PhantomPinned a una estructura, se marca como !Unpin al struct.
    Unpin:
        sin PhantomPinned, todas las estructuras son Unpin por defecto.
    Futures y !Unpin
        Los futuros generados por async si contienen referencias a sí mismos o a otros datos que son !Unpin son !Unpin. El compilador analiza el código generado por async y determina si el futuro es !Unpin en función de su contenido. Si el futuro es !Unpin, entonces se asegura de que no se mueva de su ubicación en memoria mientras está en uso, lo que es crucial para la seguridad de las referencias internas.
        En caso de no tener referencias a si mismo o a otros datos que son !Unpin, el futuro puede ser Unpin y moverse libremente en memoria.


    PIN PROTEGE DE HACER MODIFICACIONES DE UN !UNPIN
    ----------------------------------------------------

    Para cambiar el contenido se necesita un acceso ref mutable al contenido:

    std::mem::replace(&mut dest, src)
    std::mem::swap(&mut a, &mut b)
    *content = new_value

    o ownership mutable, pero si se pide una ref mut ya no se puede modificar el owner mutable.
    entonces al pedir Pin<&mut T> ya no se puede modificar el owner mutable, hay que usar el Pin para acceder al contenido.
    el caso de Pin<Box<T>> es mas sencillo porque Box<T> ya tiene ownership mutable y el Pin lo envuelve y restrige el acceso mutable al contenido.


    Tipo	     T: Unpin	     T: !Unpin
    Pin<&mut T>	 DerefMut ✅	    DerefMut ❌
    Pin<Box<T>>	 DerefMut ✅	    DerefMut ❌

    Conclucion:
    En un Pin de un tipo !Unpin no se puede alterar su contenido seguramente.
    Manteniendo la garantia de que el valor no se movera en memoria y no perdera su autoreferencia.
*/
#[cfg(test)]
mod pin_concepts {
    // This module is just for the concepts above, no code needed as per current file content.
}

/*
========================================================================
PIN EN STACK
========================================================================

    pin!(&mut T): Pin<&mut T>

    sugar syntax para pinnear en stack y obtener un Pin<&mut T> sin escribir el unsafe a mano y "esconde" la variable temporal que mueve el valor para que no pierda el ownership.

    // pin!(var1) equivalente a
    let mut __tmp = var1; // mueve var1
    let var2 = unsafe { Pin::new_unchecked(&mut __tmp) }; // ya no puedes usar var1, se movio a __tmp

    - Crea una variable temporal en stack y devuelve Pin<&mut T>.
    - Válido solo mientras el binding existe (scope local).
    - Ideal para: futuros locales, valores temporales, testing.

    // stack, scope local:
        #[test]
        fn local_future() {
            let fut = pin!(async { 42 });  // vive aquí
            // fut se dropa al salir de {}
        }


    Pin en stack se crea una direccion de memoria y se la marca como Pin<&mut T>.

    ┌──────────────────────────────────────────────┐
    │           STACK                              │
    ├──────────────────────────────────────────────┤
    │                                              │
    │ pin_ref @ 0x7fff0000                         │
    │ ┌──────────────────────────────┐             │
    │ │ Pin<&mut NoMovable>          │ 8 bytes     │
    │ │ (referencia a NoMovable)      │             │
    │ │ puntero interno = 0x7fff0010 │             │
    │ └──────────────────────────────┘             │
    │           ↓ apunta a                         │
    │ NoMovable @ 0x7fff0010 (stack)               │
    │ ┌──────────────────────────────┐             │
    │ │ value: i32      @ 0x7fff0010 │ 4 bytes     │
    │ │ ptr: *const i32 @ 0x7fff0014 │ 8 bytes     │
    │ └──────────────────────────────┘             │
    │                                              │
    └──────────────────────────────────────────────┘
*/
#[cfg(test)]
mod pin_stack {
    use std::pin::pin;

    #[test]
    pub fn pin_after_ref_wrong() {
        #[derive(Debug, Clone)]
        pub struct NoMovable {
            pub value: i32,
            pub ptr: *const i32,
        }

        let mut var1 = NoMovable {
            value: 10,
            ptr: std::ptr::null(),
        };
        var1.ptr = &var1.value as *const i32;
        let ptr1 = var1.ptr;

        let var2 = pin!(var1);

        assert_eq!(var2.ptr, ptr1);
        assert_ne!(&var2.value as *const i32, var2.ptr);
    }

    #[test]
    pub fn pin_before_ref_ok() {
        #[derive(Debug, Clone)]
        pub struct NoMovable {
            pub value: i32,
            pub ptr: *const i32,
        }

        let var1 = NoMovable {
            value: 10,
            ptr: std::ptr::null(),
        };
        let mut var2 = pin!(var1);
        // formas equivalentes en Unpin:
        {
            // mejor porque chequea Unpin en tiempo de compilacion, error especifico de !Unpin
            let var = var2.as_mut().get_mut();
            var.ptr = &var.value as *const i32;
        }
        var2.as_mut().get_mut().ptr = &var2.value as *const i32; // forma compacta
        var2.ptr = &var2.value as *const i32; // funciona en unpin, en !unpin falla: usa deref pero al no estar implementado en !unpin da error de deref

        assert_eq!(&var2.value as *const i32, var2.ptr);
    }

    /*
    La única forma de modificar el contenido de un !Unpin es usando get_unchecked_mut
    dentro de un bloque unsafe, porque DerefMut NO está implementado para !Unpin
    (solo Deref inmutable). Por lo tanto, no se puede modificar el contenido directamente
    de forma segura.
    */
    #[test]
    pub fn pin_stack_antes_de_pointer_ok_unpin_strong() {
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

        // esto funciona para Unpin y !Unpin:
        unsafe {
            let var = var2.as_mut().get_unchecked_mut();
            var.ptr = &var.value as *const i32;
        }
        // var2.ptr = &var2.value as *const i32; // imposible en !unpin porque no hay deref para unpin

        // la unica forma de acceder al contenido de !pin es usando get_unchecked_mut o get_unchecked_ref dentro de un bloque unsafe, porque el deref no esta implementado para !Unpin, por lo tanto no se puede modificar el contenido directamente.
        // replace(
        //     &mut *var2,  // error: no implementa deref en !Unpin
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
========================================================================
PIN EN HEAP
========================================================================

    Box::pin(T) : Pin<Box<T>>

    - Mueve el valor a Box<T> (heap), lo pineea, devuelve Pin<Box<T>>.
    - Válido mientras Box existe (puede ser movido entre funciones, guardado, etc.).
    - Ideal para: valores que viven más allá del scope, storage duradero, APIs que devuelven Pin<Box<dyn ...>>.

    Pin<Box<>> es una unica direccion de memoria, igual a la de Box, porque Pin para Box es un wrapper que no cambia la direccion de memoria del Box. Es decir Pin ocuparia 0 bytes.

    ┌──────────────────────────────────────────────┐
    │           STACK                              │
    ├──────────────────────────────────────────────┤
    │                                              │
    │ pin_box @ 0x7fff0000                         │
    │ ┌──────────────────────────────┐             │
    │ │ Pin<Box<NoMovable>>          │ 8 bytes     │
    │ │ (contiene Box puntero)       │             │
    │ │ puntero interno = 0x1000     │             │
    │ └──────────────────────────────┘             │
    │                                              │
    └──────────────────────────────────────────────┘
             ↓ apunta a HEAP
    ┌──────────────────────────────────────────────┐
    │           HEAP                               │
    ├──────────────────────────────────────────────┤
    │                                              │
    │ NoMovable @ 0x1000                           │
    │ ┌──────────────────────────────┐             │
    │ │ value: i32      @ 0x1000     │ 4 bytes     │
    │ │ ptr: *const i32 @ 0x1004     │ 8 bytes     │
    │ └──────────────────────────────┘             │
    │                                              │
    └──────────────────────────────────────────────┘

    // Box::pin = heap, duradero
        fn create_boxed_future() -> Pin<Box<dyn std::future::Future<Output = i32>>> {
            Box::pin(async { 42 })  // puede ser retornado, guardado
        }

        // El dato en heap NO se copia. Los datos (async { 42 }) permanecen en la misma dirección de memoria en el heap.
        // El Box (el puntero) SÍ se mueve del stack local al stack de quien llamó, pero el puntero sigue apuntando a la misma dirección de heap.
*/
#[cfg(test)]
mod pin_heap {

    #[test]
    pub fn pin_box_unpin() {
        #[derive(Debug, Clone)]
        pub struct NoMovable {
            pub value: i32,
            pub ptr: *const i32,
        }

        let mut var2 = Box::pin(NoMovable {
            value: 10,
            ptr: std::ptr::null(),
        });
        var2.ptr = &var2.value as *const i32; // funciona porque es Unpin acepta deref mut
        // o equivalente:
        {
            let var = var2.as_mut().get_mut();
            var.ptr = &var.value as *const i32;
        }

        assert_eq!(&var2.value as *const i32, var2.ptr);
    }

    #[test]
    pub fn pin_box_unpin_strong() {
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
        unsafe {
            let var = var2.as_mut().get_unchecked_mut();
            var.ptr = &var.value as *const i32;
        }

        assert_eq!(&var2.value as *const i32, var2.ptr);
    }
}

/*
========================================================================
IMPLEMENTACION !UNPIN EN STRUCT
========================================================================
*/
#[cfg(test)]
mod pin_struct {
    use std::{marker::PhantomPinned, pin::Pin};

    #[derive(Debug)]
    pub struct NoMovable {
        pub value: i32,
        pub ptr: *const i32,
        _pin: PhantomPinned, // marca como !Unpin
    }

    impl NoMovable {
        pub fn new(value: i32) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(NoMovable {
                value,
                ptr: std::ptr::null(),
                _pin: PhantomPinned,
            });
            unsafe {
                let mut_ref = boxed.as_mut().get_unchecked_mut();
                mut_ref.ptr = &mut_ref.value as *const i32;
            }
            boxed
        }
    }

    #[test]
    pub fn pin_struct_no_movable_ok() {
        let var1 = NoMovable::new(10);
        assert_eq!(&var1.value as *const i32, var1.ptr);
        // error al querer intentar modificar el contenido directamente:
        // *var1 = NoMovable {
        //     value: 20,
        //     ptr: std::ptr::null(),
        //     _pin: PhantomPinned,
        // };
    }
}

/*
AS_MUT Y AS_REF EN PIN
----------------------------------------------------

    Pin<T> tiene dos métodos para obtener referencias pinneadas al contenido:

    - as_mut(&mut self) -> Pin<&mut T>
        Devuelve una referencia mutable pinneada al contenido.
        Útil para modificar el contenido de forma segura.

    - as_ref(&self) -> Pin<&T>
        Devuelve una referencia inmutable pinneada al contenido.
        Útil para leer el contenido sin modificarlo.

    Estos métodos permiten trabajar con el contenido de un Pin<T> sin violar las garantías de no-movilidad que proporciona Pin.

    EJEMPLO VISUAL: as_mut vs &mut en Pin<T>
    ----------------------------------------------------
    instancia @ 0x1000 (MiStruct)
        ↑
        └─ pinned (Pin<&mut MiStruct>) @ 0x2000 (en stack)


    // let instancia: MiStruct = ...; // dato en 0x1000
    // let pinned: Pin<&mut MiStruct> = pin!(instancia);

    // let var2 : Pin<&mut MiStruct> = pinned.as_mut();
    // → accedes al dato en 0x1000 a través del Pin

    // let var3 : &mut Pin<&mut MiStruct> = &mut pinned;
    // → accedes al Pin mismo en 0x2000

    En resumen, as_mut() te da acceso con ref mut al dato pinneado, mientras que &mut te da acceso al Pin en sí.
*/

/*
========================================================================
FUTURES Y PIN (async/await)
========================================================================

    Los Futures generados por async/await pueden ser autorreferenciales cuando
    hay referencias que cruzan un `.await`. El compilador añade PhantomPinned
    automáticamente cuando detecta esto.

    ┌─────────────────────────────────────────────────────────────────────────┐
    │ ¿CUÁNDO UN FUTURE ES AUTORREFERENCIAL?                                  │
    ├─────────────────────────────────────────────────────────────────────────┤

      SIN self-ref (Unpin):              CON self-ref (!Unpin):
      ─────────────────────              ──────────────────────
      async fn simple() {                async fn con_ref() {
          let x = 42;                        let data = vec![1,2,3];
          some_op().await;                   let r = &data;   // ref
          x + 1  // x es Copy                some_op().await; // await
      }                                      println!("{:?}", r); // usa ref
                                         }
      → Future es Unpin                  → Future es !Unpin
    └─────────────────────────────────────────────────────────────────────────┘

    ┌─────────────────────────────────────────────────────────────────────────┐
    │ LO QUE EL COMPILADOR GENERA (Esquema)                                   │
    ├─────────────────────────────────────────────────────────────────────────┤
      async fn con_ref() { let data = ...; let r = &data; await; r }

        async fn ejemplo() {
            let mut count = 0;        // mutable, NO autorreferencial
            let data = vec![1, 2, 3]; // inmutable después de crear ref
            let r = &data;            // referencia autorreferencial

            loop {
                count += 1;           // ✅ mutación libre
                println!("{}: {:?}", count, r);
                some_op().await;
            }
        }

        // Lo que el compilador genera (simplificado):
        struct EjemploFuture {
            count: i32,
            data: Vec<i32>,
            r: *const Vec<i32>,
            state: State,
            _pin: PhantomPinned,
        }

        impl Future for EjemploFuture {
            fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
                // acceso con unsafe para editar el contenido en poll
                // el codigo lo genera el compilador
                unsafe {
                    let this = self.get_unchecked_mut();
                    this.count += 1;
                    println!("{}: {:?}", this.count, *this.r);
                }
                Poll::Pending
            }
        }

      fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Output>;
    └─────────────────────────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod futures_pin {
    // Explanation only
}

/*
PIN PROJECT
----------------------------------------------------
    pin-project es un crate que facilita la creación de structs con campos pinneados.
    Permite definir qué campos son pinneados y cuáles no, generando automáticamente
    el código necesario para proyectar los campos correctamente.

    - #[pin_project]: Macro para marcar el struct.
    - #[pin]: Atributo para marcar campos específicos como pinneados.
    - .project(): Método generado para obtener referencias proyectadas a los campos.
*/
#[cfg(test)]
mod pin_project {
    #[test]
    pub fn some_fields_pinned_strong() {
        use pin_project::pin_project;
        use std::pin::Pin;

        #[derive(Debug)]
        #[pin_project]
        struct MiStruct {
            #[pin] // Este campo será proyectado como Pin<&mut T>
            field1: i32,

            // Sin #[pin]: acceso directo (&mut T)
            field2: i32,
        }

        impl MiStruct {
            fn modificar(self: Pin<&mut Self>) {
                let this = self.project(); // macro genera este método

                // *this.field1 = 23; /// Error: no tiene DerefMut
                unsafe {
                    *this.field1.get_unchecked_mut() = 100; // ok
                }
                *this.field2 += 1; // ok, sin unsafe
            }
        }

        let mut instancia = MiStruct {
            field1: 10,
            field2: 20,
        };
        let mut pinned = Pin::new(&mut instancia);
        pinned.as_mut().modificar();
        println!("field2: {:?}", &pinned);
    }
}
