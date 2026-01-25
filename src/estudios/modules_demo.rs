// Executable demo showing different module organization strategies

#[allow(unused_imports)]
use crate::modules_demo::domain;
#[allow(unused_imports)]
use crate::modules_demo::hybrid;

/*
STRATEGY SUMMARY:

┌─────────────────────────────────────────────────────────────────┐
│                     MONOLITHIC (1 file)                         │
├─────────────────────────────────────────────────────────────────┤
│ monolithic.rs                                                    │
│   ├── User, UserRepo, UserService                               │
│   ├── Order, OrderRepo, OrderService                            │
│   └── Payment, PaymentRepo, PaymentService                      │
│                                                                  │
│ ✓ Simple for small code (<200 lines)                            │
│ ✗ Doesn't scale, merge conflicts, coupling                      │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                  DOMAIN (By Feature/Vertical)                   │
├─────────────────────────────────────────────────────────────────┤
│ domain/                                                          │
│   ├── user.rs    → User + UserRepo + UserService                │
│   ├── order.rs   → Order + OrderRepo + OrderService             │
│   └── payment.rs → Payment + PaymentRepo + PaymentService       │
│                                                                  │
│ ✓ High cohesion, low coupling                                   │
│ ✓ Ideal for microservices/DDD                                   │
│ ✗ May duplicate common code                                     │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│              HYBRID (Domain + Internal Layers)                   │
├─────────────────────────────────────────────────────────────────┤
│ hybrid/                                                          │
│   └── user/                                                      │
│       ├── model.rs      → User struct                            │
│       ├── repository.rs → UserRepository                         │
│       └── service.rs    → UserService                            │
│                                                                  │
│ ✓ Better separation of concerns                                 │
│ ✓ Maximum testability                                            │
│ ✓ Scalable for large projects                                   │
│ ✗ More files, initial overhead                                  │
└─────────────────────────────────────────────────────────────────┘

DECISION GUIDE:

Project Size:
|- <200 lines      → monolithic
|- 200-500 lines   → domain
|- >500 lines      → hybrid

Team:
|- 1 developer     → monolithic or domain
|- 2-5 devs        → domain
|- >5 devs         → hybrid

Complexity:
|- Simple CRUD     → domain
|- Complex logic   → hybrid
|- Microservices   → domain

GENERAL RULES:

1. START SIMPLE
   - Start with domain/
   - Migrate to hybrid/ when a domain grows >300 lines

2. PRIVATE BY DEFAULT
   - Only expose public API in mod.rs
   - Internal details remain private

3. ONE CONCEPT = ONE FILE
   - User in user.rs or user/model.rs
   - Don't mix User and Order in the same file

4. TESTS WITH THE CODE
   - #[cfg(test)] mod tests in the same file
   - Integration tests in tests/

5. CLEAN RE-EXPORTS
   - mod.rs does re-exports for clean API
   - use crate::domain::User; (not domain::user::User)

6. TESTS
   - Unit tests → inline with #[cfg(test)] mod tests
   - Integration tests → tests/ folder in root
   - Large tests → separate file with #[path = "..."]

   RECOMMENDED STRUCTURE:
   ┌─────────────────────────────────────────────────────────────────┐
   │ src/                                                            │
   │   └── domain/                                                   │
   │       └── user.rs          ← Unit tests inline (#[cfg(test)])  │
   │                                                                 │
   │ tests/                     ← Integration tests (public API)    │
   │   ├── user_integration_test.rs                                 │
   │   └── common/                                                   │
   │       └── mod.rs           ← Shared helpers                     │
   └─────────────────────────────────────────────────────────────────┘

   UNIT TEST EXAMPLE (inline):
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

   INTEGRATION TEST EXAMPLE (tests/):
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

   SEPARATE FILE (large tests):
   ┌─────────────────────────────────────────────────────────────────┐
   │ src/domain/                                                     │
   │   ├── user.rs                                                   │
   │   └── user_tests.rs        ← Tests in separate file            │
   │                                                                 │
   │ // user.rs                                                      │
   │ pub struct User { ... }                                         │
   │                                                                 │
   │ #[cfg(test)]                                                    │
   │ #[path = "user_tests.rs"]                                       │
   │ mod tests;                                                      │
   └─────────────────────────────────────────────────────────────────┘

   COMPARISON:
   ┌────────────────┬──────────────┬───────────────────────────────┐
   │ Type           │ Location     │ Access                        │
   ├────────────────┼──────────────┼───────────────────────────────┤
   │ Unit tests     │ inline       │ pub + private (use super::*)  │
   │ Unit separate  │ _tests.rs    │ pub + private (use super::*)  │
   │ Integration    │ tests/       │ Only pub (external API)       │
   └────────────────┴──────────────┴───────────────────────────────┘
*/

#[test]
fn index() {
    println!("=== RUST MODULE ORGANIZATION STRATEGIES DEMO ===\n");

    // ============================================================
    // 1. DOMAIN STRATEGY (domain/)
    // ============================================================
    println!("--- 1. DOMAIN STRATEGY ---");
    println!("Everything related to User in one file: domain/user.rs\n");

    let mut user_service = domain::UserService::new();

    let user1 = user_service
        .create_user("Alice".to_string(), "alice@example.com".to_string())
        .unwrap();
    println!("✓ User created: {:?}", user1);

    let user2 = user_service
        .create_user("Bob".to_string(), "bob@example.com".to_string())
        .unwrap();
    println!("✓ User created: {:?}", user2);

    // Create order using domain::order
    let mut order_service = domain::OrderService::new();
    let items = vec![domain::OrderItem {
        product_id: 101,
        quantity: 2,
        price: 25.50,
    }];

    let order = order_service.create_order(user1.id, items).unwrap();
    println!("✓ Order created: ID={}, Total=${:.2}", order.id, order.total);

    let all_users = user_service.get_all_users();
    println!("\n📋 Total users: {}", all_users.len());

    // ============================================================
    // 2. HYBRID STRATEGY (hybrid/)
    // ============================================================
    println!("\n--- 2. HYBRID STRATEGY ---");
    println!("User domain split into layers: model.rs, repository.rs, service.rs\n");

    let mut hybrid_service = hybrid::UserService::new();

    let user3 = hybrid_service
        .create_user("Charlie".to_string(), "charlie@example.com".to_string())
        .unwrap();
    println!("✓ User created: {:?}", user3);

    hybrid_service
        .update_email(user3.id, "charlie.new@example.com".to_string())
        .unwrap();
    println!("✓ Email updated");

    let updated_user = hybrid_service.get_user(user3.id).unwrap();
    println!("✓ User after update: {:?}", updated_user);

    println!(
        "\n📋 Total users (hybrid): {}",
        hybrid_service.user_count()
    );

    // ============================================================
    // 3. IMPORTS COMPARISON
    // ============================================================
    println!("\n--- 3. IMPORTS COMPARISON ---\n");

    println!("DOMAIN (Vertical Slicing):");
    println!("  use modules_demo::domain::{{User, UserService}};");
    println!("  use modules_demo::domain::{{Order, OrderService}};");
    println!("  ✓ Clean API, everything related to User together\n");

    println!("HYBRID (Domain + Layers):");
    println!("  use modules_demo::hybrid::{{User, UserService}};");
    println!("  // Repository is NOT exposed (internal implementation)");
    println!("  ✓ Cleaner API, internal details hidden\n");

    // ============================================================
    // 4. ADVANTAGES AND DISADVANTAGES
    // ============================================================
    println!("--- 4. WHEN TO USE EACH STRATEGY ---\n");

    println!("MONOLITHIC (1 file):");
    println!("  ✓ Scripts <200 lines");
    println!("  ✓ Quick prototypes");
    println!("  ✗ Doesn't scale, merge conflicts\n");

    println!("BY DOMAIN (domain/):");
    println!("  ✓ Independent features");
    println!("  ✓ 200-500 lines per domain");
    println!("  ✓ Microservices/DDD");
    println!("  ✗ May duplicate common code\n");

    println!("HYBRID (hybrid/):");
    println!("  ✓ >500 lines per domain");
    println!("  ✓ Complex business logic");
    println!("  ✓ Multiple layers (MVC, Clean Architecture)");
    println!("  ✓ Large teams");
    println!("  ✗ Initial overhead (more files)\n");

    // ============================================================
    // 5. RECOMMENDATIONS
    // ============================================================
    println!("--- 5. RECOMMENDATIONS ---\n");

    println!("1. START SIMPLE:");
    println!("   - Start with domain/ (1 file per feature)");
    println!("   - Migrate to hybrid/ when it grows >300 lines\n");

    println!("2. PRIVATE BY DEFAULT:");
    println!("   - Only make `pub` what is necessary");
    println!("   - Use `pub(crate)` for internal crate API\n");

    println!("3. RE-EXPORTS IN mod.rs:");
    println!("   - Create clean public API");
    println!("   - Hide implementation details\n");

    println!("4. TESTS ALONGSIDE CODE:");
    println!("   - #[cfg(test)] mod tests in same file");
    println!("   - Integration tests in tests/\n");

    println!("5. STRUCTURE BY PROJECT:");
    println!("   - CLI tool        → domain/");
    println!("   - Web API         → hybrid/");
    println!("   - Library         → domain/ or hybrid/");
    println!("   - Microservice    → domain/\n");

    // ============================================================
    // 6. TEST ORGANIZATION
    // ============================================================
    println!("--- 6. TEST ORGANIZATION ---\n");

    println!("UNIT TESTS (inline, recommended):");
    println!("   #[cfg(test)]");
    println!("   mod tests {{");
    println!("       use super::*;");
    println!("       #[test]");
    println!("       fn test_valid_email() {{ ... }}");
    println!("   }}");
    println!("   ✓ Tests near code, easy to refactor\n");

    println!("INTEGRATION TESTS (tests/ in root):");
    println!("   tests/");
    println!("     └── user_integration_test.rs");
    println!("   ✓ Tests public API, no access to private\n");

    println!("SEPARATE UNIT TESTS (large files):");
    println!("   src/domain/");
    println!("     ├── user.rs");
    println!("     └── user_tests.rs");
    println!("   In user.rs: #[cfg(test)] #[path = \"user_tests.rs\"] mod tests;");
    println!("   ✓ Separates code from tests when very large\n");

    println!("=== END OF DEMO ===");
}
