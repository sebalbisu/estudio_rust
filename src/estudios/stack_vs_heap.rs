#![allow(dead_code)]
use std::hint::black_box;

#[test]
fn indice() {
    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║            STACK vs HEAP - ÍNDICE DE DEMOS                        ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("1. CONCEPTOS FUNDAMENTALES");
    conceptos::diagrama_conceptos();

    println!("\n2. ANÁLISIS DE TAMAÑO");
    tamano::analisis_tamano();

    println!("\n3. PERFORMANCE DE ASIGNACIÓN");
    asignacion::benchmark_asignacion();

    println!("\n4. TRANSFERENCIA POR VALOR");
    transferencia::benchmark_transferencia();

    println!("\n5. COSTO DE ACCESO (INDIRECCIÓN)");
    acceso::benchmark_acceso();

    println!("\n6. HEAP RESERVADO (REUSO)");
    heap_reservado::benchmark_reuso();

    println!("\n7. DATOS PEQUEÑOS");
    datos_pequenos::benchmark_pequenos();

    println!("\n8. CONCLUSIONES");
    conclusiones::diagrama_conclusiones();

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                    ✅ TODOS LOS TESTS PASARON                     ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");
}

// ============================================================================
//                              DATOS DE PRUEBA
// ============================================================================

use std::mem;
use std::time::Instant;

/// Struct grande para hacer diferencias notables (60 bytes)
#[derive(Clone, Copy)]
struct LargeData {
    data: [i8; 60],
}

impl LargeData {
    fn new() -> Self {
        LargeData { data: [0; 60] }
    }
}

/// Struct pequeño (8 bytes) para comparar costo de indirección
#[derive(Clone, Copy)]
struct SmallData {
    val: i64,
}

impl SmallData {
    fn new() -> Self {
        SmallData { val: 42 }
    }
}

// ============================================================================
//                         CONCEPTOS STACK VS HEAP
// ============================================================================

/// Conceptos fundamentales de Stack vs Heap
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                        STACK vs HEAP - MEMORIA                            ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  STACK (Pila)                       HEAP (Montículo)                      ║
/// ║  ═══════════════                    ═════════════════                     ║
/// ║                                                                           ║
/// ║  ┌─────────────────┐                ┌─────────────────────────────────┐   ║
/// ║  │ variable local  │ ← SP (Stack    │  ┌───────┐     ┌───────┐       │   ║
/// ║  ├─────────────────┤    Pointer)    │  │ Box A │     │ Vec   │       │   ║
/// ║  │ variable local  │                │  └───────┘     └───────┘       │   ║
/// ║  ├─────────────────┤                │       ┌───────────┐            │   ║
/// ║  │ return address  │                │       │  String   │            │   ║
/// ║  ├─────────────────┤                │       └───────────┘            │   ║
/// ║  │ parámetros fn   │                │  (memoria dispersa, fragmentada)│   ║
/// ║  └─────────────────┘                └─────────────────────────────────┘   ║
/// ║         │                                     ▲                           ║
/// ║         ▼ crece hacia abajo                   │ crece dinámicamente      ║
/// ║                                                                           ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  CARACTERÍSTICAS:                                                         ║
/// ║                                                                           ║
/// ║  Stack:                              Heap:                                ║
/// ║  • Tamaño fijo en compilación        • Tamaño dinámico en runtime         ║
/// ║  • Muy rápido (solo mueve SP)        • Lento (malloc/free + bookkeeping)  ║
/// ║  • LIFO (Last In, First Out)         • Orden arbitrario                   ║
/// ║  • Automático (scope = lifetime)     • Manual o GC (Drop en Rust)         ║
/// ║  • Excelente cache locality          • Pobre cache locality               ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod conceptos {
    //! Módulo de conceptos - solo documentación, sin código ejecutable

    #[test]
    pub fn diagrama_conceptos() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║                    STACK vs HEAP - Conceptos                      ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║                                                                   ║
        // ║  STACK                            HEAP                            ║
        // ║  ─────                            ────                            ║
        // ║  • Asignación: O(1) instantánea   • Asignación: O(n) malloc       ║
        // ║  • Solo mover stack pointer       • Buscar bloque libre           ║
        // ║  • Liberación: automática         • Liberación: Drop trait        ║
        // ║                                                                   ║
        // ║  CUÁNDO USAR CADA UNO:                                            ║
        // ║                                                                   ║
        // ║  Stack ✓                          Heap ✓                          ║
        // ║  • Datos pequeños (<64 bytes)     • Datos muy grandes             ║
        // ║  • Vida corta (scope local)       • Vida dinámica/desconocida     ║
        // ║  • Muchas instancias rápidas      • Trait objects (dyn Trait)     ║
        // ║  • Rendimiento crítico            • Estructuras recursivas        ║
        // ║                                   • Compartir entre threads       ║
        // ║                                                                   ║
        // ╚═══════════════════════════════════════════════════════════════════╝
        println!("✅ conceptos::diagrama_conceptos");
    }
}

// ============================================================================
//                         ANÁLISIS DE TAMAÑO
// ============================================================================

/// Tamaño en memoria: Stack directo vs Box (puntero a heap)
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                    TAMAÑO EN MEMORIA - Stack vs Box                       ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  let data = LargeData::new();       let boxed = Box::new(LargeData::new())║
/// ║                                                                           ║
/// ║  STACK:                             STACK:         HEAP:                  ║
/// ║  ┌────────────────────┐             ┌────────┐     ┌────────────────────┐ ║
/// ║  │   LargeData        │             │ ptr ───│────►│   LargeData        │ ║
/// ║  │   [60 bytes]       │             │ 8 bytes│     │   [60 bytes]       │ ║
/// ║  └────────────────────┘             └────────┘     └────────────────────┘ ║
/// ║                                                                           ║
/// ║  size_of_val(&data) = 60            size_of_val(&boxed) = 8              ║
/// ║                                     (solo el puntero en stack)            ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod tamano {
    use super::*;

    #[test]
    pub fn analisis_tamano() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║                      ANÁLISIS DE TAMAÑO                           ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║  LargeData en Stack:     60 bytes (datos completos)               ║
        // ║  Box<LargeData> en Stack: 8 bytes (solo puntero, 64-bit)          ║
        // ║  Datos de Box en Heap:   60 bytes (donde realmente viven)         ║
        // ╚═══════════════════════════════════════════════════════════════════╝

        let stack_instance = LargeData::new();
        let boxed_instance = Box::new(LargeData::new());

        // Stack: datos completos en stack
        assert_eq!(mem::size_of_val(&stack_instance), 60);

        // Box: solo puntero en stack (8 bytes en 64-bit)
        assert_eq!(mem::size_of_val(&boxed_instance), 8);

        // Tipo LargeData siempre es 60 bytes
        assert_eq!(mem::size_of::<LargeData>(), 60);

        println!(
            "Stack instance: {} bytes",
            mem::size_of_val(&stack_instance)
        );
        println!(
            "Box (puntero):  {} bytes",
            mem::size_of_val(&boxed_instance)
        );
        println!("LargeData tipo: {} bytes", mem::size_of::<LargeData>());
        println!("✅ tamano::analisis_tamano");
    }
}

// ============================================================================
//                      PERFORMANCE DE ASIGNACIÓN
// ============================================================================

/// Comparación de velocidad de creación: Stack vs Heap
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                    ASIGNACIÓN - Stack vs Heap                             ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  STACK ALLOCATION:                  HEAP ALLOCATION (Box::new):           ║
/// ║                                                                           ║
/// ║  sub rsp, 60    ← 1 instrucción     1. Llamar malloc()                    ║
/// ║                   (mover SP)        2. Buscar bloque libre en freelist    ║
/// ║                                     3. Actualizar metadata del allocator  ║
/// ║                                     4. Posible syscall si no hay memoria  ║
/// ║                                     5. Retornar puntero                   ║
/// ║                                                                           ║
/// ║  Tiempo: ~1 nanosegundo             Tiempo: ~20-100+ nanosegundos         ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod asignacion {
    use super::*;

    fn stack_allocation() {
        let _data = black_box(LargeData::new());
    }

    fn heap_allocation() {
        let _data = black_box(Box::new(LargeData::new()));
    }

    #[test]
    pub fn benchmark_asignacion() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║                  BENCHMARK: CREACIÓN                              ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║  Stack: Solo mueve stack pointer (instantáneo)                    ║
        // ║  Heap:  malloc() + bookkeeping (costoso)                          ║
        // ║  Resultado esperado: Heap 10-50x más lento                        ║
        // ╚═══════════════════════════════════════════════════════════════════╝

        let iterations = 1_000_000;

        let start = Instant::now();
        for _ in 0..iterations {
            stack_allocation();
        }
        let duration_stack = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            heap_allocation();
        }
        let duration_heap = start.elapsed();

        let stack_nanos = duration_stack.as_nanos().max(1) as f64;
        let ratio = duration_heap.as_nanos() as f64 / stack_nanos;

        println!("Stack: {:?} ({} iteraciones)", duration_stack, iterations);
        println!("Heap:  {:?} ({} iteraciones)", duration_heap, iterations);
        println!("Ratio: {:.2}x (Heap es más lento)", ratio);

        // Heap debería ser significativamente más lento
        assert!(ratio > 1.0, "Heap debería ser más lento que Stack");

        println!("✅ asignacion::benchmark_asignacion");
    }
}

// ============================================================================
//                      TRANSFERENCIA POR VALOR
// ============================================================================

/// Move/Copy de datos: struct completo vs puntero Box
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                    TRANSFERENCIA POR VALOR                                ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  COPIAR STRUCT (60 bytes):          MOVER BOX (8 bytes puntero):          ║
/// ║                                                                           ║
/// ║  fn process(data: LargeData)        fn process(data: Box<LargeData>)      ║
/// ║                                                                           ║
/// ║  Caller stack:    Callee stack:     Caller stack:    Callee stack:        ║
/// ║  ┌──────────┐     ┌──────────┐      ┌──────────┐     ┌──────────┐         ║
/// ║  │ 60 bytes │────►│ 60 bytes │      │ ptr ─────│─┬──►│ ptr ─────│─┐       ║
/// ║  │ (copia)  │     │ (copia)  │      │ (move)   │ │   │ (move)   │ │       ║
/// ║  └──────────┘     └──────────┘      └──────────┘ │   └──────────┘ │       ║
/// ║                                                  │                │       ║
/// ║  Costo: memcpy(60 bytes)                         ▼                ▼       ║
/// ║                                     HEAP:   ┌────────────────────┐        ║
/// ║                                             │ LargeData 60 bytes │        ║
/// ║                                             └────────────────────┘        ║
/// ║                                     Costo: memcpy(8 bytes) + indirección  ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod transferencia {
    use super::*;

    fn process_on_stack(mut data: LargeData) {
        data.data[0] = 1;
        black_box(data);
    }

    fn process_boxed(mut data: Box<LargeData>) {
        data.data[0] = 1;
        black_box(data);
    }

    #[test]
    pub fn benchmark_transferencia() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║            BENCHMARK: CREAR + TRANSFERIR                          ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║  Stack: crear (rápido) + copiar 60 bytes                          ║
        // ║  Heap:  malloc (lento) + copiar 8 bytes puntero                   ║
        // ║  Trade-off: costo de malloc vs costo de copiar struct grande      ║
        // ╚═══════════════════════════════════════════════════════════════════╝

        let iterations = 1_000_000;

        let start = Instant::now();
        for _ in 0..iterations {
            let data = LargeData::new();
            process_on_stack(black_box(data));
        }
        let duration_stack = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let data = Box::new(LargeData::new());
            process_boxed(black_box(data));
        }
        let duration_heap = start.elapsed();

        let move_ratio = duration_stack.as_nanos() as f64 / duration_heap.as_nanos().max(1) as f64;

        println!("Stack (crear + copiar 60 bytes): {:?}", duration_stack);
        println!("Heap  (malloc + mover puntero):  {:?}", duration_heap);

        if move_ratio > 1.0 {
            println!(
                "Stack {:.2}x MÁS LENTO (copia domina sobre malloc)",
                move_ratio
            );
        } else {
            println!(
                "Heap {:.2}x MÁS LENTO (malloc domina sobre copia)",
                1.0 / move_ratio
            );
        }

        println!("✅ transferencia::benchmark_transferencia");
    }
}

// ============================================================================
//                      COSTO DE ACCESO (INDIRECCIÓN)
// ============================================================================

/// Acceso a datos: directo en stack vs indirección a través de puntero
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                    COSTO DE ACCESO - Indirección                          ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  ACCESO STACK (directo):            ACCESO HEAP (indirección):            ║
/// ║                                                                           ║
/// ║  mov rax, [rsp+offset]              mov rax, [rsp+offset]  ; cargar ptr   ║
/// ║  (1 instrucción)                    mov rax, [rax]         ; seguir ptr   ║
/// ║                                     (2 instrucciones + posible cache miss)║
/// ║                                                                           ║
/// ║  Stack:                             Heap:                                 ║
/// ║  ┌──────────┐                       ┌──────────┐                          ║
/// ║  │ data[0]  │ ← acceso directo      │ ptr ─────│───┐                      ║
/// ║  └──────────┘                       └──────────┘   │                      ║
/// ║                                                    ▼                      ║
/// ║                                     ┌──────────────────┐                  ║
/// ║                                     │ data[0] ← extra  │                  ║
/// ║                                     │           acceso │                  ║
/// ║                                     └──────────────────┘                  ║
/// ║                                                                           ║
/// ║  Cache locality: Excelente          Cache locality: Pobre                 ║
/// ║  (datos contiguos en stack)         (datos dispersos en heap)             ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod acceso {
    use super::*;

    #[test]
    pub fn benchmark_acceso() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║             BENCHMARK: ACCESO PURO (sin ownership)                ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║  Stack: acceso directo, excelente cache locality                  ║
        // ║  Heap:  indirección (seguir puntero), posible cache miss          ║
        // ╚═══════════════════════════════════════════════════════════════════╝

        let iterations = 1_000_000;
        let stack_instances: Vec<LargeData> = (0..100).map(|_| LargeData::new()).collect();
        let box_instances: Vec<Box<LargeData>> =
            (0..100).map(|_| Box::new(LargeData::new())).collect();

        // Acceso stack (referencia directa)
        let start = Instant::now();
        for _ in 0..(iterations / 100) {
            for i in 0..100 {
                let val = black_box(&stack_instances[i]).data[0];
                black_box(val);
            }
        }
        let duration_stack = start.elapsed();

        // Acceso heap (indirección)
        let start = Instant::now();
        for _ in 0..(iterations / 100) {
            for i in 0..100 {
                let val = black_box(&box_instances[i]).data[0];
                black_box(val);
            }
        }
        let duration_heap = start.elapsed();

        let access_ratio =
            duration_heap.as_nanos() as f64 / duration_stack.as_nanos().max(1) as f64;

        println!("Stack (acceso directo):    {:?}", duration_stack);
        println!("Heap  (acceso indirecto):  {:?}", duration_heap);

        if access_ratio > 1.0 {
            println!(
                "Heap {:.2}x MÁS LENTO (costo de indirección + cache)",
                access_ratio
            );
        } else {
            println!("Rendimiento similar ({:.2}x)", access_ratio);
        }

        println!("✅ acceso::benchmark_acceso");
    }
}

// ============================================================================
//                      HEAP RESERVADO (REUSO)
// ============================================================================

/// Comparación: crear en stack vs escribir en heap ya reservado
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                    HEAP RESERVADO - Reuso de memoria                      ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  CREAR EN STACK (cada vez):         HEAP RESERVADO (reuso):               ║
/// ║                                                                           ║
/// ║  loop {                             let mut reserved = Box::new(...);     ║
/// ║      let data = LargeData::new();   loop {                                ║
/// ║      // usa data                        reserved.data = [0; 60];          ║
/// ║  }                                      // usa reserved                   ║
/// ║                                     }                                     ║
/// ║                                                                           ║
/// ║  Cada iteración:                    Cada iteración:                       ║
/// ║  • Ajustar SP (instantáneo)         • Escribir a través de puntero        ║
/// ║  • Inicializar datos                • Inicializar datos                   ║
/// ║                                     • (NO hay malloc!)                    ║
/// ║                                                                           ║
/// ║  Trade-off: Stack más rápido para crear, pero Heap reservado evita        ║
/// ║             el costo de malloc en escenarios de reuso intensivo           ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod heap_reservado {
    use super::*;

    #[test]
    pub fn benchmark_reuso() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║           BENCHMARK: STACK vs HEAP RESERVADO                      ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║  Stack: crear en stack cada vez                                   ║
        // ║  Heap reservado: reusar memoria ya asignada                       ║
        // ╚═══════════════════════════════════════════════════════════════════╝

        let iterations = 1_000_000;

        // Stack: crear nuevo cada vez
        let start = Instant::now();
        for _ in 0..iterations {
            let data = black_box(LargeData::new());
            black_box(data);
        }
        let duration_stack = start.elapsed();

        // Heap reservado: reusar memoria
        let mut reserved_space = Box::new(LargeData::new());
        let start = Instant::now();
        for _ in 0..iterations {
            reserved_space.data = [0; 60];
            black_box(&reserved_space);
        }
        let duration_reserved = start.elapsed();

        let stack_nanos = duration_stack.as_nanos().max(1) as f64;
        let ratio_reserved = duration_reserved.as_nanos() as f64 / stack_nanos;

        println!("Stack (crear cada vez):    {:?}", duration_stack);
        println!("Heap reservado (reusar):   {:?}", duration_reserved);

        if ratio_reserved > 1.0 {
            println!(
                "Heap reservado {:.2}x MÁS LENTO (indirección)",
                ratio_reserved
            );
        } else {
            println!("Rendimiento similar ({:.2}x)", ratio_reserved);
        }

        println!("✅ heap_reservado::benchmark_reuso");
    }
}

// ============================================================================
//                      DATOS PEQUEÑOS (8 bytes)
// ============================================================================

/// Costo de indirección con datos pequeños (8 bytes)
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                    DATOS PEQUEÑOS - Costo puro de indirección             ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  Para datos grandes: el costo de copiar domina                            ║
/// ║  Para datos pequeños: vemos el costo PURO del puntero                     ║
/// ║                                                                           ║
/// ║  SmallData (8 bytes):               Box<SmallData>:                       ║
/// ║  ┌──────────┐                       ┌──────────┐  ┌──────────┐            ║
/// ║  │ i64: 42  │                       │ ptr ─────│─►│ i64: 42  │            ║
/// ║  │ 8 bytes  │                       │ 8 bytes  │  │ 8 bytes  │            ║
/// ║  └──────────┘                       └──────────┘  └──────────┘            ║
/// ║                                                                           ║
/// ║  Mismo tamaño de copia (8 bytes), pero Box tiene indirección extra        ║
/// ║  → Muestra el costo real de seguir punteros                               ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod datos_pequenos {
    use super::*;

    #[test]
    pub fn benchmark_pequenos() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║          BENCHMARK: DATOS PEQUEÑOS (costo puro indirección)       ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║  8 bytes = mismo costo de copia para Stack y puntero Box          ║
        // ║  Diferencia = costo puro de indirección                           ║
        // ╚═══════════════════════════════════════════════════════════════════╝

        let iterations = 1_000_000;

        // Stack: crear 8 bytes
        let start = Instant::now();
        for _ in 0..iterations {
            let _x = black_box(SmallData::new());
        }
        let duration_stack = start.elapsed();

        // Heap reservado: escribir 8 bytes
        let mut reserved_small = Box::new(SmallData::new());
        let start = Instant::now();
        for _ in 0..iterations {
            reserved_small.val = 42;
            black_box(&reserved_small);
        }
        let duration_reserved = start.elapsed();

        let ratio = duration_reserved.as_nanos() as f64 / duration_stack.as_nanos().max(1) as f64;

        println!("Stack (crear 8 bytes):     {:?}", duration_stack);
        println!("Heap (escribir 8 bytes):   {:?}", duration_reserved);

        if ratio > 1.0 {
            println!("Heap {:.2}x MÁS LENTO (costo puro de indirección)", ratio);
        } else {
            println!("Rendimiento similar ({:.2}x)", ratio);
        }

        println!("✅ datos_pequenos::benchmark_pequenos");
    }
}

// ============================================================================
//                          CONCLUSIONES
// ============================================================================

/// Conclusiones y reglas prácticas
///
/// ```text
/// ╔═══════════════════════════════════════════════════════════════════════════╗
/// ║                         CONCLUSIONES FINALES                              ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  1. CREACIÓN (malloc vs stack bump):                                      ║
/// ║     • Stack: Instantáneo (solo ajustar stack pointer)                     ║
/// ║     • Heap:  Lento (malloc + bookkeeping del allocator)                   ║
/// ║     • Diferencia: 10-50x a favor de Stack                                 ║
/// ║                                                                           ║
/// ║  2. TRANSFERENCIA POR VALOR:                                              ║
/// ║     • Datos pequeños (<16 bytes): Stack y Box similar                     ║
/// ║     • Datos grandes (60+ bytes): Copiar costoso, mejor &T o Box           ║
/// ║     • Heap incluye malloc si Box es temporal                              ║
/// ║                                                                           ║
/// ║  3. ACCESO (ya creado):                                                   ║
/// ║     • Stack: Directo, excelente cache locality                            ║
/// ║     • Heap:  Indirección (seguir puntero), posible cache miss             ║
/// ║     • Diferencia: Heap ~1.5-3x más lento en acceso                        ║
/// ║                                                                           ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                         REGLAS PRÁCTICAS                                  ║
/// ╠═══════════════════════════════════════════════════════════════════════════╣
/// ║                                                                           ║
/// ║  USA STACK para:                    USA BOX/HEAP para:                    ║
/// ║  ─────────────────                  ──────────────────                    ║
/// ║  • Datos pequeños (<64 bytes)       • Datos muy grandes (>KB)             ║
/// ║  • Vida corta (scope local)         • trait objects (dyn Trait)           ║
/// ║  • Muchas instancias rápidas        • Estructuras recursivas              ║
/// ║  • Hot paths críticos               • Vida desconocida en compilación     ║
/// ║                                     • Compartir ownership (Rc/Arc)        ║
/// ║                                                                           ║
/// ║  USA REFERENCIAS (&T) para:         USA POOL/ARENA para:                  ║
/// ║  ───────────────────────            ─────────────────────                 ║
/// ║  • Evitar copias sin malloc         • Muchos allocs similares             ║
/// ║  • Préstamo temporal                • Cuando malloc domina el costo       ║
/// ║  • Pasar datos grandes a fn         • Game objects, ECS entities          ║
/// ║                                                                           ║
/// ╚═══════════════════════════════════════════════════════════════════════════╝
/// ```
#[cfg(test)]
mod conclusiones {
    #[test]
    pub fn diagrama_conclusiones() {
        // ╔═══════════════════════════════════════════════════════════════════╗
        // ║                    RESUMEN DECISIONES                             ║
        // ╠═══════════════════════════════════════════════════════════════════╣
        // ║                                                                   ║
        // ║  ¿Tamaño conocido en compilación?                                 ║
        // ║       │                                                           ║
        // ║       ├── Sí, pequeño (<64 bytes) ──► STACK                       ║
        // ║       │                                                           ║
        // ║       ├── Sí, grande (>1KB) ────────► Box (heap) o &T             ║
        // ║       │                                                           ║
        // ║       └── No (dinámico) ───────────► Vec, String, Box             ║
        // ║                                                                   ║
        // ║  ¿Necesitas ownership compartido?                                 ║
        // ║       │                                                           ║
        // ║       ├── Sí, single thread ───────► Rc<T>                        ║
        // ║       │                                                           ║
        // ║       └── Sí, multi thread ────────► Arc<T>                       ║
        // ║                                                                   ║
        // ║  ¿Muchos allocs del mismo tipo?                                   ║
        // ║       │                                                           ║
        // ║       └── Sí ──────────────────────► Pool/Arena allocator         ║
        // ║                                                                   ║
        // ╚═══════════════════════════════════════════════════════════════════╝
        println!("✅ conclusiones::diagrama_conclusiones");
    }
}

// ============================================================================
//                              ÍNDICE
// ============================================================================

fn main() {
    println!("Ejecutar: cargo test --bin 1_stack_vs_heap -- --nocapture");
    println!("Índice:   cargo test --bin 1_stack_vs_heap indice -- --nocapture");
}
