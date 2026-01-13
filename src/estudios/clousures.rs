/*
=========================================================================
RESUMEN
=========================================================================

    CLOSURE = Función anónima que puede CAPTURAR su entorno

    Internamente un closure es un struct donde sus campos son las variables capturadas e implementa uno o mas de los traits Fn|FnMut|FnOnce(Args...) -> Output

    let x = String::from("hola");
    let callback = |y: i32| -> String { format!("{} {}", x, y) }

    es conceptualmente:
    struct __AnonymousClosure {
        x: &String,
    }
    impl Fn(i32) -> String for __AnonymousClosure {
        fn call(&self, y: i32) -> String {
            format!("{} {}", *self.x, y)
        }
    }
    al implementar Fn el compilador tambien implementa FnMut y FnOnce
    Traits implementados:
        Fn(i32) -> String
        FnMut(i32) -> String
        FnOnce(i32) -> String

    Clasificacion:
        Fn:   Si solo LEE lo capturado
        FnMut: Si MODIFICA lo capturado
        FnOnce: Si CONSUME lo capturado

    Herencia Traits:
        Fn : FnMut : FnOnce

    Familia de Traits:
        Cada closure implementa uno o mas traits segun su firma:
            Fn(Args...) -> Output
            FnMut(Args...) -> Output
            FnOnce(Args...) -> Output
        es lo que permite usarlos como genericos, en vez de tipo concreto.

    Move:
        - Sin move: captura por referencia (& o &mut) según uso
        - Con move: captura ownership de todo lo capturado
        - move implicito: cuando se consume algo dentro del closure, el compilador aplica move automáticamente.

    Impl y Dyn:
        - impl Fn... : monomorfizmo: parametros y return unicos de funciones
        - Box<dyn Fn...> : colecciones heterogéneas del mismo trait: para funciones que retornan diferentes tipos de closures con mismo trait.
*/

/*
========================================================================
CLOSURES Y CALLBACKS EN RUST
========================================================================

    CONCEPTO CLAVE: Un closure es un STRUCT ANÓNIMO generado por el
    compilador, cuyos campos son las variables capturadas, y que
    implementa Fn, FnMut, y/o FnOnce según cómo use esas capturas.

    TÚ ESCRIBES:

      let factor = 10;
      let scale = |x| x * factor;

    EL COMPILADOR GENERA (conceptualmente):

      struct __AnonymousClosure {
          factor: &i32,         // campo = variable capturada
      }

      impl Fn(i32) -> i32 for __AnonymousClosure {
          fn call(&self, x: i32) -> i32 {
              x * (*self.factor)
          }
      }


    ANALOGÍA CON REFERENCIAS & , &mut , T:
    --------------------------------------------
        Fn ~ &T:
            • Puede ser llamado múltiples veces
            • No muta ni consume lo capturado

        FnMut ~ &mut T:
            • Puede ser llamado múltiples veces
            • Muta lo capturado
            • Requiere acceso exclusivo (como &mut closure) por una variable

        FnOnce ~ T (ownership):
            • Cuando captura con move el ownership de una variable
            • Puede ser Fn, FnMut, u FnOnce dependiendo del uso
            • Fn: si solo lee
            • FnMut: si muta
            • FnOnce: si consume/retorna la variable capturada
              - solo este caso se puede ejecutar una sola vez

// @test

    HERENCIA DE TRAITS:
    --------------------------------------------
        Fn : FnMut : FnOnce
        Fn hereda de FnMut, y FnMut hereda de FnOnce.

        trait FnOnce<Args> {
            type Output;
            fn call_once(self, args: Args) -> Self::Output;
        }

        trait FnMut<Args>: FnOnce<Args> {
            fn call_mut(&mut self, args: Args) -> Self::Output;
        }

        trait Fn<Args>: FnMut<Args> {
            fn call(&self, args: Args) -> Self::Output;
        }

        Entonces Fn se puede ver conceptualmente como:

        trait Fn<Args> {
            fn call(&self, args: Args) -> Self::Output {
                // implementacion closure
            }
            fn call_mut(&mut self, args: Args) -> Self::Output {
                self.call(args)
            }
            fn call_once(self, args: Args) -> Self::Output {
                self.call_mut(&mut self, args)
            }
        }

        Es decir:
          - Todo Fn es también FnMut y FnOnce.
          - Todo FnMut es también FnOnce.

        Si un closure puede ser llamado muchas veces sin mutar ni consumir nada (Fn),
        entonces también puede ser usado donde se permite mutar (FnMut) o consumir (FnOnce),
        porque no rompe ninguna regla de los casos más generales.

        Entonces, todo closure es al menos un FnOnce.


    Traits de CLOSURES: FAMILIAS
    --------------------------------------------
        Cada familia queda determinada por dos componentes:
            * Fn / FnMut / FnOnce
            * Firma: (Args...), Output
        Ejemplos (todos son traits DIFERENTES):
            Fn()              =  Fn<(), Output=()>
            FnMut()           =  FnMut<(), Output=()>
            Fn(i32) -> i32    =  Fn<(i32,), Output=i32>
            Fn(i32, i32) -> i32 = Fn<(i32, i32), Output=i32>
            Fn(String) -> bool = Fn<(String,), Output=bool>

        ¡No puedes mezclar Fn(i32)->i32 con Fn(String)->bool!
        "Fn" sin firma NO es un trait válido. Siempre necesitas:
            Fn<Args, Output>

// @test

    THREADS SAFE: FnOnce + Send + 'static
    --------------------------------------------
        Para que una variable tipo T pueda ser enviado a un closure de un thread,
        debe ser Send + 'static:
        - Send: para que pueda ser movido entre threads
        - 'static: para que viva toda la vida del thread (Ownership, Copy, &str,Arc, const, static)

        Por eso se usa con move, para transferir ownership al thread, salvo las referencias 'static.

        Variables tipo closure adentro de threads:
        Todo closure que es Send + 'static también es FnOnce + Send + 'static.
        Mismos requisitos para usar en threads.

// @test

    ASYNC CLOSURES:
    --------------------------------------------
        F: FnOnce + 'static

        Si un async closure es !Send -> se ejecuta en el mismo thread.
        Si un async closure es Send -> puede ser enviado a otro thread.

// @test

    CLOSURES TRAITS: IMPL Y DYN
    --------------------------------------------
        Si tipo implementa el trait de Fn, FnMut, FnOnce en su declaracion
        como son traits, entonces o bien son genéricos (impl Fn...) o bien son trait objects (Box<dyn Fn...>)

// @test

    CLOSURES NO VARIADICS, TIPOS FIJOS
    --------------------------------------------
        Los closures en Rust NO SON VARIADICS:
        - Siempre tienen una firma fija de argumentos y tipo de retorno
        - No pueden tener argumentos opcionales ni por defecto
        - No pueden tener un número variable de argumentos (variadics)

        A diferencia de C++, Rust prioriza la seguridad de tipos y memoria.
        Si se quiere pasar un número variable de argumentos, se puede usar slices (&[T]),
        tuplas, vectores (Vec<T>), o usar macros.
*/

#[test]
fn indice() {}

/*
========================================================================
QUE_ES_UN_CLOSURE
========================================================================

    ¿QUÉ ES UN CLOSURE?
    --------------------------------------------
        CLOSURE = Función anónima que puede CAPTURAR su entorno

        DIAGRAMA:
        ┌─────────────────────────────────────────────────────────────────────────┐
        │  FUNCIÓN NORMAL:                     CLOSURE:                           │
        │  fn add(a: i32, b: i32) -> i32       let add = |a, b| a + b;            │
        │      a + b                                                              │
        │                                                                         │
        │  LA DIFERENCIA CLAVE: Captura del entorno                               │
        │                                                                         │
        │      let factor = 10;                                                   │
        │          │                                                              │
        │          ▼                                                              │
        │      ┌───────┐                                                          │
        │      │  10   │  ◄── variable en scope                                   │
        │      └───────┘                                                          │
        │          ▲                                                              │
        │          │ captura automática                                           │
        │      ┌───┴─────────────────┐                                            │
        │      │ let scale = |x| x * factor;                                      │
        │      │             ▲                                                    │
        │      │             └── usa factor sin pasarlo como parámetro            │
        │      └─────────────────────┘                                            │
        │                                                                         │
        │      scale(5)  →  50                                                    │
        └─────────────────────────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod que_es_un_closure {
    #[test]
    pub fn que_es_un_closure() {
        let add = |a, b| a + b;
        assert_eq!(add(2, 3), 5);

        let factor = 10;
        let scale = |x| x * factor;
        assert_eq!(scale(5), 50);
    }
}

/*
========================================================================
CAPTURA_VS_PARAMETRO
========================================================================

    CAPTURA vs PARÁMETRO - ¡Son cosas diferentes!
    --------------------------------------------
        DIAGRAMA:
        ┌─────────────────────────────────────────────────────────────────────────┐
        │   CAPTURA (del entorno):                                                │
        │   ┌─────────────────────────────────────────────────────────────────┐   │
        │   │  let multiplier = 2;                                            │   │
        │   │  let f = |x| x * multiplier;                                    │   │
        │   │              ▲       ▲                                          │   │
        │   │              │       └── CAPTURADO (viene del entorno)          │   │
        │   │              └────────── PARÁMETRO (pasado al llamar)           │   │
        │   └─────────────────────────────────────────────────────────────────┘   │
        │                                                                         │
        │   PARÁMETRO SOLAMENTE (sin captura):                                    │
        │   ┌─────────────────────────────────────────────────────────────────┐   │
        │   │  let add = |a, b| a + b;   ← No captura nada                    │   │
        │   │                                                                 │   │
        │   │  // ¡Este closure es básicamente una función!                   │   │
        │   │  // Puede coercerse a: fn(i32, i32) -> i32                      │   │
        │   └─────────────────────────────────────────────────────────────────┘   │
        └─────────────────────────────────────────────────────────────────────────┘
*/
#[cfg(test)]
mod captura_vs_parametro {
    #[test]
    pub fn captura_vs_parametro() {
        let multiplier = 2;
        let multiply = |x| x * multiplier;
        assert_eq!(multiply(5), 10);

        let add = |a: i32, b: i32| a + b;
        assert_eq!(add(3, 4), 7);
    }
}

/*
========================================================================
FN_TRAIT
========================================================================

    Fn - Captura por REFERENCIA INMUTABLE
    --------------------------------------------
    No altera ni consume lo capturado
*/
#[cfg(test)]
mod fn_trait {
    #[test]
    pub fn fn_trait() {
        let value = 10;

        let print_value = || {
            assert_eq!(value, 10);
        };

        print_value();
        print_value();
        assert_eq!(value, 10);
    }
}

/*
========================================================================
FN_MUT_TRAIT
========================================================================

    FnMut - Captura por REFERENCIA MUTABLE
    --------------------------------------------
    Permite modificar lo capturado

*/
#[cfg(test)]
mod fn_mut_trait {
    #[test]
    pub fn fn_mut_trait() {
        let mut counter = 0;

        let mut increment = || {
            counter += 1;
            counter
        };

        assert_eq!(increment(), 1);
        assert_eq!(increment(), 2);
        assert_eq!(increment(), 3);
        assert_eq!(counter, 3);
    }
}

/*
========================================================================
FN_ONCE_TRAIT
========================================================================

    FnOnce - CONSUME el valor capturado
    --------------------------------------------
    Solo puede ser llamado una vez
*/
#[cfg(test)]
mod fn_once_trait {
    #[test]
    pub fn fn_once_trait() {
        let data = vec![1, 2, 3];

        let consume = || {
            assert_eq!(data, vec![1, 2, 3]);
            drop(data);
        };

        consume();
        // consume(); // ✗ ERROR: no se puede llamar dos veces
    }
}

/*
========================================================================
MOVE
========================================================================

    SIN move: siempre captura por referencia (&T o &mut T)
              el compilador decide que es necesario según uso.
              se crea una nueva variable referenciando la original.
    CON move: siempre ownership de TODO lo capturado

    DIAGRAMA:
    ┌─────────────────────────────────────────────────────────────────────────┐
    │   ┌─────────────────────────────────────────────────────────────────┐   │
    │   │  let name = String::from("Alice");                              │   │
    │   │  let age = 30;  // i32 es Copy                                  │   │
    │   │                                                                 │   │
    │   │  SIN move:                    CON move:                         │   │
    │   │  let f = || name.len();       let f = move || name.len();       │   │
    │   │           ▲                            ▲                        │   │
    │   │           └─ captura &name             └─ MUEVE name al closure │   │
    │   │                                                                 │   │
    │   │  println!(name); ✓ OK         println!(name); ✗ ERROR           │   │
    │   └─────────────────────────────────────────────────────────────────┘   │
    │                                                                         │
    │   NOTA: Para tipos Copy (i32, bool, etc), move hace una COPIA           │
    │         Para tipos no-Copy (String, Vec), move TRANSFIERE ownership     │
    └─────────────────────────────────────────────────────────────────────────┘

    Move y referencias:
    --------------------------------------------
        A fines practicos es identico a usar sin move para referencias.
        Las referencias son Copy, por lo que crea otra referencia dentro del closure,
        dejando la variable original intacta.
        El lifetime de la referencia capturada es el mismo que el de la variable original.

    Traits Fn, FnMut, FnOnce y move
    --------------------------------------------

    Move no determina el trait que implementa el closure.
    El trait depende de que hace el closure con sus capturas.
        - Si SOLO LEE lo capturado → Fn
        - Si MODIFICA lo capturado → FnMut
        - Si CONSUME lo capturado → FnOnce

    move y 'static
    --------------------------------------------
        Si todo lo capturado con move es 'static entonces el closure es 'static
        y vive todo lo que vive la variable que lo contiene.

        Util en :
            * Threads / Async
            * Return de funciones / closures
            * Almacenamiento en estructuras de datos de larga vida

    move implicito
    --------------------------------------------
        hay casos donde el compilador aplica move implícito:
            * Al detectar un drop / consumo dentro del closure
            * Al usar closures en threads
            * Al retornar closures desde funciones

        Ejemplo:
        let x = String::from("hola");
        let c = || { drop (x); }
        let c = move || { drop(x); } // el compilador lo convierte en move
*/
#[cfg(test)]
mod move_fn_fnMut_fnOnce {

    #[test]
    pub fn no_move_fn() {
        let data = vec![1, 2, 3];
        let print_len = || data.len();
        assert_eq!(print_len(), 3);
        assert_eq!(print_len(), 3);
    }

    #[test]
    pub fn no_move_fnMut() {
        let mut data = vec![1, 2, 3];
        let mut add_value = |v| {
            data.push(v);
            return data.len();
        };
        add_value(4);
        let value = add_value(5);
        assert_eq!(value, 5);
    }

    #[test]
    pub fn move_closure_fn() {
        let data = vec![1, 2, 3];
        let print_len = move || data.len();
        assert_eq!(print_len(), 3);
        assert_eq!(print_len(), 3);
    }

    #[test]
    pub fn move_closure_fnMut() {
        let mut data = vec![1, 2, 3];
        let mut add_value = move |v| {
            data.push(v);
            return data.len();
        };
        add_value(4);
        let value = add_value(5);
        assert_eq!(value, 5);
    }

    #[test]
    pub fn move_closure_fnOnce() {
        let data = vec![1, 2, 3];
        let consume_data = move || {
            drop(data);
        };
        consume_data();
        // consume_data(); // ✗ ERROR: no se puede llamar dos veces
    }

    #[test]
    pub fn move_thread() {
        let data = vec![1, 2, 3];
        let handle = std::thread::spawn(move || {
            assert_eq!(data, vec![1, 2, 3]);
        });
        handle.join().unwrap();
    }

    #[test]
    pub fn move_return_closure() {
        fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
            move |x| x * factor
        }
        let times_5 = make_multiplier(5);
        assert_eq!(times_5(10), 50);
    }

    #[test]
    pub fn move_copy_type() {
        let number = 42;
        let print_num = move || {
            assert_eq!(number, 42);
        };
        print_num();
        assert_eq!(number, 42);
    }
}

#[cfg(test)]
mod move_references {
    #[test]
    pub fn con_move_references() {
        let text = String::from("Hello, world!");
        let text_ref = &text;
        let print_text = move || {
            // text_ref es una referencia copiada
            assert_eq!(text_ref, "Hello, world!");
        };
        print_text();
    }

    #[test]
    pub fn sin_move_references() {
        let text = String::from("Hello, world!");
        let text_ref = &text;
        let print_text = || {
            // text_ref es una referencia copiada
            assert_eq!(text_ref, "Hello, world!");
        };
        print_text();
    }
}

#[cfg(test)]
mod move_implicito {
    #[test]
    pub fn move_implicito_consumo() {
        let message = String::from("Goodbye!");
        let consume_message = || {
            drop(message); // El compilador aplica move implícito aquí
        };
        consume_message();
        // consume_message(); // ✗ ERROR: no se puede llamar dos veces
    }
}

/*
========================================================================
IMPL Y DYN DE FAMILIA DE TRAITS
========================================================================

Cada closure es un tipo único, porque cada tipo incluye las capturas propias. cada uno tiene su tamaño propio.

Cada closure implementa uno o mas traits segun su firma:
    Fn(Args...) -> Output
    FnMut(Args...) -> Output
    FnOnce(Args...) -> Output

es lo que permite usarlos como genericos, en vez de tipo concreto.

Impl: (Monomorfización)
--------------------------------------------
impl Fn(Args) -> Output

fn fn_param_impl<F: Fn(i32) -> i32>(callback: F) { callback(3); }

fn fn_return_impl() -> impl Fn(i32) -> i32 { return |x: i32| x + 1; } // return unico closure

Dyn: (Para colecciones heterogéneas)
--------------------------------------------
Box<dyn Fn(Args) -> Output>

fn fn_param_dyn(callback: Box<dyn Fn(i32) -> i32>) { callback(3); }

fn fn_return_dyn() -> Box<dyn Fn(i32) -> i32> {
    if true {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x * 2)
    }
}

Puedes mezclar closures diferentes
Útil para guardar en Vec<Box<dyn Fn...>>
*/
#[cfg(test)]
mod impl_dyn {

    #[test]
    pub fn impl_param() {
        fn fn_param_impl<F: Fn(i32) -> i32>(callback: F) {
            assert_eq!(callback(3), 6);
        }
        let callback = |x: i32| x * 2;
        fn_param_impl(callback);
    }

    #[test]
    pub fn impl_return() {
        fn fn_return_impl() -> impl Fn(i32) -> i32 {
            |x: i32| x + 1
        }
        let callback = fn_return_impl();
        assert_eq!(callback(5), 6);
    }

    #[test]
    pub fn dyn_param() {
        fn fn_param_dyn(callback: Box<dyn Fn(i32) -> i32>) {
            assert_eq!(callback(3), 6);
        }
        let callback = |x: i32| x * 2;
        fn_param_dyn(Box::new(callback));
    }

    #[test]
    pub fn dyn_return() {
        fn fn_return_dyn() -> Box<dyn Fn(i32) -> i32> {
            if true {
                Box::new(|x| x + 1)
            } else {
                Box::new(|x| x * 2)
            }
        }
        let callback = fn_return_dyn();
        assert_eq!(callback(5), 6);
    }

    #[test]
    pub fn dyn_collection() {
        let mut callbacks: Vec<Box<dyn Fn(i32) -> i32>> = Vec::new();
        callbacks.push(Box::new(|x| x + 1));
        callbacks.push(Box::new(|x| x * 2));

        assert_eq!(callbacks[0](3), 4);
        assert_eq!(callbacks[1](3), 6);
    }

    #[test]
    pub fn dyn_storage() {
        struct CallbackStorage {
            callback: Box<dyn Fn(i32) -> i32>,
        }

        let storage = CallbackStorage {
            callback: Box::new(|x| x + 10),
        };

        assert_eq!((storage.callback)(5), 15);
    }
}

#[cfg(test)]
mod patterns {

    /*
        CLOSURE FACTORIES
        --------------------------------------------
        Funciones que retornan closures.
        - impl Fn: Retorno estático (un solo tipo de closure).
        - Box<dyn Fn>: Retorno dinámico (permite lógica condicional para elegir el closure).
    */
    mod closure_factories {
        #[test]
        pub fn closure_factories() {
            // Factory estática (impl Fn)
            fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
                move |y| x + y
            }
            let add_5 = create_adder(5);
            assert_eq!(add_5(10), 15);

            // Factory dinámica (Box<dyn Fn>)
            fn create_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
                match op {
                    "add" => Box::new(|a, b| a + b),
                    "mul" => Box::new(|a, b| a * b),
                    _ => Box::new(|_, _| 0),
                }
            }
            let op = create_operation("mul");
            assert_eq!(op(3, 4), 12);
        }
    }

    /*
        STRATEGY PATTERN
        --------------------------------------------
        Permite inyectar comportamiento (algoritmos) en un struct.
        El struct define la interfaz (el trait bound) y el closure provee la implementación.
    */
    mod strategy_pattern {
        struct Validator<F>
        where
            F: Fn(&str) -> bool,
        {
            validate: F,
        }

        impl<F> Validator<F>
        where
            F: Fn(&str) -> bool,
        {
            fn new(validate: F) -> Self {
                Self { validate }
            }

            fn is_valid(&self, s: &str) -> bool {
                (self.validate)(s)
            }
        }

        #[test]
        pub fn strategy_pattern() {
            println!("  ✅ strategy_pattern::strategy_pattern");

            let numeric_validator = Validator::new(|s| s.chars().all(|c| c.is_numeric()));
            let length_validator = Validator::new(|s| s.len() > 5);

            assert!(numeric_validator.is_valid("12345"));
            assert!(!numeric_validator.is_valid("123a5"));
            assert!(length_validator.is_valid("123456"));
        }
    }

    /*
        MIDDLEWARE / DECORATORS
        --------------------------------------------
        Envolver un closure con otro para añadir funcionalidad transversal
        (logging, métricas, control de errores) sin modificar la lógica original.
    */
    // Middleware que añade logging a cualquier función/closure
    mod middleware_pattern {
        fn with_logging<F, T, R>(func: F) -> impl Fn(T) -> R
        where
            F: Fn(T) -> R,
        {
            move |arg| {
                println!("LOG: Llamando con argumento...");
                let result = func(arg);
                println!("LOG: Llamada finalizada.");
                result
            }
        }

        #[test]
        pub fn middleware_pattern() {
            println!("  ✅ middleware_pattern::middleware_pattern");

            let square = |x: i32| x * x;
            let square_with_log = with_logging(square);

            assert_eq!(square_with_log(5), 25);
        }
    }

    /*
        LAZY EVALUATION (Thunks)
        --------------------------------------------
        Diferir la ejecución de un código costoso hasta que el resultado sea necesario.
        Se usa un closure como "receta" para generar el valor bajo demanda.
    */
    mod lazy_evaluation {
        struct Lazy<T, F>
        where
            F: Fn() -> T,
        {
            value: Option<T>,
            initializer: F,
        }

        impl<T, F> Lazy<T, F>
        where
            F: Fn() -> T,
        {
            fn new(initializer: F) -> Self {
                Self {
                    value: None,
                    initializer,
                }
            }

            fn get(&mut self) -> &T {
                if self.value.is_none() {
                    self.value = Some((self.initializer)());
                }
                self.value.as_ref().unwrap()
            }
        }

        #[test]
        pub fn lazy_evaluation() {
            println!("  ✅ lazy_evaluation::lazy_evaluation");

            let mut expensive_value = Lazy::new(|| {
                println!("Calculando valor costoso...");
                42
            });

            // El primer acceso dispara el cálculo
            assert_eq!(*expensive_value.get(), 42);
            // El segundo acceso usa el valor cacheado
            assert_eq!(*expensive_value.get(), 42);
        }
    }

    /*
        EVENT LISTENERS / OBSERVER PATTERN
        --------------------------------------------
        Almacenar múltiples callbacks para ser ejecutados ante un evento.
        Requiere `Box<dyn FnMut>` para permitir que los listeners modifiquen su propio estado.
    */
    mod event_listeners {
        struct EventEmitter {
            listeners: Vec<Box<dyn FnMut(&str)>>,
        }

        impl EventEmitter {
            fn new() -> Self {
                Self {
                    listeners: Vec::new(),
                }
            }

            fn on<F>(&mut self, callback: F)
            where
                F: FnMut(&str) + 'static,
            {
                self.listeners.push(Box::new(callback));
            }

            fn emit(&mut self, event: &str) {
                for listener in self.listeners.iter_mut() {
                    listener(event);
                }
            }
        }

        #[test]
        pub fn event_listeners() {
            println!("  ✅ event_listeners::event_listeners");
            let mut emitter = EventEmitter::new();
            let mut count = 0;

            emitter.on(move |ev| println!("Evento recibido: {}", ev));
            emitter.on(move |_| count += 1); // Captura y modifica estado

            emitter.emit("click");
            emitter.emit("hover");
        }
    }

    /*
        STATEFUL CLOSURES (Encapsulación de estado)
        --------------------------------------------
        Usar un closure para crear un objeto con estado privado sin definir un struct.
    */
    mod stateful_closures {
        fn create_counter(start: i32) -> impl FnMut() -> i32 {
            let mut count = start;
            move || {
                count += 1;
                count
            }
        }

        #[test]
        pub fn stateful_closures() {
            println!("  ✅ stateful_closures::stateful_closures");

            let mut counter_a = create_counter(0);
            let mut counter_b = create_counter(100);

            assert_eq!(counter_a(), 1);
            assert_eq!(counter_a(), 2);
            assert_eq!(counter_b(), 101);
            assert_eq!(counter_a(), 3);
        }
    }

    /*
        CURRYING / PARTIAL APPLICATION
        --------------------------------------------
        Transformar una función de N argumentos en una cadena de funciones de 1 argumento.
    */
    mod currying_partial_application {
        #[test]
        pub fn currying_partial_application() {
            println!("  ✅ currying_partial_application::currying_partial_application");

            // Función currificada
            let add = |x| move |y| x + y;

            let add_ten = add(10); // Aplicación parcial
            let result = add_ten(5);

            assert_eq!(result, 15);
            assert_eq!(add(20)(30), 50);
        }
    }
}
