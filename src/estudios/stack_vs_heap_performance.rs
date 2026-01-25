#![allow(dead_code)]
#![allow(unused)]

use data_types::*;
use std::hint::black_box;
use std::mem;
use std::time::Instant;

#[test]
fn index() {
    size::size_analysis();
    performance::allocation_performance::allocation_benchmark();
    performance::value_transfer::benchmark();
    performance::access::access_benchmark();
    performance::reserved_heap::reuse_benchmark();
    performance::small_data::small_data_benchmark();
}

/*
========================================================================
STACK VS HEAP CONCEPTS
========================================================================

    Visualization:
    -------------------------------------------

        STACK                             HEAP

        ┌─────────────────┐                ┌─────────────────────────────────┐
        │ local variable  │ ← SP (Stack    │  ┌───────┐     ┌───────┐       │
        ├─────────────────┤    Pointer)    │  │ Box A │     │ Vec   │       │
        │ local variable  │                │  └───────┘     └───────┘       │
        ├─────────────────┤                │       ┌───────────┐            │
        │ return address  │                │       │  String   │            │
        ├─────────────────┤                │       └───────────┘            │
        │ fn parameters   │                │  (scattered, fragmented memory)│
        └─────────────────┘                └─────────────────────────────────┘
               │                                     ▲
               ▼ grows downward                      │ grows dynamically

    Characteristics:
    -------------------------------------------

        Stack:                              Heap:
        • Fixed size at compile time        • Dynamic size at runtime
        • Very fast (just moves SP)         • Slow (malloc/free + bookkeeping)
        • LIFO (Last In, First Out)         • Arbitrary order
        • Automatic (scope = lifetime)      • Manual or GC (Drop in Rust)
        • Excellent cache locality          • Poor cache locality
        • Allocation: O(1) instant          • Allocation: O(n) malloc
        • Just move stack pointer           • Find free block
        • Automatic deallocation            • Deallocation: Drop trait

    When to use each:
    -------------------------------------------

        Stack ✓                          Heap ✓
        • Small data (<64 bytes)          • Very large data
        • Short lifetime (local scope)    • Dynamic/unknown lifetime
        • Many fast instances             • Trait objects (dyn Trait)
        • Critical performance            • Recursive structures
                                         • Share between threads
*/

/*
========================================================================
MEMORY SIZE - Stack vs Box
========================================================================

    Concept:
    -------------------------------------------
        Stack: stores complete data on stack
        Heap: stores pointer to heap memory on stack

        let data = LargeData::new();
        let boxed = Box::new(LargeData::new());

    Visualization:
    -------------------------------------------

        STACK:                         STACK:         HEAP:
        ┌────────────────────┐         ┌────────┐     ┌────────────────────┐
        │   LargeData        │         │ ptr ───│────►│   LargeData        │
        │   [60 bytes]       │         │ 8 bytes│     │   [60 bytes]       │
        └────────────────────┘         └────────┘     └────────────────────┘

        size_of_val(&data) = 60            size_of_val(&boxed) = 8
                                           (only the pointer on stack)
*/

pub mod data_types {

    /// Small struct (8 bytes) to compare indirection cost
    #[derive(Clone, Copy)]
    pub struct SmallData {
        pub val: i64,
    }
    impl SmallData {
        pub fn new() -> SmallData {
            SmallData { val: 0 }
        }
    }

    /// Large struct to make noticeable differences (60 bytes)
    #[derive(Clone, Copy)]
    pub struct LargeData {
        pub data: [i8; 60],
    }
    impl LargeData {
        pub fn new() -> LargeData {
            LargeData { data: [0; 60] }
        }
    }

    #[derive(Clone, Copy)]
    pub struct VeryLargeData {
        pub data: [i8; 600],
    }
    impl VeryLargeData {
        pub fn new() -> VeryLargeData {
            VeryLargeData { data: [0; 600] }
        }
    }
}

#[cfg(test)]
mod size {
    use super::*;

    #[test]
    pub fn size_analysis() {
        let stack_instance = LargeData::new();
        let boxed_instance: Box<_> = Box::new(LargeData::new());

        // Stack: complete data on stack
        assert_eq!(mem::size_of_val(&stack_instance), 60);

        // Box: only pointer on stack (8 bytes on 64-bit)
        assert_eq!(mem::size_of_val(&boxed_instance), 8);

        // LargeData type is always 60 bytes
        assert_eq!(mem::size_of::<LargeData>(), 60);
    }
}

/*

========================================================================
PERFORMANCE
========================================================================

    Allocation:
    -------------------------------------------
        stack is 2.4x

    Allocation vs reserved heap:
    -------------------------------------------
        stack is 1.5x faster

    Allocation small data:
    -------------------------------------------
        similar

    Value transfer:
    -------------------------------------------
        heap is 1.8x faster

    Access:
    -------------------------------------------
        similar


    ratios using LargeData (60 bytes) and SmallData (8 bytes) as example

*/
mod performance {
    use super::*;

    /*
    ========================================================================
    ALLOCATION
    ========================================================================
    
        Stack is faster to create than Heap
        Ratio: 2.4x faster stack
    
        Stack Allocation:
        -------------------------------------------
            sub rsp, 60    ← 1 instruction (move SP)
            Time: ~1 nanosecond
    
        Heap Allocation (Box::new):
        -------------------------------------------
            1. Call malloc()
            2. Find free block in freelist
            3. Update allocator metadata
            4. Possible syscall if no memory
            5. Return pointer
            Time: ~20-100+ nanoseconds
    */
    #[cfg(test)]
    pub mod allocation_performance {
        use super::*;
    
        fn stack_allocation() {
            let _data = black_box(LargeData::new());
        }
    
        fn heap_allocation() {
            let _data = black_box(Box::new(LargeData::new()));
        }
    
        // test: stack is faster for allocation
        #[test]
        pub fn allocation_benchmark() {
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
    
            println!("duration_stack {:?}", duration_stack);
            println!("duration_heap {:?}", duration_heap);
            println!("ratio {:.2}", ratio);
    
            assert!(
                duration_stack < duration_heap,
                "Stack should be faster than Heap"
            );
            assert!(ratio > 1.0, "Heap should be slower than Stack");
        }
    }
    
    
    /*
    ========================================================================
    STACK VS RESERVED HEAP (REUSE)
    ========================================================================
    
        stack is faster to create, but reserved heap is faster than heap without preallocated
        Ratio: stack 1.5x faster than reserved heap
    
        Create on Stack (each time):
        -------------------------------------------
    
            Each iteration:
            • Adjust SP (instant)
            • Initialize data
    
        Reserved Heap (reuse):
        -------------------------------------------
    
            Each iteration:
            • Write through pointer
            • Initialize data
            • (NO malloc!)
    
        Trade-off:
        -------------------------------------------
            Stack faster to create, but reserved Heap avoids malloc cost
            in intensive reuse scenarios
    */
    #[cfg(test)]
    pub mod reserved_heap {
        use super::*;
    
        #[test]
        pub fn reuse_benchmark() {
    
            const ITERATIONS: usize = 1_000_000;
    
            let mut x: [i8; ITERATIONS] = [0; ITERATIONS];
            let mut y: Vec<i8> = vec![0; ITERATIONS];

            let start = Instant::now();
            for i in 0..ITERATIONS {
                x[i] = 1;
            }
            let duration_stack = start.elapsed();
    
            
            let start = Instant::now();
            for i in 0..ITERATIONS {
                y.push(1);
            }
            let duration_heap = start.elapsed();
            
            // simulate no preallocated heap of one element in each iteration
            let mut var_no_prealloc = Box::new(1);
            let start = Instant::now();
            for i in 0..ITERATIONS {
                var_no_prealloc = black_box(Box::new(1));
            }
            let duration_heap_no_preallocated = start.elapsed();
    
            let ratio = duration_heap.as_nanos().max(1) as f64 
              / duration_stack.as_nanos() as f64;
            let ratio_no_preallocated = duration_heap_no_preallocated.as_nanos().max(1) as f64 
              / duration_heap.as_nanos() as f64;
    
            println!("Stack (create each time):    {:?}", duration_stack);
            println!("Heap (reuse):       {:?}", duration_heap);
            println!("Heap (no preallocated): {:?}", duration_heap_no_preallocated);
            println!("ratio: {:.2}", ratio);
            println!("ratio_no_preallocated: {:.2}", ratio_no_preallocated);
        }
    }



    /*
    ========================================================================
    SMALL DATA (8 bytes) PERFORMANCE
    ========================================================================
    
        Same performance
    
        Concept:
        -------------------------------------------
            For large data: copy cost dominates
            For small data: we see the PURE pointer cost
    
        Visualization:
        -------------------------------------------
    
            SmallData (8 bytes):               Box<SmallData>:
            ┌──────────┐                       ┌──────────┐  ┌──────────┐
            │ i64: 42  │                       │ ptr ─────│─►│ i64: 42  │
            │ 8 bytes  │                       │ 8 bytes  │  │ 8 bytes  │
            └──────────┘                       └──────────┘  └──────────┘
    
            Same copy size (8 bytes), but Box has extra indirection
            → Shows the real cost of following pointers
    */
    #[cfg(test)]
    pub mod small_data {
        use super::*;
    
        #[test]
        pub fn small_data_benchmark() {
    
            const ITERATIONS: usize = 1_000_000;
    
            let stack_var = (0..ITERATIONS).map(|_| SmallData::new()).collect::<Vec<SmallData>>();
            let heap_var: Vec<Box<SmallData>> = (0..ITERATIONS).map(|_| Box::new(SmallData::new())).collect();

            let start = Instant::now();
            for i in 0..ITERATIONS {
                let _x = stack_var.get(i);
            }
            let duration_stack = start.elapsed();    

            let start = Instant::now();
            for i in 0..ITERATIONS {
                let _x = heap_var.get(i);
            }
            let duration_heap = start.elapsed();
            
            let ratio = duration_heap.as_nanos().max(1) as f64 / duration_stack.as_nanos() as f64;

            println!("Stack (create 8 bytes):     {:?}", duration_stack);
            println!("Heap (write 8 bytes):       {:?}", duration_heap);
            println!("ratio: {:.2}", ratio);
        }
    }

    /*
    ========================================================================
    VALUE TRANSFER PERFORMANCE
    ========================================================================
    
        Moving a pointer is faster than copying a large struct
        Ratio: 1.8x faster heap
    
        Copy Struct (60 bytes):
        -------------------------------------------
            fn process(data: LargeData)
    
            Caller stack:    Callee stack:
            ┌──────────┐     ┌──────────┐
            │ 60 bytes │────►│ 60 bytes │
            │ (copy)   │     │ (copy)   │
            └──────────┘     └──────────┘
    
            Cost: memcpy(60 bytes)
    
        Move Box (8 bytes pointer):
        -------------------------------------------
            fn process(data: Box<LargeData>)
    
            Caller stack:    Callee stack:
            ┌──────────┐     ┌──────────┐
            │ ptr ─────│─┬──►│ ptr ─────│─┐
            │ (move)   │ │   │ (move)   │ │
            └──────────┘ │   └──────────┘ │
                         │                │
                         ▼                ▼
            HEAP:   ┌────────────────────┐
                    │ LargeData 60 bytes │
                    └────────────────────┘
    
            Cost: memcpy(8 bytes) + indirection
    */
    #[cfg(test)]
    pub mod value_transfer {
        use super::*;
    
        fn process_on_stack(mut data: LargeData) {
            data.data[0] = 1;
            black_box(data);
        }
    
        fn process_boxed(mut data: Box<LargeData>) {
            data.data[0] = 1;
            black_box(data);
        }
    
        // Test: move is faster than copy for value transfer
        #[test]
        pub fn benchmark() {
            let iterations = 1_000_000;
    
            let store1 = (0..iterations)
                .map(|_| LargeData::new())
                .collect::<Vec<LargeData>>();
    
            let store2 = (0..iterations)
                .map(|_| Box::new(LargeData::new()))
                .collect::<Vec<Box<LargeData>>>();
    
            let duration_stack = Instant::now(); 
            for data in store1 {
                process_on_stack(data);
            }
            let duration_stack = duration_stack.elapsed();
    
            let duration_heap = Instant::now();
            for data in store2 {
                process_boxed(data);
            }
            let duration_heap = duration_heap.elapsed();   
            
            let move_ratio = duration_stack.as_nanos() as f64 
              / duration_heap.as_nanos().max(1) as f64;
    
            assert!(duration_stack > duration_heap, 
                "copying a large struct should be slower moving a pointer");
            assert!(move_ratio > 1.0, 
                "moving a pointer should be slower than copying a large struct");
    
            println!("Stack (create + copy 60 bytes): {:?}", duration_stack);
            println!("Heap  (malloc + move pointer):  {:?}", duration_heap);
            println!("move_ratio: {:.2}", move_ratio);
    
        }
    }
    
    /*
    ========================================================================
    ACCESS COST (INDIRECTION)
    ========================================================================
    
        Accessing data on stack is little bit faster than accessing data on heap
        Ratio: Similar 1.04x faster stack
    
        Stack Access (direct):
        -------------------------------------------
            mov rax, [rsp+offset]  (1 instruction)
    
            Stack:
            ┌──────────┐
            │ data[0]  │ ← direct access
            └──────────┘
    
            Cache locality: Excellent (contiguous data on stack)
    
        Heap Access (indirection):
        -------------------------------------------
            mov rax, [rsp+offset]  ; load ptr
            mov rax, [rax]         ; follow ptr
            (2 instructions + possible cache miss)
    
            Heap:
            ┌──────────┐
            │ ptr ─────│───┐
            └──────────┘   │
                           ▼
            ┌──────────────────┐
            │ data[0] ← extra  │
            │           access │
            └──────────────────┘
    
            Cache locality: Poor (scattered data on heap)
    */
    #[cfg(test)]
    pub mod access {
        use super::*;
    
        // test: accessing data on stack is faster 
        // than accessing data on heap, for the same data size
        #[test]
        pub fn access_benchmark() {
    
            let iterations = 1_000_000;
    
            let store1 = (0..iterations)
                .map(|_| LargeData::new())
                .collect::<Vec<LargeData>>();
    
            let store2 = (0..iterations)
                .map(|_| Box::new(LargeData::new()))
                .collect::<Vec<Box<LargeData>>>();
    
            let duration_stack = Instant::now(); 
            for data in store1 {
                for index in 0..60 {
                    let _ = black_box(data.data[index]);
                }
            }
            let duration_stack = duration_stack.elapsed();
    
            let duration_heap = Instant::now();
            for data in store2 {
                for index in 0..60 {
                    let _ = black_box(data.data[index]);
                }
            }
            let duration_heap = duration_heap.elapsed();   
            
            let access_ratio = duration_heap.as_nanos().max(1) as f64 / duration_stack.as_nanos() as f64;
    
            assert!(duration_stack < duration_heap, "copying a large struct should be slower moving a pointer");
            assert!(access_ratio > 1.0, "accessing a large struct should be slower than accessing a pointer");
    
            println!("Stack (create + copy 60 bytes): {:?}", duration_stack);
            println!("Heap  (malloc + move pointer):  {:?}", duration_heap);
            println!("access_ratio: {:.2}", access_ratio);
        }
    }

}
