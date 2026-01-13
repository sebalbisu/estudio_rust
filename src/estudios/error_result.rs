// ============================================================================
// MANEJO DE ERRORES EN RUST
// ============================================================================

#[test]
fn indice() {
    fundamentos::sin_source();
    fundamentos::con_source();
    impl_err::enum_err();
    impl_err::struct_err();
    impl_err::string_err();
    box_dyn_error::dyn_err();
    string_err::string_as_boxed_error();
    conversion_propagacion::demo_question_operator();
    this_error::this_error_demo();
    anyhow::anyhow_demo();
    backtrace::manual();
}

// ============================================================================
// 1. FUNDAMENTOS: TRAIT ERROR
// ============================================================================
/*
    TRAIT ERROR EN RUST:
    --------------------------------------------

    * Para implementar Error necesitas:
        * #[derive(Debug)]           // Obligatorio (trait bound)
        * impl Display for MyError   // Obligatorio (trait bound)
        * impl Error for MyError     // Puede estar vacío si no hay source

    pub trait Error: Debug + Display {
        // opcional implementar source(), por defecto retorna None
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
*/
#[cfg(test)]
mod fundamentos {
    use std::error::Error;
    use std::fmt;

    #[test]
    pub fn sin_source() {
        // Definir un error
        #[derive(Debug)]
        enum MiError {
            Variante1,
            Variante2,
        }

        impl fmt::Display for MiError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    MiError::Variante1 => write!(f, "Error de Variante 1"),
                    MiError::Variante2 => write!(f, "Error de Variante 2"),
                }
            }
        }

        impl Error for MiError {}

        // Usar el error
        fn funcion_que_falla(cond: u8) -> Result<(), MiError> {
            match cond {
                1 => Err(MiError::Variante1),
                2 => Err(MiError::Variante2),
                _ => Ok(()),
            }
        }

        // Probar la función
        match funcion_que_falla(1) {
            Ok(_) => println!("     Función exitosa"),
            Err(e) => match e {
                MiError::Variante1 => println!("     Manejando Variante 1"),
                MiError::Variante2 => println!("     Manejando Variante 2"),
            },
        }
    }

    #[test]
    pub fn con_source() {
        // Definir un error con source()
        #[derive(Debug)]
        enum OtroError {
            SubError,
        }

        impl fmt::Display for OtroError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Otro error ocurrió")
            }
        }

        impl Error for OtroError {}

        #[derive(Debug)]
        enum MiErrorConSource {
            VarianteConCausa(OtroError),
        }

        impl fmt::Display for MiErrorConSource {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    MiErrorConSource::VarianteConCausa(_) => {
                        write!(f, "Error con causa subyacente")
                    }
                }
            }
        }

        impl Error for MiErrorConSource {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match self {
                    MiErrorConSource::VarianteConCausa(e) => Some(e),
                }
            }
        }

        // Usar el error con source()
        fn funcion_con_causa() -> Result<(), MiErrorConSource> {
            Err(MiErrorConSource::VarianteConCausa(OtroError::SubError))
        }

        // Probar la función
        match funcion_con_causa() {
            Ok(_) => println!("     Función exitosa"),
            Err(e) => {
                println!("     Manejando error: {}", e);
                if let Some(causa) = e.source() {
                    println!("     Causado por: {}", causa);
                }
            }
        }
    }
}

// ============================================================================
// 2. TIPOS DE ERRORES: ENUM , STRUCT, String
// ============================================================================
/*
    IMPL ERROR:
    --------------------------------------------

    * Cualquier tipo de dato que implemente Error a modo practico (enum | struct)

    VENTAJAS de errores específicos con (enum):
        * Match exhaustivo - el caller ve todas las variantes
        * Documentación implícita de qué puede fallar
        * Permite recuperación específica por tipo de error
*/
#[cfg(test)]
mod impl_err {
    use std::error::Error;
    use std::fmt;

    // Enum
    //------------
    #[test]
    pub fn enum_err() {
        #[derive(Debug)]
        enum AgeError {
            TooYoung { age: u8, min: u8 },
            TooOld { age: u8, max: u8 },
            Invalid,
        }

        impl fmt::Display for AgeError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    AgeError::TooYoung { age, min } => {
                        write!(f, "Edad demasiado joven: {} < {}", age, min)
                    }
                    AgeError::TooOld { age, max } => {
                        write!(f, "Edad demasiado vieja: {} > {}", age, max)
                    }
                    AgeError::Invalid => write!(f, "Edad inválida"),
                }
            }
        }

        impl Error for AgeError {}
    }

    // Struct
    //------------
    #[test]
    pub fn struct_err() {
        #[derive(Debug)]
        struct DatabaseError {
            details: String,
            datetime: String,
            code: u16,
            sql: String,
        }

        impl fmt::Display for DatabaseError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "DatabaseError {}: {} ({} at {})",
                    self.code, self.details, self.sql, self.datetime
                )
            }
        }
        impl Error for DatabaseError {}

        // Solo instanciamos para verificar que compila
        let _err_struct = DatabaseError {
            details: "Connection failed".into(),
            datetime: "2023-01-01".into(),
            code: 500,
            sql: "SELECT *".into(),
        };
    }

    // String
    //------------
    #[test]
    pub fn string_err() {
        // String como error
        fn string_error() -> Result<(), String> {
            Err("Este es un error como string".into())
        }

        match string_error() {
            Ok(_) => println!("     Función exitosa"),
            Err(e) => println!("     Capturado error como string: {}", e),
        }
    }
}

// ============================================================================
// 3. BOX<DYN ERROR>: HETEROGENEIDAD
// ============================================================================
/*
    BOX<DYN ERROR>:
    --------------------------------------------

    * Ventajas de errores genéricos (Box<dyn Error>):
        * Máxima flexibilidad - cualquier error puede ser retornado
        * Permite mezclar errores de múltiples bibliotecas

    * DESVENTAJA: Pierde información del tipo concreto

    Box<dyn Error> es un "fat pointer" de 16 bytes:
    +----------------+----------------+
    | ptr to data    | ptr to vtable  |
    | (8 bytes)      | (8 bytes)      |
    +----------------+----------------+

    El vtable contiene punteros a:
        * fmt::Display::fmt()
        * fmt::Debug::fmt()
        * Error::source()
        * drop()
*/
#[cfg(test)]
mod box_dyn_error {
    use std::error::Error;

    /// Función que puede retornar diferentes tipos de error, incluido string
    #[test]
    pub fn dyn_err() {
        fn multiple_errors() -> Result<(), Box<dyn Error>> {
            "abc".parse::<u32>()?; // ParseIntError
            u8::try_from(256_u16)?; // TryFromIntError
            std::str::from_utf8(&vec![0xFF, 0xFE])?; // Utf8Error
            Ok(())
        }

        println!("  ✅ box_dyn_error::dyn_err");
        match multiple_errors() {
            Ok(_) => println!("     Función exitosa"),
            Err(e) => {
                // downcast si se quiere manejar cada error especifico
                if let Some(parse_err) = e.downcast_ref::<std::num::ParseIntError>() {
                    println!("     ParseIntError: {}", parse_err);
                } else if let Some(try_from_err) = e.downcast_ref::<std::num::TryFromIntError>() {
                    println!("     TryFromIntError: {}", try_from_err);
                } else if let Some(utf8_err) = e.downcast_ref::<std::str::Utf8Error>() {
                    println!("     Utf8Error: {}", utf8_err);
                } else {
                    println!("     Otro error: {}", e);
                }
            }
        }
    }
}

// ============================================================================
// 4. STRING ERRORS
// ============================================================================
/*
    &str y STRINGS COMO BOX<DYN ERROR>:
    --------------------------------------------

    "mensaje".into() -> Box<dyn Error>

    // Pseudocódigo (no es código real, es privado)
    impl From<&'static str> for Box<dyn Error> { ... }
    impl From<String> for Box<dyn Error> { ... }

    * Solo puedes comparar el mensaje y usar Display:
        println!("{}", e);  // Works
        assert_eq!(e.to_string(), "mensaje");  // Works
        e.downcast_ref::<String>()  // No funciona - tipo privado

    * La stdlib tiene una impl especial que crea un tipo PRIVADO.
    * No puedes hacer downcast a &str ni String porque el tipo real es privado.
*/
#[cfg(test)]
mod string_err {
    #[test]
    pub fn string_as_boxed_error() {
        fn error_as_string() -> Result<(), Box<dyn std::error::Error>> {
            Err("Este es un error como string".into()) // &str -> Box<dyn Error>
        }

        match error_as_string() {
            Ok(_) => println!("     Función exitosa"),
            Err(e) => {
                // no se puede hacer downcast a &str o String, solo comparar
                if e.to_string().contains("Este es un error como string") {
                    println!("     Capturado error como string");
                } else {
                    println!("     Otro error: {}", e);
                }
            }
        }
    }
}

// ============================================================================
// 5. CONVERSIÓN Y PROPAGACIÓN
// ============================================================================
/*
    EL OPERADOR ? Y LA CONVERSIÓN DE ERRORES:
    --------------------------------------------

    * El operador ? hace dos cosas:
        1. Si Ok(v) -> desenvuelve y continúa
        2. Si Err(e) -> llama From::from(e) y retorna

    validate_age(age)?  es equivalente a:
    match validate_age(age) {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    }

    * Para que funcione: impl From<ErrorOrigen> for ErrorDestino

    * ALTERNATIVA: map_err para conversión explícita
        file.read().map_err(|e| MyError::Io(e.to_string()))?;
*/
#[cfg(test)]
mod conversion_propagacion {
    use std::num::ParseIntError;

    #[test]
    pub fn demo_question_operator() {
        fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
            // Si parse falla, retorna el error automáticamente
            let n: i32 = s.parse()?;
            Ok(n * 2)
        }

        match parse_and_double("10") {
            Ok(n) => println!("     Exito: {}", n),
            Err(e) => println!("     Error: {}", e),
        }

        match parse_and_double("no numero") {
            Ok(n) => println!("     Exito: {}", n),
            Err(e) => println!("     Error esperado: {}", e),
        }
    }
}

// ============================================================================
// 6. THISERROR
// ============================================================================
/*
    THISERROR: REDUCE BOILERPLATE:
    --------------------------------------------

    * ¿Qué genera automáticamente?
        * impl Display         -> desde #[error("mensaje")]
        * impl Error           -> con source() automático si hay #[source]
        * impl From<E>         -> desde #[from] para conversión automática

    * ATRIBUTOS:
        #[error("...")]   -> Define el mensaje de Display
        #[from]           -> Genera impl From<T> (permite usar ?)
        #[source]         -> Marca el campo como source() (causa del error)
        {0}, {1}          -> Interpola campos por posición
        {field}           -> Interpola campos por nombre
*/
#[cfg(test)]
mod this_error {
    use std::io;
    use std::num::ParseIntError;
    use thiserror::Error as ThisError;

    #[test]
    pub fn this_error_demo() {
        #[derive(ThisError, Debug)]
        enum DatabaseError {
            #[error("Parse int error: {0}")] // {0} = primer campo posicional
            ParseInt(#[from] ParseIntError), // variante con campo posicional

            #[error("Query failed: {query}")] // {query} = campo nombrado
            QueryFailed {
                // variante con campos nombrados
                query: String,
                #[source] // marca causa del error
                cause: io::Error,
            },
        }

        fn db_operation() -> Result<(), DatabaseError> {
            let _num: u32 = "abc".parse()?; // convierte ParseIntError a DatabaseError

            // Simular un error de consulta
            let io_err = io::Error::new(io::ErrorKind::Other, "connection lost");
            return Err(DatabaseError::QueryFailed {
                query: "SELECT * FROM users".into(),
                cause: io_err,
            });
        }

        match db_operation() {
            Ok(_) => println!("     DB operation successful"),
            Err(e) => match e {
                DatabaseError::ParseInt(parse_err) => {
                    println!("     Parse error: {}", parse_err);
                }
                DatabaseError::QueryFailed { query, cause } => {
                    println!("     Query failed: {} caused by {}", query, cause);
                }
            },
        }
    }
}

// ============================================================================
// 7. ANYHOW
// ============================================================================
/*
    ANYHOW: AGREGANDO CONTEXTO:
    --------------------------------------------

    * .context("msg")      -> Siempre evalúa el string
    * .with_context(|| ..) -> Lazy, solo evalúa si hay error
    * bail!("msg")         -> Crea error y retorna inmediatamente

    * Cada .context() ENVUELVE el error anterior.
    * Múltiples contextos se apilan.
*/
#[cfg(test)]
mod anyhow {
    use anyhow::{Context, Result as AnyResult, bail};
    use std::num::ParseIntError;
    use std::str::Utf8Error;

    #[test]
    pub fn anyhow_demo() {
        fn load_port(path: &str) -> AnyResult<u16> {
            // Simulamos lectura de archivo
            let contents = if path.contains("no_existe") {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                ))
            } else {
                Ok("8080")
            };

            let contents =
                contents.with_context(|| format!("Leyendo archivo de puerto: {}", path))?;

            let port: u16 = contents
                .trim()
                .parse()
                .context("Parseando puerto como u16")?;

            if port == 0 {
                bail!("Puerto no puede ser 0");
            }
            Ok(port)
        }

        // Mostrar el error
        match load_port("/tmp/no_existe") {
            Ok(p) => println!("     Puerto cargado: {}", p),
            Err(e) => println!("     Error cargando puerto: {}", e),
        }

        // Listar causas con chain()
        // Simulamos un error de parseo para ver la cadena
        fn fail_parse() -> AnyResult<()> {
            let _n: i32 = "abc".parse().context("Parsing integer")?;
            Ok(())
        }

        if let Err(e) = fail_parse() {
            println!("     Chain de errores:");
            for (i, cause) in e.chain().enumerate() {
                println!("       [{}]: {}", i, cause);
            }
        }

        // Acceder a la causa raiz
        if let Err(e) = fail_parse() {
            if let Some(root) = e.chain().last() {
                if let Some(parse_err) = root.downcast_ref::<ParseIntError>() {
                    println!("     Causa raíz es ParseIntError: {}", parse_err);
                } else if let Some(utf8_err) = root.downcast_ref::<Utf8Error>() {
                    println!("     Causa raíz es Utf8Error: {}", utf8_err);
                }
            }
        }
    }
}

// ============================================================================
// 8. RUST_BACKTRACE
// ============================================================================
/*
    ¿CUÁNDO SE CAPTURA UN BACKTRACE?
    --------------------------------------------

    Situación                    | Backtrace con RUST_BACKTRACE=1
    -----------------------------|-------------------------------
    panic!()                     | ✅ Siempre
    Result::Err (normal)         | ❌ No
    Box<dyn Error>               | ❌ No
    thiserror                    | ❌ No
    anyhow::Error                | ✅ Sí

    Valores de RUST_BACKTRACE:
        0    -> Sin backtrace
        1    -> Backtrace resumido
        full -> Backtrace completo
*/
#[cfg(test)]
mod backtrace {

    #[test]
    pub fn manual() {
        use std::backtrace::Backtrace;
        use thiserror::Error as ThisError;

        #[derive(ThisError, Debug)]
        enum MyError {
            #[error("Critical error")]
            Critical { backtrace: String },
        }

        // Capturar manualmente:
        let _err = MyError::Critical {
            backtrace: Backtrace::capture().to_string(),
        };
    }
}
