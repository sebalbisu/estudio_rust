//! ============================================================================
//!                    TESTING IN RUST - COMPLETE GUIDE
//! ============================================================================
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//!                     RUST TESTING PYRAMID                          
//! ├─────────────────────────────────────────────────────────────────────────┤
//!                                                                          
//!                            ╱╲                                            
//!                           ╱  ╲       E2E Tests                           
//!                          ╱ 🔺 ╲      (complete integration)              
//!                         ╱──────╲                                         
//!                        ╱        ╲                                        
//!                       ╱   🔶    ╲    Integration Tests                   
//!                      ╱  Mocking  ╲   (tests/ folder, mocks)              
//!                     ╱─────────────╲                                      
//!                    ╱               ╲                                     
//!                   ╱    🟢 Unit     ╲  Unit Tests                         
//!                  ╱    Tests        ╲  (#[test], fast)                 
//!                 ╱───────────────────╲                                    
//!                ╱                     ╲                                   
//!               ╱   📊 Property-Based   ╲  Proptest/Quickcheck             
//!              ╱─────────────────────────╲                                 
//!                                                                          
//!   Faster ◄─────────────────────────────────────► Better coverage     
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
    //                      1.2 REPOSITORY TRAIT (for mocking)
    // ============================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────────┐
    //   REPOSITORY PATTERN - Abstraction for testing
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
    //   ✅ UserService depends on TRAIT, not concrete implementation
    //   ✅ In tests we inject MockRepo → fast tests, no real DB
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
        _DatabaseError,
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
    //                       1.4 HELPER FUNCTIONS
    // ============================================================================

    // Operation that can fail (to demonstrate Result testing)
    pub fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    // Operation that panics (to demonstrate #[should_panic])
    pub fn panic_if_negative(n: i32) {
        if n < 0 {
            panic!("Negative values not accepted");
        }
    }

    // For property testing: string reverse
    pub fn reverse_string(s: &str) -> String {
        s.chars().rev().collect()
    }

    // For property testing: simple addition with wrapping to avoid overflow
    pub fn add(a: i32, b: i32) -> i32 {
        a.wrapping_add(b)
    }
}

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                    PART 2: UNIT TESTS                              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   ANATOMY OF A TEST IN RUST
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   #[cfg(test)]              ← Compiles only in test mode
//   mod tests {
//       use super::*;         ← Import from parent module
//
//       #[test]               ← Mark function as test
//       fn test_name() {
//           // Arrange        ← Prepare data
//           let input = ...;
//
//           // Act            ← Execute
//           let result = function(input);
//
//           // Assert         ← Verify
//           assert_eq!(result, expected);
//       }
//   }
//
//   ASSERTION MACROS:
//   ─────────────────
//   assert!(condition)           → Verifies it's true
//   assert_eq!(a, b)             → Verifies a == b
//   assert_ne!(a, b)             → Verifies a != b
//   assert!(cond, "msg {}", x)   → With custom message
//
// ─────────────────────────────────────────────────────────────────────────
// ```
#[cfg(test)]
mod section_2_unit_tests {
    use super::section_1_domain_logic::*;
    use std::cell::RefCell;

    // ========================================================================
    //                    2.1 BASIC UNIT TESTS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   BASIC UNIT TESTS
    //   ────────────────
    //   • Verify one thing
    //   • Descriptive naming: test_<what>_<when>_<expected_result>
    //   • Fast (< 1ms)
    //   • No external dependencies
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
    //                    2.2 FIXTURES AND HELPERS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   FIXTURES AND HELPERS
    //   ────────────────────
    //   • Helper functions to create test data
    //   • Reduce duplication
    //   • Make tests more readable
    //   • Naming: create_valid_X, build_X, make_X
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
    //                    2.3 PARAMETRIC TESTS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   PARAMETRIC TESTS (Data-Driven)
    //   ──────────────────────────────
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
    //   ✅ One test, multiple cases
    //   ✅ Easy to add new cases
    //   ✅ Clear message when it fails
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
    //                    2.4 MOCKING WITH TRAITS
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   MOCKING WITH TRAITS
    //   ───────────────────
    //
    //   1. Define trait for the dependency
    //      trait UserRepository { fn find(&self, id) -> ... }
    //
    //   2. Service depends on trait, not implementation
    //      struct UserService<R: UserRepository> { repo: R }
    //
    //   3. In tests: create MockRepository
    //      struct MockUserRepository { users: Vec<User> }
    //      impl UserRepository for MockUserRepository { ... }
    //
    //   4. Inject the mock into the service
    //      let service = UserService::new(MockUserRepository::new());
    //
    //   ✅ Fast tests (no I/O)
    //   ✅ Deterministic tests
    //   ✅ Easy to simulate errors
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
        // Arrange: mock with pre-loaded user
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
    //                    2.5 ERROR AND PANIC TESTING
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   TESTING ERRORS AND PANICS
    //   ──────────────────────────
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
    //   #[should_panic]                     ← Expects any panic
    //   fn test_panics() { ... }
    //
    //   #[test]
    //   #[should_panic(expected = "msg")]   ← Expects panic with message
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
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    #[should_panic(expected = "Negative values not accepted")]
    pub fn panic_if_negative_panics_correctly() {
        panic_if_negative(-5);
    }

    #[test]
    pub fn panic_if_negative_does_not_panic_with_positive() {
        panic_if_negative(10); // Should not panic
    }

    // ========================================================================
    //                 2.6 MANUAL PROPERTY-BASED TESTING
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   PROPERTY-BASED TESTING
    //   ──────────────────────
    //
    //   Instead of: "input X produces output Y"
    //   You verify: "for ANY valid input, PROPERTY holds"
    //
    //   COMMON PROPERTIES:
    //   ──────────────────
    //   • Idempotence:  f(f(x)) == f(x)     (sort, normalize)
    //   • Inversion:    f⁻¹(f(x)) == x     (encode/decode, reverse)
    //   • Commutativity: f(a,b) == f(b,a)  (add, max)
    //   • Associativity: f(f(a,b),c) == f(a,f(b,c))
    //   • Identity:     f(x, id) == x      (add 0, multiply 1)
    //
    //   MANUAL: iterate over representative cases
    //   AUTOMATIC: proptest generates thousands of random cases
    //
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    #[test]
    pub fn reversing_string_twice_returns_original() {
        // Property: reverse(reverse(s)) == s (inversion)
        let test_cases = vec!["hello", "rust", "12345", "a", "", "🦀"];

        for original in test_cases {
            let reversed_once = reverse_string(original);
            let reversed_twice = reverse_string(&reversed_once);
            assert_eq!(original, reversed_twice, "Failed for: {}", original);
        }
    }

    #[test]
    pub fn adding_zero_returns_same_number() {
        // Property: a + 0 == a (additive identity)
        for n in -100..100 {
            assert_eq!(add(n, 0), n);
            assert_eq!(add(0, n), n);
        }
    }

    #[test]
    pub fn addition_is_commutative() {
        // Property: a + b == b + a (commutativity)
        let pairs = vec![(1, 2), (5, 10), (-3, 7), (0, 0), (i32::MAX, 0)];

        for (a, b) in pairs {
            assert_eq!(add(a, b), add(b, a), "Failed for: {} + {}", a, b);
        }
    }

    // ========================================================================
    //                    2.7 TESTS IN SUBMODULES
    // ========================================================================

    // You can organize tests in submodules
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
    //   ───────────────────────
    //
    //   Rust doesn't have @BeforeEach/@AfterEach like JUnit.
    //   Alternatives:
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

        // Register multiple users
        ctx.service
            .register_user(1, "user1@test.com".to_string(), 25)
            .unwrap();
        ctx.service
            .register_user(2, "user2@test.com".to_string(), 30)
            .unwrap();

        // Verify
        assert_eq!(ctx.service.total_users(), 2);
        assert!(ctx.service.get_user(1).is_ok());
        assert!(ctx.service.get_user(2).is_ok());
        assert!(ctx.service.get_user(3).is_err());
    }
}

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                    PART 3: PROPTEST                                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   PROPTEST - Automatic Property-Based Testing
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   PROPTEST FLOW:
//   ──────────────
//
//     ┌──────────────┐
//        Strategy    ← Define how to generate values
//      (generator)     any::<i32>(), "[a-z]+", 0..100
//     └──────┬───────┘
//
//            ▼ generates N random values (default: 256)
//     ┌──────────────┐
//        Test with
//       prop_assert
//     └──────┬───────┘
//
//       Failed?
//        |   |
//       NO  YES
//        |   |
//        ▼   ▼
//       ✅  SHRINK ← Finds the SIMPLEST case that fails
//                    [1000, -5, 42] → [0, -1, 0]
//
//   ADVANTAGES:
//   ───────────
//   • Generates edge cases you wouldn't think of
//   • Shrinking gives you the minimal reproducible case
//   • Much more coverage with less code
//
// ─────────────────────────────────────────────────────────────────────────
// ```
#[cfg(test)]
mod section_3_proptest {
    use super::section_1_domain_logic::*;
    use proptest::prelude::*;

    // ========================================================================
    //                    3.1 STRING PROPERTIES
    // ========================================================================

    proptest! {
        // Property: reverse(reverse(s)) == s
        #[test]
        fn reverse_twice_is_identity(s in ".*") {
            let reversed = reverse_string(&s);
            let back = reverse_string(&reversed);
            prop_assert_eq!(s, back);
        }

        // Property: length is preserved when reversing
        #[test]
        fn reverse_preserves_length(s in "\\PC*") {  // \PC = printable chars
            let reversed = reverse_string(&s);
            prop_assert_eq!(s.len(), reversed.len());
        }
    }

    // ========================================================================
    //                    3.2 MATHEMATICAL PROPERTIES
    // ========================================================================

    proptest! {
        // Property: a + b == b + a (commutativity)
        #[test]
        fn addition_is_commutative(a in -10000i32..10000, b in -10000i32..10000) {
            prop_assert_eq!(add(a, b), add(b, a));
        }

        // Property: a + 0 == a (identity)
        #[test]
        fn zero_is_additive_identity(a in any::<i32>()) {
            prop_assert_eq!(add(a, 0), a);
        }

        // Property: (a + b) + c == a + (b + c) (associativity)
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
    //                    3.3 CUSTOM STRATEGIES
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   CUSTOM STRATEGIES
    //   ────────────────
    //
    //   Strategy = Value generator for proptest
    //
    //   BUILT-IN:
    //   any::<T>()           → any value of type T
    //   0..100               → number range
    //   "[a-z]+"             → regex for strings
    //   prop_oneof![a, b]    → one of several values
    //
    //   CUSTOM: fn my_strategy() -> impl Strategy<Value = MyType>
    //
    //   COMPOSITION:
    //   (strat1, strat2).prop_map(|(a, b)| combine(a, b))
    //
    // └─────────────────────────────────────────────────────────────────────┘
    // ```

    // Strategy to generate valid emails
    fn valid_email_strategy() -> impl Strategy<Value = String> {
        (
            "[a-z]{3,10}",                          // name
            "[a-z]{3,8}",                           // domain
            prop_oneof!["com", "org", "net", "io"], // extension
        )
            .prop_map(|(name, domain, ext)| format!("{}@{}.{}", name, domain, ext))
    }

    // Strategy to generate valid ages (18+)
    fn valid_age_strategy() -> impl Strategy<Value = u8> {
        18u8..=120
    }

    proptest! {
        // Test with custom generators
        #[test]
        fn user_creation_succeeds_with_generated_valid_data(
            email in valid_email_strategy(),
            age in valid_age_strategy(),
            id in 1u64..1000000
        ) {
            let result = User::new(id, email.clone(), age);
            prop_assert!(
                result.is_ok(),
                "Failed with email={}, age={}, id={}", email, age, id
            );
        }
    }

    // ========================================================================
    //                    3.4 TESTING EXPECTED ERRORS
    // ========================================================================

    proptest! {
        // Verify that emails without @ always fail
        #[test]
        fn invalid_email_without_at_fails(
            name in "[a-z]{5,10}",
            domain in "[a-z]{3,5}"
        ) {
            let invalid_email = format!("{}{}.com", name, domain);  // No @
            let result = User::new(1, invalid_email, 25);
            prop_assert!(result.is_err());
        }

        // Verify that underage users always fail
        #[test]
        fn underage_users_fail(age in 0u8..18) {
            let result = User::new(1, "test@example.com".to_string(), age);
            prop_assert_eq!(result, Err(UserError::TooYoung { age }));
        }

        // Division by zero always fails
        #[test]
        fn division_by_zero_fails(numerator in any::<i32>()) {
            prop_assert!(divide(numerator, 0).is_err());
        }

        // Division by non-zero never fails
        #[test]
        fn division_by_nonzero_succeeds(
            a in any::<i32>(),
            b in any::<i32>().prop_filter("non-zero", |x| *x != 0)
        ) {
            prop_assert!(divide(a, b).is_ok());
        }
    }

    // ========================================================================
    //                    3.5 GENERATING COMPLEX STRUCTS
    // ========================================================================

    // ┌─────────────────────────────────────────────────────────────────────┐
    //   prop_compose! - Generate valid structs
    //   ──────────────────────────────────────
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
    //   Use as: my_struct in my_struct_strategy()
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
        // All valid users can vote (age >= 18 by construction)
        #[test]
        fn all_valid_users_can_vote(user in valid_user_strategy()) {
            prop_assert!(user.can_vote());
        }

        // Verify email format invariants
        #[test]
        fn user_email_always_valid_format(user in valid_user_strategy()) {
            prop_assert!(user.email.contains('@'));
            prop_assert!(user.email.contains('.'));
        }
    }

    // ========================================================================
    //                    3.6 TESTING COLLECTIONS
    // ========================================================================

    proptest! {
        // Property: sorting is idempotent (sort(sort(v)) == sort(v))
        #[test]
        fn sorting_is_idempotent(mut vec in prop::collection::vec(any::<i32>(), 0..100)) {
            vec.sort();
            let first_sort = vec.clone();
            vec.sort();
            prop_assert_eq!(first_sort, vec);
        }

        // Property: length is preserved when sorting
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

        // Property: HashSet.len() <= Vec.len() (removes duplicates)
        #[test]
        fn hashset_removes_duplicates(vec in prop::collection::vec(1i32..10, 0..50)) {
            use std::collections::HashSet;
            let set: HashSet<_> = vec.iter().copied().collect();
            prop_assert!(set.len() <= vec.len());
        }
    }

    // ========================================================================
    //                    3.7 ADVANCED CONFIGURATION
    // ========================================================================

    // ```text
    // ┌─────────────────────────────────────────────────────────────────────┐
    //   ProptestConfig
    //   ──────────────
    //
    //   ProptestConfig::with_cases(N)   → N iterations (default: 256)
    //   .max_shrink_iters(N)            → max shrink attempts
    //   .timeout(Duration)              → timeout per case
    //
    //   proptest! {
    //       #![proptest_config(ProptestConfig::with_cases(1000))]
    //       ...
    //   }
    // └─────────────────────────────────────────────────────────────────────┘
    // ```
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(500))]  // 500 cases

        #[test]
        fn more_thorough_test(x in any::<i32>(), y in any::<i32>()) {
            prop_assert_eq!(add(x, y), add(y, x));
            prop_assert_eq!(add(x, y), x.wrapping_add(y));
        }
    }
}

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                    PART 4: ASYNC TESTS                                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   ASYNC TESTS WITH TOKIO
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   #[tokio::test]                  ← Creates runtime automatically
//   async fn my_async_test() {
//       let result = my_async_fn().await;
//       assert_eq!(result, expected);
//   }
//
//   VARIANTS:
//   ─────────
//   #[tokio::test]                            ← Basic runtime
//   #[tokio::test(flavor = "multi_thread")]   ← Multi-threaded
//   #[tokio::test(start_paused = true)]       ← Time control
//
//   REQUIRES in Cargo.toml:
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
// ║                    PART 5: BENCHMARKING WITH CRITERION                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ```text
// ┌─────────────────────────────────────────────────────────────────────────┐
//   CRITERION - Professional Benchmarking
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   FEATURES:
//   ────────
//   • Robust statistics (median, std dev, outliers)
//   • Automatic regression detection
//   • HTML reports with graphs
//   • Comparison between implementations
//   • Throughput (bytes/sec, ops/sec)
//
//   SETUP in Cargo.toml:
//   ──────────────────
//   [dev-dependencies]
//   criterion = "0.5"
//
//   [[bench]]
//   name = "my_benchmarks"
//   harness = false
//
//   COMMANDS:
//   ────────
//   cargo bench                        # All benchmarks
//   cargo bench my_bench               # Specific benchmark
//   cargo bench -- --save-baseline v1  # Save baseline
//   cargo bench -- --baseline v1       # Compare with baseline
//
//   REPORTS: target/criterion/report/index.html
//
// ─────────────────────────────────────────────────────────────────────────
//
// ┌─────────────────────────────────────────────────────────────────────────┐
//   ANATOMY OF A BENCHMARK
// ├─────────────────────────────────────────────────────────────────────────┤
//
//   use criterion::{black_box, criterion_group, criterion_main, Criterion};
//
//   fn bench_my_function(c: &mut Criterion) {
//       c.bench_function("name", |b| {
//           b.iter(|| {
//               // black_box() prevents compiler optimizations
//               my_function(black_box(input))
//           })
//       });
//   }
//
//   // Compare multiple implementations
//   fn bench_comparison(c: &mut Criterion) {
//       let mut group = c.benchmark_group("comparison");
//
//       group.bench_function("impl_a", |b| b.iter(|| impl_a()));
//       group.bench_function("impl_b", |b| b.iter(|| impl_b()));
//
//       group.finish();
//   }
//
//   // Benchmark with different input sizes
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
// See file: benches/user_benchmarks.rs for complete examples
// ```
#[cfg(test)]
mod section_5_benchmarking {
    #[test]
    pub fn benchmarking_info() {
        println!("Benchmarking examples are in benches/user_benchmarks.rs");
    }
}
