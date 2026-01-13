#[allow(unused_variables)]
#[allow(dead_code)]
#[test]
fn indice() {
    arrays::arrays();
    arrays::array_iteration();

    vectors::vectors();
    vectors::vector_growth();
    vectors::vector_move();

    array_vs_vec::comparacion();
    array_vs_vec::performance_characteristics();

    slices::slices();
    slices::slice_ranges();
    slices::slice_from_vec();
    slices::slice_operations();

    slices_mutables::slices_mutables();
    slices_mutables::function_with_mut_slice();
    slices_mutables::mut_str_limited();

    slice_de_vector::slice_de_vector();

    strings::strings();
    strings::string_mutation();
    strings::string_is_move();

    string_slices::string_slices();
    string_slices::str_from_string();

    string_literals::string_literals();

    utf8_slicing::utf8_slicing();
    utf8_slicing::safe_slicing_with_get();
    utf8_slicing::char_iteration();

    borrow_checker::borrow_checker();
    borrow_checker::scoped_borrow();
}

/*
========================================================================
SLICES: FLEXIBILIDAD: MULTIPLES FUENTES
========================================================================

    DEREF COERCION en Array, Vec, String:
        Cuando esperas &[T] o &str, Rust automáticamente convierte:
        • &[T; N]  →  &[T]   (Deref en Array)
        • &Vec<T>  →  &[T]   (Deref en Vec)
        • &String  →  &str   (Deref en String)

        let vec = vec![1, 2, 3];
        fn take_slice(data: &[i32]) { ... }
        take_slice(&vec);  // ✓ Se convierte automáticamente

    COMPARACIÓN: PARÁMETROS FLEXIBLES vs RESTRICTIVOS:
    --------------------------------------------
        FLEXIBLE - Acepta múltiples fuentes:

            fn process_slice(data: &[i32]) {       // ← &[T] es flexible
                println!("{:?}", data);
            }

            let arr = [1, 2, 3];
            let vec = vec![1, 2, 3];

            process_slice(&arr);        // ✓ Array → &[i32] (Deref coercion)
            process_slice(&vec);        // ✓ Vec → &[i32]   (Deref coercion)
            process_slice(&vec[1..3]);  // ✓ slice

        RESTRICTIVO - Solo una fuente:

            fn process_vec(data: Vec<i32>) {       // ← Vec requiere ownership
                println!("{:?}", data);
            }

            process_vec(arr.to_vec());  // ✗ Debe copiar Array a Vec (ineficiente!)
            process_vec(vec);           // ✓ Solo funciona con Vec

    CASO STRING: &str vs &String:
    --------------------------------------------
        FLEXIBLE - Acepta String, &str, literales:

            fn greet(name: &str) {                 // ← &str es flexible
                println!("Hola, {}", name);
            }

            let s = String::from("Rust");
            greet(&s);                  // ✓ String → &str (Deref coercion)
            greet("Hola");              // ✓ Literal &str

        RESTRICTIVO - Solo &String:

            fn greet(name: &String) {               // ← &String muy restrictivo
                println!("Hola, {}", name);
            }

            greet(&s);                  // ✓ Funciona con &String
            greet("Hola");              // ✗ ERROR: literal es &str, no &String

    CONCLUSIÓN:
        Siempre usa &str en lugar de &String, &[T] en lugar de Vec<T>

========================================================================
ARRAYS
========================================================================

    ARRAYS [T; N] - TAMAÑO FIJO EN STACK:
    --------------------------------------------
        let arr: [i32; 4] = [10, 20, 30, 40];

        STACK (16 bytes, todo inline):
        ┌─────┬─────┬─────┬─────┐
        │  10 │  20 │  30 │  40 │  ← datos directos
        └─────┴─────┴─────┴─────┘
          [0]   [1]   [2]   [3]

        Características:
        ✓ Tamaño conocido en compilación
        ✓ Sin heap allocation
        ✓ Copy si T: Copy

    FORMAS DE CREAR ARRAYS:
    --------------------------------------------
        let arr1: [i32; 4] = [10, 20, 30, 40];     // explícito
        let arr2 = [1; 4];                         // inicializa todo con 1
        let arr4: [i32; 4];                        // sin inicializar (valores previos de memoria)
        let arr3: [i32; 0] = [];                   // array vacío, 0 bytes,
                                                   // sirve para genéricos [u8; N]
*/
#[cfg(test)]
mod arrays {

    #[test]
    pub fn arrays() {
        use std::mem;

        let arr: [i32; 4] = [10, 20, 30, 40];
        let _arr2: [i32; 4] = [1; 4]; // inicializa todo con 1
        let _arr4: [i32; 4]; // sin inicializar (valores basura)
        let _arr3: [i32; 0] = []; // array vacío

        // Tamaño en stack = N * size_of::<T>()
        assert_eq!(mem::size_of::<[i32; 4]>(), 16); // 4 * 4 bytes

        // Acceso por índice
        assert_eq!(arr[0], 10);
        assert_eq!(arr[3], 40);

        // Es Copy si T es Copy
        let arr2 = arr; // copia, no move
        assert_eq!(arr[0], arr2[0]); // arr sigue válido

        // Inicialización con valor repetido
        let zeros: [i32; 100] = [0; 100];
        assert_eq!(zeros[50], 0);
    }

    #[test]
    pub fn array_iteration() {
        let arr: [i32; 4] = [1, 2, 3, 4];

        // Iteración por referencia
        let sum: i32 = arr.iter().sum();
        assert_eq!(sum, 10);

        // Iteración con índice
        for (i, &val) in arr.iter().enumerate() {
            assert_eq!(val, (i + 1) as i32);
        }
    }
}

/*
========================================================================
VECTORS
========================================================================

    VECTORS Vec<T> - TAMAÑO DINÁMICO EN HEAP:
    --------------------------------------------
        let vec: Vec<i32> = vec![10, 20, 30, 40];

        STACK (24 bytes):                      HEAP:
        ┌─────────────────────┐               ┌─────┬─────┬─────┬─────┬─────┬─────┐
        │ ptr ────────────────┼──────────────▶│  10 │  20 │  30 │  40 │  ?  │  ?  │
        ├─────────────────────┤               └─────┴─────┴─────┴─────┴─────┴─────┘
        │ len: 4              │                 [0]   [1]   [2]   [3]  (capacity extra)
        ├─────────────────────┤
        │ cap: 6              │  ← puede haber capacidad extra
        └─────────────────────┘

        Características:
        ✓ Tamaño dinámico (push/pop)
        ✓ Heap allocation
        ✗ NO es Copy (tiene Drop)

    CAPACIDAD Y CRECIMIENTO:
    --------------------------------------------
        si se llega a ocupar la capacidad se agrega el doble:
        4, 8, 16, 32, 64, 128
        si se ubiese asignado n de capacity inicial, seria el doble cada vez que llega al limite,
        n*2, n*4, n*8, n*16...
*/
#[cfg(test)]
mod vectors {

    #[test]
    pub fn vectors() {
        use std::mem;
        let vec: Vec<i32> = vec![10, 20, 30, 40];

        // Stack size siempre 24 bytes (ptr + len + cap)
        assert_eq!(mem::size_of::<Vec<i32>>(), 24);

        // len y capacity
        assert_eq!(vec.len(), 4);
        assert!(vec.capacity() >= 4);

        // Acceso por índice
        assert_eq!(vec[0], 10);
        assert_eq!(vec[3], 40);
    }

    #[test]
    pub fn vector_growth() {
        let mut vec: Vec<i32> = Vec::new();
        assert_eq!(vec.capacity(), 0);

        // Push aumenta capacity automáticamente
        vec.push(1);
        let cap1 = vec.capacity();
        assert!(cap1 >= 4);

        // Capacity crece exponencialmente
        for i in 2..=100 {
            vec.push(i);
            dbg!(&vec.capacity()); // 4, 8, 16, 32, 64, 128
            // si se ubiese asignado n de capacity inicial, el doble cada vez que llega al limite, n^(2^1), n^(2^2), n^(2^3), n^(2^4)...
        }
        assert!(vec.capacity() >= 100);

        // with_capacity pre-aloca
        let vec2: Vec<i32> = Vec::with_capacity(1000);
        assert_eq!(vec2.len(), 0);
        assert!(vec2.capacity() >= 1000);
    }

    #[test]
    pub fn vector_move() {
        let vec1: Vec<i32> = vec![1, 2, 3];
        let ptr_before = vec1.as_ptr();

        let vec2 = vec1; // move, no copy
        let ptr_after = vec2.as_ptr();

        // El puntero al heap es el mismo
        assert_eq!(ptr_before, ptr_after);
        // vec1 ya no es válido
    }
}

/*
========================================================================
ARRAY_VS_VEC
========================================================================

    COMPARACIÓN:
    --------------------------------------------
        ┌────────────────────┬────────────────────┬────────────────────────────────┐
        │ Aspecto            │ [T; N] (Array)     │ Vec<T>                         │
        ├────────────────────┼────────────────────┼────────────────────────────────┤
        │ Allocation         │ Stack              │ Heap                           │
        │ Tamaño             │ Fijo (compilación) │ Dinámico (runtime)             │
        │ Overhead           │ 0 bytes            │ 24 bytes (ptr+len+cap)         │
        │ Copy               │ ✓ (si T: Copy)     │ ✗ (move o clone)               │
        │ Cache locality     │ Excelente          │ Buena                          │
        │ Crece/decrece      │ ✗                  │ ✓                              │
        │ Tamaño máximo      │ ~MB (stack limit)  │ ~GB (heap)                     │
        │ Velocidad alloc    │ Instantánea        │ Más lenta (syscall)            │
        └────────────────────┴────────────────────┴────────────────────────────────┘

    ¿POR QUÉ ARRAY PUEDE SER MÁS RÁPIDO?:
    --------------------------------------------
        1. STACK vs HEAP:
           Array: allocación instantánea (solo mueve stack pointer)
           Vec: syscall al OS para pedir memoria heap (más lento)

        2. SIN INDIRECCIÓN:
           Array: datos inline, acceso directo
           Vec: ptr → heap, un nivel extra de indirección

        3. OPTIMIZACIÓN DEL COMPILADOR:
           Array: tamaño conocido → loop unrolling, SIMD
           Vec: tamaño dinámico → menos optimizaciones posibles

    LOOP UNROLLING:
    --------------------------------------------
        Código original:
          for i in 0..4 {
              result[i] = arr[i] * 2;
          }

        Después de unrolling:
          result[0] = arr[0] * 2;
          result[1] = arr[1] * 2;
          result[2] = arr[2] * 2;
          result[3] = arr[3] * 2;

        ✓ Sin overhead de saltos (jumps) del loop
        ✓ CPU puede ejecutar en paralelo (ILP)
        ✗ Solo posible si tamaño conocido en compilación

    SIMD (SINGLE INSTRUCTION MULTIPLE DATA):
    --------------------------------------------
        CPU moderno tiene registros SIMD (SSE, AVX, NEON):

        Procesamiento escalar (sin SIMD):
          result[0] = arr[0] * 2;
          result[1] = arr[1] * 2;
          result[2] = arr[2] * 2;
          result[3] = arr[3] * 2;
          ✗ 4 instrucciones, 4 ciclos

        Procesamiento SIMD (AVX-256: 256 bits = 4 x i32):
          result[0..4] = arr[0..4] * 2;   (todo en paralelo!)
          ✓ 1 instrucción, 1 ciclo

        Compilador puede usar SIMD solo si:
          ✓ Tamaño conocido en compilación
          ✓ Acceso secuencial a memoria
          ✓ Sin dependencias entre iteraciones
          ✗ Vec tamaño dinámico → más difícil vetorizar

    CUÁNDO USAR CADA UNO:
    --------------------------------------------
        USAR ARRAY [T; N]:
          • Tamaño conocido en compilación
          • Datos pequeños (< 1KB típicamente)
          • Máxima performance necesaria
          • Ejemplos: coordenadas [f32; 3], matriz [f64; 16], buffer [u8; 256]

        USAR VEC<T>:
          • Tamaño dinámico o desconocido en compilación
          • Datos grandes (> varios KB)
          • Necesitas push/pop/insert/remove
          • Ejemplos: lista de usuarios, contenido de archivo, input de red
*/
#[cfg(test)]
mod array_vs_vec {
    #[test]
    pub fn comparacion() {
        // Array: Copy si T es Copy
        let arr: [i32; 4] = [1, 2, 3, 4];
        let arr2 = arr; // copia
        assert_eq!(arr[0], arr2[0]); // ambos válidos

        // Vec: Move, no Copy
        let vec: Vec<i32> = vec![1, 2, 3, 4];
        let vec2 = vec; // move
        // vec ya no es válido
        assert_eq!(vec2[0], 1);

        // Clone para copiar Vec
        let vec3 = vec2.clone();
        assert_eq!(vec2[0], vec3[0]); // ambos válidos

        println!("  ✅ array_vs_vec::comparacion");
    }

    #[test]
    pub fn performance_characteristics() {
        use std::mem;

        // Array: sin overhead
        let arr: [i32; 1000] = [0; 1000];
        assert_eq!(mem::size_of_val(&arr), 4000); // exactamente 1000 * 4 bytes

        // Vec: 24 bytes de overhead en stack
        let vec: Vec<i32> = vec![0; 1000];
        assert_eq!(mem::size_of_val(&vec), 24); // solo ptr+len+cap

        // Vec datos en heap
        assert!(vec.capacity() >= 1000);
    }
}

/*
========================================================================
SLICES
========================================================================

    SLICES &[T]:
    --------------------------------------------
        Array, Vector, String...
        ┌─────┬─────┬─────┬─────┐
        │  10 │  20 │  30 │  40 │
        └─────┴──▲──┴──▲──┴─────┘
                 │     │
                 │     └─────────────────┐
                 │                       │
        slice: &[i32] (16 bytes, fat pointer)
        ┌─────────────────────┐          │
        │ ptr ────────────────┼──────────┘  (apunta a arr[1])
        ├─────────────────────┤
        │ len: 2 (Fijo)       │
        └─────────────────────┘

    CARACTERÍSTICAS:
    --------------------------------------------
        • Len fijo: No se puede cambiar el tamaño. Hay que crear uno nuevo.
          Si cambiase, apuntarías más allá de los datos válidos.

        • Len se calcula en runtime:
          let slice: &[i32] = &vec![1, 2, 3][..];  // vec.len() desconocido en compilación

        • Inmutable: No se puede cambiar el ptr ni el len.
          let slice: &[i32] = &arr[1..3];
          let slice: &[i32] = &vec[1..4];
          let slice: &str = &s[0..4];  // acceso a bytes UTF-8 (pueden no ser chars válidos)

        • es Copy (es solo ptr + len)
*/
#[cfg(test)]
mod slices {

    #[test]
    pub fn slices() {
        use std::mem;
        let arr: [i32; 5] = [10, 20, 30, 40, 50];
        let slice: &[i32] = &arr[1..3]; // [20, 30]

        // Fat pointer: ptr + len = 16 bytes
        assert_eq!(mem::size_of::<&[i32]>(), 16);

        // Contenido del slice
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0], 20);
        assert_eq!(slice[1], 30);

        // Slice es Copy
        let slice2 = slice;
        assert_eq!(slice[0], slice2[0]); // ambos válidos
    }

    #[test]
    pub fn slice_ranges() {
        let _arr: [i32; 5] = [10, 20, 30, 40, 50];

        // Distintos rangos:
        // &arr[1..3]      // [20, 30]      (excluye índice 3)
        // &arr[1..=3]     // [20, 30, 40]  (incluye índice 3)
        // &arr[1..]       // [20, 30, 40, 50]
        // &arr[..3]       // [10, 20, 30]
        // &arr[..=3]      // [10, 20, 30, 40]
        // &arr[..]        // [10, 20, 30, 40, 50]
    }

    #[test]
    pub fn slice_from_vec() {
        let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
        let slice: &[i32] = &vec[1..4]; // [20, 30, 40]

        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], 20);

        // El slice apunta dentro del heap del Vec
        assert!(slice.as_ptr() > vec.as_ptr()); // slice apunta a vec[1]
    }

    #[test]
    pub fn slice_operations() {
        let arr: [i32; 5] = [10, 20, 30, 40, 50];

        // slice1: Slice es Copy, duplicar no consume original
        let slice1: &[i32] = &arr[1..4]; // [20, 30, 40]
        let slice2 = slice1;
        assert_eq!(slice1.as_ptr(), slice2.as_ptr());

        // slice2: Recortar slice con subrango
        let slice: &[i32] = &arr[..];
        let trimmed1 = &slice[1..4]; // [20, 30, 40]
        let trimmed2 = &slice[..3]; // [10, 20, 30]
        assert_eq!(trimmed1, &[20, 30, 40]);
        assert_eq!(trimmed2, &[10, 20, 30]);

        // slice3: Crear Vec desde slice copia datos a heap
        let vec: Vec<i32> = slice1.to_vec();
        assert_ne!(vec.as_ptr(), slice1.as_ptr()); // diferente memoria

        // slice4: Múltiples formas de copiar slice a Vec
        let v1: Vec<i32> = slice1.to_vec();
        let v2: Vec<i32> = Vec::from(slice1);
        let v3: Vec<i32> = slice1.iter().copied().collect();
        assert_eq!(v1, v2);
        assert_eq!(v2, v3);
    }
}

/*
========================================================================
SLICES_MUTABLES
========================================================================

    SLICES MUTABLES &mut [T]:
    --------------------------------------------
        ┌──────────────────────┬──────────────────┬──────────────────────────┐
        │ Operación            │ &[T] (inmutable) │ &mut [T] (mutable)       │
        ├──────────────────────┼──────────────────┼──────────────────────────┤
        │ Leer valores         │ ✓                │ ✓                        │
        │ Editar valores       │ ✗                │ ✓                        │
        │ Múltiples refs       │ ✓ (muchas)       │ ✗ (solo 1)               │
        │ Editar vec/array     │ ✓ (no con borrow)│ ✗ (mientras existe)      │
        │ is Copy (ptr + len)  │ ✓                │ ✗                        │
        └──────────────────────┴──────────────────┴──────────────────────────┘

    ¿POR QUÉ &mut [i32] ES FÁCIL PERO &mut str ES DIFÍCIL?:
    --------------------------------------------
        TIPOS DE TAMAÑO FIJO (i32, f64, etc.):
            • Cada elemento ocupa exactamente N bytes
            • Modificar un elemento NO afecta a los demás
            ✓ &mut [i32] funciona perfectamente

        STRINGS UTF-8:
            • Cada carácter ocupa 1-4 bytes (variable)
            • Cambiar 'a' (1 byte) por '🦀' (4 bytes) desplazaría todo
            ✗ &mut str muy limitado (solo cambio de characteres mismo tamaño)

    RESTRICCIONES DE REFERENCIAS MUTABLES:
    --------------------------------------------
        1. Solo UNA referencia mutable a la vez:
            let mut arr = [1, 2, 3, 4];
            let mut_slice1 = &mut arr[0..2];
            let mut_slice2 = &mut arr[2..4];  // ✗ ERROR ya existe mut_slice1

        2. No puedes mutar el vec/array mientras existe el slice mutable:
            let mut vec = vec![1, 2, 3, 4, 5];
            let mut_slice = &mut vec[1..4];
            vec.push(6);  // ✗ ERROR: no puedes mutar vec mientras existe mut_slice
*/
#[cfg(test)]
mod slices_mutables {
    #[test]
    pub fn slices_mutables() {
        let mut arr: [i32; 4] = [10, 20, 30, 40];
        let slice_mut: &mut [i32] = &mut arr[1..3];

        // Modificar elementos
        slice_mut[0] = 200;
        slice_mut[1] *= 10;

        assert_eq!(slice_mut[0], 200);
        assert_eq!(slice_mut[1], 300);
        assert_eq!(arr, [10, 200, 300, 40]);
    }

    #[test]
    pub fn function_with_mut_slice() {
        fn double_values(data: &mut [i32]) {
            for x in data.iter_mut() {
                *x *= 2;
            }
        }

        let mut vec = vec![1, 2, 3, 4, 5];
        double_values(&mut vec[1..4]); // Solo modifica [1], [2], [3]

        assert_eq!(vec, [1, 4, 6, 8, 5]);
    }

    #[test]
    pub fn mut_str_limited() {
        let mut s = String::from("hello");

        // Solo operaciones que NO cambian longitud
        s.make_ascii_uppercase();
        assert_eq!(s, "HELLO");

        // Esto funciona porque 'H' y 'h' ocupan el mismo byte
    }
}

/*
========================================================================
SLICE_DE_VECTOR
========================================================================

    SLICE DE VECTOR:
    --------------------------------------------
        let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
        let slice: &[i32] = &vec[1..4];  // [20, 30, 40]

        STACK                                 HEAP
        vec: Vec<i32> (24 bytes)
        ┌─────────────────────┐               ┌─────┬─────┬─────┬─────┬─────┐
        │ ptr ────────────────┼──────────────▶│  10 │  20 │  30 │  40 │  50 │
        ├─────────────────────┤               └─────┴──▲──┴─────┴──▲──┴─────┘
        │ len: 5              │                        │           │
        ├─────────────────────┤                        │           │
        │ cap: 5              │                        │           │
        └─────────────────────┘                        │           │
                                                       │           │
        slice: &[i32] (16 bytes)                       │           │
        ┌─────────────────────┐                        │           │
        │ ptr ────────────────┼────────────────────────┘           │
        ├─────────────────────┤         (apunta a vec[1])          │
        │ len: 3              │  ──────────────────────────────────┘
        └─────────────────────┘         (cubre hasta vec[3])

        ✓ slice apunta DENTRO del heap de vec
        ✓ No hay copia de datos
        ✓ slice debe vivir menos que vec (lifetime)
*/
#[cfg(test)]
mod slice_de_vector {
    #[test]
    pub fn slice_de_vector() {
        let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
        let slice: &[i32] = &vec[1..4];

        // Slice apunta dentro del heap
        assert_eq!(slice.len(), 3);
        assert_eq!(slice, &[20, 30, 40]);

        // Verificar que apunta al mismo heap
        let vec_ptr = vec.as_ptr();
        let slice_ptr = slice.as_ptr();

        // slice_ptr debe ser vec_ptr + 4 bytes (offset de 1 i32)
        unsafe {
            assert_eq!(slice_ptr, vec_ptr.add(1));
        }
    }
}

/*
========================================================================
STRINGS
========================================================================

    STRINGS String - UTF-8 en heap:
    --------------------------------------------
        let s = String::from("Hola 🦀");

        STACK (24 bytes):                      HEAP:
        ┌─────────────────────┐               ┌───┬───┬───┬───┬───┬────┬────┬────┬────┐
        │ ptr ────────────────┼──────────────▶│ H │ o │ l │ a │   │0xF0│0x9F│0xA6│0x80│
        ├─────────────────────┤               └───┴───┴───┴───┴───┴────┴────┴────┴────┘
        │ len: 9              │                 UTF-8 bytes (🦀 = 4 bytes)
        ├─────────────────────┤
        │ cap: 9              │
        └─────────────────────┘

    CARACTERÍSTICAS:
    --------------------------------------------
        ✓ Igual que Vec<u8> pero garantiza UTF-8 válido
        ✗ NO es Copy
*/
#[cfg(test)]
mod strings {
    #[test]
    pub fn strings() {
        use std::mem;
        let s = String::from("Hola 🦀");

        // Stack size siempre 24 bytes
        assert_eq!(mem::size_of::<String>(), 24);

        // len es en bytes, no caracteres
        assert_eq!(s.len(), 9); // "Hola " (5 bytes) + 🦀 (4 bytes)
        assert_eq!(s.chars().count(), 6); // 6 caracteres
    }

    #[test]
    pub fn string_mutation() {
        let mut s = String::from("Hola");

        s.push(' ');
        s.push_str("mundo");

        assert_eq!(s, "Hola mundo");
        assert!(s.capacity() >= s.len());
    }

    #[test]
    pub fn string_is_move() {
        let s1 = String::from("test");
        let ptr_before = s1.as_ptr();

        let s2 = s1; // move
        let ptr_after = s2.as_ptr();

        // El puntero al heap es el mismo
        assert_eq!(ptr_before, ptr_after);
        // s1 ya no es válido
    }
}

/*
========================================================================
STRING_SLICES
========================================================================

    STRING SLICES &str:
    --------------------------------------------
        let s = String::from("Hola mundo");
        let slice: &str = &s[0..4];  // "Hola"

        STACK                                 HEAP
        s: String (24 bytes)
        ┌─────────────────────┐               ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
        │ ptr ────────────────┼──────────────▶│ H │ o │ l │ a │   │ m │ u │ n │ d │ o │
        ├─────────────────────┤               └─▲─┴───┴───┴─▲─┴───┴───┴───┴───┴───┴───┘
        │ len: 10             │                 │           │
        ├─────────────────────┤                 │           │
        │ cap: 10             │                 │           │
        └─────────────────────┘                 │           │
                                                │           │
        slice: &str (16 bytes)                  │           │
        ┌─────────────────────┐                 │           │
        │ ptr ────────────────┼─────────────────┘           │
        ├─────────────────────┤    (apunta a s[0])          │
        │ len: 4              │  ───────────────────────────┘
        └─────────────────────┘    (cubre hasta s[3])

    CARACTERÍSTICAS:
    --------------------------------------------
        ✓ Vista a bytes UTF-8 (no copia)
        ✓ Copy (es solo ptr + len)
        ✓ Puede apuntar a String, literal, u otro &str
*/
#[cfg(test)]
mod string_slices {
    #[test]
    pub fn string_slices() {
        use std::mem;
        let s = String::from("Hola mundo");
        let slice: &str = &s[0..4];

        // Fat pointer: 16 bytes
        assert_eq!(mem::size_of::<&str>(), 16);

        assert_eq!(slice, "Hola");
        assert_eq!(slice.len(), 4);

        // &str es Copy
        let slice2 = slice;
        assert_eq!(slice, slice2);
    }

    #[test]
    pub fn str_from_string() {
        let s = String::from("hello");

        // Múltiples formas de obtener &str
        let slice1: &str = &s; // Deref coercion
        let slice2: &str = s.as_str(); // Explícito
        let slice3: &str = &s[..]; // Full slice

        assert_eq!(slice1, slice2);
        assert_eq!(slice2, slice3);
    }
}

/*
========================================================================
STRING_LITERALS
========================================================================

    STRING LITERALS &'static str:
    --------------------------------------------
        let literal: &'static str = "Hola 🦀";

        STACK (16 bytes):                      BINARIO (.rodata):
        ┌─────────────────────┐               ┌───┬───┬───┬───┬───┬────┬────┬────┬────┐
        │ ptr ────────────────┼──────────────▶│ H │ o │ l │ a │   │0xF0│0x9F│0xA6│0x80│
        ├─────────────────────┤               └───┴───┴───┴───┴───┴────┴────┴────┴────┘
        │ len: 9              │                 Embebido en el ejecutable
        └─────────────────────┘

    CARACTERÍSTICAS:
    --------------------------------------------
        ✓ Datos en .rodata (read-only data section)
        ✓ Vive durante todo el programa ('static)
        ✓ NO hay heap allocation
        ✓ Copy
*/
#[cfg(test)]
mod string_literals {
    #[test]
    pub fn string_literals() {
        let literal: &'static str = "Hola 🦀";

        // No hay heap allocation
        assert_eq!(literal.len(), 9);
        assert_eq!(literal.chars().count(), 6);

        // Es Copy
        let literal2 = literal;
        assert_eq!(literal, literal2);

        // Vive para siempre ('static)
        fn get_static() -> &'static str {
            "esto vive para siempre"
        }
        let s = get_static();
        assert!(!s.is_empty());
    }
}

/*
========================================================================
UTF8_SLICING
========================================================================

    UTF-8 SLICING - Peligros:
    --------------------------------------------
        let s = String::from("Hola 🦀 rustaceans");

        Mapa de bytes:
        ┌───┬───┬───┬───┬───┬────┬────┬────┬────┬───┬───┬───┬...┐
        │ H │ o │ l │ a │   │0xF0│0x9F│0xA6│0x80│   │ r │ u │...│
        └───┴───┴───┴───┴───┴────┴────┴────┴────┴───┴───┴───┴...┘
          0   1   2   3   4   5    6    7    8    9  10  11  ...
                          ◄──────── 🦀 ────────►
                          │    │    │    │
                          ✓    ✗    ✗    ✗    ✓  ← char boundaries
                         [5]  [6]  [7]  [8]  [9]

        ┌────────────────────────┬─────────────────────────────────────────────┐
        │ Operación              │ Resultado                                   │
        ├────────────────────────┼─────────────────────────────────────────────┤
        │ &s[0..5]               │ ✓ "Hola " (termina antes del emoji)         │
        │ &s[5..9]               │ ✓ "🦀" (emoji completo, 4 bytes)            │
        │ &s[9..20]              │ ✓ " rustaceans" (después del emoji)         │
        ├────────────────────────┼─────────────────────────────────────────────┤
        │ &s[0..6]               │ ✗ PANIC! corta dentro del emoji             │
        │ &s[6..9]               │ ✗ PANIC! empieza dentro del emoji           │
        └────────────────────────┴─────────────────────────────────────────────┘

    CÓMO EVITAR EL PANIC:
    --------------------------------------------
        1. Verificar antes: s.is_char_boundary(idx)
        2. Usar chars(): s.chars().take(n).collect::<String>()
        3. Usar s.get(start..end) que retorna Option<&str>
*/
#[cfg(test)]
mod utf8_slicing {
    #[test]
    pub fn utf8_slicing() {
        let s = String::from("Hola 🦀 rustaceans");

        // Verificar char boundaries
        assert!(s.is_char_boundary(0));
        assert!(s.is_char_boundary(5)); // inicio de 🦀
        assert!(!s.is_char_boundary(6)); // dentro de 🦀
        assert!(!s.is_char_boundary(7)); // dentro de 🦀
        assert!(!s.is_char_boundary(8)); // dentro de 🦀
        assert!(s.is_char_boundary(9)); // después de 🦀

        // Slicing válido
        assert_eq!(&s[0..5], "Hola ");
        assert_eq!(&s[5..9], "🦀");
        assert_eq!(&s[9..], " rustaceans");
    }

    #[test]
    pub fn safe_slicing_with_get() {
        let s = String::from("Hola 🦀");

        // .get() retorna Option en vez de panic
        assert!(s.get(0..6).is_none()); // inválido
        assert!(s.get(0..5).is_some()); // válido
        assert_eq!(s.get(5..9), Some("🦀"));
    }

    #[test]
    pub fn char_iteration() {
        let s = String::from("Hola 🦀");

        // Iterar por caracteres (no bytes)
        let chars: Vec<char> = s.chars().collect();
        assert_eq!(chars.len(), 6);
        assert_eq!(chars[5], '🦀');

        // char_indices da índice de byte + carácter
        let indices: Vec<(usize, char)> = s.char_indices().collect();
        assert_eq!(indices[5], (5, '🦀'));
    }

    #[test]
    #[should_panic(expected = "byte index 6 is not a char boundary")]
    pub fn invalid_slice_panics() {
        let s = String::from("Hola 🦀");
        let _ = &s[0..6]; // PANIC!
    }
}

/*
========================================================================
BORROW_CHECKER
========================================================================

    BORROW CHECKER - Previene slices inválidos:
    --------------------------------------------
        Ejemplo que NO compila:
        ┌──────────────────────────────────────────────────────────────────────┐
        │ let mut s = String::from("Hola 🦀");                                 │
        │ let slice: &str = &s[0..5];  // borrow inmutable                     │
        │                                                                      │
        │ s.push_str(" mundo");  // ✗ ERROR: cannot borrow `s` as mutable     │
        │                        //   because it is also borrowed as immutable│
        │                                                                      │
        │ println!("{}", slice);  // slice todavía en uso                      │
        └──────────────────────────────────────────────────────────────────────┘

    REGLAS DE BORROWING:
    --------------------------------------------
        1. Puedes tener MUCHOS &T (borrows inmutables) al mismo tiempo
        2. O UN SOLO &mut T (borrow mutable) a la vez
        3. NUNCA ambos simultáneamente
        4. El borrow debe vivir menos que el owner

*/
#[cfg(test)]
mod borrow_checker {
    #[test]
    pub fn borrow_checker() {
        let mut s = String::from("Hola");

        // Múltiples borrows inmutables OK
        let r1: &str = &s;
        let r2: &str = &s;
        assert_eq!(r1, r2);

        // Después de usar los borrows, podemos mutar, r1 y r2 ya no se usarían
        s.push_str(" mundo");
        assert_eq!(s, "Hola mundo");
    }

    #[test]
    pub fn scoped_borrow() {
        let mut s = String::from("Hola");

        // Borrow en scope interno
        {
            let slice: &str = &s[..];
            assert_eq!(slice, "Hola");
        } // slice sale del scope

        // Ahora podemos mutar
        s.push_str(" mundo");
        assert_eq!(s, "Hola mundo");

        println!("  ✅ borrow_checker::scoped_borrow");
    }
}
