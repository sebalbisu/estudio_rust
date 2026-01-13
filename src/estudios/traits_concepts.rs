#[test]
fn indice() {
    utilidades::responsabilidad_unica();
    utilidades::polimorfirsmo();
    utilidades::inversion_de_dependencias();
    utilidades::mocking();

    conceptos::que_es_un_trait();
    conceptos::manual();
    conceptos::derive();
    conceptos::blanket_impl();
    conceptos::traits_como_parametros();
    conceptos::traits_como_retorno();

    testing::testing_con_traits();

    avanzado::object_safety();
    avanzado::vtable_y_dyn();
    avanzado::patrones();
    avanzado::types_asociados();
    avanzado::lifetimes();
    avanzado::multiples_traits();
    avanzado::ufcs();
    avanzado::formas_de_llamar();
    avanzado::operadores();
}

// ============================================================================
//                     UTILIDADES
// ============================================================================

#[cfg(test)]
mod utilidades {
    ///     1. RESPONSABILIDAD ÚNICA (SRP)
    ///     ────────────────────────────────────────────────
    ///     Cada implementacion/trait define un aspecto aislado del
    ///     comportamiento del struct
    ///     La implementacion del trait tambien va separada.
    ///         → `impl Validatable for User`
    ///     
    #[test]
    pub fn responsabilidad_unica() {
        trait Validatable {
            fn validate(&self) -> Result<(), Vec<String>>;
        }
        struct User {
            #[allow(dead_code)]
            id: u32,
            #[allow(dead_code)]
            email: String,
        }

        // implementacion base
        impl User {
            fn new(id: u32, email: String) -> Self {
                Self { id, email }
            }
            #[allow(dead_code)]
            fn get_email(&self) -> &str {
                &self.email
            }
        }
        // implementacion del trait separada
        impl Validatable for User {
            fn validate(&self) -> Result<(), Vec<String>> {
                Ok(())
            }
        }

        let user = User::new(1, "test@example.com".into());
        assert!(user.validate().is_ok());
    }

    ///     2. POLIMORFISMO Y EXTENSIBILIDAD
    ///     ────────────────────────────────
    ///     Permiten escribir código que opera sobre "capacidades" en lugar de tipos
    ///     concretos. Esto hace que el sistema sea extensible: puedes añadir nuevos
    ///     tipos en el futuro y las funciones existentes funcionarán
    ///     para tipos que implementen el trait.
    ///         `fn procesar(dato: impl SerializableTrait)`
    ///         // funciona para cualquier T que sea SerializableTrait
    ///
    #[test]
    pub fn polimorfirsmo() {
        trait Serializable {
            fn serialize(&self) -> String;
        }

        struct User {
            id: u32,
            name: String,
        }

        struct Product {
            id: u32,
            title: String,
        }

        impl Serializable for User {
            fn serialize(&self) -> String {
                format!("User {{ id: {}, name: '{}' }}", self.id, self.name)
            }
        }

        impl Serializable for Product {
            fn serialize(&self) -> String {
                format!("Product {{ id: {}, title: '{}' }}", self.id, self.title)
            }
        }

        fn guardar_serializable(item: &impl Serializable) -> String {
            item.serialize()
        }

        let user = User {
            id: 1,
            name: "Alice".into(),
        };
        let product = Product {
            id: 101,
            title: "Gadget".into(),
        };

        assert_eq!(guardar_serializable(&user), "User { id: 1, name: 'Alice' }");
        assert_eq!(
            guardar_serializable(&product),
            "Product { id: 101, title: 'Gadget' }"
        );
    }

    ///     3. INVERSIÓN DE DEPENDENCIAS (IoC)
    ///     ──────────────────────────────────
    ///     Las dependencias se adaptan a quien las use:
    ///     En vez de consumir dependencias como tipos concretos T que acoplan el código,
    ///     Se consumen implementaciones de traits que respetan contratos.
    ///     Entonces ahora las dependencias tienen que adaptarse al contrato (trait),
    ///     y no al revés. Esto facilita el cambio de implementaciones sin afectar
    ///     al código consumidor.
    ///
    #[test]
    pub fn inversion_de_dependencias() {
        // 1. LA ABSTRACCIÓN (El contrato que define el nivel superior)
        trait Logger {
            fn log(&self, message: &str) -> String;
        }

        // 2. NIVEL SUPERIOR (Lógica de negocio)
        // No sabe CÓMO se loguea, solo sabe que puede hacerlo a través del Trait.
        struct AppLogic<L: Logger> {
            logger: L,
        }

        impl<L: Logger> AppLogic<L> {
            fn do_work(&self) -> String {
                self.logger.log("Realizando operación importante...")
            }
        }

        // 3. NIVEL INFERIOR (Detalles de implementación)
        struct ConsoleLogger;
        impl Logger for ConsoleLogger {
            fn log(&self, message: &str) -> String {
                format!("[Consola]: {}", message)
            }
        }

        struct FileLogger;
        impl Logger for FileLogger {
            fn log(&self, message: &str) -> String {
                format!("[Archivo]: {}", message)
            }
        }

        // 4. INYECCIÓN (Decidimos la dependencia en el momento de la creación)
        let app_con_consola = AppLogic {
            logger: ConsoleLogger,
        };
        assert_eq!(
            app_con_consola.do_work(),
            "[Consola]: Realizando operación importante..."
        );

        let app_con_archivo = AppLogic { logger: FileLogger };
        assert_eq!(
            app_con_archivo.do_work(),
            "[Archivo]: Realizando operación importante..."
        );
    }

    ///     4. TESTABILIDAD Y MOCKING
    ///     ─────────────────────────
    ///     Facilitan la creación de "dobles de prueba" (mocks/stubs). Puedes sustituir
    ///     servicios pesados o externos por implementaciones ligeras en memoria
    ///     durante los tests, garantizando que las pruebas sean rápidas y deterministas.
    #[test]
    pub fn mocking() {
        trait DataService {
            fn get_data(&self, id: u32) -> String;
        }

        struct RealDataService;
        impl DataService for RealDataService {
            fn get_data(&self, id: u32) -> String {
                format!("Datos reales para id {}", id) // Simulación
            }
        }

        struct MockDataService;
        impl DataService for MockDataService {
            fn get_data(&self, id: u32) -> String {
                format!("Mock datos para id {}", id)
            }
        }

        struct DataProcessor<D: DataService> {
            service: D,
        }

        impl<D: DataService> DataProcessor<D> {
            fn process(&self, id: u32) -> String {
                self.service.get_data(id)
            }
        }

        // En producción
        let real_processor = DataProcessor {
            service: RealDataService,
        };
        assert_eq!(real_processor.process(1), "Datos reales para id 1");

        // En tests
        let mock_processor = DataProcessor {
            service: MockDataService,
        };
        assert_eq!(mock_processor.process(1), "Mock datos para id 1");
    }
}

// ============================================================================
//                     CONCEPTOS
// ============================================================================

#[cfg(test)]
mod conceptos {
    /*
    Un trait define un contrato de comportamiento:
    Cualquier tipo que implemente el trait debe tener esos metodos publicos
    */
    #[test]
    pub fn que_es_un_trait() {
        trait Saludable {
            fn saludar(&self) -> String;

            // Los traits pueden tener métodos con implementación default
            fn saludar_formal(&self) -> String {
                format!("Formalmente: {}", self.saludar())
            }
        }

        // Implementamos para diferentes tipos
        struct Persona {
            nombre: String,
        }
        impl Saludable for Persona {
            // public: hereda visibilidad del trait
            fn saludar(&self) -> String {
                format!("Hola, soy {}", self.nombre)
            }
        }

        struct Robot {
            modelo: String,
        }
        impl Saludable for Robot {
            fn saludar(&self) -> String {
                format!("BEEP BOOP. Unidad {}", self.modelo)
            }
        }

        let persona = Persona {
            nombre: "Alice".into(),
        };
        let robot = Robot {
            modelo: "R2D2".into(),
        };

        assert_eq!(persona.saludar(), "Hola, soy Alice");
        assert_eq!(persona.saludar_formal(), "Formalmente: Hola, soy Alice");
        assert_eq!(robot.saludar(), "BEEP BOOP. Unidad R2D2");
    }

    /// Formas de implementar traits es structs:
    ///
    /// ```text
    ///   IMPLEMENTACIÓN DE TRAITS
    ///
    ///   1. impl Trait for Type
    ///      └── Implementación manual
    ///
    ///   2. #[derive(Trait)]
    ///      └── El compilador genera la implementación
    ///
    ///   3. Blanket implementations (for any T: OtroTrait)
    ///      └── impl<T: OtroTrait> MiTrait for T
    ///
    ///   REGLA DE ORFANDAD (Orphan Rule):
    ///   ─────────────────────────────────
    ///   Al menos UNO debe ser local a tu crate.
    ///   ✅ impl Display for MiStruct    (MiStruct es local)
    ///   ✅ impl MiTrait for String      (MiTrait es local)
    ///   ❌ impl Display for String      (ambos son externos)
    ///   (Sin esta regla, dos crates podrían implementar el mismo trait para el mismo tipo:)
    /// ```

    // Manual:
    // ----------------------------------
    #[test]
    pub fn manual() {
        trait Operacion {
            fn aplicar(&self, x: i64) -> i64;

            // Método con implementación default
            fn aplicar_doble(&self, x: i64) -> i64 {
                self.aplicar(self.aplicar(x))
            }
        }

        // Implementación manual
        struct Sumar(i64);
        impl Operacion for Sumar {
            fn aplicar(&self, x: i64) -> i64 {
                x + self.0
            }
        }

        struct Multiplicar(i64);
        impl Operacion for Multiplicar {
            fn aplicar(&self, x: i64) -> i64 {
                x * self.0
            }
        }

        let suma = Sumar(10);
        let mult = Multiplicar(3);

        assert_eq!(suma.aplicar(5), 15);
        assert_eq!(suma.aplicar_doble(5), 25);
        assert_eq!(mult.aplicar(5), 15);
    }

    // Derive:
    // ----------------------------------
    pub fn derive() {
        #[derive(Debug, Clone, PartialEq, Default)]
        struct Punto {
            x: i32,
            y: i32,
        }

        let p1 = Punto { x: 1, y: 2 };
        let p2 = p1.clone();
        println!("{:?}", p1);
        assert_eq!(p2.clone(), p2);
        assert_eq!(p1, p2);
        assert_eq!(Punto::default(), Punto { x: 0, y: 0 });
    }

    // Blanket implementation: (where clause)
    // ----------------------------------
    // Blanket = sabana, cubre muchos casos, se usa con el generic T
    //
    //  impl<T: TraitB> TraitA for T
    //
    //  impl<T> TraitA for T
    //      where T: TraitB
    //
    // todos los T que implementen TraitB ahora tienen TraitA implementado
    #[test]
    pub fn blanket_impl() {
        use std::fmt::Display;

        // 1. Definimos un trait nuevo
        trait Logger {
            fn log_error(&self);
        }

        // 2. BLANKET IMPL: "Cualquier T que implemente Display, ahora implementa Logger"
        impl<T: Display> Logger for T {
            fn log_error(&self) {
                println!("❌ ERROR: {}", self); // Usamos el Display de T
            }
        }

        // i32 implementa Display, por lo tanto ahora también Logger
        404.log_error();
    }

    // Orphan Rule:
    // ----------------------------------
    // Aplicar mi trait a un tipo externo(String)
    // idem a la inversa
    #[test]
    pub fn orphan_rule() {
        trait Saludable {
            fn saludar(&self) -> String;
        }

        impl Saludable for String {
            fn saludar(&self) -> String {
                format!("Hola, {}", self)
            }
        }

        let nombre = "Mundo".to_string();
        assert_eq!(nombre.saludar(), "Hola, Mundo");
    }

    /// Las tres formas de aceptar traits como parámetros:
    ///
    /// ```text
    ///   TRAITS COMO PARÁMETROS: 3 FORMAS
    ///
    ///   ┌──────────────────────────────────────────────────────────────────┐
    ///   │ 1. impl Trait (syntax sugar)                                     │
    ///   │    fn procesar(x: impl Display)                                  │
    ///   │    └── Fácil de escribir, el compilador infiere el tipo          │
    ///   └──────────────────────────────────────────────────────────────────┘
    ///
    ///   ┌──────────────────────────────────────────────────────────────────┐
    ///   │ 2. Generic <T: Trait>                                            │
    ///   │    fn procesar<T: Display>(x: T)                                 │
    ///   │    └── Más flexible, permite bounds complejos, turbofish         │
    ///   └──────────────────────────────────────────────────────────────────┘
    ///
    ///   ┌──────────────────────────────────────────────────────────────────┐
    ///   │ 3. dyn Trait (trait object)                                      │
    ///   │    fn procesar(x: &dyn Display)                                  │
    ///   │    └── Polimorfismo en runtime, colecciones heterogéneas         │
    ///   └──────────────────────────────────────────────────────────────────┘
    ///
    ///   CUÁNDO USAR CADA UNO:
    ///   ─────────────────────
    ///   impl Trait  → Default, simple, un solo tipo concreto
    ///   <T: Trait>  → Múltiples bounds, turbofish, relaciones entre params
    ///   dyn Trait   → Colecciones heterogéneas, plugins, late binding
    /// ```
    #[test]
    pub fn traits_como_parametros() {
        trait Operacion {
            fn aplicar(&self, x: i64) -> i64;
        }

        struct Sumar(i64);
        impl Operacion for Sumar {
            fn aplicar(&self, x: i64) -> i64 {
                x + self.0
            }
        }

        struct Multiplicar(i64);
        impl Operacion for Multiplicar {
            fn aplicar(&self, x: i64) -> i64 {
                x * self.0
            }
        }

        // ─────────────────────────────────────────────────────────────
        // FORMA 1: impl Trait
        // ─────────────────────────────────────────────────────────────
        // impl Trait en parámetros es syntax sugar para generics <T: Trait>

        /*
            Syntax sugar	     Se expande a
            fn f(x: impl A)	     // fn f<T: A>(x: T)
            fn f(x: impl A + B)  //	fn f<T: A + B>(x: T)
            fn f(x: impl A, y: impl B) // fn f<T: A, U: B>(x: T, y: U)
        */
        fn con_impl_trait(op: impl Operacion, x: i64) -> i64 {
            op.aplicar(x)
        }

        assert_eq!(con_impl_trait(Sumar(10), 5), 15);

        // ─────────────────────────────────────────────────────────────
        // FORMA 2: Generic <T: Trait>
        // ─────────────────────────────────────────────────────────────
        fn con_generic<T: Operacion>(op: T, x: i64) -> i64 {
            op.aplicar(x)
        }

        assert_eq!(con_generic(Sumar(10), 5), 15);

        // Con where clause (más legible para bounds complejos)
        fn con_where<T>(op: T, x: i64) -> i64
        where
            T: Operacion,
        {
            op.aplicar(x)
        }

        assert_eq!(con_where(Multiplicar(2), 5), 10);

        // ─────────────────────────────────────────────────────────────
        // FORMA 3: dyn Trait
        // ─────────────────────────────────────────────────────────────
        // dyn Trait NO es syntax sugar de generics

        // Con referencia (borrowed)
        fn con_dyn(op: &dyn Operacion, x: i64) -> i64 {
            op.aplicar(x)
        }

        assert_eq!(con_dyn(&Sumar(10), 5), 15);

        // Con Box (ownership)
        fn con_box_dyn(op: Box<dyn Operacion>, x: i64) -> i64 {
            op.aplicar(x)
        }

        assert_eq!(con_box_dyn(Box::new(Multiplicar(2)), 5), 10);

        // ─────────────────────────────────────────────────────────────
        // DIFERENCIA CLAVE: Colecciones heterogéneas
        // ─────────────────────────────────────────────────────────────

        // Solo con dyn, porque el tamaño de los elementos puede variar en runtime

        // ❌ Esto NO compila:
        // let ops: Vec<impl Operacion> = vec![Sumar(1), Multiplicar(2)];

        // ✅ Esto SÍ compila:
        let ops: Vec<Box<dyn Operacion>> = vec![Box::new(Sumar(1)), Box::new(Multiplicar(2))];

        let mut resultado = 10;
        for (_i, op) in ops.iter().enumerate() {
            let next = op.aplicar(resultado);
            assert_eq!(next, con_dyn(&**op, resultado));
            resultado = next;
        }
    }

    /// Retornar traits de funciones:
    ///
    ///   TRAITS COMO RETORNO
    ///
    ///   ┌────────────────────────────────────────────────────────────────┐
    ///   │ 1. -> impl Trait                                               │
    ///   │    ✓ Un solo tipo concreto (decidido en compile time)          │
    ///   │    ✓ Zero-cost abstraction                                     │
    ///   │    ✗ No puede retornar tipos diferentes según condición        │
    ///   └────────────────────────────────────────────────────────────────┘
    ///
    ///   ┌────────────────────────────────────────────────────────────────┐
    ///   │ 2. -> Box<dyn Trait>                                           │
    ///   │    ✓ Puede retornar cualquier tipo que implemente el trait     │
    ///   │    ✓ Decisión en runtime                                       │
    ///   │    ✗ Allocation en heap + indirección                          │
    ///   └────────────────────────────────────────────────────────────────┘
    ///
    ///   EJEMPLO:
    ///   ─────────
    ///   fn crear_op(tipo: &str) -> Box<dyn Op> {
    ///       match tipo {
    ///           "suma" => Box::new(Sumar(1)),    // tipo A
    ///           _      => Box::new(Mult(2)),     // tipo B
    ///       }
    ///   }
    ///   // ^ Esto REQUIERE dyn porque retorna tipos diferentes
    /// ```
    #[test]
    pub fn traits_como_retorno() {
        trait Operacion {
            fn aplicar(&self, x: i64) -> i64;
        }

        struct Sumar(i64);
        impl Operacion for Sumar {
            fn aplicar(&self, x: i64) -> i64 {
                x + self.0
            }
        }

        struct Multiplicar(i64);
        impl Operacion for Multiplicar {
            fn aplicar(&self, x: i64) -> i64 {
                x * self.0
            }
        }

        // ─────────────────────────────────────────────────────────────
        // FORMA 1: -> impl Trait (un solo tipo concreto)
        // ─────────────────────────────────────────────────────────────
        fn crear_sumador() -> impl Operacion {
            Sumar(100)
        }

        // ❌ Esto NO compila (dos tipos diferentes):
        // fn crear_segun_flag(flag: bool) -> impl Operacion {
        //     if flag { Sumar(1) } else { Multiplicar(2) }
        // }

        let op = crear_sumador();
        assert_eq!(op.aplicar(5), 105);

        // ─────────────────────────────────────────────────────────────
        // FORMA 2: -> Box<dyn Trait> (múltiples tipos posibles)
        // ─────────────────────────────────────────────────────────────
        fn crear_segun_tipo(tipo: &str) -> Box<dyn Operacion> {
            match tipo {
                "suma" => Box::new(Sumar(10)),
                "mult" => Box::new(Multiplicar(5)),
                _ => Box::new(Sumar(0)), // identidad para suma
            }
        }

        let suma = crear_segun_tipo("suma");
        let mult = crear_segun_tipo("mult");
        assert_eq!(suma.aplicar(5), 15);
        assert_eq!(mult.aplicar(5), 25);

        // ─────────────────────────────────────────────────────────────
        // Caso especial: Closures
        // ─────────────────────────────────────────────────────────────
        // Cada closure es un tipo único concreto, anónimo y con tamaño distinto.
        // otro closure con mismo trait es otro tipo distinto y no respeta el impl Trait.
        // por no ser el mismo tipo concreto y tampoco mismo tamaño.
        fn crear_closure() -> impl Fn(i64) -> i64 {
            |x| x * 2
        }

        let f = crear_closure();
        assert_eq!(f(10), 20);
    }
}

// ============================================================================
//                     TESTING
// ============================================================================
#[cfg(test)]
mod testing {
    use std::collections::HashMap;

    // ============================================================================
    //              DEMO 7: TESTING CON TRAITS
    // ============================================================================

    /// Traits permiten inyección de dependencias y mocking:
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────────────────────────────┐
    /// │  TESTING CON TRAITS - Inyección de Dependencias                         │
    /// │                                                                         │
    /// │  PRODUCCIÓN                          TESTING                            │
    /// │  ───────────                         ───────                            │
    /// │  ┌─────────────────┐                ┌─────────────────┐                 │
    /// │  │   PostgresDB    │                │    MockDB       │                 │
    /// │  │   impl DB       │                │    impl DB      │                 │
    /// │  └────────┬────────┘                └────────┬────────┘                 │
    /// │           │                                  │                          │
    /// │           └──────────────┬───────────────────┘                          │
    /// │                          │                                              │
    /// │                          ▼                                              │
    /// │                  trait Database {                                       │
    /// │                      fn query(&self, sql: &str) -> Vec<Row>;            │
    /// │                  }                                                      │
    /// │                          │                                              │
    /// │                          ▼                                              │
    /// │                  ┌───────────────┐                                      │
    /// │                  │   Service<D>  │                                      │
    /// │                  │   D: Database │                                      │
    /// │                  └───────────────┘                                      │
    /// │                                                                         │
    /// │  El Service no sabe ni le importa si es PostgresDB o MockDB.            │
    /// │  Solo sabe que tiene algo que implementa Database.                      │
    /// └─────────────────────────────────────────────────────────────────────────┘
    /// ```
    #[test]
    pub fn testing_con_traits() {
        // ─────────────────────────────────────────────────────────────
        // Definimos el trait (contrato)
        // ─────────────────────────────────────────────────────────────
        #[derive(Clone, Debug, PartialEq)]
        struct User {
            id: u64,
            name: String,
        }

        #[derive(Debug, PartialEq, Clone)]
        enum RepoError {
            NotFound,
        }

        trait UserRepo {
            fn find(&self, id: u64) -> Result<User, RepoError>;
            fn save(&mut self, user: User) -> Result<(), RepoError>;
        }

        // ─────────────────────────────────────────────────────────────
        // Implementación REAL (producción)
        // ─────────────────────────────────────────────────────────────
        struct MemoryRepo {
            data: HashMap<u64, User>,
        }

        impl MemoryRepo {
            fn new() -> Self {
                Self {
                    data: HashMap::new(),
                }
            }
        }

        impl UserRepo for MemoryRepo {
            fn find(&self, id: u64) -> Result<User, RepoError> {
                self.data.get(&id).cloned().ok_or(RepoError::NotFound)
            }

            fn save(&mut self, user: User) -> Result<(), RepoError> {
                self.data.insert(user.id, user);
                Ok(())
            }
        }

        // ─────────────────────────────────────────────────────────────
        // Implementación MOCK (testing)
        // ─────────────────────────────────────────────────────────────
        struct MockRepo {
            responses: HashMap<u64, Result<User, RepoError>>,
            #[allow(dead_code)]
            save_calls: Vec<User>,
        }

        impl MockRepo {
            fn new() -> Self {
                Self {
                    responses: HashMap::new(),
                    save_calls: Vec::new(),
                }
            }

            fn when_find(&mut self, id: u64, result: Result<User, RepoError>) {
                self.responses.insert(id, result);
            }
        }

        impl UserRepo for MockRepo {
            fn find(&self, id: u64) -> Result<User, RepoError> {
                self.responses
                    .get(&id)
                    .cloned()
                    .unwrap_or(Err(RepoError::NotFound))
            }

            fn save(&mut self, user: User) -> Result<(), RepoError> {
                self.save_calls.push(user);
                Ok(())
            }
        }

        // ─────────────────────────────────────────────────────────────
        // Service genérico sobre el trait
        // ─────────────────────────────────────────────────────────────
        struct UserService<R: UserRepo> {
            repo: R,
        }

        impl<R: UserRepo> UserService<R> {
            fn new(repo: R) -> Self {
                Self { repo }
            }

            fn get_name(&self, id: u64) -> Result<String, RepoError> {
                let user = self.repo.find(id)?;
                Ok(user.name)
            }
        }

        // ─────────────────────────────────────────────────────────────
        // Uso en "producción"
        // ─────────────────────────────────────────────────────────────
        println!("USO CON IMPLEMENTACIÓN REAL:");
        let mut real_repo = MemoryRepo::new();
        real_repo
            .save(User {
                id: 1,
                name: "Alice".into(),
            })
            .unwrap();

        let service = UserService::new(real_repo);
        println!("  service.get_name(1) = {:?}", service.get_name(1));
        println!("  service.get_name(99) = {:?}", service.get_name(99));
        println!();

        // ─────────────────────────────────────────────────────────────
        // Uso en testing con mock
        // ─────────────────────────────────────────────────────────────
        println!("USO CON MOCK (testing):");
        let mut mock = MockRepo::new();
        mock.when_find(
            42,
            Ok(User {
                id: 42,
                name: "Test User".into(),
            }),
        );
        mock.when_find(99, Err(RepoError::NotFound));

        let test_service = UserService::new(mock);
        println!(
            "  test_service.get_name(42) = {:?}",
            test_service.get_name(42)
        );
        println!(
            "  test_service.get_name(99) = {:?}",
            test_service.get_name(99)
        );
        println!();

        // ─────────────────────────────────────────────────────────────
        // Alternativa: Box<dyn Trait> para late binding
        // ─────────────────────────────────────────────────────────────
        println!("ALTERNATIVA CON Box<dyn Trait>:");

        struct DynUserService {
            repo: Box<dyn UserRepo>,
        }

        impl DynUserService {
            fn new(repo: Box<dyn UserRepo>) -> Self {
                Self { repo }
            }

            #[allow(dead_code)]
            fn get_name(&self, id: u64) -> Result<String, RepoError> {
                Ok(self.repo.find(id)?.name)
            }
        }

        let dyn_service = DynUserService::new(Box::new(MemoryRepo::new()));
        println!("  DynUserService creado con Box<dyn UserRepo>");
        println!("  Útil cuando no conocemos el tipo en compile time");
        let _ = dyn_service;
        println!();
    }
}

// ============================================================================
//                     AVANZADO
// ============================================================================
#[cfg(test)]
mod avanzado {

    // ============================================================================
    //              DEMO 5: OBJECT SAFETY en dyn TRAITS
    // ============================================================================

    ///
    ///   Un trait es "object safe" si:
    ///
    ///   PERMITIDO:
    ///   ─────────────
    ///   • Métodos con &self, &mut self, self: Box<Self>
    ///   • Métodos que retornan tipos concretos
    ///   • Constantes asociadas
    ///
    ///   PROHIBIDO:
    ///   ─────────────
    ///   • Métodos con Self en parámetros o retorno
    ///   • Métodos genéricos
    ///   • Funciones asociadas (sin self)
    ///   • where Self: Sized
    ///
    ///   No todos los traits pueden usarse con dyn:
    ///   ──────────
    ///   El compilador necesita saber el tamaño de todos los argumentos
    ///   y retornos en compile time para construir la vtable.
    ///
    ///   trait Clone {
    ///       fn clone(&self) -> Self;  // ❌ Self = tipo desconocido
    ///   }
    ///   No puedo hacer dyn Clone porque ¿qué tamaño tiene el retorno?
    ///   dyn Trait usa una vtable (tabla de punteros a funciones).

    /// ```
    #[test]
    pub fn object_safety() {
        // ─────────────────────────────────────────────────────────────
        // TRAIT OBJECT-SAFE
        // ─────────────────────────────────────────────────────────────
        trait Dibujable {
            fn dibujar(&self) -> String;
        }

        struct Circulo;
        impl Dibujable for Circulo {
            fn dibujar(&self) -> String {
                "○".into()
            }
        }

        struct Cuadrado;
        impl Dibujable for Cuadrado {
            fn dibujar(&self) -> String {
                "□".into()
            }
        }

        // ✅ Puedo usar dyn porque es object-safe
        let formas: Vec<Box<dyn Dibujable>> = vec![Box::new(Circulo), Box::new(Cuadrado)];

        println!("✅ Trait object-safe (Dibujable):");
        for forma in &formas {
            println!("   {}", forma.dibujar());
        }
        println!();

        // ─────────────────────────────────────────────────────────────
        // TRAIT NO OBJECT-SAFE (ejemplo conceptual)
        // ─────────────────────────────────────────────────────────────
        /*

          trait Clonable {{
              fn clone(&self) -> Self;  // Self en retorno
          }}
          // Vec<Box<dyn Clonable>> ← ERROR

          trait Comparable {{
              fn comparar(self, otro: bool) -> bool;  // self en param
          }}
          // &dyn Comparable ← ERROR

          trait Generico {{
              fn procesar<T>(&self, x: T);  // Método genérico
          }}
          // &dyn Generico ← ERROR
        */

        // ─────────────────────────────────────────────────────────────
        // WORKAROUND: where Self: Sized
        // ─────────────────────────────────────────────────────────────
        #[allow(dead_code)]
        trait MixtoTrait {
            fn metodo_normal(&self) -> i32;

            // Este método NO estará en la vtable, pero el trait sigue siendo object-safe
            // Este metodo estara disponible solo si el compilador sabe el tamaño del tipo concreto
            // solo las implementaciones directas del trait podran usar este metodo
            fn metodo_con_self(&self) -> Self
            where
                Self: Sized;
        }
    }

    // ============================================================================
    //           DEMO 5B: VTABLE Y DYN EN DETALLE
    // ============================================================================

    /// Explicación detallada de cómo funciona dyn Trait internamente:
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────────────────────────────────┐
    /// │  &dyn Trait = FAT POINTER (16 bytes en 64-bit)                              │
    /// │                                                                             │
    /// │  ┌────────────────────────────────────────┐                                 │
    /// │  │ data_ptr   │ vtable_ptr               │                                 │
    /// │  │ (8 bytes)  │ (8 bytes)                │                                 │
    /// │  └─────┬──────┴─────────┬────────────────┘                                 │
    /// │        │                │                                                   │
    /// │        ▼                ▼                                                   │
    /// │  ┌──────────┐     ┌─────────────────────────────────────┐                   │
    /// │  │ DATOS    │     │ VTABLE                              │                   │
    /// │  │ del tipo │     │                                     │                   │
    /// │  │ concreto │     │  ┌───────────────┬────────────────┐ │                   │
    /// │  │          │     │  │ drop_fn       │ → destructor   │ │                   │
    /// │  │ Sumar(10)│     │  │ size          │ → 8 bytes      │ │                   │
    /// │  │          │     │  │ align         │ → 8            │ │                   │
    /// │  └──────────┘     │  │ aplicar_fn    │ → Sumar::aplicar│ │                   │
    /// │                   │  │ otro_metodo_fn│ → ...          │ │                   │
    /// │                   │  └───────────────┴────────────────┘ │                   │
    /// │                   └─────────────────────────────────────┘                   │
    /// └─────────────────────────────────────────────────────────────────────────────┘
    ///
    /*
    Cada tipo del dyn Trait tiene su propia VTABLE generada en compile time.

        ┌─────────────────────────────────────────────────────────────────────────────┐
        │  VTABLE (Virtual Table) - Tabla de punteros a funciones                     │
        │                                                                             │
        │  Cada combinación (Tipo, Trait) genera UNA vtable en compile time:          │
        │                                                                             │
        │  VTABLE para Sumar                     VTABLE para Multiplicar              │
        │  ───────────────────────────────────   ─────────────────────────────────    │
        │  drop_fn:       Sumar::drop            drop_fn:       Multiplicar::drop     │
        │  size:          8 bytes                size:          8 bytes               │
        │  align:         8                      align:         8                     │
        │  aplicar_fn:    Sumar::aplicar         aplicar_fn:    Multiplicar::aplicar  │
        │  descripcion_fn: Sumar::descripcion    descripcion_fn: Mult::descripcion    │
        └─────────────────────────────────────────────────────────────────────────────┘
      ┌─────────────────────────────────────────────────────────────────────────────┐
        │  CÓMO FUNCIONA UNA LLAMADA A MÉTODO:                                        │
        │                                                                             │
        │  op.aplicar(x)   donde op: &dyn Operacion                                   │
        │                                                                             │
        │  1. Leer vtable_ptr del fat pointer                                         │
        │  2. Buscar aplicar_fn en la vtable (offset conocido en compile time)        │
        │  3. Llamar: (vtable.aplicar_fn)(data_ptr, x)                                │
        │                                                                             │
        │  Pseudocódigo:                                                              │
        │  ┌────────────────────────────────────────────────────────────────┐         │
        │  │ let fn_ptr = op.vtable[offset_de_aplicar];  // Buscar en tabla │         │
        │  │ fn_ptr(op.data, x);                         // Llamar función  │         │
        │  └────────────────────────────────────────────────────────────────┘         │
        └─────────────────────────────────────────────────────────────────────────────┘
        ┌─────────────────────────────────────────────────────────────────────────┐
        │  STATIC DISPATCH (impl Trait / Generics)                                │
        │  ────────────────────────────────────────                               │
        │                                                                         │
        │  fn procesar<T: Op>(x: T) { x.aplicar() }                               │
        │                                                                         │
        │  procesar(Sumar(1));    → El compilador genera: procesar_Sumar()        │
        │  procesar(Mult(2));     → El compilador genera: procesar_Mult()         │
        │                                                                         │
        │  ┌────────────────┐                                                     │
        │  │ MONOMORPHIZATION│  = Genera código especializado para cada tipo      │
        │  └────────────────┘                                                     │
        │                                                                         │
        │  ✅ Ventajas:                     ❌ Desventajas:                        │
        │  • Inlining posible              • Más código generado (binary size)    │
        │  • Sin indirección               • Compile time más largo               │
        │  • Sin allocation extra          • No colecciones heterogéneas          │
        │                                                                         │
        ├─────────────────────────────────────────────────────────────────────────┤
        │  DYNAMIC DISPATCH (dyn Trait)                                           │
        │  ────────────────────────────                                           │
        │                                                                         │
        │  fn procesar(x: &dyn Op) { x.aplicar() }                                │
        │                                                                         │
        │  ┌─────────────────────────────────────────────────────────────────┐    │
        │  │  &dyn Op = FAT POINTER (16 bytes en 64-bit)                     │    │
        │  │                                                                 │    │
        │  │  ┌────────────┬────────────┐                                    │    │
        │  │  │ data_ptr   │ vtable_ptr │                                    │    │
        │  │  │ (8 bytes)  │ (8 bytes)  │                                    │    │
        │  │  └─────┬──────┴─────┬──────┘                                    │    │
        │  │        │            │                                           │    │
        │  │        ▼            ▼                                           │    │
        │  │  ┌──────────┐  ┌─────────────────────┐                          │    │
        │  │  │ Sumar(1) │  │ vtable para Sumar   │                          │    │
        │  │  │ (datos)  │  │ ┌─────────────────┐ │                          │    │
        │  │  └──────────┘  │ │ drop_fn ptr     │ │                          │    │
        │  │                │ │ size            │ │                          │    │
        │  │                │ │ align           │ │                          │    │
        │  │                │ │ aplicar_fn ptr ─┼─┼─→ Sumar::aplicar()       │    │
        │  │                │ └─────────────────┘ │                          │    │
        │  │                └─────────────────────┘                          │    │
        │  └─────────────────────────────────────────────────────────────────┘    │
        │                                                                         │
        │  ✅ Ventajas:                     ❌ Desventajas:                        │
        │  • Colecciones heterogéneas      • Indirección (cache miss)             │
        │  • Binary más pequeño            • No inlining                          │
        │  • Late binding / plugins        • Allocation para Box<dyn>             │
        └─────────────────────────────────────────────────────────────────────────┘
            ┌─────────────────────────────────────────────────────────────────────────────┐
        │  ¿POR QUÉ -> Self NO ES OBJECT-SAFE?                                        │
        │                                                                             │
        │  El problema NO es encontrar el método (eso funciona via vtable).           │
        │  El problema es: ¿DÓNDE PONGO EL RESULTADO?                                 │
        │                                                                             │
        │  trait Clonable {{                                                           │
        │      fn clone(&self) -> Self;  // ← ¿Cuántos bytes tiene el retorno?        │
        │  }}                                                                          │
        │                                                                             │
        │  fn problema(x: &dyn Clonable) {{                                            │
        │      let copia = x.clone();  // ← ¿Reservar 1 byte? ¿1000 bytes?            │
        │  }}                                                                          │
        │                                                                             │
        │  STACK FRAME (se construye en COMPILE TIME):                                │
        │  ┌──────────────────────────────────┐                                       │
        │  │ x: &dyn Clonable (16 bytes)      │  ← Esto sí lo sabe                    │
        │  ├──────────────────────────────────┤                                       │
        │  │ copia: ???                       │  ← ¿1 byte? ¿1000 bytes?              │
        │  │        ↑                         │     NO LO SABE                        │
        │  │   ¿Cuánto espacio reservar?      │                                       │
        │  └──────────────────────────────────┘                                       │
        │                                                                             │
        │  El tipo concreto se conoce en RUNTIME, pero el stack frame                 │
        │  se construye en COMPILE TIME. Contradicción irresoluble.                   │
        │                                                                             │
        │  SOLUCIÓN: -> Box<Self> (siempre 8 bytes, el dato va al heap)               │
        └─────────────────────────────────────────────────────────────────────────────┘
    */
    #[test]
    pub fn vtable_y_dyn() {
        trait Operacion {
            fn aplicar(&self, x: i64) -> i64;
            #[allow(dead_code)]
            fn descripcion(&self) -> &str;
        }

        struct Sumar(i64);
        impl Operacion for Sumar {
            fn aplicar(&self, x: i64) -> i64 {
                x + self.0
            }
            fn descripcion(&self) -> &str {
                "suma"
            }
        }

        struct Multiplicar(i64);
        impl Operacion for Multiplicar {
            fn aplicar(&self, x: i64) -> i64 {
                x * self.0
            }
            fn descripcion(&self) -> &str {
                "multiplicación"
            }
        }

        // Crear valores y fat pointers
        let suma = Sumar(10);
        let mult = Multiplicar(10);

        let dyn_suma: &dyn Operacion = &suma;
        let dyn_mult: &dyn Operacion = &mult;

        assert_eq!(dyn_suma.aplicar(5), 15);
        assert_eq!(dyn_mult.aplicar(5), 50);
    }

    // ============================================================================
    //              DEMO 8: PATRONES AVANZADOS
    // ============================================================================

    /// Patrones avanzados con traits:
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────────────────────────────┐
    /// │  PATRONES AVANZADOS                                                     │
    /// │                                                                         │
    /// │  1. TRAIT BOUNDS MÚLTIPLES                                              │
    /// │     fn f<T: Clone + Debug + Send>(x: T)                                 │
    /// │     fn f<T>(x: T) where T: Clone + Debug + Send                         │
    /// │                                                                         │
    /// │  2. ASSOCIATED TYPES vs GENERICS                                        │
    /// │     trait Iterator { type Item; }     ← Un solo Item por impl           │
    /// │     trait Add<Rhs> { ... }            ← Múltiples Rhs por impl          │
    /// │                                                                         │
    /// │  3. SUPERTRAITS                                                         │
    /// │     trait Drawable: Clone + Debug { }                                   │
    /// │     └── Quien implemente Drawable DEBE implementar Clone y Debug        │
    /// │                                                                         │
    /// │  4. EXTENSION TRAITS                                                    │
    /// │     trait StringExt { fn shout(&self) -> String; }                      │
    /// │     impl StringExt for str { ... }                                      │
    /// │     └── Añadir métodos a tipos existentes                               │
    /// │                                                                         │
    /// │  5. MARKER TRAITS                                                       │
    /// │     trait Marker {}                                                     │
    /// │     └── Sin métodos, solo para "etiquetar" tipos (Send, Sync, Copy)     │
    /// │                                                                         │
    /// │  6. IMPL PARA TIPOS CONCRETOS DESDE GENERICS                            │
    /// │     struct Wrapper<T> { value: T }                                      │
    /// │     impl<T> Wrapper<T> { ... }      ← Para todos los T                  │
    /// │     impl Wrapper<i32> { ... }       ← Solo para Wrapper<i32>            │
    /// │     impl Wrapper<String> { ... }    ← Solo para Wrapper<String>         │
    /// │     └── Especialización manual de comportamiento por tipo               │
    /// └─────────────────────────────────────────────────────────────────────────┘
    /// ```
    #[test]
    pub fn patrones() {
        println!("═══════════════════════════════════════════════════════════════════");
        println!("  DEMO 8: PATRONES AVANZADOS");
        println!("═══════════════════════════════════════════════════════════════════\n");

        // ─────────────────────────────────────────────────────────────
        // 1. TRAIT BOUNDS MÚLTIPLES
        // ─────────────────────────────────────────────────────────────
        use std::fmt::Debug;

        fn procesar_y_mostrar<T: Clone + Debug>(x: T) {
            let copia = x.clone();
            println!("  Original: {:?}, Copia: {:?}", x, copia);
        }

        println!("1. TRAIT BOUNDS MÚLTIPLES (T: Clone + Debug):");
        procesar_y_mostrar(vec![1, 2, 3]);
        println!();

        // ─────────────────────────────────────────────────────────────
        // 2. ASSOCIATED TYPES
        // ─────────────────────────────────────────────────────────────
        trait Container {
            type Item;

            fn get(&self) -> Option<&Self::Item>;
            fn put(&mut self, item: Self::Item);
        }

        struct Stack<T> {
            items: Vec<T>,
        }

        impl<T> Container for Stack<T> {
            type Item = T;

            fn get(&self) -> Option<&T> {
                self.items.last()
            }

            fn put(&mut self, item: T) {
                self.items.push(item);
            }
        }

        println!("2. ASSOCIATED TYPES:");
        let mut stack: Stack<i32> = Stack { items: vec![] };
        stack.put(10);
        stack.put(20);
        println!("  Stack top: {:?}", stack.get());
        println!();

        // ─────────────────────────────────────────────────────────────
        // 3. SUPERTRAITS
        // ─────────────────────────────────────────────────────────────
        trait Printable: Debug {
            fn print(&self) {
                println!("  Printable: {:?}", self);
            }
        }

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Documento {
            titulo: String,
        }

        impl Printable for Documento {}

        println!("3. SUPERTRAITS (Printable: Debug):");
        let doc = Documento {
            titulo: "Mi Doc".into(),
        };
        doc.print();
        println!();

        // ─────────────────────────────────────────────────────────────
        // 4. EXTENSION TRAITS
        // ─────────────────────────────────────────────────────────────
        trait StringExt {
            fn gritar(&self) -> String;
            fn es_pregunta(&self) -> bool;
        }

        impl StringExt for str {
            fn gritar(&self) -> String {
                self.to_uppercase() + "!"
            }

            fn es_pregunta(&self) -> bool {
                self.ends_with('?')
            }
        }

        println!("4. EXTENSION TRAITS (añadir métodos a str):");
        println!("  \"hola\".gritar() = {}", "hola".gritar());
        println!(
            "  \"como estas?\".es_pregunta() = {}",
            "como estas?".es_pregunta()
        );
        println!();

        // ─────────────────────────────────────────────────────────────
        // 5. BLANKET IMPLEMENTATIONS
        // ─────────────────────────────────────────────────────────────
        trait Describir {
            fn describir(&self) -> String;
        }

        // Implementación "blanket": para TODOS los T que sean Debug
        impl<T: Debug> Describir for T {
            fn describir(&self) -> String {
                format!("Soy: {:?}", self)
            }
        }

        println!("5. BLANKET IMPLEMENTATIONS (impl<T: Debug> Describir for T):");
        println!("  42.describir() = {}", 42.describir());
        println!("  vec![1,2].describir() = {}", vec![1, 2].describir());
        println!();

        // ─────────────────────────────────────────────────────────────
        // 6. IMPL PARA TIPOS CONCRETOS DESDE GENERICS
        // ─────────────────────────────────────────────────────────────
        // Puedes definir un struct/trait genérico y luego implementar
        // métodos SOLO para ciertos tipos concretos.

        println!("6. IMPL PARA TIPOS CONCRETOS DESDE GENERICS:");
        println!();

        // Struct genérico
        struct Wrapper<T> {
            value: T,
        }

        // Implementación genérica (disponible para TODOS los T)
        impl<T> Wrapper<T> {
            fn new(value: T) -> Self {
                Wrapper { value }
            }

            fn get(&self) -> &T {
                &self.value
            }
        }

        // Implementación SOLO para Wrapper<i32>
        impl Wrapper<i32> {
            fn double(&self) -> i32 {
                self.value * 2
            }

            fn is_positive(&self) -> bool {
                self.value > 0
            }
        }

        // Implementación SOLO para Wrapper<String>
        impl Wrapper<String> {
            fn shout(&self) -> String {
                self.value.to_uppercase() + "!"
            }

            fn len(&self) -> usize {
                self.value.len()
            }
        }

        // Implementación para Wrapper<T> donde T: Clone + Debug
        impl<T: Clone + Debug> Wrapper<T> {
            fn clone_and_describe(&self) -> String {
                format!("Clonado: {:?}", self.value.clone())
            }
        }

        let w_int = Wrapper::new(42);
        let w_str = Wrapper::new(String::from("hola"));
        let w_vec = Wrapper::new(vec![1, 2, 3]);

        println!("  Wrapper<i32>:");
        println!("    w_int.get() = {}", w_int.get());
        println!("    w_int.double() = {}", w_int.double()); // Solo disponible para i32
        println!("    w_int.is_positive() = {}", w_int.is_positive());
        println!();

        println!("  Wrapper<String>:");
        println!("    w_str.get() = {}", w_str.get());
        println!("    w_str.shout() = {}", w_str.shout()); // Solo disponible para String
        println!("    w_str.len() = {}", w_str.len());
        println!();

        println!("  Wrapper<Vec<i32>>:");
        println!("    w_vec.get() = {:?}", w_vec.get());
        // w_vec.double() ❌ NO COMPILA - double() solo existe para Wrapper<i32>
        // w_vec.shout()  ❌ NO COMPILA - shout() solo existe para Wrapper<String>
        println!(
            "    w_vec.clone_and_describe() = {}",
            w_vec.clone_and_describe()
        );
        println!();

        println!(
            r#"  ┌────────────────────────────────────────────────────────────────────┐
      │ RESUMEN: Impl para tipos concretos desde generics                  │
      ├────────────────────────────────────────────────────────────────────┤
      │                                                                    │
      │  struct Wrapper<T> {{ value: T }}                                  │
      │                                                                    │
      │  impl<T> Wrapper<T>         → Métodos para TODOS los T             │
      │  impl Wrapper<i32>          → Métodos SOLO para Wrapper<i32>       │
      │  impl Wrapper<String>       → Métodos SOLO para Wrapper<String>    │
      │  impl<T: Bound> Wrapper<T>  → Métodos para T que cumplan Bound     │
      │                                                                    │
      │  Esto permite "especialización" manual:                            │
      │  • Comportamiento genérico por defecto                             │
      │  • Métodos optimizados/específicos para ciertos tipos              │
      │  • API más rica para tipos donde tiene sentido                     │
      │                                                                    │
      └────────────────────────────────────────────────────────────────────┘
    "#
        );
    }

    // ============================================================================
    //              DEMO 9: TRAIT CON TYPE ASOCIADO + IMPL/DYN
    // ============================================================================

    /// Ejemplo de trait con 3 tipos asociados y uso con `impl` y `dyn`:
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────────────────────────────┐
    /// │  TRAIT CON 3 TYPES ASOCIADOS (RESTRICCIONES)                            │
    /// │                                                                         │
    /// │  trait TripleTransform {                                                │
    /// │      type Input;                                                        │
    /// │      type Output;                                                       │
    /// │      type Error;                                                        │
    /// │      fn transform(&self, input: Self::Input)                            │
    /// │          -> Result<Self::Output, Self::Error>;                          │
    /// │  }                                                                      │
    /// │                                                                         │
    /// │  RESTRICCIÓN 1: Associated Types en dyn Trait                           │
    /// │  ────────────────────────────────────────────────────────               │
    /// │  Para usar dyn, TODOS los tipos asociados deben estar fijados:          │
    /// │  ✅ &dyn TripleTransform<Input=i32, Output=String, Error=String>        │
    /// │  ❌ &dyn TripleTransform  (falta fijar los tipos)                       │
    /// │                                                                         │
    /// │  RESTRICCIÓN 2: Object Safety con Self en retorno                       │
    /// │  ──────────────────────────────────────────────────                    │
    /// │  Si el trait retorna Self o Self::Output (sin fijar), NO es object-safe:│
    /// │  ❌ fn method(&self) -> Self                                            │
    /// │  ❌ fn method(&self) -> Self::Output  (si Output no está fijado)         │
    /// │  ✅ fn method(&self) -> Result<Self::Output, Self::Error>               │
    /// │     (porque Output y Error están en el dyn)                             │
    /// │                                                                         │
    /// │  RESTRICCIÓN 3: Métodos genéricos en dyn Trait                          │
    /// │  ────────────────────────────────────────────────────                   │
    /// │  ❌ fn procesar<T>(&self, x: T)  (método genérico)                      │
    /// │  ✅ fn procesar(&self, x: Self::Input)  (usa tipo asociado)             │
    /// │                                                                         │
    /// │  RESTRICCIÓN 4: Funciones asociadas en dyn Trait                        │
    /// │  ────────────────────────────────────────────────                       │
    /// │  ❌ fn new() -> Self  (sin self, no puede estar en vtable)               │
    /// │  ✅ fn transform(&self, ...)  (tiene self, puede estar en vtable)        │
    /// └─────────────────────────────────────────────────────────────────────────┘
    /// ```
    #[test]
    pub fn types_asociados() {
        println!("═══════════════════════════════════════════════════════════════════");
        println!("  DEMO 9: TRAIT CON 3 TYPES ASOCIADOS (RESTRICCIONES OBJECT-SAFE)");
        println!("═══════════════════════════════════════════════════════════════════\n");

        trait TripleTransform {
            type Input;
            type Output;
            type Error;

            fn transform(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
        }

        struct SafeConverter;
        impl TripleTransform for SafeConverter {
            type Input = i32;
            type Output = String;
            type Error = String;

            fn transform(&self, input: i32) -> Result<String, String> {
                if input < 0 {
                    Err("No se permiten negativos".to_string())
                } else {
                    Ok(format!("Valor seguro: {}", input))
                }
            }
        }

        struct MathConverter;
        impl TripleTransform for MathConverter {
            type Input = i32;
            type Output = i32;
            type Error = &'static str;

            fn transform(&self, input: i32) -> Result<i32, &'static str> {
                if input == 0 {
                    Err("No se puede dividir entre cero")
                } else {
                    Ok(100 / input)
                }
            }
        }

        // ─────────────────────────────────────────────────────────────
        // Uso con impl (Static Dispatch)
        // ─────────────────────────────────────────────────────────────
        fn usar_impl(
            t: impl TripleTransform<Input = i32, Output = String, Error = String>,
            val: i32,
        ) {
            match t.transform(val) {
                Ok(s) => println!("  [impl] Éxito: {}", s),
                Err(e) => println!("  [impl] Error: {}", e),
            }
        }

        // ─────────────────────────────────────────────────────────────
        // Uso con dyn (Dynamic Dispatch) - TODOS los tipos fijados
        // ─────────────────────────────────────────────────────────────
        fn usar_dyn(
            t: &dyn TripleTransform<Input = i32, Output = String, Error = String>,
            val: i32,
        ) {
            match t.transform(val) {
                Ok(s) => println!("  [dyn] Éxito: {}", s),
                Err(e) => println!("  [dyn] Error: {}", e),
            }
        }

        let converter = SafeConverter;
        let _math = MathConverter;

        println!("RESTRICCIÓN 1: En dyn, TODOS los tipos asociados deben estar fijados");
        println!("────────────────────────────────────────────────────────────────────\n");

        println!("1. Uso con impl Trait (Static Dispatch):");
        println!("  impl TripleTransform<Input = i32, Output = String, Error = String>");
        usar_impl(converter, 42);
        usar_impl(SafeConverter, -5);
        println!();

        println!("2. Uso con dyn Trait (Dynamic Dispatch):");
        println!("  &dyn TripleTransform<Input = i32, Output = String, Error = String>");
        let boxed_converter: Box<
            dyn TripleTransform<Input = i32, Output = String, Error = String>,
        > = Box::new(SafeConverter);

        usar_dyn(&*boxed_converter, 100);

        let transforms: Vec<
            Box<dyn TripleTransform<Input = i32, Output = String, Error = String>>,
        > = vec![Box::new(SafeConverter)];

        for t in &transforms {
            usar_dyn(t.as_ref(), 7);
        }
        println!();

        // ─────────────────────────────────────────────────────────────
        // Demostración de por qué falta fijar tipos es un problema
        // ─────────────────────────────────────────────────────────────
        println!("RESTRICCIÓN 2: ¿Por qué deben fijarse TODOS los tipos?");
        println!("────────────────────────────────────────────────────────────────────\n");

        println!(
            r#"  Cuando el compilador ve:
        &dyn TripleTransform<Input = i32, Output = String, Error = String>
      
      Genera una VTABLE con punteros a funciones que tienen signatures concretas:
      
        struct VTABLE {{
            drop:      fn(*mut u8),
            size:      usize,
            align:     usize,
            transform: fn(*mut u8, i32) -> Result<String, String>,  ← Tipos fijados!
        }}
      
      Los argumentos y retornos DEBEN tener tamaño conocido en compile time.
      
      Si dejaras un tipo sin fijar:
        &dyn TripleTransform<Input = i32>  // Output y Error SIN FIJAR
      
      La vtable no sabría qué tamaño tiene Output o Error.
      ¿Son 1 byte? ¿1000 bytes? Imposible de decidir en compile time.
      
      Por eso: "Todos los Associated Types deben estar fijados en dyn Trait"
    "#
        );

        // ─────────────────────────────────────────────────────────────
        // Restricción 3: Object Safety con métodos
        // ─────────────────────────────────────────────────────────────
        println!("RESTRICCIÓN 3: Object Safety y métodos en Traits");
        println!("────────────────────────────────────────────────────────────────────\n");

        println!(
            r#"  Trait TripleTransform ES object-safe porque:
      
      ✅ transform(&self, ...) tiene &self (no Self)
      ✅ Retorna Result<Self::Output, Self::Error>
         (y ambos están fijados en dyn, así que tamaño CONOCIDO)
      ✅ Sin métodos genéricos <T>
      ✅ Sin funciones asociadas sin self (como fn new())
      
      Si tuviera algo de esto NO sería object-safe:
      
      ❌ fn clone(&self) -> Self
         (Output estaría sin usar, pero retorna Self desconocido)
         
      ❌ fn procesar<T>(&self, x: T)
         (T no está fijado, el compilador no sabe qué tamaño tiene)
         
      ❌ fn new() -> Self
         (Sin self, no puede estar en vtable, necesitaría estática)
    "#
        );

        // ─────────────────────────────────────────────────────────────
        // Demostración: múltiples implementaciones con distintos tipos
        // ─────────────────────────────────────────────────────────────
        println!("RESTRICCIÓN 4: Colecciones heterogéneas requieren tipos uniformes");
        println!("────────────────────────────────────────────────────────────────────\n");

        println!("Puedes tener múltiples implementaciones CON DISTINTOS tipos asociados:");
        println!();

        // SafeConverter: Input=i32, Output=String, Error=String
        // MathConverter: Input=i32, Output=i32, Error=&'static str

        // Pero colecciones heterogéneas requieren que TODOS tengan el MISMO conjunto de tipos:
        // ✅ Vec<Box<dyn T<Input=i32, Output=String, Error=String>>>
        //    (puede mezclar SafeConverter con otras impl que tengan EXACTAMENTE estos tipos)

        // ❌ Vec<Box<dyn T>>  (NO COMPILA - tipos sin fijar)
        // ❌ Vec![SafeConverter, MathConverter]
        //    (ambos impl T, pero con DISTINTOS tipos asociados)

        println!("SafeConverter impl TripleTransform {{");
        println!("    type Input = i32");
        println!("    type Output = String");
        println!("    type Error = String");
        println!("}}");
        println!();

        println!("MathConverter impl TripleTransform {{");
        println!("    type Input = i32");
        println!("    type Output = i32          ← DIFERENTE!");
        println!("    type Error = &'static str  ← DIFERENTE!");
        println!("}}");
        println!();

        println!("∴ NO puedes mezclarlos en la misma colección:");
        println!("  ❌ vec![Box::new(SafeConverter), Box::new(MathConverter)]");
        println!("     ↑ Tipos diferentes para Output y Error");
        println!();

        println!("✅ Puedes mezclar si tienen el MISMO conjunto de tipos:");

        // Si definiéramos otra impl con los mismos tipos que SafeConverter:
        struct AnotherConverter;
        impl TripleTransform for AnotherConverter {
            type Input = i32;
            type Output = String;
            type Error = String;

            fn transform(&self, input: i32) -> Result<String, String> {
                Ok(format!("Convertido: {}", input))
            }
        }

        let mixed: Vec<Box<dyn TripleTransform<Input = i32, Output = String, Error = String>>> =
            vec![Box::new(SafeConverter), Box::new(AnotherConverter)];

        println!("  vec![Box::new(SafeConverter), Box::new(AnotherConverter)]");
        for (i, t) in mixed.iter().enumerate() {
            println!("    [{}] ", i);
            usar_dyn(t.as_ref(), 10);
        }
        println!();
    }

    // ============================================================================
    //              DEMO 10: TRAIT BOUNDS + LIFETIMES (EL OPERADOR +)
    // ============================================================================

    /// Explicación de cómo usar el operador `+` para combinar trait bounds y lifetimes:
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────────────────────────────┐
    /// │  EL OPERADOR + EN TRAITS                                                │
    /// │                                                                         │
    /// │  El `+` combina múltiples requisitos que un tipo DEBE cumplir:          │
    /// │                                                                         │
    /// │  impl Trait + Send + Sync + 'static                                     │
    /// │    │         │      │      │                                           │
    /// │    │         │      │      └─ Lifetime: vive todo el programa          │
    /// │    │         │      └────── Trait: thread-safe para lectura            │
    /// │    │         └───────────── Trait: thread-safe para escritura          │
    /// │    └──────────────────────── Trait principal                           │
    /// │                                                                         │
    /// │  SIN BOUNDS (cualquier tipo que implemente Trait):                      │
    /// │    fn procesar(x: impl Trait) { ... }                                   │
    /// │                                                                         │
    /// │  CON BOUNDS (tipo debe cumplir TODOS los requisitos):                   │
    /// │    fn procesar(x: impl Trait + Send + Sync) { ... }                     │
    /// │                                                                         │
    /// │  REGLA: El tipo retornado por impl Trait debe ser:                      │
    /// │    ✓ Una implementación de Trait                                        │
    /// │    ✓ Implementar Send (si lo especificas)                              │
    /// │    ✓ Implementar Sync (si lo especificas)                              │
    /// │    ✓ Vivir con el lifetime especificado (si lo especificas)             │
    /// │                                                                         │
    /// │  Si falta algún bound → ERROR DE COMPILACIÓN                            │
    /// └─────────────────────────────────────────────────────────────────────────┘
    /// ```
    #[test]
    pub fn lifetimes() {
        // ─────────────────────────────────────────────────────────────
        // EJEMPLO 1: Trait + Send
        // ─────────────────────────────────────────────────────────────
        trait Procesable {
            fn procesar(&self) -> String;
        }

        #[derive(Copy, Clone)]
        struct TipoSeguro;
        impl Procesable for TipoSeguro {
            fn procesar(&self) -> String {
                "Procesado".to_string()
            }
        }

        // Función que REQUIERE que el tipo sea Send (thread-safe para mover)
        fn usar_con_send(x: impl Procesable + Send) {
            println!(
                "  [Send] Tipo puede enviarse entre threads: {}",
                x.procesar()
            );
        }

        // Función que REQUIERE que el tipo sea Sync (thread-safe para compartir)
        fn usar_con_sync(x: impl Procesable + Sync) {
            println!(
                "  [Sync] Tipo puede compartirse entre threads: {}",
                x.procesar()
            );
        }

        // Función que REQUIERE AMBOS
        fn usar_con_ambos(x: impl Procesable + Send + Sync) {
            println!(
                "  [Send+Sync] Tipo totalmente thread-safe: {}",
                x.procesar()
            );
        }

        let tipo = TipoSeguro;

        println!("1. Usando trait bounds con Send/Sync:");
        usar_con_send(tipo); // TipoSeguro implementa Send
        usar_con_sync(tipo); // TipoSeguro implementa Sync
        usar_con_ambos(tipo); // TipoSeguro implementa ambos
        println!();

        // ─────────────────────────────────────────────────────────────
        // EJEMPLO 2: Trait + Lifetime
        // ─────────────────────────────────────────────────────────────
        println!("2. Usando trait bounds con lifetime:");

        // Función que retorna un tipo que vive solo mientras 'a
        fn crear_con_lifetime<'a>(data: &'a str) -> impl Procesable + 'a {
            struct DatosRef<'a> {
                data: &'a str,
            }

            impl<'a> Procesable for DatosRef<'a> {
                fn procesar(&self) -> String {
                    format!("Datos: {}", self.data)
                }
            }

            DatosRef { data }
        }

        let texto = String::from("Hola");
        let procesable = crear_con_lifetime(&texto);
        println!("  [Lifetime] {}", procesable.procesar());
        println!();

        // ─────────────────────────────────────────────────────────────
        // EJEMPLO 3: Múltiples bounds combinados
        // ─────────────────────────────────────────────────────────────
        println!("3. Combinando múltiples bounds:");

        fn usar_todo<'a>(x: impl Procesable + Send + Sync + Clone + 'a) {
            let copia = x.clone();
            println!("  [Send+Sync+Clone+Lifetime] {}", copia.procesar());
        }

        #[derive(Clone)]
        struct TipoCompleto;
        impl Procesable for TipoCompleto {
            fn procesar(&self) -> String {
                "Completo".to_string()
            }
        }

        usar_todo(TipoCompleto);
        println!();

        // ─────────────────────────────────────────────────────────────
        // EJEMPLO 4: Lo que pasaría sin los bounds
        // ─────────────────────────────────────────────────────────────
        println!("4. Por qué los bounds son importantes:");
        println!();

        println!("  Sin bounds:");
        println!("    fn procesar(x: impl Procesable) {{ ... }}");
        println!("    → x puede NO ser Send, NO ser Sync");
        println!("    → x puede tener referencias que no viven lo suficiente");
        println!("    → No puedes enviar a threads, no puedes compartir, etc.");
        println!();

        println!("  Con bounds:");
        println!("    fn procesar(x: impl Procesable + Send + Sync + 'static) {{ ... }}");
        println!("    → x es Send (puedes enviarlo a threads)");
        println!("    → x es Sync (puedes compartirlo entre threads)");
        println!("    → x vive 'static (no tiene referencias)");
        println!("    → Compilador verifica TODOS estos requisitos");
        println!();

        // ─────────────────────────────────────────────────────────────
        // EJEMPLO 5: Diferencia entre impl Trait y dyn Trait con bounds
        // ─────────────────────────────────────────────────────────────
        println!("5. Bounds en impl Trait vs dyn Trait:");
        println!();

        // Con impl Trait:
        fn retorna_impl() -> impl Procesable + Send + Sync {
            TipoSeguro
        }

        // Con dyn Trait:
        fn retorna_dyn() -> Box<dyn Procesable + Send + Sync> {
            Box::new(TipoSeguro)
        }

        let _impl_tipo = retorna_impl();
        let _dyn_tipo = retorna_dyn();

        println!("  ✓ impl Procesable + Send + Sync → monomorphization");
        println!("    - Tipo concreto, inlineable, eficiente");
        println!();
        println!("  ✓ Box<dyn Procesable + Send + Sync> → dynamic dispatch");
        println!("    - Tipo heterogéneo, vtable, flexible");
        println!();
    }

    // ============================================================================
    //                  DEMO 11: MÚLTIPLES TRAITS CON MISMO MÉTODO
    // ============================================================================

    /// Cuando un struct implementa dos traits que tienen métodos con el mismo nombre,
    /// necesitas usar la sintaxis de función explícita para desambiguar cuál llamar.

    #[test]
    pub fn multiples_traits() {
        println!("═══════════════════════════════════════════════════════════════════");
        println!("  DEMO 11: MÚLTIPLES TRAITS CON MISMO MÉTODO");
        println!("═══════════════════════════════════════════════════════════════════\n");

        // Definimos dos traits con el mismo método
        trait TraitA {
            fn foo(&self) -> &'static str;
        }

        trait TraitB {
            fn foo(&self) -> &'static str;
        }

        struct MyStruct;

        // Implementamos ambos traits en el mismo struct
        impl TraitA for MyStruct {
            fn foo(&self) -> &'static str {
                "Vengo de TraitA"
            }
        }

        impl TraitB for MyStruct {
            fn foo(&self) -> &'static str {
                "Vengo de TraitB"
            }
        }

        let s = MyStruct;

        // ⚠️ s.foo() es ambiguo - ¿cuál foo?
        // s.foo();  // ❌ ERROR: multiple `foo` found

        // ✅ SOLUCIÓN: Usar sintaxis completa (Fully Qualified Syntax)
        let a = TraitA::foo(&s);
        let b = TraitB::foo(&s);

        println!("  TraitA::foo(&s) = {}", a);
        println!("  TraitB::foo(&s) = {}", b);

        // Si quisieras llamarlo como método:
        // let a = <MyStruct as TraitA>::foo(&s);  // También funciona
        // let b = <MyStruct as TraitB>::foo(&s);

        println!();
    }

    // ============================================================================
    //          DEMO 12: LLAMAR MÉTODO COMO FUNCIÓN (UFCS)
    // ============================================================================

    /// Universal Function Call Syntax (UFCS) permite llamar un método como función.
    /// Especialmente útil cuando el método no toma `self` o necesitas pasar
    /// argumentos que no son la instancia.

    #[test]
    pub fn ufcs() {
        println!("═══════════════════════════════════════════════════════════════════");
        println!("  DEMO 12: LLAMAR MÉTODO COMO FUNCIÓN (UFCS)");
        println!("═══════════════════════════════════════════════════════════════════\n");

        // Trait con métodos de diferentes tipos
        trait Calculator {
            // Toma &self
            fn valor(&self) -> i32;

            // Método estático (no toma self)
            fn sumar(a: i32, b: i32) -> i32;
        }

        struct Calculadora {
            valor: i32,
        }

        impl Calculator for Calculadora {
            fn valor(&self) -> i32 {
                self.valor
            }

            fn sumar(a: i32, b: i32) -> i32 {
                a + b
            }
        }

        let calc = Calculadora { valor: 42 };

        println!("  EJEMPLO 1: Método que toma &self");
        println!("  ─────────────────────────────────");

        // Forma normal (method call syntax)
        let v1 = calc.valor();
        println!("    calc.valor() = {}", v1);

        // Forma de función (UFCS)
        let v2 = Calculator::valor(&calc);
        println!("    Calculator::valor(&calc) = {}", v2);

        println!();
        println!("  EJEMPLO 2: Método estático (no toma self)");
        println!("  ──────────────────────────────────────────");

        // Solo se puede llamar como función
        let resultado = <Calculadora as Calculator>::sumar(10, 20);
        println!("    Calculator::sumar(10, 20) = {}", resultado);

        // La sintaxis método no funciona para métodos estáticos:
        // calc.sumar(10, 20);  // ❌ ERROR: no `self` parameter

        println!();
        println!("  EJEMPLO 3: Cuando UFCS es particularmente útil");
        println!("  ──────────────────────────────────────────────");

        // Pasar una función como parámetro
        trait Mapper {
            fn mapear(x: i32) -> i32;
        }

        struct Duplicador;
        impl Mapper for Duplicador {
            fn mapear(x: i32) -> i32 {
                x * 2
            }
        }

        // UFCS permite pasar el método como función a otra función
        fn aplicar<T: Mapper>(nums: Vec<i32>) -> Vec<i32> {
            // Aquí usamos UFCS para pasar T::mapear como función
            nums.iter().map(|&x| T::mapear(x)).collect()
        }

        let valores = vec![1, 2, 3, 4, 5];
        let duplicados = aplicar::<Duplicador>(valores);
        println!("    Aplicar Duplicador: {:?}", duplicados);

        println!();
    }

    // ============================================================================
    //         DEMO 13: DIFERENTES FORMAS DE LLAMAR UNA IMPLEMENTACIÓN
    // ============================================================================

    /// Cuando un trait está implementado para un tipo, hay varias formas de
    /// llamar a ese método. Todas son equivalentes pero con diferentes usos.

    #[test]
    pub fn formas_de_llamar() {
        println!("═══════════════════════════════════════════════════════════════════");
        println!("  DEMO 13: DIFERENTES FORMAS DE LLAMAR UNA IMPLEMENTACIÓN");
        println!("═══════════════════════════════════════════════════════════════════\n");

        use std::ops::Add;

        let x = 5i32;
        let y = 3i32;

        println!("  Todos estos son equivalentes para llamar Add::add(x, y):\n");

        // FORMA 1: Usar el operador (syntax sugar)
        let r1 = x + y;
        println!("  1. x + y");
        println!("     → Syntax sugar para Add::add(x, y)");
        println!("     → Resultado: {}\n", r1);

        // FORMA 2: Llamar como método
        let r2 = x.add(y);
        println!("  2. x.add(y)");
        println!("     → Método call syntax (auto-deref, auto-borrow)");
        println!("     → Internamente: Add::add(&x, y)");
        println!("     → Resultado: {}\n", r2);

        // FORMA 3: UFCS con trait (requiere especificar el tipo)
        let r3 = <i32 as Add>::add(x, y);
        println!("  3. <i32 as Add>::add(x, y)");
        println!("     → Universal Function Call Syntax (UFCS)");
        println!("     → Llama a la implementación de Add para i32");
        println!("     → Resultado: {}\n", r3);

        // FORMA 4: UFCS con tipo concreto
        let r4 = i32::add(x, y);
        println!("  4. i32::add(x, y)");
        println!("     → UFCS con tipo concreto");
        println!("     → Especifica explícitamente que es i32");
        println!("     → Resultado: {}\n", r4);

        // FORMA 5: UFCS totalmente cualificado (cuando hay ambigüedad)
        let r5 = <i32 as Add>::add(x, y);
        println!("  5. <i32 as Add>::add(x, y)");
        println!("     → Fully Qualified UFCS");
        println!("     → Se usa cuando hay ambigüedad (ej: múltiples traits)");
        println!("     → Resultado: {}\n", r5);

        println!("  ┌─────────────────────────────────────────────────────────────┐");
        println!("  │ COMPARACIÓN DE FORMAS                                       │");
        println!("  ├─────────────────────────────────────────────────────────────┤");
        println!("  │                                                             │");
        println!("  │ x + y                  → OPERADOR (más legible)             │");
        println!("  │ x.add(y)               → MÉTODO (automático borrow)         │");
        println!("  │ <i32 as Add>::add(x,y) → UFCS (específica implementación)   │");
        println!("  │ i32::add(x, y)         → TIPO (con inferencia)              │");
        println!("  │                                                             │");
        println!("  └─────────────────────────────────────────────────────────────┘");

        println!();
        println!("  ¿CUÁNDO USAR CADA UNA?");
        println!("  ───────────────────────");

        println!();
        println!("  OPERADOR (x + y)");
        println!("    ✓ Cuando el operador es familiar");
        println!("    ✓ Código más legible");
        println!("    ✗ Solo funciona con operadores sobrecargados\n");

        println!("  MÉTODO (x.add(y))");
        println!("    ✓ Cuando necesitas llamar como método");
        println!("    ✓ Auto-deref y auto-borrow automático");
        println!("    ✓ Funciona con cualquier trait\n");

        println!("  UFCS (<i32 as Add>::add(x, y))");
        println!("    ✓ Cuando quieres SER EXPLÍCITO sobre la implementación");
        println!("    ✓ Útil cuando hay múltiples implementaciones");
        println!("    ✓ Soluciona ambigüedad\n");

        println!("  TIPO (i32::add(x, y))");
        println!("    ✓ Cuando quieres aprovechar la inferencia");
        println!("    ✓ Menos verboso que UFCS totalmente cualificado\n");

        // EJEMPLO PRÁCTICO: Pasar una función genérica
        println!("  EJEMPLO PRÁCTICO: Abstracción genérica");
        println!("  ──────────────────────────────────────\n");

        fn operar<T: Add<Output = T>>(a: T, b: T) -> T {
            // Usamos la forma de trait para abstraer la implementación
            Add::add(a, b)
        }

        let result_i32 = operar(5i32, 3i32);
        let result_f64 = operar(5.5f64, 3.2f64);

        println!("    operar(5i32, 3i32)     = {}", result_i32);
        println!("    operar(5.5f64, 3.2f64) = {}\n", result_f64);

        println!("    La función operar() NO CONOCE si es i32 o f64");
        println!("    Solo sabe que T implementa Add");
        println!("    Esto permite CODE REUSE sin duplicación\n");

        println!();
    }

    // ============================================================================
    //              DEMO 14: IMPLEMENTAR OPERADORES PERSONALIZADOS
    // ============================================================================

    /// En Rust, los operadores (+, -, *, etc.) son syntax sugar para traits.
    /// Implementando el trait correcto, puedes personalizar el comportamiento
    /// de los operadores para tus propios tipos.

    #[test]
    pub fn operadores() {
        println!("═══════════════════════════════════════════════════════════════════");
        println!("  DEMO 14: IMPLEMENTAR OPERADORES PERSONALIZADOS");
        println!("═══════════════════════════════════════════════════════════════════\n");

        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│  TRAITS PARA OPERADORES (std::ops)                              │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│                                                                 │");
        println!("│  Add<RHS = Self>          → a + b   (suma)                      │");
        println!("│  Sub<RHS = Self>          → a - b   (resta)                     │");
        println!("│  Mul<RHS = Self>          → a * b   (multiplicación)            │");
        println!("│  Div<RHS = Self>          → a / b   (división)                  │");
        println!("│  Rem<RHS = Self>          → a % b   (módulo)                    │");
        println!("│  Neg                      → -a      (negación)                  │");
        println!("│  Not                      → !a      (NOT lógico)                │");
        println!("│  BitAnd, BitOr, BitXor    → &, |, ^ (operadores bit)           │");
        println!("│  Shl, Shr                 → <<, >>  (shift)                     │");
        println!("│  Index<I>                 → a[i]    (indexación)                │");
        println!("│  IndexMut<I>              → a[i]=v  (indexación mut)            │");
        println!("│  AddAssign, SubAssign...  → +=, -= (operadores asignación)     │");
        println!("│  Deref, DerefMut          → *a      (desreferenciación)         │");
        println!("│                                                                 │");
        println!("└─────────────────────────────────────────────────────────────────┘\n");

        // ─────────────────────────────────────────────────────────────────────────
        // EJEMPLO 1: Implementar Add para un tipo personalizado
        // ─────────────────────────────────────────────────────────────────────────
        println!("  EJEMPLO 1: Vector 2D con suma");
        println!("  ──────────────────────────────\n");

        use std::ops::{Add, AddAssign, Mul, Neg};

        #[derive(Debug, Clone, Copy)]
        struct Vector2D {
            x: f64,
            y: f64,
        }

        impl Add for Vector2D {
            type Output = Vector2D;

            fn add(self, other: Vector2D) -> Vector2D {
                Vector2D {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        let v1 = Vector2D { x: 1.0, y: 2.0 };
        let v2 = Vector2D { x: 3.0, y: 4.0 };

        println!("    v1 = {:?}", v1);
        println!("    v2 = {:?}", v2);

        // Usamos el operador +
        let v3 = v1 + v2;
        println!("    v1 + v2 = {:?}", v3);
        println!("    (internamente: Add::add(v1, v2))\n");

        // ─────────────────────────────────────────────────────────────────────────
        // EJEMPLO 2: Multiplicar un Vector por un escalar
        // ─────────────────────────────────────────────────────────────────────────
        println!("  EJEMPLO 2: Multiplicación escalar");
        println!("  ──────────────────────────────────\n");

        // Vector * escalar
        impl Mul<f64> for Vector2D {
            type Output = Vector2D;

            fn mul(self, scalar: f64) -> Vector2D {
                Vector2D {
                    x: self.x * scalar,
                    y: self.y * scalar,
                }
            }
        }

        // escalar * Vector (reflexión)
        impl Mul<Vector2D> for f64 {
            type Output = Vector2D;

            fn mul(self, vec: Vector2D) -> Vector2D {
                Vector2D {
                    x: self * vec.x,
                    y: self * vec.y,
                }
            }
        }

        println!("    v1 * 2.0 = {:?}", v1 * 2.0);
        println!("    3.0 * v1 = {:?}", 3.0 * v1);
        println!("    (Ambas formas funcionan porque implementamos ambas)\n");

        // ─────────────────────────────────────────────────────────────────────────
        // EJEMPLO 3: Operadores de comparación personalizados
        // ─────────────────────────────────────────────────────────────────────────
        println!("  EJEMPLO 3: Comparación personalizada");
        println!("  ────────────────────────────────────\n");

        #[derive(Debug)]
        struct Precio {
            cantidad: u32,
            moneda: &'static str,
        }

        impl PartialEq for Precio {
            fn eq(&self, other: &Self) -> bool {
                // Para simplificar: comparamos ignorando moneda
                // En producción sería más complejo
                self.cantidad == other.cantidad && self.moneda == other.moneda
            }
        }

        impl PartialOrd for Precio {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                // Solo comparamos si la moneda es igual
                if self.moneda != other.moneda {
                    return None; // No comparables (monedas diferentes)
                }
                self.cantidad.partial_cmp(&other.cantidad)
            }
        }

        let p1 = Precio {
            cantidad: 100,
            moneda: "USD",
        };
        let p2 = Precio {
            cantidad: 50,
            moneda: "USD",
        };
        let p3 = Precio {
            cantidad: 100,
            moneda: "EUR",
        };

        println!("    p1 = {:?}", p1);
        println!("    p2 = {:?}", p2);
        println!("    p3 = {:?}", p3);
        println!();
        println!("    p1 == p2: {} (falso, diferentes cantidades)", p1 == p2);
        println!("    p1 == p3: {} (falso, diferentes monedas)", p1 == p3);
        println!("    p1 > p2:  {} (verdadero, 100 USD > 50 USD)", p1 > p2);
        println!("    p1 > p3:  None (no comparables, diferentes monedas)\n");

        // ─────────────────────────────────────────────────────────────────────────
        // EJEMPLO 4: Operador Neg (negación)
        // ─────────────────────────────────────────────────────────────────────────
        println!("  EJEMPLO 4: Negación (Neg trait)");
        println!("  ─────────────────────────────────\n");

        impl Neg for Vector2D {
            type Output = Vector2D;

            fn neg(self) -> Vector2D {
                Vector2D {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        let v = Vector2D { x: 1.0, y: 2.0 };
        let negado = -v;

        println!("    v      = {:?}", v);
        println!("    -v     = {:?}", negado);
        println!("    (implementamos Neg para Vector2D)\n");

        // ─────────────────────────────────────────────────────────────────────────
        // EJEMPLO 5: AddAssign (+=)
        // ─────────────────────────────────────────────────────────────────────────
        println!("  EJEMPLO 5: AddAssign (+=)");
        println!("  ─────────────────────────\n");

        impl AddAssign for Vector2D {
            fn add_assign(&mut self, other: Vector2D) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        let mut v1 = Vector2D { x: 1.0, y: 2.0 };
        let v2 = Vector2D { x: 3.0, y: 4.0 };

        println!("    v1 = {:?}", v1);
        println!("    v2 = {:?}", v2);
        v1 += v2;
        println!("    v1 += v2");
        println!("    v1 ahora = {:?}\n", v1);

        // ─────────────────────────────────────────────────────────────────────────
        // TABLA RESUMEN
        // ─────────────────────────────────────────────────────────────────────────
        println!("  ┌─────────────────────────────────────────────────────────────┐");
        println!("  │ PATRÓN GENERAL PARA IMPLEMENTAR UN OPERADOR                 │");
        println!("  ├─────────────────────────────────────────────────────────────┤");
        println!("  │                                                             │");
        println!("  │  use std::ops::Add;  // Importar el trait                   │");
        println!("  │                                                             │");
        println!("  │  impl Add for MiTipo {{                                      │");
        println!("  │      type Output = MiTipo;  // Tipo del resultado           │");
        println!("  │                                                             │");
        println!("  │      fn add(self, other: MiTipo) -> MiTipo {{                │");
        println!("  │          // Implementar la lógica                           │");
        println!("  │      }}                                                       │");
        println!("  │  }}                                                           │");
        println!("  │                                                             │");
        println!("  │  // Ahora puedes usar: let z = x + y;                       │");
        println!("  │                                                             │");
        println!("  └─────────────────────────────────────────────────────────────┘\n");

        println!("  NOTAS IMPORTANTES:");
        println!("  ──────────────────\n");
        println!("  • RHS (Right Hand Side) = tipo del operando derecho");
        println!("    Si omites RHS, por defecto es Self\n");

        println!("  • Output = tipo del resultado de la operación\n");

        println!("  • Para operadores binarios simétricos (a + b = b + a),");
        println!("    considera implementar ambas direcciones\n");

        println!("  • Los operadores de asignación (+=, -=, etc.) usan");
        println!("    AddAssign, SubAssign, etc.\n");

        println!("  • No todos los operadores son sobrecargables");
        println!("    (ej: &&, ||, =, . no se pueden sobrecargar)\n");

        println!();
    }
}
