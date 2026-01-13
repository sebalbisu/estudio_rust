// Demo ejecutable que muestra las diferentes estrategias de módulos

use crate::modules_demo::domain;
use crate::modules_demo::hybrid;

/*
RESUMEN DE ESTRATEGIAS:

┌─────────────────────────────────────────────────────────────────┐
│                     MONOLITHIC (1 archivo)                       │
├─────────────────────────────────────────────────────────────────┤
│ monolithic.rs                                                    │
│   ├── User, UserRepo, UserService                               │
│   ├── Order, OrderRepo, OrderService                            │
│   └── Payment, PaymentRepo, PaymentService                      │
│                                                                  │
│ ✓ Simple para código pequeño (<200 líneas)                      │
│ ✗ No escala, merge conflicts, acoplamiento                      │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                  DOMAIN (Por Feature/Vertical)                   │
├─────────────────────────────────────────────────────────────────┤
│ domain/                                                          │
│   ├── user.rs    → User + UserRepo + UserService                │
│   ├── order.rs   → Order + OrderRepo + OrderService             │
│   └── payment.rs → Payment + PaymentRepo + PaymentService       │
│                                                                  │
│ ✓ Alta cohesión, bajo acoplamiento                              │
│ ✓ Ideal para microservicios/DDD                                 │
│ ✗ Puede duplicar código común                                   │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│              HYBRID (Dominio + Capas Internas)                   │
├─────────────────────────────────────────────────────────────────┤
│ hybrid/                                                          │
│   └── user/                                                      │
│       ├── model.rs      → User struct                            │
│       ├── repository.rs → UserRepository                         │
│       └── service.rs    → UserService                            │
│                                                                  │
│ ✓ Mejor separación de responsabilidades                         │
│ ✓ Testabilidad máxima                                            │
│ ✓ Escalable para proyectos grandes                              │
│ ✗ Más archivos, overhead inicial                                │
└─────────────────────────────────────────────────────────────────┘

GUÍA DE DECISIÓN:

Tamaño del proyecto:
- <200 líneas      → monolithic
- 200-500 líneas   → domain
- >500 líneas      → hybrid

Equipo:
- 1 desarrollador  → monolithic o domain
- 2-5 devs         → domain
- >5 devs          → hybrid

Complejidad:
- CRUD simple      → domain
- Lógica compleja  → hybrid
- Microservicios   → domain

REGLAS GENERALES:

1. EMPEZAR SIMPLE
   - Comenzar con domain/
   - Migrar a hybrid/ cuando un dominio crece >300 líneas

2. PRIVADO POR DEFECTO
   - Solo exponer API pública en mod.rs
   - Detalles internos quedan privados

3. UN CONCEPTO = UN ARCHIVO
   - User en user.rs o user/model.rs
   - No mezclar User y Order en el mismo archivo

4. TESTS JUNTO AL CÓDIGO
   - #[cfg(test)] mod tests en el mismo archivo
   - Tests de integración en tests/

5. RE-EXPORTS LIMPIOS
   - mod.rs hace re-exports para API limpia
   - use crate::domain::User; (no domain::user::User)

6. TESTS
   - Unit tests → inline con #[cfg(test)] mod tests
   - Integration tests → carpeta tests/ en raíz
   - Tests muy grandes → archivo separado con #[path = "..."]

   ESTRUCTURA RECOMENDADA:
   ┌─────────────────────────────────────────────────────────────────┐
   │ src/                                                            │
   │   └── domain/                                                   │
   │       └── user.rs          ← Unit tests inline (#[cfg(test)])  │
   │                                                                 │
   │ tests/                     ← Integration tests (API pública)   │
   │   ├── user_integration_test.rs                                 │
   │   └── common/                                                   │
   │       └── mod.rs           ← Helpers compartidos               │
   └─────────────────────────────────────────────────────────────────┘

   EJEMPLO UNIT TEST (inline):
   ┌─────────────────────────────────────────────────────────────────┐
   │ // user.rs                                                      │
   │ pub struct User { ... }                                         │
   │                                                                 │
   │ #[cfg(test)]                                                    │
   │ mod tests {                                                     │
   │     use super::*;                                               │
   │                                                                 │
   │     #[test]                                                     │
   │     fn test_valid_email() {                                     │
   │         assert!(User::is_valid_email("test@example.com"));      │
   │     }                                                           │
   │ }                                                               │
   └─────────────────────────────────────────────────────────────────┘

   EJEMPLO INTEGRATION TEST (tests/):
   ┌─────────────────────────────────────────────────────────────────┐
   │ // tests/user_integration_test.rs                               │
   │ use estudio_01::domain::User;                                   │
   │                                                                 │
   │ #[test]                                                         │
   │ fn test_user_workflow() {                                       │
   │     let user = User::new(1, "Test".into(), "test@mail.com".into());
   │     assert!(User::is_valid_email(&user.email));                 │
   │ }                                                               │
   └─────────────────────────────────────────────────────────────────┘

   ARCHIVO SEPARADO (tests grandes):
   ┌─────────────────────────────────────────────────────────────────┐
   │ src/domain/                                                     │
   │   ├── user.rs                                                   │
   │   └── user_tests.rs        ← Tests en archivo separado         │
   │                                                                 │
   │ // user.rs                                                      │
   │ pub struct User { ... }                                         │
   │                                                                 │
   │ #[cfg(test)]                                                    │
   │ #[path = "user_tests.rs"]                                       │
   │ mod tests;                                                      │
   └─────────────────────────────────────────────────────────────────┘

   COMPARACIÓN:
   ┌────────────────┬──────────────┬───────────────────────────────┐
   │ Tipo           │ Ubicación    │ Acceso                        │
   ├────────────────┼──────────────┼───────────────────────────────┤
   │ Unit tests     │ inline       │ pub + privado (use super::*)  │
   │ Unit separado  │ _tests.rs    │ pub + privado (use super::*)  │
   │ Integration    │ tests/       │ Solo pub (API externa)        │
   └────────────────┴──────────────┴───────────────────────────────┘
*/

fn main() {
    println!("=== DEMOSTRACIÓN DE ESTRATEGIAS DE MÓDULOS EN RUST ===\n");

    // ============================================================
    // 1. ESTRATEGIA POR DOMINIO (domain/)
    // ============================================================
    println!("--- 1. ESTRATEGIA POR DOMINIO ---");
    println!("Todo relacionado a User en un archivo: domain/user.rs\n");

    let mut user_service = domain::UserService::new();

    let user1 = user_service
        .create_user("Alice".to_string(), "alice@example.com".to_string())
        .unwrap();
    println!("✓ Usuario creado: {:?}", user1);

    let user2 = user_service
        .create_user("Bob".to_string(), "bob@example.com".to_string())
        .unwrap();
    println!("✓ Usuario creado: {:?}", user2);

    // Crear orden usando domain::order
    let mut order_service = domain::OrderService::new();
    let items = vec![domain::OrderItem {
        product_id: 101,
        quantity: 2,
        price: 25.50,
    }];

    let order = order_service.create_order(user1.id, items).unwrap();
    println!("✓ Orden creada: ID={}, Total=${:.2}", order.id, order.total);

    let all_users = user_service.get_all_users();
    println!("\n📋 Total de usuarios: {}", all_users.len());

    // ============================================================
    // 2. ESTRATEGIA HÍBRIDA (hybrid/)
    // ============================================================
    println!("\n--- 2. ESTRATEGIA HÍBRIDA ---");
    println!("Dominio User separado en capas: model.rs, repository.rs, service.rs\n");

    let mut hybrid_service = hybrid::UserService::new();

    let user3 = hybrid_service
        .create_user("Charlie".to_string(), "charlie@example.com".to_string())
        .unwrap();
    println!("✓ Usuario creado: {:?}", user3);

    hybrid_service
        .update_email(user3.id, "charlie.new@example.com".to_string())
        .unwrap();
    println!("✓ Email actualizado");

    let updated_user = hybrid_service.get_user(user3.id).unwrap();
    println!("✓ Usuario después de actualización: {:?}", updated_user);

    println!(
        "\n📋 Total de usuarios (hybrid): {}",
        hybrid_service.user_count()
    );

    // ============================================================
    // 3. COMPARACIÓN DE IMPORTS
    // ============================================================
    println!("\n--- 3. COMPARACIÓN DE IMPORTS ---\n");

    println!("DOMAIN (Vertical Slicing):");
    println!("  use modules_demo::domain::{{User, UserService}};");
    println!("  use modules_demo::domain::{{Order, OrderService}};");
    println!("  ✓ API limpia, todo relacionado a User junto\n");

    println!("HYBRID (Dominio + Capas):");
    println!("  use modules_demo::hybrid::{{User, UserService}};");
    println!("  // Repository NO está expuesto (implementación interna)");
    println!("  ✓ API más limpia, detalles internos ocultos\n");

    // ============================================================
    // 4. VENTAJAS Y DESVENTAJAS
    // ============================================================
    println!("--- 4. CUÁNDO USAR CADA ESTRATEGIA ---\n");

    println!("MONOLÍTICO (1 archivo):");
    println!("  ✓ Scripts <200 líneas");
    println!("  ✓ Prototipos rápidos");
    println!("  ✗ No escala, merge conflicts\n");

    println!("POR DOMINIO (domain/):");
    println!("  ✓ Features independientes");
    println!("  ✓ 200-500 líneas por dominio");
    println!("  ✓ Microservicios/DDD");
    println!("  ✗ Puede duplicar código común\n");

    println!("HÍBRIDO (hybrid/):");
    println!("  ✓ >500 líneas por dominio");
    println!("  ✓ Lógica de negocio compleja");
    println!("  ✓ Múltiples capas (MVC, Clean Architecture)");
    println!("  ✓ Equipos grandes");
    println!("  ✗ Overhead inicial (más archivos)\n");

    // ============================================================
    // 5. RECOMENDACIONES
    // ============================================================
    println!("--- 5. RECOMENDACIONES ---\n");

    println!("1. EMPEZAR SIMPLE:");
    println!("   - Comenzar con domain/ (1 archivo por feature)");
    println!("   - Migrar a hybrid/ cuando crece >300 líneas\n");

    println!("2. PRIVADO POR DEFECTO:");
    println!("   - Solo hacer `pub` lo necesario");
    println!("   - Usar `pub(crate)` para API interna del crate\n");

    println!("3. RE-EXPORTS EN mod.rs:");
    println!("   - Hacer API pública limpia");
    println!("   - Ocultar detalles de implementación\n");

    println!("4. TESTS JUNTO AL CÓDIGO:");
    println!("   - #[cfg(test)] mod tests en mismo archivo");
    println!("   - Tests de integración en tests/\n");

    println!("5. ESTRUCTURA POR PROYECTO:");
    println!("   - CLI tool        → domain/");
    println!("   - Web API         → hybrid/");
    println!("   - Librería        → domain/ o hybrid/");
    println!("   - Microservicio   → domain/\n");

    // ============================================================
    // 6. ORGANIZACIÓN DE TESTS
    // ============================================================
    println!("--- 6. ORGANIZACIÓN DE TESTS ---\n");

    println!("UNIT TESTS (inline, recomendado):");
    println!("   #[cfg(test)]");
    println!("   mod tests {{");
    println!("       use super::*;");
    println!("       #[test]");
    println!("       fn test_valid_email() {{ ... }}");
    println!("   }}");
    println!("   ✓ Tests cerca del código, fácil refactorizar\n");

    println!("INTEGRATION TESTS (tests/ en raíz):");
    println!("   tests/");
    println!("     └── user_integration_test.rs");
    println!("   ✓ Prueban API pública, sin acceso a privado\n");

    println!("UNIT TESTS SEPARADOS (archivos grandes):");
    println!("   src/domain/");
    println!("     ├── user.rs");
    println!("     └── user_tests.rs");
    println!("   En user.rs: #[cfg(test)] #[path = \"user_tests.rs\"] mod tests;");
    println!("   ✓ Separa código de tests cuando son muy grandes\n");

    println!("=== FIN DE LA DEMO ===");
}
