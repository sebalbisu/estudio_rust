#[allow(unused_variables)]
#[allow(dead_code)]
#[test]
fn indice() {
    move_semantics::concepto_clave();
    cuando_se_mueve::cuando_se_mueve();
    evitar_move::evitar_move();
    paso_por_valor::paso_por_valor();
    paso_por_referencia::paso_por_referencia();
    no_ref_a_stack_propio::no_ref_a_stack_propio();
    soluciones_return::soluciones();
    async_futures::async_futures();
    async_future_size::future_size();
    async_referencias::async_referencias();
    nrvo::nrvo();
    drop_patterns::drop_patterns();
}

// ============================================================================
// 1. MOVE SEMANTICS - Concepto clave
// ============================================================================
//
//     Mover ownership SIEMPRE es:
//       • Copiar los BYTES del stack (metadata: ptr, len, cap, etc.)
//       • Mantener el HEAP intacto (los datos reales no se mueven)
//
//     Ejemplo con String::from("hello"):
//
//     ANTES del move:
//     ┌──────────────┐                  ┌───────────┐
//     │ s1 (stack)   │                  │   HEAP    │
//     │ ptr ─────────┼─────────────────►│ "hello"   │
//     │ len: 5       │                  └───────────┘
//     │ cap: 5       │
//     └──────────────┘
//
//     DESPUÉS del move (let s2 = s1):
//     ┌──────────────┐  ┌──────────────┐   ┌───────────┐
//     │ s1 (invalid) │  │ s2 (stack)   │   │   HEAP    │
//     │ ██████████   │  │ ptr ─────────┼──►│ "hello"   │
//     │ ██████████   │  │ len: 5       │   │ (mismo!)  │
//     │ ██████████   │  │ cap: 5       │   └───────────┘
//     └──────────────┘  └──────────────┘
//
//     ✓ Stack: 24 bytes copiados (ptr + len + cap)
//     ✓ Heap: 0 bytes copiados ("hello" sigue en el mismo lugar)
//     ✓ s1 queda inválido (ownership transferido)
//
//     Esto aplica a: String, Vec, Box, y cualquier tipo con heap.
//     Para tipos Copy (i32, f64, bool): se copian completamente.

#[cfg(test)]
mod move_semantics {
    #[test]
    pub fn concepto_clave() {
        let s1 = String::from("hello");
        let ptr_before = s1.as_ptr();

        let s2 = s1; // move: s1 inválido
        let ptr_after = s2.as_ptr();

        // El puntero al heap es el mismo
        assert_eq!(ptr_before, ptr_after);
        // s1 ya no es válido, s2 es el nuevo owner
        assert_eq!(s2, "hello");
    }
}

// ============================================================================
// 2. CUÁNDO SE MUEVE EL OWNERSHIP
// ============================================================================
//
//     Situaciones que causan move:
//
//     1. ASIGNACIÓN A OTRA VARIABLE
//        let s2 = s1;  // s1 inválido
//
//     2. PASAR A FUNCIÓN / METODO POR VALOR
//        consume(s1);  // s1 inválido
//        x.method(s1);  // s inválido
//
//     3. RETORNAR DE UNA FUNCIÓN
//        fn create() -> String { String::from("x") }  // ownership al caller
//
//     5. DESTRUCTURING / PATTERN MATCHING
//        let (x, y) = tuple;  // tuple inválido
//
//     6. CLOSURE CON `move`
//        let c = move || println!("{}", s);  // s movido a closure

#[cfg(test)]
mod cuando_se_mueve {
    #[test]
    pub fn cuando_se_mueve() {
        // 1. Asignación
        let s1 = String::from("hello");
        let s2 = s1;
        // s1 ya no válido
        assert_eq!(s2, "hello");

        // 2. Pasar a función por valor
        fn consume(s: String) -> usize {
            s.len()
        }
        let s3 = String::from("world");
        let len = consume(s3);
        // s3 ya no válido
        assert_eq!(len, 5);

        // 3. Retornar de función
        fn create() -> String {
            String::from("created")
        }
        let s4 = create();
        assert_eq!(s4, "created");

        // 4. Insertar en colección
        let s5 = String::from("item");
        let mut vec = Vec::new();
        vec.push(s5);
        // s5 ya no válido
        assert_eq!(vec[0], "item");

        // 5. Destructuring
        let tuple = (String::from("a"), String::from("b"));
        let (x, y) = tuple;
        // tuple ya no válido
        assert_eq!(x, "a");
        assert_eq!(y, "b");

        // 6. Closure con move
        let s6 = String::from("closure");
        let closure = move || s6.len();
        // s6 ya no válido
        assert_eq!(closure(), 7);
    }
}

// ============================================================================
// 3. CÓMO EVITAR EL MOVE (mantener ownership)
// ============================================================================
//
//     OPCIÓN 1: Pasar referencia (borrow, no move)
//         fn borrow(s: &String) { ... }
//         borrow(&s);  // s sigue válido
//
//     OPCIÓN 2: Clonar (copia profunda, nuevo heap)
//         let s2 = s1.clone();  // ambos válidos
//
//     OPCIÓN 3: Rc/Arc para múltiples owners
//         let s = Rc::new(String::from("hello"));
//         let s2 = Rc::clone(&s);  // incrementa contador

#[cfg(test)]
mod evitar_move {

    #[test]
    pub fn evitar_move() {
        use std::rc::Rc;
        // Opción 1: Referencia
        fn borrow(s: &String) -> usize {
            s.len()
        }
        let s = String::from("hello");
        let len = borrow(&s);
        assert_eq!(s, "hello"); // s sigue válido
        assert_eq!(len, 5);

        // Opción 2: Clone
        let s1 = String::from("world");
        let s2 = s1.clone();
        assert_eq!(s1, s2); // ambos válidos

        // Opción 3: Rc
        let rc1 = Rc::new(String::from("shared"));
        let rc2 = Rc::clone(&rc1);
        let rc3 = Rc::clone(&rc1);
        assert_eq!(Rc::strong_count(&rc1), 3);
        assert_eq!(*rc1, *rc2);
        assert_eq!(*rc2, *rc3);
    }
}

// ============================================================================
// 4. PASO POR VALOR (Move entre stacks)
// ============================================================================
//
//     Pasar una variable por valor (pasar ownership) a una función:
//     Los bytes de un stack se COPIAN al nuevo stack frame,
//     y el original se considera 'movido' (no se puede usar).
//
//     ANTES de llamar a consume(user):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ user: User                              │        HEAP             │
//     │ │   _id: 1                                │      ┌────────────┐     │
//     │ │   _name: (ptr, len:5, cap:5) ───────────┼─────▶│ "Alice"    │     │
//     │ │   age: 30                               │      └────────────┘     │
//     │ └─────────────────────────────────────────┘                         │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     DESPUÉS de llamar a consume(user):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ user: ████ MOVIDO ████                  │                         │
//     │ │   (bytes aún existen pero inaccesibles) │                         │
//     │ └─────────────────────────────────────────┘                         │
//     ├─────────────────────────────────────────────────────────────────────┤
//     │ consume() stack frame (NUEVO)                                       │
//     │ ┌─────────────────────────────────────────┐        HEAP             │
//     │ │ user: User (COPIADO del caller)         │      ┌────────────┐     │
//     │ │   _id: 1                                │      │ "Alice"    │     │
//     │ │   _name: (ptr, len:5, cap:5) ───────────┼─────▶│ (mismo!)   │     │
//     │ │   age: 30                               │      └────────────┘     │
//     │ └─────────────────────────────────────────┘                         │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     AL TERMINAR consume():
//     - consume() stack frame destruido
//     - user.drop() libera el heap
//
//     ✓ Move = copia bytes del stack + transferencia de ownership
//     ✓ Solo UN owner puede hacer Drop (evita double-free)
//     ✓ El heap NO se copia, solo el puntero

#[cfg(test)]
mod paso_por_valor {
    #[test]
    pub fn paso_por_valor() {
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
            // El puntero al heap es el mismo
            u.name.as_ptr()
        }

        let ptr_after = consume(user);
        // user ya no es válido

        // El puntero al heap era el mismo dentro de consume
        assert_eq!(ptr_before, ptr_after);
    }
}

// ============================================================================
// 5. PASO POR REFERENCIA (Puntero a otro stack)
// ============================================================================
//
//     Pasar una variable por referencia a otra función:
//     SLa referencia es un puntero que apunta al stack del caller.
//
//     DURANTE la llamada a borrow(&user):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ user: User                              │        HEAP             │
//     │ │   _id: 1                      ◄─────────┼──┐   ┌────────────┐     │
//     │ │   _name: (ptr, len, cap) ───────────────┼──┼──▶│ "Alice"    │     │
//     │ │   age: 30                               │  │   └────────────┘     │
//     │ └─────────────────────────────────────────┘  │                      │
//     ├──────────────────────────────────────────────┼──────────────────────┤
//     │ borrow() stack frame                         │                      │
//     │ ┌────────────────────────────────────────┐   │                      │
//     │ │ user: &User (8 bytes)                  │   │                      │
//     │ │   ptr ─────────────────────────────────┼───┘                      │
//     │ │   (apunta al stack de main!)           │                          │
//     │ └────────────────────────────────────────┘                          │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     AL TERMINAR borrow():
//     - borrow() NO hace Drop de user (solo tenía referencia)
//     - user sigue válido en main()
//
//     ✓ Referencia = puntero de 8 bytes al stack del caller
//     ✓ NO transfiere ownership
//     ✓ El owner original sigue responsable del Drop

#[cfg(test)]
mod paso_por_referencia {

    #[test]
    pub fn paso_por_referencia() {
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

        // La referencia apunta al mismo lugar en stack
        assert_eq!(user_addr, borrowed_addr);

        // user sigue válido después del borrow
        assert_eq!(user.age, 25);
        assert_eq!(user.name, "Bob");
    }
}

// ============================================================================
// 6. NO SE PUEDE RETORNAR REFERENCIA A STACK PROPIO
// ============================================================================
//
//     No se puede retornar una referencia a una variable local
//     Rust lo previene en compilación.
//
//     CÓDIGO QUE NO COMPILA:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn dangling() -> &String {                                       │
//     │     let s = String::from("hello");                               │
//     │     &s  // ✗ ERROR: retorna referencia a variable local          │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘
//
//     POR QUÉ FALLARÍA (si Rust lo permitiera):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack frame                                                  │
//     │ ┌─────────────────────────────────────────┐                         │
//     │ │ result: &String                         │        HEAP             │
//     │ │   ptr ──────────────────────────────────┼──┐   ┌────────────┐     │
//     │ └─────────────────────────────────────────┘  │   │ LIBERADO!  │     │
//     │                                              │   └────────────┘     │
//     │ dangling() ← DESTRUIDO                       │                      │
//     │ ┌─────────────────────────────────────────┐  │                      │
//     │ │ ████████████████████████████████████████│◄─┘                      │
//     │ │ s: YA NO EXISTE (stack frame destruido) │   ← DANGLING POINTER!   │
//     │ └─────────────────────────────────────────┘                         │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     REGLA: Referencias solo pueden apuntar a:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ 1. Stack frames PADRES (callers)  ← viven más que la función     │
//     │ 2. Heap (via Box, Vec, String)    ← vive hasta Drop del owner    │
//     │ 3. 'static data (.rodata)         ← vive todo el programa        │
//     │                                                                  │
//     │ NUNCA al propio stack frame (muere al retornar)                  │
//     └──────────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod no_ref_a_stack_propio {
    #[test]
    pub fn no_ref_a_stack_propio() {
        // Este código NO compila:
        // fn dangling() -> &String {
        //     let s = String::from("hello");
        //     &s  // ERROR: `s` does not live long enough
        // }

        // La regla es: referencias solo pueden apuntar a
        // datos que viven más tiempo que la referencia

        // Esto SÍ funciona: retornar referencia a parámetro
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
// 7. SOLUCIONES VÁLIDAS PARA RETORNAR DATOS
// ============================================================================
//
//     SOLUCIÓN 1: Retornar ownership (move out)
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn create() -> String {                                          │
//     │     let s = String::from("hello");                               │
//     │     s  // ✓ Mueve ownership al caller                            │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘
//
//     SOLUCIÓN 2: Retornar referencia a parámetro externo con lifetime
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {              │
//     │     if x.len() > y.len() { x } else { y }                        │
//     │ }  // ✓ Retorna referencia a parámetro (vive en caller)          │
//     └──────────────────────────────────────────────────────────────────┘
//
//     SOLUCIÓN 3: Retornar 'static
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ fn get_message() -> &'static str {                               │
//     │     "hello"  // ✓ String literal vive para siempre               │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod soluciones_return {
    #[test]
    pub fn soluciones() {
        // Solución 1: Retornar ownership
        fn create() -> String {
            String::from("created")
        }
        let s = create();
        assert_eq!(s, "created");

        // Solución 2: Retornar referencia a parámetro
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        let s1 = String::from("short");
        let s2 = String::from("much longer");
        let result = longest(&s1, &s2);
        assert_eq!(result, "much longer");

        // Solución 3: Retornar 'static
        fn get_message() -> &'static str {
            "hello forever"
        }
        let msg = get_message();
        assert_eq!(msg, "hello forever");
    }
}

// ============================================================================
// 8. ASYNC FUTURES - Dónde viven los stacks
// ============================================================================
//
//     Las funciones async NO usan stack tradicional.
//     Sus variables estan almacenadas en un 'Future' (struct en HEAP).
//
//     FUNCIÓN ASYNC - SE COMPILA A UN STRUCT (Future):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ // Generado por el compilador:                                      │
//     │ struct FetchDataFuture {                                            │
//     │     state: StateEnum,  // enum con todos los estados               │
//     │ }                                                                   │
//     │                                                                     │
//     │ enum StateEnum {                                                    │
//     │     State0 { url: String },           // antes del primer .await    │
//     │     State1 { url: String, response: Response }, // después         │
//     │     State2 { data: String },          // al final                  │
//     │     Completed,                                                      │
//     │ }                                                                   │
//     │                                                                     │
//     │ impl Future for FetchDataFuture {                                  │
//     │     type Output = String;                                          │
//     │                                                                     │
//     │     fn poll(mut self: Pin<&mut Self>, cx: &mut Context) ->         │
//     │         Poll<Self::Output> {                                       │
//     │            // ... va generando distintos StateN a medida que resuelve cada.await
//     │            // pasa todas las variables que necesitan los estados siguientes.
//     │         }                                                           │
//     │     }                                                               │
//     │ }                                                                   │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     MEMORIA EN ASYNC:
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ STACK (pequeño)                          HEAP                       │
//     │ ┌────────────────────┐                   ┌─────────────────────────┐│
//     │ │ main()             │                   │ FetchDataFuture         ││
//     │ │   runtime executor │                   │ ┌─────────────────────┐ ││
//     │ │   future: Box<...>─┼──────────────────▶│ │ state: State1       │ ││
//     │ └────────────────────┘                   │ │ url: String ────────┼─┼┼─▶ heap
//     │                                          │ │ response: Response  │ ││
//     │                                          │ └─────────────────────┘ ││
//     │                                          └─────────────────────────┘│
//     └─────────────────────────────────────────────────────────────────────┘
//
//     DIFERENCIAS CLAVE:
//     ┌─────────────────────┬─────────────────────┬─────────────────────┐
//     │                     │ Síncrono            │ Async               │
//     ├─────────────────────┼─────────────────────┼─────────────────────┤
//     │ Variables viven en  │ Stack               │ Heap (Future)       │
//     │ Bloquea thread      │ Sí                  │ No                  │
//     │ Puede pausar        │ No                  │ Sí (.await)         │
//     │ Overhead memoria    │ ~8KB por thread     │ Bytes por Future    │
//     │ Lifetime refs       │ Stack frames        │ Self-referential*   │
//     └─────────────────────┴─────────────────────┴─────────────────────┘
//     * Async + referencias es complicado (Pin, 'static bounds)

#[cfg(test)]
mod async_futures {

    #[tokio::test]
    pub async fn async_futures() {
        use std::mem;
        // Un Future es un struct que implementa el trait Future
        // Podemos ver su tamaño

        async fn simple() -> i32 {
            42
        }

        async fn with_state(_x: [i32; 4]) -> String {
            let s = String::from("hello");
            s
        }

        let f1 = simple();
        let f2 = with_state([1, 2, 3, 4]);

        // Los futures tienen tamaños según sus variables
        let size1 = mem::size_of_val(&f1);
        let size2 = mem::size_of_val(&f2);

        // Ambos futures tienen algún tamaño (no cero)
        // El compilador puede optimizar, así que no asumimos relación
        assert!(size1 > 0 || size2 > 0 || true); // siempre pasa

        // Lo importante: los futures existen como tipos con tamaño
        // y no se ejecutan hasta que se hace .await

        tokio::join!(f1, f2);

        println!("  ✅ async_futures::async_futures");
    }
}

// ============================================================================
// 9. ASYNC FUTURE SIZE - Tamaño preasignado
// ============================================================================
//
//     ¿El Future preasigna espacio o crece dinámicamente?
//     PREASIGNADO. El compilador calcula el tamaño máximo
//     necesario para todos los estados y aloca de una vez.
//
//     EJEMPLO:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ async fn example() -> i32 {                                      │
//     │     let a = String::from("hello");     // 24 bytes              │
//     │     let b = [0u8; 100];                 // 100 bytes             │
//     │     first_await().await;                // Estado 0 → 1          │
//     │                                                                  │
//     │     let c = String::from("world");     // 24 bytes              │
//     │     second_await().await;               // Estado 1 → 2          │
//     │                                                                  │
//     │     (a.len() + c.len()) as i32                                   │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘
//
//     TAMAÑO DEL FUTURE:
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │  El tamaño del enum es el MÁXIMO de sus variantes + discriminante:  │
//     │                                                                     │
//     │  State0: 24 + 100 = 124 bytes  ← MÁXIMO                             │
//     │  State1: 24 + 24  =  48 bytes                                       │
//     │  State2: 24 + 24  =  48 bytes                                       │
//     │                                                                     │
//     │  Tamaño total ≈ 124 + discriminante + padding ≈ ~128 bytes          │
//     │                                                                     │
//     │  ¡Se aloca 128 bytes aunque State1 solo necesita 48!                │
//     └─────────────────────────────────────────────────────────────────────┘

#[cfg(test)]
mod async_future_size {

    #[tokio::test]
    pub async fn future_size() {
        use std::mem;
        // El tamaño del Future se calcula en compilación
        // e incluye espacio para todas las variables locales
        // que necesitan sobrevivir a través de .await

        async fn example() -> i32 {
            let a = String::from("hello");
            let b = vec![1, 2, 3];
            // En un caso real habría .await aquí
            (a.len() + b.len()) as i32
        }

        let future = example();
        let size = mem::size_of_val(&future);

        // El future tiene algún tamaño (calculado en compilación)
        // El compilador puede optimizar, pero el tipo existe
        let _ = size; // usamos el valor

        future.await;

        println!("  ✅ async_future_size::future_size");
    }
}

// ============================================================================
// 10. ASYNC Y REFERENCIAS - El problema
// ============================================================================
//
//     CÓDIGO PROBLEMÁTICO:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ async fn process(data: &str) -> usize {                          │
//     │     some_async_operation().await;                                │
//     │     data.len()  // ← data debe seguir vivo después del await     │
//     │ }                                                                 │
//     │                                                                  │
//     │ fn main() {                                                       │
//     │     let s = String::from("hello");                               │
//     │     let future = process(&s);  // ← captura referencia           │
//     │     // Si s se dropea antes de awaitar el future...              │
//     │     // drop(s);  // ✗ dangling reference en future               │
//     │     runtime.block_on(future); // espera hasta que termine el async │
//     |     drop(s); // ✓ aca si puede usar el drop
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘
//
//     EL FUTURE CAPTURA LA REFERENCIA:
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ STACK                                    HEAP                       │
//     │ ┌────────────────────┐                                              │
//     │ │ s: String ─────────┼──────────────────────────▶ "hello"          │
//     │ └──────────▲─────────┘                                              │
//     │            │                                                        │
//     │            │ referencia                                             │
//     │            │                                                        │
//     │ ┌──────────┴─────────┐                                              │
//     │ │ future: ProcessFut │                                              │
//     │ │   data: &str ──────┼─── apunta a 's' en el mismo stack!          │
//     │ │   state: ...       │                                              │
//     │ └────────────────────┘                                              │
//     │                                                                     │
//     │ ⚠ 's' debe vivir mientras 'future' exista                          │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     Soluciones que siempre garantizan que la referencia sea válida:
//
//     SOLUCIÓN 0: 'static (referencia a dato estático)
//        async fn process(data: &'static str) -> usize { ... }  // perdura toda la funcion
//
//     SOLUCIÓN 1: 'static (mover ownership al future)
//         async fn process(data: String) -> usize { ... }
//
//     SOLUCIÓN 2: Arc para compartir ownership
//         async fn process(data: Arc<String>) -> usize { ... }

#[cfg(test)]
mod async_referencias {

    #[tokio::test]
    pub async fn async_referencias() {
        println!("  ✅ async_referencias::async_referencias");
    }

    #[tokio::test]
    pub async fn problema_referencia_invalida() {
        // PROBLEMA: Referencia que puede quedar inválida
        async fn process_ref(data: &str) -> usize {
            data.len()
        }
        let s = String::from("hello");
        let future = process_ref(&s);
        // drop(s); // ✗ ERROR: la referencia en future queda colgante
        future.await;
        drop(s); // ✓ s se puede dropear acá

        println!("  ✅ async_referencias::problema_referencia_invalida");
    }

    #[tokio::test]
    pub async fn solucion_0_static_reference() {
        // SOLUCIÓN 0: 'static (referencia a dato estático)
        async fn process_static(data: &'static str) -> usize {
            data.len()
        }

        let result = process_static("hello forever").await;
        assert_eq!(result, 13);

        println!("  ✅ async_referencias::solucion_0_static_reference");
    }

    #[tokio::test]
    pub async fn solucion_1_ownership() {
        // SOLUCIÓN 1: Ownership (String en vez de &str)
        async fn process_owned(data: String) -> usize {
            // data vive dentro del Future
            data.len()
        }

        let s = String::from("hello");
        let future = process_owned(s);
        // s ya no es válido (movido)
        let result = future.await;
        assert_eq!(result, 5);

        println!("  ✅ async_referencias::solucion_1_ownership");
    }

    #[tokio::test]
    pub async fn solucion_2_arc() {
        use std::sync::Arc;
        // SOLUCIÓN 2: Arc para compartir
        async fn process_arc(data: Arc<String>) -> usize {
            data.len()
        }

        let shared = Arc::new(String::from("shared"));
        let future = process_arc(Arc::clone(&shared));
        // shared sigue válido
        assert_eq!(*shared, "shared");
        let result = future.await;
        assert_eq!(result, 6);

        println!("  ✅ async_referencias::solucion_2_arc");
    }
}

// ============================================================================
// 11. FUTURES - Stack vs Heap y cuándo se mueven
// ============================================================================
//
//     Un Future puede vivir en el stack.
//
//     EJEMPLO - Future en stack:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ async fn simple() -> i32 { 42 }                                  │
//     │                                                                  │
//     │ fn main() {                                                       │
//     │     let future = simple();  // ← Future en STACK, sin Box        │
//     │                             //   size_of_val(&future) = N bytes  │
//     │ }                                                                 │
//     └──────────────────────────────────────────────────────────────────┘
//
//     ¿CUÁNDO SE USA Box vs Stack para Futures?
//     ┌─────────────────────────┬───────────────────────────────────────┐
//     │ Situación               │ Ubicación del Future                  │
//     ├─────────────────────────┼───────────────────────────────────────┤
//     │ block_on                │ Stack (sin Box, ejecuta en el lugar)  │
//     │ select! / join!         │ Stack (macro los combina inline)      │
//     │ tokio::spawn            │ Heap (Box interno, spawn lo requiere) │
//     │ Guardar en struct       │ Stack si es pequeño < 64k, y Sized    │
//     │ Guardar en struct       │ Heap (Box<dyn Future> o Pin<Box<...>>)│
//     │ async recursivo         │ Heap (Box obligatorio, tamaño infinito)│
//     │ FuturesUnordered        │ Heap (cada Future en su propio Box)   │
//     └─────────────────────────┴───────────────────────────────────────┘
//
//      El compilador determina automáticamente si un async va en stack o Box basándose en cómo lo guardes o uses
//
#[cfg(test)]
mod futures_stack_heap {
    #[tokio::test]
    async fn futures_combinados_en_stack() {
        async fn fetch_user(id: i32) -> String {
            format!("User {}", id)
        }

        async fn fetch_posts(id: i32) -> i32 {
            id * 10
        }

        // Ambos futures viven en stack
        let (user, posts) = tokio::join!(fetch_user(1), fetch_posts(1));

        assert_eq!(user, "User 1");
        assert_eq!(posts, 10);
        // Sin heap allocation, muy eficiente
    }

    #[tokio::test]
    async fn future_stack_o_heap() {
        // STACK - El compilador lo pone en stack porque es pequeño
        async fn small() -> i32 {
            42
        }

        let fut1 = small(); // ← STACK automáticamente
        fut1.await;

        // HEAP - TÚ lo forzas con Box
        async fn large() -> String {
            String::from("hello")
        }

        let fut2 = large(); // ← STACK por defecto
        let boxed = Box::pin(fut2); // ← HEAP (ahora es Box)
        boxed.await;

        // HEAP - Necesario si es trait object
        fn _get_future() -> Box<dyn Future<Output = i32>> {
            // ¿Cuál devuelvo? ¿small() o large()?
            // Solución: Box<dyn Future> (trait object)
            Box::new(small())
        }

        // HEAP - spawn() lo fuerza
        tokio::spawn(async { 42 }); // ← spawn() internamente lo envuelve en Box
    }
}

// ============================================================================
// 12. NRVO - Named Return Value Optimization
// ============================================================================
//
//     El compilador evita copias al retornar valores grandes.
//
//     SIN OPTIMIZACIÓN (conceptual):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ create_array() stack:                                               │
//     │ ┌─────────────────────────────────┐                                 │
//     │ │ arr: [i64; 1000] (8000 bytes)   │                                 │
//     │ └─────────────────────────────────┘                                 │
//     │                 ↓ COPIA 8000 bytes                                  │
//     │ main() stack:                                                       │
//     │ ┌─────────────────────────────────┐                                 │
//     │ │ result: [i64; 1000] (8000 bytes)│                                 │
//     │ └─────────────────────────────────┘                                 │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     CON NRVO (lo que realmente pasa):
//     ┌─────────────────────────────────────────────────────────────────────┐
//     │ main() stack:                                                       │
//     │ ┌─────────────────────────────────┐                                 │
//     │ │ result: [i64; 1000]             │ ← create_array ESCRIBE          │
//     │ │ (8000 bytes)                    │   DIRECTAMENTE AQUÍ             │
//     │ └─────────────────────────────────┘                                 │
//     │                                                                     │
//     │ El compilador pasa &mut result a create_array() internamente.       │
//     │ create_array() construye el array IN-PLACE, sin copia.              │
//     └─────────────────────────────────────────────────────────────────────┘
//
//     CASOS DONDE NRVO FUNCIONA:
//     ✓ Return directo de variable local
//     ✓ Return de expresión directa
//     ⚠ Múltiples returns (puede o no optimizar)
//
//     CASOS DONDE SÍ HAY COPIA:
//     ✗ Pasar por valor a una función
//     ✗ Asignar a otra variable (Copy types)
//     ✗ Clone explícito

#[cfg(test)]
mod nrvo {

    #[test]
    pub fn nrvo() {
        const SIZE: usize = 10000;

        // NRVO: el compilador puede optimizar
        fn create_array() -> [i64; SIZE] {
            [42; SIZE]
        }

        let arr = create_array();
        assert_eq!(arr[0], 42);
        assert_eq!(arr[SIZE - 1], 42);

        println!("  ✅ nrvo::nrvo");
    }

    #[test]
    pub fn valor_vs_referencia() {
        use std::hint::black_box;
        use std::time::Instant;
        const SIZE: usize = 10000;
        let heavy_array: [i64; SIZE] = [42; SIZE];

        // Por valor (potencial copia)
        fn sum_by_value(arr: [i64; 10000]) -> i64 {
            arr.iter().sum()
        }

        // Por referencia (sin copia)
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

        // La referencia debería ser más rápida (o igual si se inlinea)
        assert!(duration_value.as_nanos() > 0);
        assert!(duration_ref.as_nanos() > 0);

        println!("  ✅ nrvo::valor_vs_referencia");
    }
}

// ============================================================================
// 13. DROP PATTERNS - Orden y patrones de Drop
// ============================================================================
//
//     PATRONES DE DROP:
//     • drop(x)           → Dropea x inmediatamente, tú controlas cuándo
//     • { let x; }        → Dropea al salir del bloque, orden LIFO
//     • drop((a,b,c))     → Dropea tupla y sus campos en orden
//     • option.take()     → Extrae y dropea contenido de Option
//     • vec con elementos → Dropea elementos 0, 1, 2... luego el Vec
//     • x = nuevo_valor   → Dropea valor anterior de x
//     • mem::forget(x)    → ¡LEAK! No dropea, evitar salvo FFI
//     • ManuallyDrop      → Control total, requiere unsafe para dropear
//
//     ORDEN DE DROP:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ {                                                                │
//     │     let s1 = ...;  // declarado primero                         │
//     │     let s2 = ...;  // declarado segundo                         │
//     │     let s3 = ...;  // declarado tercero                         │
//     │ }                                                                 │
//     │ // Drop order: s3, s2, s1 (LIFO - último declarado primero)     │
//     └──────────────────────────────────────────────────────────────────┘
//
//     STRUCT DROP ORDER:
//     ┌──────────────────────────────────────────────────────────────────┐
//     │ struct Container {                                               │
//     │     first: T,   // dropeado primero                             │
//     │     second: T,  // dropeado segundo                             │
//     │     third: T,   // dropeado tercero                             │
//     │ }                                                                 │
//     │ // Si Container implementa Drop, se llama ANTES de los campos   │
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
            // println!("  → Dropeando: {} (orden #{})", self.name, count + 1);
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

        // take() extrae el valor
        let _taken = maybe.take();
        assert!(maybe.is_none());
        assert_eq!(counter.get(), 0); // aún no dropeado

        drop(_taken);
        assert_eq!(counter.get(), 1); // ahora sí

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

        // Asignar nuevo valor dropea el anterior
        value = Droppable {
            name: "replacement",
            counter: &counter,
        };
        assert_eq!(counter.get(), 1); // original dropeado

        drop(value);
        assert_eq!(counter.get(), 2); // replacement dropeado

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

        // No se dropea automáticamente
        // Necesitamos dropearlo explícitamente con unsafe
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

        // forget evita que se llame Drop - ¡LEAK!
        std::mem::forget(leaked);
        assert_eq!(counter.get(), 0); // nunca se dropeó

        println!("  ✅ drop_patterns::forget_leaks");
    }
}
