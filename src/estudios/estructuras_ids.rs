#![allow(dead_code)]
#![allow(unused_variables)]

#[test]
fn indice() {
    value_ownership::value_ownership();

    id_reference::id_reference();

    rc_shared::rc_shared();

    arc_shared::arc_shared();

    lifetimes_ref::lifetimes_ref();

    weak_references::weak_references();

    arena_allocation::arena_allocation();

    ecs_pattern::ecs_hashmap::ecs_pattern();
    ecs_pattern::ecs_contiguous::ecs_contiguous_demo();
    ecs_pattern::ecs_sparse_set::test_sparse_set();
}

/*
========================================================================
ENTIDADES RELACIONADAS: id vs value vs Rc vs Arc
========================================================================

    ¿Cómo manejar entidades que pueden pertenecer a múltiples dueños?

    ESCENARIO: Un Producto puede estar en múltiples lugares:
    --------------------------------------------
        - En un Carrito (Cart)
        - En una Orden (Order)
        - En una Wishlist

    ¿Cómo modelamos esta relación?
*/

#[derive(Debug, Clone)]
struct Product {
    id: u64,
    name: String,
    price: f64,
}

impl Product {
    fn new(id: u64, name: &str, price: f64) -> Self {
        Product {
            id,
            name: name.to_string(),
            price,
        }
    }
}

/*
========================================================================
VALUE_OWNERSHIP
========================================================================

    VALUE OWNERSHIP (Copia completa):
    --------------------------------------------
        Cada entidad POSEE su propia copia del producto.
        Simple pero duplica datos en memoria.

        DIAGRAMA:
        ┌─────────────────────────────────────────────────────────────┐
        │  (stack)                (heap 1)              (heap 2)      │
        │  product (original)     cart.items[0]       order.items[0]   │
        │  ┌──────────────┐      ┌──────────────┐   ┌──────────────┐  │
        │  │ id: 1        │      │ id: 1        │   │ id: 1        │  │
        │  │ name: ───────┼──┐   │ name: ───────┼─┐ │ name: ───────┼┐ │
        │  │ price: 999.99│  │   │ price: 999.99│ │ │ price: 999.99││ │
        │  └──────────────┘  │   └──────────────┘ │ └──────────────┘│ │
        │                    ▼                    ▼                 ▼ │
        │              ┌────────┐          ┌────────┐         ┌────────┐
        │              │"Laptop"│          │"Laptop"│         │"Laptop"│
        │              └────────┘          └────────┘         └────────┘
        │              (heap 3)            (heap 4)           (heap 5) │
        │                                                             │
        │  → 3 copias independientes en memoria                       │
        └─────────────────────────────────────────────────────────────┘

    CARACTERÍSTICAS:
    --------------------------------------------
        ✓ Ventajas: Simple, sin lifetimes, cada entidad es independiente
        ✗ Desventajas: Duplica memoria, cambios no se propagan
*/
#[cfg(test)]
mod value_ownership {
    use super::Product;

    #[derive(Debug, Clone)]
    pub struct CartItem {
        pub product: Product, // Ownership: Cart POSEE el producto
        pub quantity: u32,
    }

    #[derive(Debug, Clone)]
    pub struct OrderItem {
        pub product: Product, // Ownership: Order POSEE el producto
        pub quantity: u32,
    }

    #[derive(Debug)]
    pub struct Cart {
        pub items: Vec<CartItem>,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: u64,
        pub items: Vec<OrderItem>,
    }

    #[test]
    pub fn value_ownership() {
        let product = Product::new(1, "Laptop", 999.99);

        // Cada entidad tiene su PROPIA COPIA
        let cart = Cart {
            items: vec![CartItem {
                product: product.clone(), // COPIA
                quantity: 1,
            }],
        };

        let order = Order {
            id: 1,
            items: vec![OrderItem {
                product: product.clone(), // OTRA COPIA
                quantity: 1,
            }],
        };

        println!("  Producto original: {:?}", product);
        println!("  En Cart: {:?}", cart.items[0].product.name);
        println!("  En Order: {:?}", order.items[0].product.name);

        // Verificar que son copias independientes
        let ptr_orig = product.name.as_ptr();
        let ptr_cart = cart.items[0].product.name.as_ptr();
        let ptr_order = order.items[0].product.name.as_ptr();

        println!("\n  Direcciones de memoria (heap del String):");
        println!("    Original: {:p}", ptr_orig);
        println!("    Cart:     {:p}", ptr_cart);
        println!("    Order:    {:p}", ptr_order);
        println!(
            "    ¿Son diferentes? {}",
            ptr_orig != ptr_cart && ptr_cart != ptr_order
        );

        println!("  ✅ value_ownership::value_ownership");
    }
}

/*
========================================================================
ID_REFERENCE
========================================================================

    ID REFERENCE (Solo almacenar IDs):
    --------------------------------------------
        Las entidades solo guardan el ID del producto.
        Los productos viven en un repositorio central.

        DIAGRAMA:
        ┌─────────────────────────────────────────────────────────────┐
        │                                                             │
        │  cart.items[0]       order.items[0]      ProductRepository  │
        │  ┌────────────┐     ┌────────────┐      ┌────────────────┐  │
        │  │ product_id:│     │ product_id:│      │ 1 → Product    │  │
        │  │     1  ────┼─────┼────────────┼─────▶│    id: 1       │  │
        │  │ quantity: 1│     │     1      │      │    name:"Laptop"  │
        │  └────────────┘     └────────────┘      │    price: 899.99  │
        │                                         ├────────────────┤  │
        │  cart.items[1]                          │ 2 → Product    │  │
        │  ┌────────────┐                         │    id: 2       │  │
        │  │ product_id:│                         │    name:"Mouse"│  │
        │  │     2  ────┼────────────────────────▶│    price: 29.99│  │
        │  │ quantity: 2│                         └────────────────┘  │
        │  └────────────┘                                             │
        │                                                             │
        │  → Un solo Product en memoria, múltiples referencias por ID │
        └─────────────────────────────────────────────────────────────┘

    CARACTERÍSTICAS:
    --------------------------------------------
        ✓ Ventajas: Memoria eficiente, cambios se propagan, serializable
        ✗ Desventajas: Lookup en cada acceso, repo debe vivir más
*/
#[cfg(test)]
mod id_reference {
    use super::Product;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct CartItem {
        pub product_id: u64, // Solo el ID, no el objeto
        pub quantity: u32,
    }

    #[derive(Debug, Clone)]
    pub struct OrderItem {
        pub product_id: u64,
        pub quantity: u32,
    }

    #[derive(Debug)]
    pub struct Cart {
        pub items: Vec<CartItem>,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: u64,
        pub items: Vec<OrderItem>,
    }

    // Repositorio central que POSEE los productos
    pub struct ProductRepository {
        products: HashMap<u64, Product>,
    }

    impl ProductRepository {
        pub fn new() -> Self {
            ProductRepository {
                products: HashMap::new(),
            }
        }

        pub fn add(&mut self, product: Product) {
            self.products.insert(product.id, product);
        }

        pub fn get(&self, id: u64) -> Option<&Product> {
            self.products.get(&id)
        }

        pub fn update_price(&mut self, id: u64, new_price: f64) {
            if let Some(product) = self.products.get_mut(&id) {
                product.price = new_price;
            }
        }
    }

    #[test]
    pub fn id_reference() {
        let product1 = Product::new(1, "Laptop", 999.99);
        let product2 = Product::new(2, "Mouse", 29.99);

        let mut repo = ProductRepository::new();
        repo.add(product1.clone());
        repo.add(product2.clone());

        // Cart y Order solo guardan IDs
        let cart = Cart {
            items: vec![
                CartItem {
                    product_id: product1.id,
                    quantity: 1,
                },
                CartItem {
                    product_id: product2.id,
                    quantity: 2,
                },
            ],
        };

        let order = Order {
            id: 1,
            items: vec![OrderItem {
                product_id: product1.id,
                quantity: 1,
            }],
        };

        // Para acceder al producto, buscamos en el repo
        println!("  Cart items:");
        for item in &cart.items {
            if let Some(product) = repo.get(item.product_id) {
                println!(
                    "    - {} x{} = ${:.2}",
                    product.name, item.quantity, product.price
                );
            }
        }

        println!("\n  Order items:");
        for item in &order.items {
            if let Some(product) = repo.get(item.product_id) {
                println!(
                    "    - {} x{} = ${:.2}",
                    product.name, item.quantity, product.price
                );
            }
        }

        // Cambiar precio en repo afecta a todos
        println!("\n  → Actualizando precio de Laptop a $899.99...");
        repo.update_price(1, 899.99);

        println!("  Ahora en Cart:");
        if let Some(product) = repo.get(1) {
            println!(
                "    - {} = ${:.2} (actualizado!)",
                product.name, product.price
            );
        }

        println!("  ✅ id_reference::id_reference");
    }
}

/*
========================================================================
RC_SHARED
========================================================================

    Rc (Reference Counted) - Single-thread:
    --------------------------------------------
        Múltiples dueños del mismo objeto en memoria.
        Contador de referencias, libera cuando llega a 0.

        DIAGRAMA:
        ┌─────────────────────────────────────────────────────────────┐
        │                                                             │
        │  product              cart.items[0]        order.items[0]   │
        │  ┌─────┐              ┌─────┐              ┌─────┐          │
        │  │ Rc  │              │ Rc  │              │ Rc  │          │
        │  │ ptr─┼──────┐       │ ptr─┼──────┐       │ ptr─┼──────┐   │
        │  └─────┘      │       └─────┘      │       └─────┘      │   │
        │               │                    │                    │   │
        │               ▼                    ▼                    ▼   │
        │            ┌─────────────────────────────────────────────┐  │
        │            │              RefCell<Product>               │  │
        │            │  ┌─────────────────────────────────────┐    │  │
        │            │  │ strong_count: 3                     │    │  │
        │            │  │ ┌─────────────────────────────────┐ │    │  │
        │            │  │ │ Product                         │ │    │  │
        │            │  │ │   id: 1                         │ │    │  │
        │            │  │ │   name: "Laptop" ──▶ [heap]     │ │    │  │
        │            │  │ │   price: 899.99                 │ │    │  │
        │            │  │ └─────────────────────────────────┘ │    │  │
        │            │  └─────────────────────────────────────┘    │  │
        │            └─────────────────────────────────────────────┘  │
        │                                                             │
        │  → UN SOLO objeto en memoria, 3 referencias contadas        │
        │  → RefCell permite mutabilidad interior (runtime borrow check)
        └─────────────────────────────────────────────────────────────┘

    CARACTERÍSTICAS:
    --------------------------------------------
        ✓ Ventajas: Un objeto, múltiples dueños, cambios visibles para todos
        ✗ Desventajas: Solo single-thread, overhead de contador, no serializable

    Rc<RefCell<T>>:
    --------------------------------------------
        Rc solo te entrega referencias inmutables (&T)
        RefCell permite mutabilidad interior (runtime borrow check) aun en
        referencias inmutables (&T). Sin esa combinacion product seria readonly.

    SEGURIDAD:
    --------------------------------------------
        Rec<RefCell<T>> no es multi-thread safe:
        - RefCell no es Sync.
        - Rc no es Send ni Sync.
*/
#[cfg(test)]
mod rc_shared {
    use super::Product;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    pub struct CartItem {
        pub product: Rc<RefCell<Product>>, // Shared ownership + mutabilidad
        pub quantity: u32,
    }

    #[derive(Debug, Clone)]
    pub struct OrderItem {
        pub product: Rc<RefCell<Product>>,
        pub quantity: u32,
    }

    #[derive(Debug)]
    pub struct Cart {
        pub items: Vec<CartItem>,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: u64,
        pub items: Vec<OrderItem>,
    }

    #[test]
    pub fn rc_shared() {
        // Crear producto con Rc<RefCell<T>>
        let product = Rc::new(RefCell::new(Product::new(1, "Laptop", 999.99)));

        println!("  Rc strong_count inicial: {}", Rc::strong_count(&product));

        // Cart y Order comparten el MISMO producto
        let cart = Cart {
            items: vec![CartItem {
                product: Rc::clone(&product), // Incrementa contador, no copia datos
                quantity: 1,
            }],
        };

        println!(
            "  Rc strong_count después de Cart: {}",
            Rc::strong_count(&product)
        );

        let order = Order {
            id: 1,
            items: vec![OrderItem {
                product: Rc::clone(&product), // Incrementa contador
                quantity: 1,
            }],
        };

        println!(
            "  Rc strong_count después de Order: {}",
            Rc::strong_count(&product)
        );

        // Verificar que es el MISMO objeto
        println!("\n  Direcciones de memoria:");
        println!("    product:           {:p}", Rc::as_ptr(&product));
        println!(
            "    cart.items[0]:     {:p}",
            Rc::as_ptr(&cart.items[0].product)
        );
        println!(
            "    order.items[0]:    {:p}",
            Rc::as_ptr(&order.items[0].product)
        );
        println!(
            "    ¿Son el mismo? {}",
            Rc::ptr_eq(&product, &cart.items[0].product)
        );

        // Modificar desde cualquier lugar afecta a todos
        println!("\n  Precio original: ${:.2}", product.borrow().price);
        product.borrow_mut().price = 899.99;
        println!("  Precio modificado: ${:.2}", product.borrow().price);
        println!(
            "  Visto desde Cart: ${:.2}",
            cart.items[0].product.borrow().price
        );
        println!(
            "  Visto desde Order: ${:.2}",
            order.items[0].product.borrow().price
        );

        println!("  ✅ rc_shared::rc_shared");
    }
}

/*
========================================================================
ARC_SHARED
========================================================================

    Arc (Atomic Reference Counted) - Multi-thread:
    --------------------------------------------
        Como Rc pero thread-safe. Usa operaciones atómicas.

        DIAGRAMA:
        ┌─────────────────────────────────────────────────────────────┐
        │                                                             │
        │  Thread 1              Thread 2              Thread 3       │
        │  ┌─────┐              ┌─────┐              ┌─────┐          │
        │  │ Arc │              │ Arc │              │ Arc │          │
        │  │ ptr─┼──────┐       │ ptr─┼──────┐       │ ptr─┼──────┐   │
        │  └─────┘      │       └─────┘      │       └─────┘      │   │
        │               │                    │                    │   │
        │               ▼                    ▼                    ▼   │
        │            ┌─────────────────────────────────────────────┐  │
        │            │              RwLock<Product>                │  │
        │            │  ┌─────────────────────────────────────┐    │  │
        │            │  │ atomic_count: 3                     │    │  │
        │            │  │ ┌─────────────────────────────────┐ │    │  │
        │            │  │ │ Product (protegido por RwLock)  │ │    │  │
        │            │  │ │   id: 1                         │ │    │  │
        │            │  │ │   name: "Laptop"                │ │    │  │
        │            │  │ │   price: 849.99                 │ │    │  │
        │            │  │ └─────────────────────────────────┘ │    │  │
        │            │  └─────────────────────────────────────┘    │  │
        │            └─────────────────────────────────────────────┘  │
        │                                                             │
        │  → Thread-safe: múltiples threads pueden acceder            │
        │  → RwLock: múltiples lectores O un escritor                 │
        │  → Operaciones atómicas (más overhead que Rc)               │
        └─────────────────────────────────────────────────────────────┘

    CARACTERÍSTICAS:
    --------------------------------------------
        ✓ Ventajas: Thread-safe, un objeto, múltiples dueños
        ✗ Desventajas: Overhead atómico, locks, no serializable

    COMPARATIVA: Rc<RefCell<T>> vs Arc<RwLock<T>>:
    --------------------------------------------
        1. El costo de la "Atomicidad" (Arc vs Rc):
           Arc debe usar instrucciones atómicas de la CPU (más lentas).

        2. El costo del "Bloqueo" (RwLock vs RefCell):
           RwLock interactúa con el SO (syscalls), RefCell es casi instantáneo.

        3. Claridad de Intención:
           Rc indica objeto local al hilo, Arc indica concurrencia.
*/
#[cfg(test)]
mod arc_shared {
    use super::Product;
    use std::sync::{Arc, RwLock};

    #[derive(Debug, Clone)]
    pub struct CartItem {
        pub product: Arc<RwLock<Product>>, // Thread-safe shared ownership
        pub quantity: u32,
    }

    #[derive(Debug, Clone)]
    pub struct OrderItem {
        pub product: Arc<RwLock<Product>>,
        pub quantity: u32,
    }

    #[derive(Debug)]
    pub struct Cart {
        pub items: Vec<CartItem>,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: u64,
        pub items: Vec<OrderItem>,
    }

    #[test]
    pub fn arc_shared() {
        // Crear producto con Arc<RwLock<T>>
        let product = Arc::new(RwLock::new(Product::new(1, "Laptop", 999.99)));

        println!(
            "  Arc strong_count inicial: {}",
            Arc::strong_count(&product)
        );

        let cart = Cart {
            items: vec![CartItem {
                product: Arc::clone(&product),
                quantity: 1,
            }],
        };

        let order = Order {
            id: 1,
            items: vec![OrderItem {
                product: Arc::clone(&product),
                quantity: 1,
            }],
        };

        println!(
            "  Arc strong_count después de Cart y Order: {}",
            Arc::strong_count(&product)
        );

        // Simular acceso desde múltiples threads
        let product_for_thread = Arc::clone(&product);

        let handle = std::thread::spawn(move || {
            // Modificar precio desde otro thread
            let mut p = product_for_thread.write().unwrap();
            p.price = 849.99;
            println!("  [Thread] Precio modificado a: ${:.2}", p.price);
        });

        handle.join().unwrap();

        // Ver cambio desde thread principal
        println!(
            "  [Main] Precio visto: ${:.2}",
            product.read().unwrap().price
        );
        println!(
            "  [Cart] Precio visto: ${:.2}",
            cart.items[0].product.read().unwrap().price
        );

        println!("  ✅ arc_shared::arc_shared");
    }
}

/*
========================================================================
LIFETIMES_REF: REFERENCIAS CON LIFETIMES (&'a T)
========================================================================

    La forma más "pura" de Rust. No hay contador de referencias ni IDs.
    El compilador garantiza que el dueño viva más que la referencia.

    ✓ Ventajas: Zero-cost, máxima performance.
    ✗ Desventajas: Muy difícil de gestionar en grafos complejos o estructuras
       que deben vivir mucho tiempo (el dueño debe sobrevivir a todos).
*/

#[cfg(test)]
mod lifetimes_ref {
    use super::Product;

    pub struct CartItem<'a> {
        pub product: &'a Product,
        pub quantity: u32,
    }

    #[test]
    pub fn lifetimes_ref() {
        let product = Product::new(1, "Laptop", 999.99);

        // El CartItem solo vive mientras viva 'product'
        let item = CartItem {
            product: &product,
            quantity: 1,
        };

        println!(
            "  [Lifetimes] Producto: {}, Precio: {}",
            item.product.name, item.product.price
        );
        assert_eq!(item.product.id, 1);
    }
}

/*
========================================================================
WEAK_REFERENCES: REFERENCIAS DÉBILES (Weak<T>)
========================================================================

    El Weak trata de agregar un Rc::clone o Arc::clone a pedido
    mientras su contenido (el interior de rc o arc) exista en memoria.

    Si el objeto original muere, la referencia débil ya no puede ser
    "subida" (upgrade) a Rc/Arc.

    ✓ Ventajas:
        Cachés: Puedes tener un Weak en un caché. Si nadie más está usando el objeto (el Arc murió), el caché no debería mantenerlo vivo artificialmente.

        Estructuras Circulares en Threads: Si tienes dos servicios que se necesitan mutuamente pero corren en hilos distintos, uno debe usar Weak para que el sistema pueda cerrarse correctamente (hacer el shutdown) sin quedarse esperando infinitamente al otro.

    ✗ Desventajas: Debes "intentar" (upgrade) convertirlo a Rc/Arc antes
       de usarlo, ya que el objeto podría haber sido liberado.
*/

#[cfg(test)]
mod weak_references {
    use super::Product;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    pub fn weak_references() {
        let product = Rc::new(RefCell::new(Product::new(1, "Laptop", 999.99)));

        // Creamos una referencia débil
        let weak_product: Weak<RefCell<Product>> = Rc::downgrade(&product);

        // Para usarla, debemos intentar "subirla" a Rc
        if let Some(strong_product) = weak_product.upgrade() {
            println!(
                "  [Weak] Producto recuperado: {}",
                strong_product.borrow().name
            );
        }

        // Si el original muere...
        drop(product);

        if weak_product.upgrade().is_none() {
            println!("  [Weak] El producto ya no existe (evitamos dangling pointers)");
        }
    }
}

/*
========================================================================
ARENA_ALLOCATION: ASIGNACIÓN EN ARENA
========================================================================

    Se reservan bloques grandes de memoria donde viven todos los objetos enteros.
    La Arena es la dueña de todo, y nos presta referencias.

    ✓ Ventajas: Extremadamente rápido, permite usar referencias simples (&T)
       porque todos los objetos mueren al mismo tiempo que el "Arena".
    ✗ Desventajas: No puedes liberar objetos individuales fácilmente.
*/

#[cfg(test)]
mod arena_allocation {
    // Nota: En producción usarías crates como 'bumpalo' o 'typed-arena'
    use super::Product;
    use std::cell::RefCell;

    pub struct Arena {
        // Usamos RefCell para permitir 'alloc' con &self
        products: RefCell<Vec<Product>>,
    }

    impl Arena {
        // En una arena real, esto devolvería una referencia que vive tanto como la arena
        // Aquí, para simplificar, devolvemos el ID o simplemente mostramos el concepto.
        // Pero para que compile con &T, necesitamos que el Vec no se mueva.
        fn alloc(&self, p: Product) {
            self.products.borrow_mut().push(p);
        }

        fn get_all(&self) {
            for p in self.products.borrow().iter() {
                println!("  [Arena] Producto: {}", p.name);
            }
        }
    }

    #[test]
    pub fn arena_allocation() {
        let arena = Arena {
            products: RefCell::new(Vec::new()),
        };

        // La arena es dueña de los datos
        arena.alloc(Product::new(1, "Laptop", 999.99));
        arena.alloc(Product::new(2, "Mouse", 25.00));

        arena.get_all();
        assert_eq!(arena.products.borrow().len(), 2);
    }
}

/*
========================================================================
ECS_PATTERN: ENTITY COMPONENT SYSTEM
========================================================================

    Data-Oriented (orientada a los datos) en lugar de orientada a los objetos.

    Arquitectura donde las entidades son solo un número (ID), y los datos
    (Componentes) viven en arrays contiguos en memoria.

    TRADICIONAL (AoS):
        [ ID, Nombre, Precio ] [ ID, Nombre, Precio ] [ ID, Nombre, Precio ]
            ^       ^       ^
            └───────┴───────┴── El CPU carga todo esto aunque solo quiera el precio.

    ECS (SoA):
        Nombres: [ Nombre ] [ Nombre ] [ Nombre ]
        Precios: [ Precio ] [ Precio ] [ Precio ]  <-- El CPU vuela recorriendo esto.

    Puedes procesar miles de precios en un solo ciclo de reloj
    porque están pegados unos a otros en memoria.

    ✓ Ventajas: Cache-friendly, desacoplamiento total, muy escalable.
    ✗ Desventajas: Curva de aprendizaje alta, arquitectura más compleja.
*/

#[cfg(test)]
mod ecs_pattern {

    /*
    ========================================================================
    ECS: IMPLEMENTACIÓN CON HASHMAP
    ========================================================================

    Estructura:
        let mut names: HashMap<u32, String> = HashMap::new();

    Borrar un item de hashmap:

        Cuando haces hashmap.remove(&id), la entrada desaparece. Si iteras el HashMap, ese elemento simplemente no está. No tienes que lidiar con un Option::None como en el Vec<Option<T>>.

        Internamente, un HashMap es un array de "buckets". Cuando borras algo:

        El HashMap marca ese espacio como "vacío" o usa un "tombstone" (una marca que dice "aquí hubo algo").
        El "hueco" físico existe en la memoria RAM, pero el HashMap lo gestiona por ti.
        El problema de performance:
        Aunque el HashMap oculte el hueco, los datos siguen estando dispersos. El CPU no puede predecir dónde está el siguiente elemento porque los hashes son aleatorios. Saltas de una dirección de memoria a otra (esto se llama Pointer Chasing), lo que vacía el cache del CPU.
     */
    pub mod ecs_hashmap {
        use std::collections::HashMap;

        #[test]
        pub fn ecs_pattern() {
            // Las entidades son solo IDs
            let entity_id = 100;

            // Los componentes viven en storages separados
            let mut names: HashMap<u32, String> = HashMap::new();
            let mut prices: HashMap<u32, f64> = HashMap::new();

            names.insert(entity_id, "Laptop".to_string());
            prices.insert(entity_id, 999.99);

            // Accedemos a los datos por ID
            if let (Some(name), Some(price)) = (names.get(&entity_id), prices.get(&entity_id)) {
                println!("  [ECS] Entidad {}: {} cuesta ${}", entity_id, name, price);
            }

            assert_eq!(names.len(), 1);
        }
    }

    /*
    ========================================================================
    ECS_CONTIGUOUS: IMPLEMENTACIÓN CON VECTORES (SoA)
    ========================================================================

        Estructura Interna:

            names: Vec<Option<String>>,

        Aquí los componentes viven en Vecs. El ID de la entidad es el índice.
        Es la forma más rápida para el CPU porque los datos están pegados.

        Sin Hashing:
        No hay que calcular ninguna función matemática para encontrar el dato. Si quieres el precio de la entidad 5, vas directo a prices[5].

        Prefetching del CPU:
        Cuando el CPU ve que estás recorriendo un Vec<f64>, el hardware "adivina" que vas a necesitar el siguiente número y lo trae del RAM al Cache antes de que se lo pidas.

        Densidad de datos: E
        En un HashMap, hay espacios vacíos (buckets) y metadatos. En un Vec<f64>, cada byte del cache está lleno de información útil.

        Borrado con Huecos:
        Si borras una entidad, puedes dejar un None en su lugar. El CPU simplemente salta ese espacio cuando itera.
    */

    pub mod ecs_contiguous {
        // instead of using object, one single World struct with Vecs for components
        // with data from all the entities
        pub struct World {
            // Cada índice es una entidad. Option permite que no todas tengan todo.
            names: Vec<Option<String>>,
            prices: Vec<Option<f64>>,
        }

        impl World {
            fn new() -> Self {
                Self {
                    names: Vec::new(),
                    prices: Vec::new(),
                }
            }

            fn spawn(&mut self, name: &str, price: f64) -> usize {
                let id = self.names.len();
                self.names.push(Some(name.to_string()));
                self.prices.push(Some(price));
                id
            }
        }

        #[test]
        pub fn ecs_contiguous_demo() {
            let mut world = World::new();
            world.spawn("Laptop", 999.99);
            world.spawn("Mouse", 25.00);
            world.spawn("Teclado", 50.00);

            // SISTEMA: Aplicar descuento del 10% a TODO
            // El CPU vuela aquí porque recorre un array de f64 puro y duro.
            for price_opt in world.prices.iter_mut() {
                if let Some(price) = price_opt {
                    *price *= 0.9;
                }
            }

            println!("  [ECS Contiguo] Precios actualizados en bloque.");
            assert_eq!(world.prices[0], Some(899.991));
        }
    }

    /*
    ========================================================================
    ECS_SPARSE_SET: IMPLEMENTACIÓN CON SPARSE SET
    ========================================================================

        Estructura Interna:
            dense: vec<T>
                // con los datos contiguos [T, T, T, T]
            sparse: vec<isize>
                // con los indices en dense para cada entidad
                // [id1 -> idx_dense, id2 -> idx_dense, ...]
                // es un vector con huecos (-1) donde no hay datos ej si id = 1.000.000
                // pone -1 en todas las posiciones vacias hasta llegar a 1.000.000
            entities: vec<usize>
                // inverso de sparce
                // [idx_dense -> id1, idx_dense -> id2, ...]
                // tambien tiene huecos

        Ideal para componentes que se agregan/quitan frecuentemente.
        Al borrar un item, mueves el último al hueco, manteniendo todo pegado.

        Iteración Perfecta:
        El método iter() recorre self.dense. No hay Option, no hay if let Some, no hay saltos. El CPU puede hacer SIMD (procesar varios números a la vez).

        Borrado O(1):
        No importa si tienes 3 o 3 millones de elementos, borrar siempre cuesta lo mismo porque solo intercambias dos posiciones y haces un pop().

        Cache Locality:
        Al no haber huecos, cada vez que el CPU trae una línea de cache de la RAM, el 100

        "Debilidad" del Sparse Set: el consumo de memoria en el array sparse:

            El costo es bajo: Un isize (o u32) ocupa solo 4 u 8 bytes. Tener un array sparse de 1 millón de elementos ocupa unos 4MB o 8MB. Para una PC moderna, eso es insignificante comparado con la velocidad que ganas.

            Velocidad vs Memoria: Prefieres gastar 8MB de RAM para tener acceso
            O ( 1 ) O(1) instantáneo, en lugar de usar un HashMap que es más lento y "ensucia" el cache del CPU.

            Los IDs suelen ser contiguos: En la mayoría de los motores de juegos o sistemas de alta performance, los IDs se reciclan. Si una entidad muere, su ID se guarda para la siguiente, manteniendo los números lo más bajos posible.

        Solucion a Debilidad:
            Si tu aplicación maneja IDs muy dispersos (ej. 1 y 1.000.000), usar paginado o sharding para dividir el sparse en bloques más pequeños puede ayudar a reducir el consumo de memoria.

     */
    pub mod ecs_sparse_set {
        pub struct SparseSet<T> {
            dense: Vec<T>,        // Datos pegados (Cache-friendly)
            entities: Vec<usize>, // ID de la entidad en cada posición del dense
            sparse: Vec<isize>,   // Mapa: ID_Entidad -> Posición en dense (-1 si no existe)
        }

        impl<T> SparseSet<T> {
            pub fn new() -> Self {
                Self {
                    dense: Vec::new(),
                    entities: Vec::new(),
                    sparse: Vec::new(),
                }
            }

            pub fn get(&self, entity_id: usize) -> Option<&T> {
                let idx = self.sparse.get(entity_id)?;
                if *idx == -1 {
                    None
                } else {
                    self.dense.get(*idx as usize)
                }
            }

            pub fn insert(&mut self, entity_id: usize, data: T) {
                // Asegurar espacio en el array sparse
                if entity_id >= self.sparse.len() {
                    self.sparse.resize(entity_id + 1, -1);
                }
                // Guardar índice actual
                self.sparse[entity_id] = self.dense.len() as isize;
                self.dense.push(data);
                self.entities.push(entity_id);
            }

            pub fn remove(&mut self, entity_id: usize) {
                let idx = self.sparse[entity_id] as usize;
                let last_idx = self.dense.len() - 1;

                // 1. Mover el último elemento al hueco del que borramos
                self.dense.swap(idx, last_idx);
                self.entities.swap(idx, last_idx);

                // 2. Actualizar el mapa sparse para la entidad que movimos
                let moved_entity = self.entities[idx];
                self.sparse[moved_entity] = idx as isize;

                // 3. Limpiar
                self.sparse[entity_id] = -1;
                self.dense.pop();
                self.entities.pop();
            }

            pub fn iter(&self) -> impl Iterator<Item = &T> {
                self.dense.iter() // ¡Iteración pura y dura sin Options!
            }
        }

        #[test]
        pub fn test_sparse_set() {
            let mut prices = SparseSet::new();
            prices.insert(0, 100.0);
            prices.insert(1, 200.0);
            prices.insert(2, 300.0);

            prices.remove(1); // Borramos el del medio

            // El array dense ahora tiene [100.0, 300.0] pegados.
            // El CPU no sabe que hubo un borrado, solo ve datos contiguos.
            for price in prices.iter() {
                println!("  [SparseSet] Precio: {}", price);
            }
        }
    }
}
