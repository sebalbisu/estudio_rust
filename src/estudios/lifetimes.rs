#[allow(dead_code)]
#[allow(unused_variables)]
#[test]
fn indice() {
    intro::basico();

    cuando_no_necesitas::print_both();
    cuando_no_necesitas::join_strings();
    cuando_no_necesitas::append_suffix();

    por_que_explicitos::por_que_explicitos();

    en_structs::en_structs();

    elision_rules::regla_1_multi_params();
    elision_rules::regla_2_single_input();
    elision_rules::regla_3_self();
    elision_rules::elision_fails();

    static_lifetime::static_intro();
    static_lifetime::static_literals();
    static_lifetime::const_inline();
    static_lifetime::static_basic();
    static_lifetime::static_return_test();
    static_lifetime::static_mutable();

    static_lifetime::static_box_leak();
    static_lifetime::leak_mut();
    static_lifetime::static_trait_bounds();

    placeholder::placeholder_intro();
    placeholder::placeholder_obvious();
    placeholder::placeholder_impl_blocks();
    placeholder::placeholder_nested_types();
    placeholder::placeholder_ambiguity();
}

/*
========================================================================
INTRO: ¿QUÉ SON LOS LIFETIMES?
========================================================================

    CONCEPTO CLAVE:
        Lifetimes SIEMPRE existen para cada referencia (&T).
        El compilador los infiere automáticamente en la mayoría de
        casos. Solo los escribes cuando hay ambigüedad.

    TODA REFERENCIA tiene un LIFETIME asociado:
        let value: i32 = 123;        // value vive en este scope
        let ref_value: &i32 = &value; // ref tiene lifetime implícito
        Internamente el compilador ve algo como:
        let ref_value: &'a i32 = &value;   // 'a = lifetime de value

    'static
        El lifetime de T es 'static, no depende de datos locales, tiene la capacidad de vivir todo el programa
            * Si son referencias 'static, viven mientras viva la referencia, independientemente de cualquier variable.
            * Los tipos Owner (tipos que no son referencias), se puede decir que tienen el lifetime 'static implícito

    'a
        lifetime condicionado por el lifetime 'static del owner.

    '_
        lifetime anónimo (placeholder) que el compilador infiere automáticamente.

    REGLA FUNDAMENTAL:
        El lifetime de la referencia DEBE ser ≤ que el del dueño
        (la referencia no puede vivir más que el objeto referenciado)
        value:     ├──────────────────────┤  (vive líneas 1-10)
        ref_value: │     ├────────┤       │  (vive líneas 3-7)
                         └────────┘  ✓ OK: ref dentro del dueño

    ANALOGÍA CON TIPOS:
        let x = 5;           // tipo i32 inferido
        let x: i32 = 5;      // tipo explícito (redundante)
        fn foo(s: &str)              // lifetime inferido
        fn foo<'a>(s: &'a str)       // lifetime explícito (redundante)

    FIRMAS CON LIFETIMES:
        En las firmas de funciones, structs, impls, metodos, van los lifetimes si hay
        referencias involucradas y tipos los genericos.
        Van en la firma para que el compilador sepa cómo relacionar las duraciones
        de las referencias usadas.
        Si se se omiten en la firma porque son evidentes, se usa elision = syntax suggar,
        internamente se agregan igual a la firma.

        fn foo<'a>(s: &'a str)
        struct Parser<'a> { ...; input: &'a str }
        impl<'a> Parser<'a> { ... }
*/

#[cfg(test)]
mod intro {
    #[test]
    pub fn basico() {
        {
            // Un lifetime es "cuánto tiempo vive una referencia"
            let owner = String::from("hola");
            let referencia = &owner;
            assert!(referencia.is_empty() == false);
        }
        // línea 5: owner muere, referencia ya no puede usarse
    }
}

/*
========================================================================
CUÁNDO NO NECESITAS LIFETIMES EXPLÍCITOS
========================================================================

    REGLA CLAVE:
    --------------------------------------------
        Si la función NO retorna referencia, NO necesitas lifetimes explícitos.

        ¿Por qué? Los lifetimes relacionan la duración del OUTPUT con la duración
        de los INPUTS. Sin output de referencia → nada que relacionar.

    COMPARA: CON vs SIN LIFETIMES
    --------------------------------------------

        SIN lifetimes (retorna owned/void/primitivo):
          fn process(a: &str, b: &str)                   // void
          fn process(a: &str, b: &str) -> String         // owned
          fn process(a: &str, b: &str) -> usize          // Copy

        CON lifetimes (retorna referencia):
          fn process<'a>(a: &'a str, b: &str) -> &'a str // ref!
          fn process<'a>(a: &'a str) -> &'a [u8]         // ref!

        Elision: con 1 input, no escribes 'a)
          fn process(a: &str) -> &str              // ref! elision

    CASOS ESPECIALES:
    --------------------------------------------

        STRUCTS que contienen &T siempre necesitan 'a:

          struct Parser<'a> {
              input: &'a str,  // El struct "borrow" el string
          }

          impl<'a> Parser<'a> {
              fn new(input: &'a str) -> Self {
                  Parser { input }  // retorna struct con ref
              }

              fn len(&self) -> usize {
                  self.input.len()  // ✓ NO lifetime - retorna usize
              }

              // Elision en metodos: referencias sin lifetimes en returns
              // usan el lifetime del &'a self
              fn input(&self) -> &str {
                  self.input
              }
          }
*/

#[cfg(test)]
mod cuando_no_necesitas {

    // Sin retorno
    #[test]
    pub fn print_both() {
        fn print_both(a: &str, b: &str) {
            // ✓ NO necesita lifetimes - no retorna nada
            println!("{} {}", a, b);
        }
        print_both("hola", "mundo");
    }

    // Retorna valor owned
    #[test]
    pub fn join_strings() {
        fn join_strings(a: &str, b: &str) -> String {
            // ✓ NO necesita lifetimes - retorna String (owned)
            format!("{} {}", a, b)
        }
        let joined = join_strings("hello", "world");
        assert_eq!(joined, "hello world");
    }

    // Modifica in-place
    #[test]
    pub fn append_suffix() {
        fn append_suffix(s: &mut String, suffix: &str) {
            // ✓ NO necesita lifetimes - no retorna referencia
            s.push_str(suffix);
        }
        let mut text = String::from("Hello");
        append_suffix(&mut text, "!");
        assert_eq!(text, "Hello!");
    }
}

/*
========================================================================
POR_QUE_EXPLICITOS
========================================================================

    Cuando hay DOS referencias de entrada y un output de ellas → AMBIGÜEDAD
*/

#[cfg(test)]
mod por_que_explicitos {
    #[test]
    pub fn por_que_explicitos() {
        // Caso 1: UNA entrada → compilador infiere
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        // El compilador sabe: output vive tanto como input

        // Caso 2: DOS entradas → AMBIGÜEDAD
        // fn longest(x: &str, y: &str) -> &str { ... }
        // ERROR: ¿el resultado vive como x? ¿o como y?

        // Solución: especificar con 'a
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        let s1 = String::from("corta");
        let s2 = String::from("mas larga");
        println!("  Primera palabra: {}", first_word(&s1));
        println!("  La más larga: {}", longest(&s1, &s2));

        assert_eq!(first_word(&s1), "corta");
        assert_eq!(longest(&s1, &s2), "mas larga");
    }
}

/*
========================================================================
LIFETIMES EN STRUCTS
========================================================================

    STRUCT CON REFERENCIA:
    --------------------------------------------
        Si un struct contiene referencias, DEBE tener lifetime.

        let texto = String::from("Hola mundo");
            │
            ▼
        ┌─────────┐     ┌────────────────┐
        │ texto   │────▶│ "Hola mundo"   │  (heap)
        │ └───────┘     └────────────────┘
                               ▲
                               │
        ┌─────────────────┐    │
        │ Excerpt<'a>     │    │
        │   part: &'a str─┼────┘  (apunta a texto)
        └─────────────────┘

        → Excerpt NO puede vivir más que 'texto'
*/

#[cfg(test)]
mod en_structs {
    #[test]
    pub fn en_structs() {
        #[derive(Debug)]
        struct Excerpt<'a> {
            part: &'a str,
        }

        //  impl<'a> = "Voy a usar un lifetime llamado 'a"
        //      ↑      (como si fuera una variable)
        //
        //  Excerpt<'a> = "Para todos los Excerpt que tengan lifetime 'a"
        //            ↑   (le paso el valor 'a)
        impl<'a> Excerpt<'a> {
            fn get_part(&self) -> &str {
                self.part
            }
        }

        let texto = String::from("Hola mundo cruel");
        let excerpt = Excerpt { part: &texto[0..4] };
        println!("  Excerpt: {:?}", excerpt.get_part());
        assert_eq!(excerpt.get_part(), "Hola");
    }
}

/*
========================================================================
ELISION_RULES: (INFERENCIA AUTOMÁTICA)
========================================================================

    "ELISION" = El compilador OMITE/INFIERE lifetimes por ti.

    Cuando escribes:    fn foo(s: &str) -> &str
    El compilador ve:   fn foo<'a>(s: &'a str) -> &'a str

    ¡No necesitas escribir 'a explícitamente!
*/

#[cfg(test)]
mod elision_rules {

    /*
    REGLA 1: Cada parámetro recibe su propio lifetime
    --------------------------------------------
        Escribes:
          fn foo(x: &str, y: &str, z: &i32)

        Compilador infiere:
          fn foo<'a, 'b, 'c>(x: &'a str, y: &'b str, z: &'c i32)
                 ↑   ↑   ↑
                 └───┴───┴── cada uno recibe lifetime único
    */
    #[test]
    pub fn regla_1_multi_params() {
        fn _foo(x: &str, y: &str) {
            println!("  x: {}, y: {}", x, y);
        }
        // El compilador ve: fn foo<'a, 'b>(x: &'a str, y: &'b str)
    }

    /*
    REGLA 2: 1 input → output hereda ese lifetime
    --------------------------------------------
        Escribes:
          fn first(s: &str) -> &str

        Compilador infiere:
          fn first<'a>(s: &'a str) -> &'a str
                          ↑              ↑
                          └──────────────┘  mismo lifetime

        ¿Por qué? Solo hay 1 opción de dónde viene el output.
    */
    #[test]
    pub fn regla_2_single_input() {
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        let s = String::from("hola mundo");
        let word = first_word(&s);
        println!("  First word: {}", word);
        assert_eq!(word, "hola");
    }

    /*
    REGLA 3: &self → output hereda lifetime de self
    --------------------------------------------
        Escribes:
          fn method(&self, other: &str) -> &str

        Compilador infiere:
          fn method<'a, 'b>(&'a self, other: &'b str) -> &'a str
                             ↑                            ↑
                             └────────────────────────────┘
                             cuando no se especifica usa el del &self

        Para que devuelva other hay que especificarlo explícitamente &'b str.
        ¿Por qué? Los métodos usualmente retornan datos del struct.
    */
    #[test]
    pub fn regla_3_self() {
        struct Reader<'a> {
            content: &'a str,
        }

        impl<'a> Reader<'a> {
            // Elision: el retorno hereda el lifetime de &self
            fn get_content(&self) -> &str {
                self.content
            }
        }

        let text = String::from("contenido");
        let reader = Reader { content: &text };
        println!("  Content: {}", reader.get_content());
        assert_eq!(reader.get_content(), "contenido");
    }

    /*
    CUANDO LAS REGLAS FALLAN → Debes escribir lifetimes
    --------------------------------------------
        fn longest(x: &str, y: &str) -> &str

        Regla 1: asigna 'a a x, 'b a y
        Regla 2: ✗ hay 2 lifetimes, no 1
        Regla 3: ✗ no hay &self

        → ERROR: "missing lifetime specifier"
        → Debes escribir: fn longest<'a>(x: &'a str, y: &'a str)
    */
    #[test]
    pub fn elision_fails() {}
}

/*
========================================================================
'STATIC LIFETIME
========================================================================

    'static = no depende de datos locales, tiene la capacidad de vivir todo el programa, solo depende del tiempo de vida de si mismo.

    los owners tambien tienen lifetime 'static implícito.
    let x = 100; // i32 tiene lifetime 'static implícito

    referencias 'static, viven mientras viva la referencia, independientemente de cualquier variable.
*/

#[cfg(test)]
mod static_lifetime {

    /*
    OWNERS Y 'static IMPLÍCITO
    --------------------------------------------
        Los tipos que NO son referencias (&T) tienen lifetime 'static implícito.
     */
    #[test]
    pub fn owners() {
        fn test_static<T: 'static>(_owner: T) {
            //...
        }
        let owner = 100; // i32 tiene lifetime 'static implícito
        test_static(owner);
    }

    /*
    LIFETIME 'static:
    --------------------------------------------
    */
    #[test]
    pub fn static_intro() {
        // 'static es un lifetime especial que solo depende del tiempo de vida de si mismo
        {
            let s: &'static str = "Soy estático";
            println!("{s}");
        }
        // println!("{s}"); // ERROR: s no vive más allá del scope
    }

    /*
    String Literals
    --------------------------------------------
        let s: &'static str = "hola";

        Lifetime todo el programa, su owner es el binario ejecutable.
        Se puede devolver de funciones sin problemas.
    */
    #[test]
    pub fn static_literals() {
        fn get_static_str() -> &'static str {
            "cadena literal"
        }

        let _s1: &str = get_static_str(); // El tipo real es &'static str
        let _s2: &'static str = "literal explícito";
    }

    /*
    CONST
    --------------------------------------------
        const PI: f64 = 3.14159;

        Valor copiado donde se use (inlining) se reemplaza la variable por su valor cuando compila.
        Tipos que acepta
            * Copy
            * Referencias 'static
        Es accesible según su visibilidad y ubicación.
            * Si esta definido a nivel de modulo, es global.
            * Si esta dentro de una función, es local a esa función.
        Es inmutable por defecto.
        Es thread-safe
     */
    #[test]
    pub fn const_inline() {
        const PI: f64 = 3.14159;
        fn area_circle(radius: f64) -> f64 {
            PI * radius * radius
            // 3.14159 * radius * radius  ← PI se reemplaza directo por su valor
        }
        let area = area_circle(2.0);
        assert_eq!(area, PI * 4.0);
    }

    /*
    STATIC var
    --------------------------------------------
        static ID: i32 = 100;           // inmutable, 'static
        static NAME: &str = "Rust";     // inmutable, 'static
        static mut COUNT: i32 = 0;      // mutable, requiere unsafe

        Dirección fija en memoria durante todo el programa.
        Tipos que acepta:
            * Copy
            * Referencias 'static
        Es accesible según su visibilidad y ubicación.
            * Si esta definido a nivel de modulo, es global.
            * Si esta dentro de una función, es local a esa función, pero persiste entre llamadas. Tambien se puede devolver como referencia 'static desde la funcion.
        Si es static mut:
            * unsafe: Requiere unsafe para leer o escribir, otros threads tienen acceso a la variable.
            * +Sync si se usa en threads
        */

    // Acceso a variable estática dentro de otro stack
    #[test]
    pub fn static_basic() {
        static COUNT: i32 = 42;
        fn print_count() {
            // Acceso a variable estática
            println!("  Static COUNT inside fn: {}", COUNT);
        }
        print_count();
    }

    // return de referencia 'static desde función con static local
    #[test]
    pub fn static_return_test() {
        fn return_static() -> &'static i32 {
            static GREETING: i32 = 1;
            // retorna referencia 'static
            &GREETING
        }
        // valido, 'static depende de si mismo, en este caso direccion fija en memoria.
        let _greeting_ref: &'static i32 = return_static();
    }

    // Acceso mutable con UNSAFE
    // otros threads pueden acceder a la variable, entonces no garantiza seguridad.
    #[test]
    pub fn static_mutable() {
        static mut COUNTER: i32 = 0;
        unsafe {
            let _ = COUNTER; // unsafe read
        }
        unsafe {
            COUNTER += 1; // unsafe write
        }
    }

    /*
    Box::leak
    --------------------------------------------
        Convertir heap a 'static

        let s = String::from("runtime");
        let leaked: &'static str = Box::leak(s.into_boxed_str());

        Toma ownership del Box, lo pone en una memoria que NUNCA se libera y retorna
        una referencia 'static a ese dato. Entonces tiene lifetime 'static.
        Se libera la memoria al terminar el programa.

        Puede ser mutable tambien.

        Tipos que acepta: T: 'static
    */
    #[test]
    pub fn static_box_leak() {
        let x = Box::new(42);
        let _static_1: &'static i32 = Box::leak(x);

        let s = String::from("creado en runtime");
        let _static_2: &'static str = Box::leak(s.into_boxed_str());
    }

    #[test]
    pub fn leak_mut() {
        let v = 1;
        let static_vec: &'static mut i32 = Box::leak(Box::new(v));
        *static_vec += 1;
        assert_eq!(*static_vec, 2);
    }

    /*
    CASO 5: 'static en Trait Bounds
    --------------------------------------------
        fn spawn<F>(f: F)
        where
            F: FnOnce() + Send + 'static
                                ^^^^^^^^

        ¿Por qué tokio::spawn requiere 'static?

        El Future puede ejecutarse en cualquier momento futuro.
        Si contuviera referencias a datos locales, esos datos
        podrían morir antes de que el Future termine.

        'static garantiza que el Future no depende de datos
        que puedan morir (solo owned data o refs 'static).
    */
    #[test]
    pub fn static_trait_bounds() {}
}

/*
========================================================================
PLACEHOLDER: '_
========================================================================

    '_ = Hay un lifetime aquí, el compilador lo deduce automaticamente.
*/

#[cfg(test)]
mod placeholder {

    /*
    LIFETIME ANÓNIMO '_ (Placeholder)
    --------------------------------------------
    */
    #[test]
    pub fn placeholder_intro() {}

    /*
    USO 1: En parámetros donde el lifetime es obvio
    --------------------------------------------
        struct Excerpt<'a> { ... }

        // Con placeholder (recomendado):
        fn print(e: &Excerpt<'_>) { ... }

        // Equivalente explícito:
        fn print<'a>(e: &Excerpt<'a>) { ... }

        ¿Por qué usar '_? Cuando NO necesitas relacionar el lifetime con otros
        parámetros o con el retorno.
    */
    #[test]
    pub fn placeholder_obvious() {
        #[derive(Debug)]
        struct Excerpt<'a> {
            _part: &'a str,
        }

        // CON placeholder (más limpio):
        fn print_excerpt(_e: &Excerpt<'_>) {}

        // EQUIVALENTE con lifetime explícito (más verboso):
        fn _print_excerpt2<'a>(_e: &Excerpt<'a>) {}

        let texto = String::from("Hola mundo");
        let exc = Excerpt { _part: &texto };
        print_excerpt(&exc);
    }

    /*
    USO 2: En impl blocks
    --------------------------------------------
        // Con placeholder:
        impl Excerpt<'_> {
            fn len(&self) -> usize { ... }
        }

        // Equivalente explícito:
        impl<'a> Excerpt<'a> {
            fn len(&self) -> usize { ... }
        }

        Usa '_ cuando el método no necesita referenciar 'a.
    */
    #[test]
    pub fn placeholder_impl_blocks() {
        #[derive(Debug)]
        struct Excerpt<'a> {
            part: &'a str,
        }

        // Cuando implementas para un tipo con lifetime pero no necesitas nombrarlo:
        impl Excerpt<'_> {
            fn len(&self) -> usize {
                self.part.len()
            }
        }

        let texto = String::from("Hola mundo");
        let exc = Excerpt { part: &texto };
        println!("  Excerpt len: {}", exc.len());
        assert_eq!(exc.len(), 10);
    }

    /*
    USO 3: En tipos anidados
    --------------------------------------------
        fn process(data: &[&'_ str]) { ... }
                           ^^
                           placeholder para el lifetime interno

        Equivalente: fn process<'a>(data: &[&'a str])
    */
    #[test]
    pub fn placeholder_nested_types() {
        fn process_refs(data: &[&'_ str]) {
            for s in data {
                println!("  Item: {}", s);
            }
        }

        let items = ["uno", "dos", "tres"];
        process_refs(&items);
    }

    /*
    CUANDO '_ NO FUNCIONA:
    --------------------------------------------
        // Esto NO compila:
        fn longest(x: &str, y: &str) -> &'_ str { ... }

        Error: "missing lifetime specifier"

        ¿Por qué? Hay 2 lifetimes de entrada, el compilador no sabe cuál usar
        para el output. Debes ser explícito:

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    */
    #[test]
    pub fn placeholder_ambiguity() {}
}

/*
========================================
FOR<'a>
========================================
    for<'a> = "Para todo lifetime 'a"

    Una forma de indicar que un lifetime puede ser cualquiera, es independiente, no se agrega a la firma T<'a>

    Usado en:
        * Bounds en closures como paramámetros (mayormente)
        * Definición de Traits genéricos

    Ej:
        fn apply_to_str<F>(s: &str, f: F) -> &str
        where
            F: for<'a> Fn(&'a str) -> &'a str,
                     ^^^^^^^^^^^^^^^^
                     para todo lifetime 'a

        El closure F debe funcionar para cualquier lifetime 'a que se le pase.


        Version sin for<'a>:
          * mas restrictivo, solo funciona con un lifetime específico
          * firma con mas dependencias
        fn apply_to_str2<'a, F>(s: &'a str, f: F) -> String
        where
            F: Fn(&'a str) -> String,
        {
            f(s)
        }
*/

#[cfg(test)]
mod for_lifetimes {
    #[test]
    pub fn for_lifetimes_intro() {
        // para cualquier lifetime 'a independiente de apply_to_str
        fn apply_to_str<F>(s: &str, f: F) -> String
        where
            F: for<'a> Fn(&'a str) -> String,
        {
            f(s)
        }

        let f = |input: &str| -> String { input.to_uppercase() };
        let result = apply_to_str("hola", f);

        // Opción 2: sin for<'a>
        //  * mas restrictivo, solo funciona con un lifetime específico
        //  * firma con mas dependencias
        fn apply_to_str2<'a, F>(s: &'a str, f: F) -> String
        where
            F: Fn(&'a str) -> String,
        {
            f(s)
        }

        let f2 = |input: &str| -> String { input.to_lowercase() };
        let result2 = apply_to_str2("HOLA", f2);
    }
}

/*
========================================
BOUNDS CON LIFETIMES
========================================

    T: 'a = "El tipo T vive al menos tanto como el lifetime 'a"

    Ej:
        struct Container<'a, T>
        where
            T: 'a,
        {
            reference: &'a T,
        }

        El tipo T debe vivir al menos tanto como 'a, porque Container tiene
        una referencia &'a T.

        Si T fuera un tipo con lifetime más corto que 'a, la referencia
        podría quedar colgando.

    ----------------------------------------

    T: Trait1 + Trait2 + 'a + 'b

        El tipo T cumple Trait1, Trait2, y vive al menos tanto como el min de 'a y 'b

    T: Send + 'static

        El tipo T puede ser enviado a otros threads y vive siempre que exista su variable, no esta condicionado a otros datos locales.

*/
