//! ============================================================================
//!                    TESTING EN RUST - GUÍA COMPLETA
//! ============================================================================
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//!                     PIRÁMIDE DE TESTING EN RUST                          
//! ├─────────────────────────────────────────────────────────────────────────┤
//!                                                                          
//!                            ╱╲                                            
//!                           ╱  ╲       E2E Tests                           
//!                          ╱ 🔺 ╲      (integración completa)              
//!                         ╱──────╲                                         
//!                        ╱        ╲                                        
//!                       ╱   🔶    ╲    Integration Tests                   
//!                      ╱  Mocking  ╲   (tests/ folder, mocks)              
//!                     ╱─────────────╲                                      
//!                    ╱               ╲                                     
//!                   ╱    🟢 Unit     ╲  Unit Tests                         
//!                  ╱    Tests        ╲  (#[test], rápidos)                 
//!                 ╱───────────────────╲                                    
//!                ╱                     ╲                                   
//!               ╱   📊 Property-Based   ╲  Proptest/Quickcheck             
//!              ╱─────────────────────────╲                                 
//!                                                                          
//!   Más rápidos ◄─────────────────────────────────────► Más cobertura     
//!                                                                          
//! ─────────────────────────────────────────────────────────────────────────
//! ```
//!

#[test]
fn indice() {
    section_2_unit_tests::user_creation_succeeds_with_valid_data();
    section_4_async_tests::async_fetch_returns_data();
    section_5_benchmarking::benchmarking_info();
}

// ============================================================================
//                      1. DOMAIN LOGIC & TRAITS
// ============================================================================
#[cfg(test)]
mod section_1_domain_logic {
    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        pub id: u64,
        pub email: String,
        pub age: u8,
    }

    impl User {
        pub fn new(id: u64, email: String, age: u8) -> Result<Self, UserError> {
            if !Self::is_valid_email(&email) {
                return Err(UserError::InvalidEmail);
            }
            if age < 18 {
                return Err(UserError::TooYoung { age });
            }
            Ok(User { id, email, age })
        }

        fn is_valid_email(email: &str) -> bool {
            email.contains('@') && email.contains('.')
        }

        pub fn can_vote(&self) -> bool {
            self.age >= 18
        }

        pub fn is_senior(&self) -> bool {
            self.age >= 65
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum UserError {
        InvalidEmail,
        TooYoung { age: u8 },
    }

    // ============================================================================
    //                      1.2 REPOSITORY TRAIT (para mocking)
    // ============================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────────┐
    //   PATRÓN REPOSITORY - Abstracción para testing
    // ├─────────────────────────────────────────────────────────────────────────┤
    //
    //                     trait UserRepository
    //                     ┌───────────────────┐
    //                      find(id)
    //                      save(user)
    //                      count()
    //                     └─────────┬─────────┘
    //
    //               ┌───────────────┼───────────────┐
    //
    //               ▼               ▼               ▼
    //      ┌────────────┐   ┌────────────┐   ┌────────────┐
    //       PostgresDB      MockRepo      InMemory
    //         (prod)        (tests)        (dev)
    //      └────────────┘   └────────────┘   └────────────┘
    //
    //   ✅ UserService depende del TRAIT, no de implementación concreta
    //   ✅ En tests inyectamos MockRepo → tests rápidos, sin DB real
    //
    // ─────────────────────────────────────────────────────────────────────────
    // ```
    pub trait UserRepository {
        fn find(&self, id: u64) -> Result<User, RepoError>;
        fn save(&self, user: User) -> Result<(), RepoError>;
        fn count(&self) -> usize;
    }

    #[derive(Debug, PartialEq)]
    pub enum RepoError {
        NotFound,
        DatabaseError,
    }

    // ============================================================================
    //                           1.3 USER SERVICE
    // ============================================================================

    // Servicio que depende del trait UserRepository (permite inyección de mocks)
    pub struct UserService<R: UserRepository> {
        repo: R,
    }

    impl<R: UserRepository> UserService<R> {
        pub fn new(repo: R) -> Self {
            Self { repo }
        }

        pub fn get_user(&self, id: u64) -> Result<User, RepoError> {
            self.repo.find(id)
        }

        pub fn register_user(&self, id: u64, email: String, age: u8) -> Result<(), String> {
            let user = User::new(id, email, age).map_err(|e| format!("{:?}", e))?;
            self.repo.save(user).map_err(|e| format!("{:?}", e))?;
            Ok(())
        }

        pub fn total_users(&self) -> usize {
            self.repo.count()
        }
    }

    // ============================================================================
    //                       1.4 FUNCIONES AUXILIARES
    // ============================================================================

    // Operación que puede fallar (para demostrar testing de Result)
    pub fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division por cero".to_string())
        } else {
            Ok(a / b)
        }
    }

    // Operación que paniquea (para demostrar #[should_panic])
    pub fn panic_if_negative(n: i32) {
        if n < 0 {
            panic!("No se aceptan negativos");
        }
    }

    // Para property testing: reverse de string
    pub fn reverse_string(s: &str) -> String {
        s.chars().rev().collect()
    }

    // Para property testing: suma simple con wrapping para evitar overflow
    pub fn add(a: i32, b: i32) -> i32 {
        a.wrapping_add(b)
    }
}

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                    PARTE 2: TESTS UNITARIOS                              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   ANATOMÍA DE UN TEST EN RUST
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   #[cfg(test)]              ← Solo compila en modo test
//   mod tests {
//       use super::*;         ← Importa del módulo padre
//
//       #[test]               ← Marca la función como test
//       fn test_name() {
//           // Arrange        ← Preparar datos
//           let input = ...;
//
//           // Act            ← Ejecutar
//           let result = function(input);
//
//           // Assert         ← Verificar
//           assert_eq!(result, expected);
//       }
//   }
//
//   MACROS DE ASERCIÓN:
//   ───────────────────
//   assert!(condition)           → Verifica que sea true
//   assert_eq!(a, b)             → Verifica a == b
//   assert_ne!(a, b)             → Verifica a != b
//   assert!(cond, "msg {}", x)   → Con mensaje personalizado
//
// ─────────────────────────────────────────────────────────────────────────
// ```
#[cfg(test)]
mod section_2_unit_tests {
    use super::section_1_domain_logic::*;
    use std::cell::RefCell;

    // ========================================================================
    //                    2.1 UNIT TESTS BÁSICOS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   UNIT TESTS BÁSICOS
    //   ───────────────────
    //   • Verifican una sola cosa
    //   • Nombre descriptivo: test_<qué>_<cuándo>_<resultado_esperado>
    //   • Rápidos (< 1ms)
    //   • Sin dependencias externas
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    #[test]
    pub fn user_creation_succeeds_with_valid_data() {
        // Arrange
        let id = 1;
        let email = "test@example.com".to_string();
        let age = 25;

        // Act
        let user = User::new(id, email.clone(), age).unwrap();

        // Assert
        assert_eq!(user.id, id);
        assert_eq!(user.email, email);
        assert_eq!(user.age, age);
    }

    #[test]
    pub fn user_creation_fails_with_invalid_email() {
        let result = User::new(1, "invalid-email".to_string(), 25);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserError::InvalidEmail);
    }

    #[test]
    pub fn user_creation_fails_if_underage() {
        let result = User::new(1, "kid@example.com".to_string(), 15);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserError::TooYoung { age: 15 });
    }

    #[test]
    pub fn users_over_18_can_vote() {
        let user = User::new(1, "voter@example.com".to_string(), 20).unwrap();
        assert!(user.can_vote());
    }

    #[test]
    pub fn users_over_65_are_senior() {
        let senior = User::new(1, "senior@example.com".to_string(), 70).unwrap();
        let adult = User::new(2, "adult@example.com".to_string(), 30).unwrap();

        assert!(senior.is_senior());
        assert!(!adult.is_senior());
    }

    // ========================================================================
    //                    2.2 FIXTURES Y HELPERS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   FIXTURES Y HELPERS
    //   ─────────────────────
    //   • Funciones auxiliares para crear datos de test
    //   • Reducen duplicación
    //   • Hacen tests más legibles
    //   • Nombre: create_valid_X, build_X, make_X
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    fn create_valid_user(id: u64) -> User {
        User::new(id, format!("user{}@example.com", id), 25).unwrap()
    }

    #[test]
    pub fn multiple_users_with_helper() {
        let user1 = create_valid_user(1);
        let user2 = create_valid_user(2);

        assert_ne!(user1.id, user2.id);
        assert_ne!(user1.email, user2.email);
    }

    // ========================================================================
    //                    2.3 TESTS PARAMÉTRICOS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   TESTS PARAMÉTRICOS (Data-Driven)
    //   ─────────────────────────────────
    //
    //   let cases = vec![
    //       (input1, expected1),
    //       (input2, expected2),
    //       ...
    //   ];
    //
    //   for (input, expected) in cases {
    //       assert_eq!(function(input), expected, "msg: {}", input);
    //   }
    //
    //   ✅ Un test, múltiples casos
    //   ✅ Fácil agregar nuevos casos
    //   ✅ Mensaje claro cuando falla
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    #[test]
    pub fn test_email_validation_cases() {
        let cases = vec![
            ("valid@example.com", true),
            ("another.valid@test.org", true),
            ("invalid", false),
            ("no-at.com", false),
            ("no-dot@com", false),
        ];

        for (email, expected) in cases {
            let result = User::new(1, email.to_string(), 25);
            assert_eq!(
                result.is_ok(),
                expected,
                "Email validation failed for: {}",
                email
            );
        }
    }

    // ========================================================================
    //                    2.4 MOCKING CON TRAITS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   MOCKING CON TRAITS
    //   ─────────────────────
    //
    //   1. Define trait para la dependencia
    //      trait UserRepository { fn find(&self, id) -> ... }
    //
    //   2. Service depende del trait, no de implementación
    //      struct UserService<R: UserRepository> { repo: R }
    //
    //   3. En tests: crea MockRepository
    //      struct MockUserRepository { users: Vec<User> }
    //      impl UserRepository for MockUserRepository { ... }
    //
    //   4. Inyecta el mock en el service
    //      let service = UserService::new(MockUserRepository::new());
    //
    //   ✅ Tests rápidos (sin I/O)
    //   ✅ Tests determinísticos
    //   ✅ Puedes simular errores fácilmente
    // └─────────────────────────────────────────────────────────────────────┘
    // ```

    struct MockUserRepository {
        users: RefCell<Vec<User>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: RefCell::new(Vec::new()),
            }
        }

        fn with_users(users: Vec<User>) -> Self {
            Self {
                users: RefCell::new(users),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        fn find(&self, id: u64) -> Result<User, RepoError> {
            self.users
                .borrow()
                .iter()
                .find(|u| u.id == id)
                .cloned()
                .ok_or(RepoError::NotFound)
        }

        fn save(&self, user: User) -> Result<(), RepoError> {
            self.users.borrow_mut().push(user);
            Ok(())
        }

        fn count(&self) -> usize {
            self.users.borrow().len()
        }
    }

    #[test]
    pub fn service_finds_existing_user() {
        // Arrange: mock con usuario precargado
        let user = create_valid_user(1);
        let repo = MockUserRepository::with_users(vec![user.clone()]);
        let service = UserService::new(repo);

        // Act
        let found = service.get_user(1).unwrap();

        // Assert
        assert_eq!(found, user);
    }

    #[test]
    pub fn service_returns_error_for_missing_user() {
        let repo = MockUserRepository::new();
        let service = UserService::new(repo);

        let result = service.get_user(999);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RepoError::NotFound);
    }

    #[test]
    pub fn service_saves_new_user() {
        let repo = MockUserRepository::new();
        let service = UserService::new(repo);

        assert_eq!(service.total_users(), 0);

        service
            .register_user(1, "new@example.com".to_string(), 30)
            .unwrap();

        assert_eq!(service.total_users(), 1);
        let saved = service.get_user(1).unwrap();
        assert_eq!(saved.email, "new@example.com");
    }

    // ========================================================================
    //                    2.5 TESTS DE ERRORES Y PANICS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   TESTING DE ERRORES Y PANICS
    //   ───────────────────────────
    //
    //   RESULT<T, E>:
    //   ─────────────
    //   assert!(result.is_ok());
    //   assert!(result.is_err());
    //   assert_eq!(result.unwrap_err(), ExpectedError);
    //
    //   PANICS:
    //   ───────
    //   #[test]
    //   #[should_panic]                     ← Espera cualquier panic
    //   fn test_panics() { ... }
    //
    //   #[test]
    //   #[should_panic(expected = "msg")]   ← Espera panic con mensaje
    //   fn test_panics_with_msg() { ... }
    //
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    #[test]
    pub fn divide_succeeds_with_valid_input() {
        assert_eq!(divide(10, 2).unwrap(), 5);
        assert_eq!(divide(7, 3).unwrap(), 2);
    }

    #[test]
    pub fn divide_returns_error_on_zero() {
        let result = divide(10, 0);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division por cero");
    }

    #[test]
    #[should_panic(expected = "No se aceptan negativos")]
    pub fn panic_if_negative_panics_correctly() {
        panic_if_negative(-5);
    }

    #[test]
    pub fn panic_if_negative_does_not_panic_with_positive() {
        panic_if_negative(10); // No debe paniquear
    }

    // ========================================================================
    //                 2.6 PROPERTY-BASED TESTING MANUAL
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   PROPERTY-BASED TESTING
    //   ───────────────────────
    //
    //   En vez de: "input X produce output Y"
    //   Verificas: "para CUALQUIER input válido, se cumple PROPIEDAD"
    //
    //   PROPIEDADES COMUNES:
    //   ─────────────────────
    //   • Idempotencia:  f(f(x)) == f(x)     (sort, normalize)
    //   • Inversión:     f⁻¹(f(x)) == x     (encode/decode, reverse)
    //   • Conmutatividad: f(a,b) == f(b,a)  (add, max)
    //   • Asociatividad: f(f(a,b),c) == f(a,f(b,c))
    //   • Identidad:     f(x, id) == x      (add 0, multiply 1)
    //
    //   MANUAL: iteramos sobre casos representativos
    //   AUTOMÁTICO: proptest genera miles de casos aleatorios
    //
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    #[test]
    pub fn reversing_string_twice_returns_original() {
        // Propiedad: reverse(reverse(s)) == s (inversión)
        let test_cases = vec!["hello", "rust", "12345", "a", "", "🦀"];

        for original in test_cases {
            let reversed_once = reverse_string(original);
            let reversed_twice = reverse_string(&reversed_once);
            assert_eq!(original, reversed_twice, "Failed for: {}", original);
        }
    }

    #[test]
    pub fn adding_zero_returns_same_number() {
        // Propiedad: a + 0 == a (identidad aditiva)
        for n in -100..100 {
            assert_eq!(add(n, 0), n);
            assert_eq!(add(0, n), n);
        }
    }

    #[test]
    pub fn addition_is_commutative() {
        // Propiedad: a + b == b + a (conmutatividad)
        let pairs = vec![(1, 2), (5, 10), (-3, 7), (0, 0), (i32::MAX, 0)];

        for (a, b) in pairs {
            assert_eq!(add(a, b), add(b, a), "Failed for: {} + {}", a, b);
        }
    }

    // ========================================================================
    //                    2.7 TESTS EN SUBMODULOS
    // ========================================================================

    // Puedes organizar tests en submódulos
    mod nested_module_tests {
        use super::*;

        #[test]
        fn user_can_be_cloned() {
            let user = create_valid_user(1);
            let cloned = user.clone();
            assert_eq!(user, cloned);
        }

        #[test]
        fn user_implements_debug() {
            let user = create_valid_user(1);
            let debug_str = format!("{:?}", user);
            assert!(debug_str.contains("User"));
            assert!(debug_str.contains("user1@example.com"));
        }
    }

    // ========================================================================
    //                  2.8 SETUP/TEARDOWN PATTERNS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   SETUP/TEARDOWN PATTERNS
    //   ────────────────────────
    //
    //   Rust no tiene @BeforeEach/@AfterEach como JUnit.
    //   Alternativas:
    //
    //   A. STRUCT CONTEXT
    //      struct TestContext {
    //           Service,
    //           MockRepo,
    //           ...
    //       }
    //      impl TestContext { fn setup() -> Self { ... } }
    //       impl Drop for TestContext { fn drop(&mut self) { cleanup() } }
    //
    //   B. HELPER FUNCTIONS
    //      fn setup() -> (Service, MockRepo) { ... }
    //
    //
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    struct TestContext {
        service: UserService<MockUserRepository>,
    }

    impl TestContext {
        fn setup() -> Self {
            let repo = MockUserRepository::new();
            let service = UserService::new(repo);
            Self { service }
        }
    }

    #[test]
    pub fn integration_test_with_context() {
        let ctx = TestContext::setup();

        // Registrar múltiples usuarios
        ctx.service
            .register_user(1, "user1@test.com".to_string(), 25)
            .unwrap();
        ctx.service
            .register_user(2, "user2@test.com".to_string(), 30)
            .unwrap();

        // Verificar
        assert_eq!(ctx.service.total_users(), 2);
        assert!(ctx.service.get_user(1).is_ok());
        assert!(ctx.service.get_user(2).is_ok());
        assert!(ctx.service.get_user(3).is_err());
    }
}

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                    PARTE 3: PROPTEST                                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   PROPTEST - Property-Based Testing Automático
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   FLUJO DE PROPTEST:
//   ──────────────────
//
//     ┌──────────────┐
//        Strategy    ← Define cómo generar valores
//      (generador)     any::<i32>(), "[a-z]+", 0..100
//     └──────┬───────┘
//
//            ▼ genera N valores aleatorios (default: 256)
//     ┌──────────────┐
//        Test con
//       prop_assert
//     └──────┬───────┘
//
//       ¿Falló?
//        |   |
//       NO  YES
//        |   |
//        ▼   ▼
//       ✅  SHRINK ← Encuentra el caso MÁS SIMPLE que falla
//                    [1000, -5, 42] → [0, -1, 0]
//
//   VENTAJAS:
//   ──────────
//   • Genera edge cases que no pensarías
//   • Shrinking te da el caso mínimo reproducible
//   • Mucha más cobertura con menos código
//
// ─────────────────────────────────────────────────────────────────────────
// ```
#[cfg(test)]
mod section_3_proptest {
    use super::section_1_domain_logic::*;
    use proptest::prelude::*;

    // ========================================================================
    //                    3.1 PROPIEDADES DE STRINGS
    // ========================================================================

    proptest! {
        // Propiedad: reverse(reverse(s)) == s
        #[test]
        fn reverse_twice_is_identity(s in ".*") {
            let reversed = reverse_string(&s);
            let back = reverse_string(&reversed);
            prop_assert_eq!(s, back);
        }

        // Propiedad: la longitud se preserva al hacer reverse
        #[test]
        fn reverse_preserves_length(s in "\\PC*") {  // \PC = printable chars
            let reversed = reverse_string(&s);
            prop_assert_eq!(s.len(), reversed.len());
        }
    }

    // ========================================================================
    //                    3.2 PROPIEDADES MATEMÁTICAS
    // ========================================================================

    proptest! {
        // Propiedad: a + b == b + a (conmutatividad)
        #[test]
        fn addition_is_commutative(a in -10000i32..10000, b in -10000i32..10000) {
            prop_assert_eq!(add(a, b), add(b, a));
        }

        // Propiedad: a + 0 == a (identidad)
        #[test]
        fn zero_is_additive_identity(a in any::<i32>()) {
            prop_assert_eq!(add(a, 0), a);
        }

        // Propiedad: (a + b) + c == a + (b + c) (asociatividad)
        #[test]
        fn addition_is_associative(
            a in -1000i32..1000,
            b in -1000i32..1000,
            c in -1000i32..1000
        ) {
            prop_assert_eq!(add(add(a, b), c), add(a, add(b, c)));
        }
    }

    // ========================================================================
    //                    3.3 STRATEGIES PERSONALIZADAS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   STRATEGIES PERSONALIZADAS
    //   ─────────────────────────────
    //
    //   Strategy = Generador de valores para proptest
    //
    //   BUILT-IN:
    //   any::<T>()           → cualquier valor de tipo T
    //   0..100               → rango de números
    //   "[a-z]+"             → regex para strings
    //   prop_oneof![a, b]    → uno de varios valores
    //
    //   CUSTOM: fn my_strategy() -> impl Strategy<Value = MyType>
    //
    //   COMPOSICIÓN:
    //   (strat1, strat2).prop_map(|(a, b)| combine(a, b))
    //
    // └─────────────────────────────────────────────────────────────────────┘
    // ```

    // Strategy para generar emails válidos
    fn valid_email_strategy() -> impl Strategy<Value = String> {
        (
            "[a-z]{3,10}",                          // nombre
            "[a-z]{3,8}",                           // dominio
            prop_oneof!["com", "org", "net", "io"], // extensión
        )
            .prop_map(|(name, domain, ext)| format!("{}@{}.{}", name, domain, ext))
    }

    // Strategy para generar edades válidas (18+)
    fn valid_age_strategy() -> impl Strategy<Value = u8> {
        18u8..=120
    }

    proptest! {
        // Test con generadores personalizados
        #[test]
        fn user_creation_succeeds_with_generated_valid_data(
            email in valid_email_strategy(),
            age in valid_age_strategy(),
            id in 1u64..1000000
        ) {
            let result = User::new(id, email.clone(), age);
            prop_assert!(
                result.is_ok(),
                "Falló con email={}, age={}, id={}", email, age, id
            );
        }
    }

    // ========================================================================
    //                    3.4 TESTING DE ERRORES ESPERADOS
    // ========================================================================

    proptest! {
        // Verificar que emails sin @ siempre fallan
        #[test]
        fn invalid_email_without_at_fails(
            name in "[a-z]{5,10}",
            domain in "[a-z]{3,5}"
        ) {
            let invalid_email = format!("{}{}.com", name, domain);  // Sin @
            let result = User::new(1, invalid_email, 25);
            prop_assert!(result.is_err());
        }

        // Verificar que menores de 18 siempre fallan
        #[test]
        fn underage_users_fail(age in 0u8..18) {
            let result = User::new(1, "test@example.com".to_string(), age);
            prop_assert_eq!(result, Err(UserError::TooYoung { age }));
        }

        // División por cero siempre falla
        #[test]
        fn division_by_zero_fails(numerator in any::<i32>()) {
            prop_assert!(divide(numerator, 0).is_err());
        }

        // División por no-cero nunca falla
        #[test]
        fn division_by_nonzero_succeeds(
            a in any::<i32>(),
            b in any::<i32>().prop_filter("non-zero", |x| *x != 0)
        ) {
            prop_assert!(divide(a, b).is_ok());
        }
    }

    // ========================================================================
    //                    3.5 GENERACIÓN DE STRUCTS COMPLEJOS
    // ========================================================================

    // ┌─────────────────────────────────────────────────────────────────────┐
    //   prop_compose! - Generar structs válidos
    //   ────────────────────────────────────
    //
    //   prop_compose! {
    //       fn my_struct_strategy()(
    //           field1 in strategy1,
    //           field2 in strategy2,
    //       ) -> MyStruct {
    //           MyStruct::new(field1, field2).unwrap()
    //       }
    //   }
    //
    //   Úsalo como: my_struct in my_struct_strategy()
    //
    // └─────────────────────────────────────────────────────────────────────┘

    prop_compose! {
        fn valid_user_strategy()(
            id in 1u64..1000000,
            email in valid_email_strategy(),
            age in valid_age_strategy()
        ) -> User {
            User::new(id, email, age).unwrap()
        }
    }

    proptest! {
        // Todos los usuarios válidos pueden votar (age >= 18 por construcción)
        #[test]
        fn all_valid_users_can_vote(user in valid_user_strategy()) {
            prop_assert!(user.can_vote());
        }

        // Verificar invariantes del email
        #[test]
        fn user_email_always_valid_format(user in valid_user_strategy()) {
            prop_assert!(user.email.contains('@'));
            prop_assert!(user.email.contains('.'));
        }
    }

    // ========================================================================
    //                    3.6 TESTING DE COLECCIONES
    // ========================================================================

    proptest! {
        // Propiedad: ordenar es idempotente (sort(sort(v)) == sort(v))
        #[test]
        fn sorting_is_idempotent(mut vec in prop::collection::vec(any::<i32>(), 0..100)) {
            vec.sort();
            let first_sort = vec.clone();
            vec.sort();
            prop_assert_eq!(first_sort, vec);
        }

        // Propiedad: la longitud se preserva al ordenar
        #[test]
        fn sorting_preserves_length(vec in prop::collection::vec(any::<i32>(), 0..100)) {
            let len_before = vec.len();
            let sorted: Vec<_> = {
                let mut v = vec;
                v.sort();
                v
            };
            prop_assert_eq!(len_before, sorted.len());
        }

        // Propiedad: HashSet.len() <= Vec.len() (elimina duplicados)
        #[test]
        fn hashset_removes_duplicates(vec in prop::collection::vec(1i32..10, 0..50)) {
            use std::collections::HashSet;
            let set: HashSet<_> = vec.iter().copied().collect();
            prop_assert!(set.len() <= vec.len());
        }
    }

    // ========================================================================
    //                    3.7 CONFIGURACIÓN AVANZADA
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   ProptestConfig
    //   ──────────────
    //
    //   ProptestConfig::with_cases(N)   → N iteraciones (default: 256)
    //   .max_shrink_iters(N)            → máx intentos de shrink
    //   .timeout(Duration)              → timeout por caso
    //
    //   proptest! {
    //       #![proptest_config(ProptestConfig::with_cases(1000))]
    //       ...
    //   }
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(500))]  // 500 casos

        #[test]
        fn more_thorough_test(x in any::<i32>(), y in any::<i32>()) {
            prop_assert_eq!(add(x, y), add(y, x));
            prop_assert_eq!(add(x, y), x.wrapping_add(y));
        }
    }
}

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                    PARTE 4: ASYNC TESTS                                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   ASYNC TESTS CON TOKIO
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   #[tokio::test]                  ← Crea runtime automáticamente
//   async fn my_async_test() {
//       let result = my_async_fn().await;
//       assert_eq!(result, expected);
//   }
//
//   VARIANTES:
//   ──────────
//   #[tokio::test]                            ← Runtime básico
//   #[tokio::test(flavor = "multi_thread")]   ← Multi-threaded
//   #[tokio::test(start_paused = true)]       ← Control de tiempo
//
//   REQUIERE en Cargo.toml:
//   [dev-dependencies]
//   tokio = { version = "1", features = ["full", "test-util"] }
//
// ─────────────────────────────────────────────────────────────────────────
// ```
#[cfg(test)]
mod section_4_async_tests {
    use tokio::time::{Duration, sleep};

    async fn fetch_data(id: u64) -> String {
        sleep(Duration::from_millis(10)).await;
        format!("Data for id {}", id)
    }

    #[tokio::test]
    pub async fn async_fetch_returns_data() {
        let result = fetch_data(42).await;
        assert_eq!(result, "Data for id 42");
    }

    #[tokio::test]
    pub async fn multiple_async_operations() {
        let results = tokio::join!(fetch_data(1), fetch_data(2), fetch_data(3));

        assert_eq!(results.0, "Data for id 1");
        assert_eq!(results.1, "Data for id 2");
        assert_eq!(results.2, "Data for id 3");
    }
}

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                    PARTE 5: BENCHMARKING CON CRITERION                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   CRITERION - Benchmarking Profesional
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   FEATURES:
//   ─────────
//   • Estadísticas robustas (median, std dev, outliers)
//   • Detección automática de regresiones
//   • Reportes HTML con gráficos
//   • Comparación entre implementaciones
//   • Throughput (bytes/sec, ops/sec)
//
//   SETUP en Cargo.toml:
//   ─────────────────────
//   [dev-dependencies]
//   criterion = "0.5"
//
//   [[bench]]
//   name = "my_benchmarks"
//   harness = false
//
//   COMANDOS:
//   ─────────
//   cargo bench                        # Todos los benchmarks
//   cargo bench my_bench               # Benchmark específico
//   cargo bench -- --save-baseline v1  # Guardar baseline
//   cargo bench -- --baseline v1       # Comparar con baseline
//
//   REPORTES: target/criterion/report/index.html
//
// ─────────────────────────────────────────────────────────────────────────
//
// ┌─────────────────────────────────────────────────────────────────────────┐
//   ANATOMÍA DE UN BENCHMARK
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   use criterion::{black_box, criterion_group, criterion_main, Criterion};
//
//   fn bench_my_function(c: &mut Criterion) {
//       c.bench_function("nombre", |b| {
//           b.iter(|| {
//               // black_box() evita que el compilador optimice
//               my_function(black_box(input))
//           })
//       });
//   }
//
//   // Comparar múltiples implementaciones
//   fn bench_comparison(c: &mut Criterion) {
//       let mut group = c.benchmark_group("comparacion");
//
//       group.bench_function("impl_a", |b| b.iter(|| impl_a()));
//       group.bench_function("impl_b", |b| b.iter(|| impl_b()));
//
//       group.finish();
//   }
//
//   // Benchmark con diferentes tamaños de input
//   fn bench_scaling(c: &mut Criterion) {
//       let mut group = c.benchmark_group("scaling");
//
//       for size in [10, 100, 1000, 10000] {
//           group.bench_with_input(
//               BenchmarkId::new("op", size),
//               &size,
//               |b, &size| b.iter(|| operation(size))
//           );
//       }
//       group.finish();
//   }
//
//   criterion_group!(benches, bench_my_function, bench_comparison);
//   criterion_main!(benches);
//
// ─────────────────────────────────────────────────────────────────────────
//
// Ver archivo: benches/user_benchmarks.rs para ejemplos completos
// ```
#[cfg(test)]
mod section_5_benchmarking {
    #[test]
    pub fn benchmarking_info() {
        println!("Benchmarking examples are in benches/user_benchmarks.rs");
    }
}
