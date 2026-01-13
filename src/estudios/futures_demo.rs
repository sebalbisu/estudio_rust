use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use futures_util::future;

// ============================================================================
// ÍNDICE - Ejecuta todas las demos
// ============================================================================

#[tokio::test]
async fn indice() {}

// ============================================================================
// 1. FUTURE MANUAL - Implementación básica
// ============================================================================
//
// Un Future es un valor que puede no estar listo todavía.
// El trait Future tiene un método poll() que retorna:
//   - Poll::Ready(T)  → el valor está listo
//   - Poll::Pending   → todavía no está listo, llamar de nuevo
//
//     struct ReadyFuture { value: i32 }
//
//     impl Future for ReadyFuture {
//         type Output = i32;
//         fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<i32> {
//             Poll::Ready(self.value)  // Siempre listo inmediatamente
//         }
//     }
//
// Flujo de poll():
//     ┌─────────────────────────────────────────────────────────────┐
//     │ Executor                                                    │
//     │    │                                                        │
//     │    ├──▶ future.poll(cx)                                     │
//     │    │         │                                              │
//     │    │         ├──▶ Poll::Ready(T) → Completado, retorna T   │
//     │    │         │                                              │
//     │    │         └──▶ Poll::Pending → Esperar, poll de nuevo   │
//     │    │                    │                                   │
//     │    │◀───────────────────┘                                   │
//     │    │                                                        │
//     └─────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod future_manual {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    /// Future básico que retorna un valor fijo inmediatamente
    pub struct ReadyFuture {
        pub value: i32,
    }

    impl Future for ReadyFuture {
        type Output = i32;

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            // Este future se completa inmediatamente
            Poll::Ready(self.value)
        }
    }

    #[tokio::test]
    pub async fn ready_future() {
        let fut = ReadyFuture { value: 42 };
        let result = fut.await;
        assert_eq!(result, 42);
    }

    /// Future que requiere varios polls antes de completarse
    pub struct CountdownFuture {
        count: u32,
    }

    impl CountdownFuture {
        pub fn new(start: u32) -> Self {
            Self { count: start }
        }
    }

    impl Future for CountdownFuture {
        type Output = &'static str;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.count == 0 {
                Poll::Ready("¡Completado!")
            } else {
                self.count -= 1;
                // Programar para ser polleado de nuevo
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    #[tokio::test]
    pub async fn countdown_future() {
        let fut = CountdownFuture::new(3);
        let result = fut.await;
        assert_eq!(result, "¡Completado!");
    }
}

// ============================================================================
// 8. SYNTAX SUGAR - Qué genera async/await
// ============================================================================
//
// `async fn` y `async {}` se transforman en una máquina de estados.
//
// TÚ ESCRIBES (con lifetime):
//     async fn fetch_data(url: &str) -> String {
//         let response = http_get(url).await;
//         let data = parse(response).await;
//         data
//     }
//
// EL COMPILADOR GENERA (conceptualmente):
//     fn fetch_data(url: &'a str) -> impl Future<Output = String> + 'a {
//         FetchDataFuture::State0 { url }
//     }
//
//     enum FetchDataFuture<'a> {
//         State0 { url: &'a str },                     // Antes del 1er await
//         State1 { url: &'a str, response: Response }, // Entre awaits
//         State2 { data: String },                     // Después del 2do await
//         Completed,
//     }
//
//     IMPORTANTE: El Future captura &'a str, así que:
//     - El Future NO PUEDE VIVIR más que 'a
//     - El Future debe completarse antes de que url sea dropeado
//     - El lifetime es PARTE del tipo del Future
//
//     impl Future for FetchDataFuture<'_> {
//         type Output = String;
//         fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
//             match self.get_mut() {
//                 State0 { url } => { /* llamar http_get(url), transicionar */ }
//                 State1 { response, .. } => { /* llamar parse */ }
//                 State2 { data } => Poll::Ready(data.clone()),
//                 Completed => panic!("polled after completion"),
//             }
//         }
//     }
//
// `.await` es syntax sugar para:
//     loop {
//         match future.poll(cx) {
//             Poll::Ready(value) => break value,
//             Poll::Pending => {
//                 // Guardar estado
//                 return Poll::Pending;
//             }
//         }
//     }
//
// TABLA RESUMEN:
//     ┌─────────────────────────────┬───────────────────────────────────┐
//     │ Syntax                      │ Se transforma en                  │
//     ├─────────────────────────────┼───────────────────────────────────┤
//     │ async fn foo() -> T         │ fn foo() -> impl Future<Output=T> │
//     │ async fn foo(x: &T) -> R    │ fn foo(x: &T) -> impl Fut<Ou=R>+' │
//     │ async { expr }              │ Struct anónimo que impl Future    │
//     │ future.await                │ Loop de poll + yield Pending      │
//     │ tokio::spawn(fut)           │ Box::pin(fut) + agregar a cola    │
//     └─────────────────────────────┴───────────────────────────────────┘
//
// ⚠️ IMPORTANTE CON REFERENCES:
//     async fn fetch_data(url: &str) -> String { ... }
//
//     El Future resultante tiene lifetime 'a:
//     impl Future<Output = String> + 'a
//
//     Esto significa:
//     ✓ let fut = fetch_data(&url); fut.await; // OK (url vive)
//     ✗ let fut = fetch_data(&url); drop(url); fut.await; // ERROR

#[cfg(test)]
mod syntax_sugar {
    use std::{future::Future, pin::Pin};

    // async fn se transforma en fn que retorna impl Future
    async fn example_async() -> i32 {
        42
    }

    // Equivalente manual (conceptual)
    fn example_manual() -> impl Future<Output = i32> {
        async { 42 }
    }

    #[test]
    pub fn syntax_sugar() {
        let _fut1 = example_async(); // impl Future<Output = i32>
        let _fut2 = example_manual(); // impl Future<Output = i32>
        let _fut4 = Box::pin(example_manual()); // Pin<Box<dyn Future<Output = i32>>>

        println!("  ✅ syntax_sugar::syntax_sugar");
    }

    #[tokio::test]
    async fn both_equivalent() {
        let r1 = example_async().await;
        let r2 = example_manual().await;
        assert_eq!(r1, r2);
    }
}

/*
PIN = ver archivo de PIN para entender problemas y solucion a autoreferencias en struct
*/

// ============================================================================
// 4. ASYNC EN TRAITS: DIFERENTES FORMAS
// ============================================================================
//
// Hay diferentes formas de definir métodos async en traits:
//
// 1. Manualmente con impl Future
// 2. Manualmente con Pin<Box<dyn Future>>
// 3. Usando async fn nativo
// 4. Usando #[async_trait] con Send obligatorio
// 5. Usando #[async_trait(?Send)] con Send opcional
//

/*
//  Analisis de una firma con Pinned Boxed Future:
//  -----------------------------------------------------------
//     ┌─────────────────────────────────────────────────────────────────┐
//     │ Pin<Box<dyn Future<Output = i32> + Send + '_>>                  │
//     │  │    │    │                        │      │                    │
//     │  │    │    │                        │      └─ Lifetime del self │
//     │  │    │    │                        └─ Thread-safe              │
//     │  │    │    └─ Trait object (dynamic dispatch)                   │
//     │  │    └─ Heap allocation                                        │
//     │  └─ Garantiza inmovibilidad                                     │
//     └─────────────────────────────────────────────────────────────────┘
//
//  * dyn Future<Outptut = i32> // cualquier que implemente Future y retorne i32
//  * + Send // el future/closure async es seguro para hilos => sus parametros también deben ser Send, y tambien sus capturas
//  * + '_   // el future vive como max como las referencias capturadas
// * Box // aloca el future en el heap, necesario para trait objects
// * Pin // garantiza que el future no se moverá en memoria mientras esté activo si es !Unpin (o sea tiene self-referencias)
*/

#[cfg(test)]
mod async_traits {

    #[test]
    pub fn diferentes_formas_de_traits() {
        use async_trait::async_trait;
        use std::{pin::Pin, rc::Rc};
        use tokio::{sync::futures, task::block_in_place};

        trait ManualAsyncImpl {
            fn call(&self) -> impl Future<Output = i32> + Send + '_;
        }

        trait ManualAsyncDyn {
            fn call(&self) -> Pin<Box<dyn Future<Output = i32> + Send + '_>>;
        }

        trait NativeAsync {
            async fn call(&self) -> i32;
            // fn call(&self) -> impl Future<Output = i32> + '_;
            // el future es Send si su impl lo es <=> las capturas y parametros lo son
        }

        #[async_trait]
        trait LibAsyncTraitSend {
            async fn call(&self) -> i32;
            // fn call(&self) -> Pin<Box<dyn Future<Output = i32> + Send + '_>>;
            // obliga a ser Send
        }

        #[async_trait(?Send)]
        trait LibAsyncTraitOptionalSend {
            async fn call(&self) -> i32;
            // fn call(&self) -> Pin<Box<dyn Future<Output = i32> + '_>>;
            // no obliga a ser Send
        }

        /*
        CASO: Tipo de dato send requerido, falla si implementación es no Send
        -----------------------------------------
         */
        #[async_trait]
        trait LibAsyncTraitSendErr {
            async fn call(&self, rc: Rc<i32>) -> i32;
        }

        struct MyImpl;

        // #[async_trait]
        // impl LibAsyncTraitSendErr for MyImpl {
        // async fn call(&self, rc: Rc<i32>) -> i32 {
        //     // El parámetro rc (!Send) se captura a través del await
        //     // ❌ ERROR EN COMPILACIÓN: Future no es Send
        //     let _x = tokio::time::sleep(std::time::Duration::from_secs(0)).await;
        //     *rc
        // }
        // }

        /*
        CASO: tipo de dato send requerido, falla si viene sin Send
        -----------------------------------------
        */
        #[async_trait(?Send)]
        trait LibAsyncTraitOptionalSendData {
            async fn call(&self, data: Rc<i32>) -> i32;
        }

        struct MyImpl2;

        #[async_trait(?Send)]
        impl LibAsyncTraitOptionalSendData for MyImpl2 {
            async fn call(&self, data: Rc<i32>) -> i32 {
                // El parámetro data (!Send) se captura a través del await
                let _x = tokio::time::sleep(std::time::Duration::from_secs(0)).await;
                *data
            }
        }

        fn asdf(x: Pin<Box<dyn Future<Output = i32> + Send + 'static>>) -> i32 {
            tokio::runtime::Runtime::new().unwrap().block_on(x)
        }
        #[tokio::test]
        async fn testx() {
            let impl2 = MyImpl2 {};
            let rc = Rc::new(42);
            let fut = impl2.call(rc);
            // asdf(Box::pin(fut)); // Error: Future no es Send
        }
    }
}

// ============================================================================
// 5. LIFETIMES EN ASYNC - Referencias deben vivir hasta el await
// ============================================================================
//
// En async, las referencias capturadas deben vivir hasta que el Future complete.
//
//     async fn process_ref(data: &str) -> String {
//         format!("Processed: {}", data)
//     }
//
//     async fn caller() {
//         let data = String::from("test");
//         let result = process_ref(&data).await;  // ✅ OK
//         // data vive hasta aquí
//     }
//
// ❌ ESTO NO COMPILA:
//     async fn bad() {
//         let data = String::from("test");
//         let fut = process_ref(&data);  // Future captura &data
//         drop(data);                     // ❌ Error: data aún prestado
//         fut.await;
//     }
//
// Diagrama de lifetime:
//     ┌──────────────────────────────────────────────────────────────┐
//     │ let data = String::from("test");                             │
//     │     │                                                        │
//     │     │  let fut = process_ref(&data);                         │
//     │     │      │                                                 │
//     │     │      │  ← Future captura &data                         │
//     │     │      │                                                 │
//     │     │      │  fut.await                                      │
//     │     │      │      │                                          │
//     │     │      └──────┘  ← Future completa, libera &data         │
//     │     │                                                        │
//     │     └─────────────── ← data puede ser dropeado aquí          │
//     └──────────────────────────────────────────────────────────────┘

// ============================================================================
// 6. SEND + SYNC EN FUTURES
// ============================================================================
//
// Un Future es Send si puede moverse entre threads.
// Esto requiere que todos los valores capturados sean Send.
//
//     async fn test_() -> i32 {
//         42  // ✅ Send: no captura nada no-Send
//     }
//
//     async fn not_test_() -> i32 {
//         let rc = Rc::new(42);  // ❌ Rc no es Send
//         *rc
//     }
//
// tokio::spawn() requiere Future: Send + 'static:
//
//     tokio::spawn(test_());      // ✅ OK
//     tokio::spawn(not_test_());  // ❌ Error: Future is not Send
//
// REGLA:
//     ┌─────────────────────────────────────────────────────────────┐
//     │ Si capturas Rc, Cell, RefCell, *mut T → Future es !Send     │
//     │ Solución: usar Arc, Mutex, AtomicXxx en su lugar            │
//     └─────────────────────────────────────────────────────────────┘

// ============================================================================
// 7. BOXED FUTURES - Box<dyn Future> para trait objects
// ============================================================================
//
// Pin<Box<dyn Future<Output = T> + Send>> permite:
// - Almacenar diferentes tipos de Futures en una colección
// - Retornar Futures de funciones sin tipo concreto
// - Crear APIs flexibles con trait objects
//
//     fn create_future(val: i32) -> Pin<Box<dyn Future<Output = i32> + Send>> {
//         Box::pin(async move { val * 2 })
//     }
//
//     // Colección heterogénea de Futures
//     let mut futures: Vec<Pin<Box<dyn Future<...>>>> = vec![];
//     futures.push(create_future(1));
//     futures.push(create_future(2));
//
// Costo:
//     ┌─────────────────────────────────────────────────────────────┐
//     │ • Heap allocation (Box)                                     │
//     │ • Dynamic dispatch (dyn) - llamadas indirectas              │
//     │ • Pero necesario para heterogeneidad                        │
//     └─────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod boxed_futures {
    use super::*;

    pub fn create_boxed_future(val: i32) -> Pin<Box<dyn Future<Output = i32> + Send>> {
        Box::pin(async move { val * 2 })
    }

    #[test]
    pub fn boxed_future() {
        let fut1 = create_boxed_future(10);
        let fut2 = create_boxed_future(20);

        // Ambos tienen el mismo tipo, pueden ir en un Vec
        let futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> = vec![fut1, fut2];
        assert_eq!(futures.len(), 2);

        println!("  ✅ boxed_futures::boxed_future");
    }

    #[tokio::test]
    async fn boxed_await() {
        let fut = create_boxed_future(21);
        let result = fut.await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn multiple_boxed() {
        let futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> = vec![
            create_boxed_future(1),
            create_boxed_future(2),
            create_boxed_future(3),
        ];

        let results: Vec<i32> = futures_util::future::join_all(futures).await;
        assert_eq!(results, vec![2, 4, 6]);
    }
}

// ============================================================================
// 9. GUARDAR FUTURES - Por qué necesitas Pin al almacenar
// ============================================================================
//
// Cuando GUARDAS un Future para ejecutar después, necesitas Pin.
// Esto es porque el Future puede tener self-referencias internas, y podria moverse su contenido
// si no está pinneado. Luego al ejecutar tendria Undefined Behavior.
//
#[cfg(test)]
mod guardar_futures {
    use super::*;
    use tokio::time::{Duration, sleep};

    async fn process_data(id: u32) -> String {
        let prefix = format!("Task-{}", id);
        sleep(Duration::from_millis(1)).await;
        format!("{}: done", prefix)
    }

    /// Scheduler que guarda Futures con Pin
    pub struct Scheduler {
        tasks: Vec<Pin<Box<dyn Future<Output = String> + Send>>>,
    }

    impl Scheduler {
        pub fn new() -> Self {
            Self { tasks: Vec::new() }
        }

        pub fn schedule<F>(&mut self, fut: F)
        where
            F: Future<Output = String> + Send + 'static,
        {
            // Box::pin: aloca en heap + garantiza inmovibilidad
            self.tasks.push(Box::pin(fut));
        }

        pub fn task_count(&self) -> usize {
            self.tasks.len()
        }
    }

    #[test]
    pub fn scheduler() {
        let mut scheduler = Scheduler::new();

        scheduler.schedule(process_data(1));
        scheduler.schedule(process_data(2));
        scheduler.schedule(process_data(3));

        assert_eq!(scheduler.task_count(), 3);

        println!("  ✅ guardar_futures::scheduler");
    }

    #[tokio::test]
    async fn execute_scheduled() {
        let mut scheduler = Scheduler::new();
        scheduler.schedule(process_data(1));
        scheduler.schedule(process_data(2));

        let mut results = Vec::new();
        for task in scheduler.tasks.iter_mut() {
            let result = task.as_mut().await;
            results.push(result);
        }

        assert_eq!(results.len(), 2);
        assert!(results[0].contains("Task-1"));
        assert!(results[1].contains("Task-2"));
    }
}
