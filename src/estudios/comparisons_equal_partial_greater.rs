#[allow(unused_variables)]
#[allow(dead_code)]
#[test]
pub fn indice() {
    println!("\n════════════════════════════════════════════════");
    println!("Módulo: Comparaciones - Equal, Partial y Greater");
    println!("════════════════════════════════════════════════");

    traits::test_partial_eq_manual();
    traits::test_partial_eq_derive();
    traits::test_eq_manual();
    traits::test_eq_derive();
    traits::test_partial_ord_derive();
    traits::test_partial_ord_manual();
    traits::test_ord_derive();
    traits::test_ord_manual();

    tipos_primitivos::test_bool();
    tipos_primitivos::test_char();
    tipos_primitivos::test_integers();
    tipos_primitivos::test_floats();
    tipos_primitivos::test_strings();

    referencias_vs_punteros::test_referencias_iguales();
    referencias_vs_punteros::test_punteros_direcciones();
    referencias_vs_punteros::test_punteros_heap();
    referencias_vs_punteros::test_referencias_vs_punteros();

    colecciones::test_arrays();
    colecciones::test_slices();
    colecciones::test_vectores();
    colecciones::test_orden_colecciones();

    tipos_compuestos::test_enum_ord();
    tipos_compuestos::test_custom_impl();

    tuplas::test_tuples_eq();
    tuplas::test_tuples_ord();
    tuplas::test_nested_tuples();
}

/*

// ═════════════════════════════════════════════════════════════════════════════
// TRAITS:
// ═════════════════════════════════════════════════════════════════════════════
*/

#[cfg(test)]
mod traits {
    /*
    ═════════════════════════════════════════════════════════════════════════════
    PartialEq
    ═════════════════════════════════════════════════════════════════════════════

        pub trait PartialEq<Rhs = Self>
        where
            Rhs: ?Sized,
        {
            fn eq(&self, other: &Rhs) -> bool;

            // Implementaciones por defecto:
            fn ne(&self, other: &Rhs) -> bool {
                !self.eq(other)
            }
        }


    QUÉ HACE:
    • Define el operador ==  y !==
    • NO requiere reflexividad (a == a puede ser false, ej: NaN) <- importante
    • Pueden haber valores "incomparables", ej NaN
    */

    // Implementacion Manual de PartialEq:
    #[test]
    pub fn test_partial_eq_manual() {
        #[derive(Debug)]
        struct Age(u8);

        impl PartialEq for Age {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        assert_eq!(Age(30) == Age(30), true);
        assert_eq!(Age(30).eq(&Age(25)), false);
        assert_eq!(Age(30), Age(30));
        assert_ne!(Age(30), Age(25));
    }

    // Implementacion automática con derive de PartialEq:
    #[test]
    pub fn test_partial_eq_derive() {
        #[derive(PartialEq, Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let p1 = Person {
            name: "Alice".into(),
            age: 30,
        };
        let p2 = Person {
            name: "Alice".into(),
            age: 30,
        };
        assert!(p1 == p2); // Compara: name == name AND age == age         
    }

    /*
    ═════════════════════════════════════════════════════════════════════════════
    Eq
    ═════════════════════════════════════════════════════════════════════════════

        pub trait Eq: PartialEq<Self> {
            // Sin métodos adicionales
            // Solo marca que PartialEq es reflexivo ( a == a SIEMPRE es true )
        }

        QUÉ HACE:
        • Extiende PartialEq
        • Garantiza REFLEXIVIDAD: a == a SIEMPRE es true
        • Se usa para tipos que NO tienen valores incomparables
        • Es un "marker trait" (sin métodos, solo propiedades matemáticas)
    */

    // Implementacion Manual de Eq:
    #[test]
    pub fn test_eq_manual() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl PartialEq for Point {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl Eq for Point {}

        let p = Point { x: 5, y: 10 };
        assert_eq!(p, p); // ✓ Reflexividad garantizada
    }

    // Implementacion automática con derive de Eq:
    #[test]
    pub fn test_eq_derive() {
        #[derive(PartialEq, Eq, Debug)]
        struct UserId(u64);

        let id1 = UserId(123);
        assert_eq!(id1, id1); // Reflexividad: garantizado por Eq
    }
    /*
    ═════════════════════════════════════════════════════════════════════════════
    PartialOrd
    ═════════════════════════════════════════════════════════════════════════════

        pub trait PartialOrd<Rhs = Self>
        where
            Rhs: ?Sized,   // permite tipos con tamaño fijo o dinamico (conocido en runtime)
        {
            fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

            // Implementaciones por defecto:
            fn lt(&self, other: &Rhs) -> bool {
                matches!(self.partial_cmp(other), Some(Ordering::Less))
            }
            fn le(&self, other: &Rhs) -> bool {
                matches!(self.partial_cmp(other), Some(Ordering::Less | Ordering::Equal))
            }
            fn gt(&self, other: &Rhs) -> bool {
                matches!(self.partial_cmp(other), Some(Ordering::Greater))
            }
            fn ge(&self, other: &Rhs) -> bool {
                matches!(self.partial_cmp(other), Some(Ordering::Greater | Ordering::Equal))
            }
        }

        QUÉ HACE:
        • Define operadores <, <=, >, >=
        • Retorna Option<Ordering> (pueden ser incomparables, ej: NaN) <- importante
        • REQUIERE implementar PartialEq primero

        OPERADORES QUE IMPLEMENTA:
        • <, <=, >, >=
        • partial_cmp() → Option<Ordering> (si se pudo comparar o no)

    */

    // Implementacion automática con derive de PartialOrd:
    #[test]
    pub fn test_partial_ord_derive() {
        #[derive(PartialEq, PartialOrd)]
        struct Score(f64);

        let s1 = Score(85.5);
        let s2 = Score(90.0);
        // operadores de comparación
        assert_eq!(s1 < s2, true);
        assert_eq!(s1 <= s2, true);
        assert_eq!(s2 > s1, true);
        assert_eq!(s2 >= s1, true);

        // Permite saber si se puede comparar o no
        let nan_score = Score(f64::NAN);
        assert_eq!(nan_score < s1, false);
        assert_eq!(nan_score.partial_cmp(&s1), None); // Option<Ordering>
    }

    // Implementacion Manual de PartialOrd:
    #[test]
    pub fn test_partial_ord_manual() {
        use std::cmp::Ordering;

        struct Distance(f64);

        impl PartialEq for Distance {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialOrd for Distance {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        let d1 = Distance(5.0);
        let d2 = Distance(10.0);
        assert_eq!(d1 < d2, true);
        assert_eq!(d1.partial_cmp(&d2), Some(Ordering::Less));
    }
    /*
    ═════════════════════════════════════════════════════════════════════════════
    4. Ord TRAIT
    ═════════════════════════════════════════════════════════════════════════════

        pub trait Ord: Eq + PartialOrd<Self> {
            fn cmp(&self, other: &Self) -> Ordering;
        }

        QUÉ HACE:
        • Define "orden total": TODOS los elementos son comparables <- importante
        • Retorna Ordering directo (NO Option)
        • REQUIERE implementar Eq y PartialOrd primero

        OPERADORES QUE IMPLEMENTA:
        • <, <=, >, >= (heredados de PartialOrd)
        • cmp() → Ordering directo

    */

    // Implementacion automática con derive de Ord:
    #[test]
    pub fn test_ord_derive() {
        use std::cmp::Ordering;

        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct Priority {
            level: u8,
        }

        let p1 = Priority { level: 1 };
        let p2 = Priority { level: 5 };

        assert_eq!(p1 < p2, true);
        assert_eq!(p1.cmp(&p2), Ordering::Less);

        let mut levels = vec![p2, p1];
        levels.sort();
        assert_eq!(levels[0].level, 1);
    }

    // Implementacion Manual de Ord:
    #[test]
    pub fn test_ord_manual() {
        use std::cmp::Ordering;

        struct UserId(u64);

        impl PartialEq for UserId {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for UserId {}

        impl PartialOrd for UserId {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl Ord for UserId {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        let id1 = UserId(100);
        let id2 = UserId(200);
        assert_eq!(id1.cmp(&id2), Ordering::Less);
    }

    /*

    ═════════════════════════════════════════════════════════════════════════════
    JERARQUÍA DE TRAITS Y REQUISITOS
    ═════════════════════════════════════════════════════════════════════════════


                              PartialEq (==, !=) (eq, ne)
                             /          \
                            /            \
                           Eq         PartialOrd (<, <=, >, >=)
                            \            /       (partial_cmp -> Option<Ordering>)
                             \          /
                              \        /
                               \      /
                                 Ord (<, <=, >, >=), (cmp -> Ordering)
    */
}

// ═════════════════════════════════════════════════════════════════════════════
// MÓDULO 1: COMPARACIÓN DE TIPOS PRIMITIVOS
// ═════════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tipos_primitivos {
    /*
    COMPARACIÓN DE TIPOS PRIMITIVOS

    bool    → TRUE/FALSE: igualdad y ordenamiento (false < true)
    int     → i8, i16, i32, i64, isize: comparación total
    uint    → u8, u16, u32, u64, usize: comparación total
    float   → f32, f64: PartialOrd/PartialEq (NaN rompe Eq)
        NaN != float es siempre true
        NaN < <= > >= float, es siempre false
    char    → Compara por valor Unicode
        '😀' = U+1F600 = 128512 (decimal)
        'a'  = U+0061  = 97 (decimal)
    String  → &str,  compara como si fuesen varios chars, Lexicográfico por valor de código Unicode
        '😀' = U+1F600 = 128512 (decimal)
        'a'  = U+0061  = 97 (decimal)
    */

    #[test]
    pub fn test_bool() {
        assert_eq!(true == true, true);
        assert_eq!(true == false, false);
        assert_eq!(true > false, true); // false = 0, true = 1
        assert_eq!(false < true, true);
    }

    #[test]
    pub fn test_char() {
        assert_eq!('a' == 'a', true);
        assert_eq!('a' != 'b', true);
        assert_eq!('a' < 'b', true); // Compara código Unicode
        assert_eq!('0' < '9', true); // '0' = U+0030, '9' = U+0039
        assert_eq!('A' < 'a', true); // U+0041 < U+0061
        assert_eq!('a' < '😀', true); // 'a' = U+0061 (97), '😀' = U+1F600 (128512)
    }

    #[test]
    pub fn test_integers() {
        let a: i32 = -42;
        let b: i32 = 42;
        let c: i32 = 42;

        assert_eq!(a == b, false);
        assert_eq!(b == c, true);
        assert_eq!(a < b, true);
        assert_eq!(b > a, true);

        // Diferentes tipos requieren casting
        let x: i8 = 10;
        let y: u32 = 10;
        assert_eq!(x as u32 == y, true);
    }

    #[test]
    pub fn test_floats() {
        let a: f64 = 3.14;
        let b: f64 = 3.14;
        let nan = f64::NAN;

        // Igualdad normal
        assert_eq!(a == b, true);
        assert_eq!(a != (a + 1.0), true);

        // NaN != NaN, (no reflexivo) PartialEq
        // NaN != float (no comparable) PartialEq
        // NaN < <= > >= float, siempre es false (no ordenable) PartialOrd

        // ⚠️ NaN rompe reflexividad
        assert_eq!(nan == nan, false); // ¡¡NaN ≠ NaN!!
        assert_eq!(nan < 0.0, false); // NaN < X siempre false
        assert_eq!(nan > 0.0, false); // NaN > X siempre false
        assert_eq!(nan == 0.0, false); // NaN == X siempre false
        assert!(nan != nan); // Esto es TRUE
    }

    #[test]
    pub fn test_strings() {
        let s1 = "apple";
        let s2 = "apple";
        let s3 = "banana";

        // Comparación de valores
        assert_eq!(s1 == s2, true);
        assert_eq!(s1 != s3, true);

        // Orden lexicográfico (alfabético)
        assert_eq!(s1 < s3, true); // "apple" < "banana"
        assert_eq!("abc" < "abd", true); // Compara punto a punto
        assert_eq!("a" < "aa", true); // Prefijo es menor
        assert_eq!("hola_😀" > "hola_a", true); // '😀' = U+1F600 (128512) > 'a' = U+0061 (97)

        // String vs &str
        let owned = String::from("apple");
        assert_eq!(owned == s1, true); // Se dereferencia automáticamente
    }
}

/*
Float

═════════════════════════════════════════════════════════════════════════════
NaN (Not a Number) EN FLOATS
═════════════════════════════════════════════════════════════════════════════

1. CUÁNDO APARECE NaN
─────────────────────────────────────────────────────────────────────────────

    NaN es la solucion al problema matemático de representar un valor que no es un número inderminado. por ejemplo 0.0 / 0.0

    * Hardware nativo lo soporta
    * Esto permite que el cálculo continúe sin paniquear (fault tolerance)
    * Detección fácil: .is_nan() al final en lugar de try/catch
    * Compatible con librerías matemáticas complejas

  A) OPERACIONES MATEMÁTICAS INDETERMINADAS:
     0.0 / 0.0 = NaN
     Inf - Inf = NaN
     Inf / Inf = NaN
     Inf * 0.0 = NaN
     (-Inf) + Inf = NaN
     (-1.0).sqrt() = NaN
     (-5.0).ln() = NaN
     (-2.0).log10() = NaN

  C) OPERACIONES CON NaN: (propagación de NaN)
     NaN + 5.0               → NaN      (NaN propaga)
     NaN * 0.0               → NaN      (NaN propaga)
     NaN / 2.0               → NaN      (NaN propaga)
     (5.0).min(NaN)          → NaN      (min con NaN = NaN)

  D) CONSTANTE DIRECTA:
     f64::NAN                → NaN      (constante predefinida)
     f32::NAN                → NaN      (en f32)

  E) PARSING "Nan" de STRING:
     "NaN".parse::<f64>()    → Ok(NaN)  (parse exitoso de "NaN")
     "nan".parse::<f64>()    → Error    (Rust es case-sensitive)
     "NAN".parse::<f64>()    → Error    (debe ser exactamente "NaN")

  F) PARSING ERRÓNEO NO PRODUCE NaN: produce Err
     "abc".parse::<f64>()    → Err
     "12.34.56".parse()      → Err
     "".parse::<f64>()       → Err


2. COMPARACIONES CON NaN
─────────────────────────────────────────────────────────────────────────────

  A) REFLEXIVIDAD ROTA (problema principal):
     NaN == NaN  : false    ⚠️ (¡¡No es igual a sí mismo!!)
     NaN == (any float) : false    (son distintos)

  B) COMPARACIONES ORDENADAS (todas falsas):
     NaN < <= > >= (any float) : false
     (any float) < <= > >= NaN : false


═════════════════════════════════════════════════════════════════════════════
INFINITO (Inf) EN FLOATS
═════════════════════════════════════════════════════════════════════════════

1. CUÁNDO APARECE INFINITO
─────────────────────────────────────────────────────────────────────────────
    +Inf representa un valor numérico que es más grande que cualquier otro número finito.
    -Inf representa un valor numérico que es más pequeño que cualquier otro número finito.
    f64::MAX < Inf

  A) DIVISIÓN POR CERO:
     1.0 / 0.0    → +Inf (infinito positivo)
     -1.0 / 0.0   → -Inf (infinito negativo)
     5.0 / 0.0    → +Inf

  B) DESBORDAMIENTO (overflow):
     f64::MAX + f64::MAX     → +Inf
     f64::MAX * 2.0          → +Inf
     10.0_f64.powi(400)      → +Inf (número muy grande)

  C) CONSTANTES DIRECTAS:
     f64::INFINITY           → +Inf
     f64::NEG_INFINITY       → -Inf
     f32::INFINITY           → +Inf (en f32)

  D) PARSING DE STRING:
     "inf".parse::<f64>()    → Ok(f64::INFINITY)
     "-inf".parse::<f64>()   → Ok(f64::NEG_INFINITY)
     "Infinity".parse()      → Error (no válido en Rust)


2. OPERACIONES CON INFINITO
─────────────────────────────────────────────────────────────────────────────

  A) ARITMÉTICA BÁSICA:
    Inf + - * / (float finito): Inf

  B) CASOS INDETERMINADOS (retornan NaN):
     Inf - Inf       → NaN         (indeterminado)
     Inf + (-Inf)    → NaN         (indeterminado)
     Inf / Inf       → NaN         (indeterminado)
     Inf * 0.0       → NaN         (indeterminado)
     Inf + - NaN     → NaN         (NaN propaga)

  C) OPERACIONES CON CERO:
     0.0 * Inf       → NaN
     0.0 / Inf       → 0.0         (cero es "pequeño" comparado a Inf)

  D) INFINITO NEGATIVO:
     -Inf + 100      → -Inf
     -Inf - 100      → -Inf
     -Inf * -1.0     → +Inf        (negativo × negativo = positivo)


3. COMPARACIONES CON INFINITO
─────────────────────────────────────────────────────────────────────────────

  A) REFLEXIVIDAD (igual a sí mismo): Eq
     Inf == Inf              → true   ✓ (a diferencia de NaN)
     -Inf == -Inf            → true   ✓
     Inf == -Inf             → false  (signos opuestos)

  B) COMPARACIONES DE ORDEN: Ord
     Inf > Inf              → false  (no mayor que sí mismo)
     Inf > 1e308            → true   (mayor que cualquier número finito)
     -Inf < -1e308          → true   (menor que cualquier número finito)
     Inf > -Inf             → true
     Inf >= > < <= NaN      → false  (NaN rompe comparaciones)

*/

// ═════════════════════════════════════════════════════════════════════════════
// MÓDULO 2: REFERENCIAS VS PUNTEROS CRUDOS
// ═════════════════════════════════════════════════════════════════════════════
/*
    DIFERENCIA CRÍTICA:

    &T (referencia)
    • compara el CONTENIDO (dereferencia automática)
    • &5 == &5 → TRUE (compara valores)

    *const T (puntero crudo)
    • Compara la DIRECCIÓN de memoria (no el contenido)
    • 0x7fff1234 == 0x7fff5678 → FALSE (direcciones distintas)
*/
#[cfg(test)]
mod referencias_vs_punteros {

    // referencias comparan valores
    #[test]
    pub fn test_referencias_iguales() {
        let x = 5;
        let y = 5;

        // ✅ Referencia compara valores
        assert_eq!(&x, &y); // TRUE (ambos valen 5)
    }

    // punteros comparan direcciones
    #[test]
    pub fn test_punteros_direcciones() {
        println!("\n▶ PUNTEROS CRUDOS - Comparan DIRECCIONES");
        let x = 5;
        let y = 5;

        // ❌ Puntero compara dirección en stack (distintas variables)
        let ptr_x: *const i32 = &x as *const i32;
        let ptr_y: *const i32 = &y as *const i32;
        assert_ne!(ptr_x, ptr_y); // FALSE (direcciones distintas)

        // ✅ El MISMO puntero a sí mismo es igual
        assert_eq!(ptr_x, ptr_x); // TRUE (mismo número de dirección)
    }

    #[test]
    pub fn test_punteros_heap() {
        let vec1: Vec<i32> = vec![1, 2, 3];
        let ptr_before = vec1.as_ptr(); // Puntero a datos en heap

        let vec2 = vec1; // Move (ownership cambió pero datos en heap no se copian)
        let ptr_after = vec2.as_ptr(); // Mismo puntero a heap

        // ✅ Ambos apuntan al MISMO lugar en heap
        assert_eq!(ptr_before, ptr_after);
    }

    //contenido de puntero contra referencia
    #[test]
    pub fn test_referencias_vs_punteros() {
        let x = 10;
        let ref_x: &i32 = &x; // Referencia
        let ptr_x: *const i32 = &x; // Puntero crudo

        // ✅ Referencia compara valor
        assert_eq!(ref_x, &x); // TRUE 
        // ✅ Puntero crudo compara dirección
        assert_eq!(ptr_x, ref_x as *const i32); // TRUE (misma dirección)
        // contenido del puntero es igual al valor de x
        assert_eq!(unsafe { *ptr_x }, *ref_x); // Dereferencia puntero crudo (unsafe)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// MÓDULO 3: ARRAYS, SLICES Y VECTORES
// ═════════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod colecciones {
    /*
    COMPARACIÓN EN COLECCIONES: (Arrays, Slices, Vectores)

        • PartialEq/Eq: compara elemento por elemento por valor, no por direccion de memoria.
            [1,2,3] == [1,2,3] → TRUE
            [1,2,3] == [1,2,4] → FALSE
    */

    // Arrays comparan contenido, no dirección
    #[test]
    pub fn test_arrays() {
        let arr1 = [1, 2, 3];
        let arr2 = [1, 2, 3];
        let arr3 = [1, 2, 4];

        assert_eq!(arr1, arr2); // TRUE (mismo contenido)
        assert_ne!(arr1, arr3); // FALSE (distinto elemento)
        assert_eq!(arr1 < arr3, true); // Orden lexicográfico

        println!("✓ arrays: comparación elemento por elemento");
    }

    // Slices comparan contenido, no dirección
    #[test]
    pub fn test_slices() {
        let arr = [1, 2, 3, 4, 5];
        let slice1 = &arr[0..3]; // [1, 2, 3]
        let slice2 = &arr[0..3];
        let slice3 = &arr[1..4]; // [2, 3, 4]

        assert_eq!(slice1, slice2); // TRUE (mismo contenido)
        assert_ne!(slice1, slice3); // FALSE (contenido distinto)
        assert_eq!(slice1.len(), 3);
    }

    // Vectores comparan contenido, no dirección
    #[test]
    pub fn test_vectores() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![1, 2, 3];
        let vec3 = vec![1, 2, 3, 4];

        // ✅ Compara contenido, NO dirección en heap
        assert_eq!(vec1, vec2); // TRUE (mismo contenido)
        assert_ne!(vec1, vec3); // FALSE (distinto tamaño/contenido)

        // Direcciones heap distintas
        assert_ne!(vec1.as_ptr(), vec2.as_ptr()); // Distintos lugares en heap
    }

    // Orden lexicográfico en colecciones
    #[test]
    pub fn test_orden_colecciones() {
        let a = [1, 2, 3];
        let b = [1, 2, 4];

        assert_eq!(a < b, true); // [1,2,3] < [1,2,4] (en posición 2: 3<4)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// ENUMS
// ═════════════════════════════════════════════════════════════════════════════
/*
 Los Enums se ordenan según el orden de definición de sus variantes y no por su contenido asociado.

*/
#[cfg(test)]
mod tipos_compuestos {

    #[allow(unused_variables)]
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Priority {
        Low,
        Medium,
        High,
    }

    #[test]
    pub fn test_enum_ord() {
        println!("\n▶ ENUM WITH #[derive(Ord)]");
        let low = Priority::Low;
        let high = Priority::High;

        assert_ne!(low, high);
        assert_eq!(low < high, true); // Orden: Low < Medium < High

        // Orden de definición en enum
        assert_eq!(Priority::Low < Priority::Medium, true);
        assert_eq!(Priority::Medium < Priority::High, true);
        println!("✓ Enums: orden por posición de definición (arriba < abajo)");
    }

    // Ejemplo con datos asociados
    // sigue comparando por orden y no por contenido

    #[allow(unused_variables)]
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum PriorityComplex {
        Low(String),
        Medium(u8),
        High(bool),
    }

    #[test]
    pub fn test_custom_impl() {
        println!("\n▶ CUSTOM TYPE WITH DERIVED ORD");
        let p1 = PriorityComplex::Low("Task A".into());
        let p2 = PriorityComplex::Medium(5);
        let p3 = PriorityComplex::High(true);

        assert_eq!(p1 < p2, true);
        assert_eq!(p2 < p3, true);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// MÓDULO 6: TUPLAS
// ═════════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tuplas {
    /*
    COMPARACIÓN EN TUPLAS:

    Las tuplas comparan elemento por elemento, en orden:
    (1, 'a') < (1, 'b') → TRUE (primer elemento igual, segundo a<b)
    (1, 'b') < (2, 'a') → TRUE (primer elemento 1<2)

    Requieren que TODOS los tipos implementen el trait de comparación.
    */

    #[test]
    pub fn test_tuples_eq() {
        println!("\n▶ TUPLE EQUALITY");
        let t1 = (1, "hello", 3.14);
        let t2 = (1, "hello", 3.14);
        let t3 = (1, "hello", 3.15);

        assert_eq!(t1, t2); // TRUE
        assert_ne!(t1, t3); // FALSE
        println!("✓ tuplas: comparan elemento por elemento");
    }

    #[test]
    pub fn test_tuples_ord() {
        println!("\n▶ TUPLE ORDERING (lexicográfico)");
        let t1 = (1, 2, 3);
        let t2 = (1, 2, 4);
        let t3 = (1, 3, 0);
        let t4 = (2, 0, 0);

        assert_eq!(t1 < t2, true); // Posición 2: 3<4
        assert_eq!(t1 < t3, true); // Posición 1: 2<3
        assert_eq!(t1 < t4, true); // Posición 0: 1<2

        // Orden por campo: primero → segundo → tercero
        println!("✓ tuplas: orden lexicográfico (campo a campo)");
    }

    #[test]
    pub fn test_nested_tuples() {
        println!("\n▶ NESTED TUPLES");
        let nested1 = ((1, 2), (3, 4));
        let nested2 = ((1, 2), (3, 4));

        assert_eq!(nested1, nested2);
        assert_eq!(((1, 2), (3, 3)) < nested1, true);
        println!("✓ tuplas anidadas: orden recursivo");
    }
}
