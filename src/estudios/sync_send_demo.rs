use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// Indices: match secion name with module name, and test funcion with section topics
#[test]
fn indice() {
    conceptos_teoricos::definiciones();
    marcadores_send_sync::comportamiento_punteros();
    primitivas_sincronizacion::demo_arc();
    primitivas_sincronizacion::demo_mutex();
    primitivas_sincronizacion::demo_rwlock();
}

/*
=========================================================================
RESUMEN
=========================================================================

Send & Sync:
---------------
Se marca a un tipo T con traits de marcacion Send o Sync, segun si es seguro moverlo a threads (Send) o si es seguro compartir referencias inmutables entre threads (Sync).

Tipos no simples Send/Sync:
    T:  FnOnce | Futures | struct | ...
        T es Send, si sus campos/capturas son Send.
        T es Sync, si sus campos/capturas son Sync.

Send:
    Para mover el ownership (o &mut T) a un thread/future, T debe ser Send.

Sync:
    Para enviar una referencia (&T) a un thread/future, T debe ser Sync.
        T es Sync <=> &T es Send
        T no es Sync <=> &T no es Send

Lifespan Bounds:
    T: 'a         El tipo T vive al menos tanto como 'a
    T: '_         T vive tanto como todas sus referencias
    T: 'static    El lifetime de T es 'static.

T: 'static
    El lifetime de T es 'static, no depende de datos locales, tiene la capacidad de vivir todo el programa.
        No: &u8 = &'a u8 (referencias locales, lifetimes menores que 'static)
        Si: tipos simples (i32, String, Vec, etc) porque son dueños del dato.
        un struct es 'static si todos sus campos son 'static
        un closure es 'static si todas sus capturas son 'static
        Arc<T>, Rc<T> si T es 'static

Arc<T>, Rc<T> 'static:
    Si T es 'static, entonces Rc<T> y Arc<T> son 'static.
    Mueven el dato al heap y asumen la propiedad (ownership) COMPARTIDA. Vive siempre que exista una referencia a él, comparten el ownership, no depende de datos locales.

F: Send + 'static (T: Send y T: 'static)
    No puede capturar referencias &T de tipos simples/locales.
    Tampoco capturar struct con referencias.
*/

/*
========================================================================
CONCEPTOS TEORICOS
========================================================================

    * Conceptos básicos sobre concurrencia y condiciones de carrera.

    Race Condition:
    -------------------------------------------

        (Single o Multi-thread) Error lógico por el orden de ejecución o uso de datos desactualizados.
        El programa es "seguro" (no crashea), pero el resultado es incorrecto.

        Visualización de Race Condition (Single Thread - Stale Data):

          Variable (X)          Copia Local (stale)           Operación
             [ 5 ]
               |
               |-------------------> [ 5 ]                  (1. Guardar copia)
               |
             [ 10 ] <-----------------------------------    (2. X cambia: ej. evento)
               |
               |                     [ 5 ] ---------------> (3. Usar copia antigua)
               |                                               5 + 1 = 6
             [ 6 ] <--------------------------------------- (4. Sobrescribir X)
               |
            ¡ERROR!
        (X debería ser 11, pero volvimos atrás en el tiempo porque usamos un dato viejo)


    Data Race:
    -------------------------------------------

        (Multi-thread) Acceso concurrente a memoria sin sincronización. UB (Undefined Behavior).

        No necesariamente la lectura o escritura tiene que ser simultánea, basta con que no haya sincronización.
        Es decir, al desconocer el accionar de otros threads, puede generarse una condición de carrera.


       Thread A                Memoria (Counter)               Thread B
                                     [ 5 ]
                                       |
    (t1) Leer (5) <--------------------|
          |                            |
    (t2) Incrementar (5+1)             |
          |                            |----------------> Leer (5) (t3)
    (t4) Escribir (6) ---------------> |                     |
                                     [ 6 ]             (t5) Incrementar (5+1)
                                       |                     |
                                       |<--------------- Escribir (6) (t6)
                                     [ 6 ]
                                       |
                                    ¡ERROR!
                 (Se perdió un incremento, debería ser 7)


    Reglas fundamentales:
    -------------------------------------------

    Send
        es sobre MOVIMIENTO: (Exclusividad)
        "Puedo pasarle la pelota a otro hilo."
        Garantiza que es seguro transferir el ownership (o &mut T) a otro hilo.
        Solo un hilo tiene acceso al dato en un momento dado.

    Sync
        es sobre ACCESO CONCURRENTE: (Lectura Syncronizada)
        "Varios hilos pueden mirar la misma pelota a la vez."
        Garantiza que es seguro que varios hilos accedan al dato simultáneamente vía &T.
        Varios hilos pueden leer el dato al mismo tiempo, pero ninguno puede modificarlo mientras se lee.

        El tipo debe ser seguro de leer (o tener sincronización interna como Mutex) desde varios sitios.
*/
#[cfg(test)]
mod conceptos_teoricos {
    #[test]
    pub fn definiciones() {
        println!(
            "Revisar comentarios del módulo para definiciones de Race Condition, Data Race, Send y Sync."
        );
    }
}

/*
========================================================================
MARCADORES SEND Y SYNC
========================================================================

    Traits Marcacion:
    -------------------------------------------
        Send y Sync son traits de marcacion (marker traits).

    Structs: marcacion automatica:
    -------------------------------------------
        * un Struct/Enum es Send si todos sus campos son Send
        * un Struct/Enum es Sync si todos sus campos son Sync

        Tambien le puedes decir al compilador que un tipo es Send o Sync manualmente:

        struct SendWrapper(*const i32);

        // LE PROMETES AL COMPILADOR QUE ES SEGURO
        unsafe impl Send for SendWrapper {}

    Punteros crudos NO son Send ni Sync:
    -------------------------------------------
        Los punteros crudos (*const T y *mut T) no son Sync (ni Send) por una razón fundamental:
        el compilador de Rust no puede garantizar qué hay al otro lado del puntero ni cuánto tiempo va a durar.

    Ejemplo Ni Send ni Sync:
    -------------------------------------------
        Ver struct _NoSendSyncStruct en el código.
*/
#[cfg(test)]
mod marcadores_send_sync {
    use super::*;

    #[derive(Clone)]
    struct _NoSendSyncStruct {
        pointer: *const i32, // No es Send ni Sync => struct tampoco
    }

    #[test]
    pub fn comportamiento_punteros() {
        let no_send1 = _NoSendSyncStruct {
            pointer: Box::into_raw(Box::new(10)),
        };
        let no_send2 = no_send1.clone();

        // same pointer address, two objects
        assert!(no_send1.pointer == no_send2.pointer);
        assert!(unsafe { *no_send1.pointer } == unsafe { *no_send2.pointer });

        // Pointer no es Send:
        // `*const i32` no es Send
        // Porque desde no_send2 se puede liberar la memoria y dejar no_send1 con un puntero colgante.
        // thread::spawn(move || {
        //     println!("Value: {}", unsafe { *no_send1.pointer });
        // });

        //----
        // Pointer no es Send, &mut ref
        // mismo motivo anterior
        let mut no_send3 = _NoSendSyncStruct {
            pointer: Box::into_raw(Box::new(10)),
        };
        let _no_send3_mut = &mut no_send3;
        // thread::spawn(move || {
        //     println!("Value: {}", unsafe { *_no_send3_mut.pointer });
        // });

        //----
        // Pointer no Sync: &ref
        // *const i32 no es Sync
        // Otro thread podría modificar el dato apuntado y dejar este puntero colgante.
        // Un puntero colgante (o dangling pointer) es un puntero que apunta a una dirección de memoria que ya ha sido liberada o de-alocada.
        let no_send4 = _NoSendSyncStruct {
            pointer: Box::into_raw(Box::new(10)),
        };
        let _no_send4_ref = &no_send4;
        // thread::spawn(move || {
        //     println!("Value: {}", unsafe { *_no_send4_ref.pointer });
        // });
    }
}

/*
========================================================================
PRIMITIVAS DE SINCRONIZACION
========================================================================

    Resumen de Tipos:
    -------------------------------------------

        * La mayoría de tipos son Send + Sync automáticamente.

        Tipos que NO son Send:
            * Rc: usa contador no atómico => data race
            * Raw pointers: no garantizan validez ni sincronización

        Tipos que NO son Sync:
            * Cell: permite mutación interior con &T, no es Sync
            * RefCell: permite mutación interior con &T, no es Sync

        Aclaraciones Importantes:
            * Arc<T> es Send + Sync si T: Send + Sync
            * Mutex<T> y RwLock<T> son Sync si T: Send

    Arc<T>:
    -------------------------------------------
        * Similar a Rc<T>, pero seguro para threads.
        * Contador Atomico: Arc garantiza que no haya data races en el contador de referencias.
        * Inmutabilidad del Dato: Arc<T> solo te da acceso de solo lectura.
        * T debe ser Send + Sync para que Arc<T> sea Send + Sync.

    Mutex<T>:
    -------------------------------------------
        * Permitir que varios hilos accedan a un mismo dato de forma segura, garantizando que solo uno pueda escribir o leer a la vez.
        * T debe ser Send para que Mutex<T> sea Sync.

    RwLock<T>:
    -------------------------------------------
        * Múltiples lectores o un escritor.
*/
#[cfg(test)]
mod primitivas_sincronizacion {
    use super::*;

    #[test]
    pub fn demo_arc() {
        println!("2. Arc<T> - compartir dato entre threads:");
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            println!("  Thread: suma = {}", data_clone.iter().sum::<i32>());
        });

        println!("  Main: primeros dos = {:?}", &data[0..2]);
        handle.join().unwrap();
        println!();
    }

    #[test]
    pub fn demo_mutex() {
        println!("3. Mutex<T> - mutación compartida:");
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 0..3 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                println!("  Thread {}: incrementó a {}", i, *num);
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }

        println!("  Final counter: {}", *counter.lock().unwrap());
        println!();
    }

    #[test]
    pub fn demo_rwlock() {
        println!("4. RwLock<T> - múltiples lectores:");
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let mut handles = vec![];

        // Varios lectores
        for i in 0..2 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let read_guard = data_clone.read().unwrap();
                println!("  Reader {}: {:?}", i, *read_guard);
            });
            handles.push(handle);
        }

        // Un escritor
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_guard = data_clone.write().unwrap();
            write_guard.push(4);
            println!("  Writer: agregó 4");
        });
        handles.push(handle);

        for h in handles {
            h.join().unwrap();
        }
        println!();
    }
}
