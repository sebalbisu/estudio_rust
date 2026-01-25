// ============================================================================
// ERROR HANDLING IN RUST
// ============================================================================

#[test]
fn index() {
    fundamentals::without_source();
    fundamentals::with_source();
    impl_err::enum_err();
    impl_err::struct_err();
    impl_err::string_err();
    box_dyn_error::dyn_err();
    string_err::string_as_boxed_error();
    conversion_propagation::demo_question_operator();
    this_error::this_error_demo();
    anyhow::anyhow_demo();
    backtrace::manual();
}

// ============================================================================
// 1. FUNDAMENTALS: ERROR TRAIT
// ============================================================================
/*
    ERROR TRAIT IN RUST:
    --------------------------------------------

    * To implement Error you need:
        * #[derive(Debug)]           // Mandatory (trait bound)
        * impl Display for MyError   // Mandatory (trait bound)
        * impl Error for MyError     // Can be empty if no source

    pub trait Error: Debug + Display {
        // optional to implement source(), returns None by default
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
*/
#[cfg(test)]
mod fundamentals {
    use std::error::Error;
    use std::fmt;

    #[test]
    pub fn without_source() {
        // Define an error
        #[derive(Debug)]
        enum MyError {
            Variant1,
            Variant2,
        }

        impl fmt::Display for MyError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    MyError::Variant1 => write!(f, "Error of Variant 1"),
                    MyError::Variant2 => write!(f, "Error of Variant 2"),
                }
            }
        }

        impl Error for MyError {}

        // Use the error
        fn function_that_fails(cond: u8) -> Result<(), MyError> {
            match cond {
                1 => Err(MyError::Variant1),
                2 => Err(MyError::Variant2),
                _ => Ok(()),
            }
        }

        // Test the function
        match function_that_fails(1) {
            Ok(_) => println!("     Function successful"),
            Err(e) => match e {
                MyError::Variant1 => println!("     Handling Variant 1"),
                MyError::Variant2 => println!("     Handling Variant 2"),
            },
        }
    }

    #[test]
    pub fn with_source() {
        // Define an error with source()
        #[derive(Debug)]
        enum AnotherError {
            SubError,
        }

        impl fmt::Display for AnotherError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Another error occurred")
            }
        }

        impl Error for AnotherError {}

        #[derive(Debug)]
        enum MyErrorWithSource {
            VariantWithCause(AnotherError),
        }

        impl fmt::Display for MyErrorWithSource {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    MyErrorWithSource::VariantWithCause(_) => {
                        write!(f, "Error with underlying cause")
                    }
                }
            }
        }

        impl Error for MyErrorWithSource {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match self {
                    MyErrorWithSource::VariantWithCause(e) => Some(e),
                }
            }
        }

        // Use the error with source()
        fn function_with_cause() -> Result<(), MyErrorWithSource> {
            Err(MyErrorWithSource::VariantWithCause(AnotherError::SubError))
        }

        // Test the function
        match function_with_cause() {
            Ok(_) => println!("     Function successful"),
            Err(e) => {
                println!("     Handling error: {}", e);
                if let Some(cause) = e.source() {
                    println!("     Caused by: {}", cause);
                }
            }
        }
    }
}

// ============================================================================
// 2. ERROR TYPES: ENUM, STRUCT, String
// ============================================================================
/*
    IMPL ERROR:
    --------------------------------------------

    * Any data type that implements Error in a practical way (enum | struct)

    ADVANTAGES of specific errors with (enum):
        * Exhaustive match - the caller sees all variants
        * Implicit documentation of what can fail
        * Allows specific recovery by error type
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
        enum _AgeError {
            TooYoung { age: u8, min: u8 },
            TooOld { age: u8, max: u8 },
            Invalid,
        }

        impl fmt::Display for _AgeError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    _AgeError::TooYoung { age, min } => {
                        write!(f, "Age too young: {} < {}", age, min)
                    }
                    _AgeError::TooOld { age, max } => {
                        write!(f, "Age too old: {} > {}", age, max)
                    }
                    _AgeError::Invalid => write!(f, "Invalid age"),
                }
            }
        }

        impl Error for _AgeError {}
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

        // We only instantiate to verify it compiles
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
        // String as error
        fn string_error() -> Result<(), String> {
            Err("This is an error as string".into())
        }

        match string_error() {
            Ok(_) => println!("     Function successful"),
            Err(e) => println!("     Captured error as string: {}", e),
        }
    }
}

// ============================================================================
// 3. BOX<DYN ERROR>: HETEROGENEITY
// ============================================================================
/*
    BOX<DYN ERROR>:
    --------------------------------------------

    * Advantages of generic errors (Box<dyn Error>):
        * Maximum flexibility - any error can be returned
        * Allows mixing errors from multiple libraries

    * DISADVANTAGE: Loses information about the concrete type

    Box<dyn Error> is a 16-byte "fat pointer":
    +----------------+----------------+
    | ptr to data    | ptr to vtable  |
    | (8 bytes)      | (8 bytes)      |
    +----------------+----------------+

    The vtable contains pointers to:
        * fmt::Display::fmt()
        * fmt::Debug::fmt()
        * Error::source()
        * drop()
*/
#[cfg(test)]
mod box_dyn_error {
    use std::error::Error;

    /// Function that can return different error types, including string
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
            Ok(_) => println!("     Function successful"),
            Err(e) => {
                // downcast if you want to handle each specific error
                if let Some(parse_err) = e.downcast_ref::<std::num::ParseIntError>() {
                    println!("     ParseIntError: {}", parse_err);
                } else if let Some(try_from_err) = e.downcast_ref::<std::num::TryFromIntError>() {
                    println!("     TryFromIntError: {}", try_from_err);
                } else if let Some(utf8_err) = e.downcast_ref::<std::str::Utf8Error>() {
                    println!("     Utf8Error: {}", utf8_err);
                } else {
                    println!("     Other error: {}", e);
                }
            }
        }
    }
}

// ============================================================================
// 4. STRING ERRORS
// ============================================================================
/*
    &str and STRINGS AS BOX<DYN ERROR>:
    --------------------------------------------

    "message".into() -> Box<dyn Error>

    // Pseudocode (not real code, it's private)
    impl From<&'static str> for Box<dyn Error> { ... }
    impl From<String> for Box<dyn Error> { ... }

    * You can only compare the message and use Display:
        println!("{}", e);  // Works
        assert_eq!(e.to_string(), "message");  // Works
        e.downcast_ref::<String>()  // Doesn't work - private type

    * The stdlib has a special impl that creates a PRIVATE type.
    * You cannot downcast to &str or String because the real type is private.
*/
#[cfg(test)]
mod string_err {
    #[test]
    pub fn string_as_boxed_error() {
        fn error_as_string() -> Result<(), Box<dyn std::error::Error>> {
            Err("This is an error as string".into()) // &str -> Box<dyn Error>
        }

        match error_as_string() {
            Ok(_) => println!("     Function successful"),
            Err(e) => {
                // cannot downcast to &str or String, only compare
                if e.to_string().contains("This is an error as string") {
                    println!("     Captured error as string");
                } else {
                    println!("     Other error: {}", e);
                }
            }
        }
    }
}

// ============================================================================
// 5. CONVERSION AND PROPAGATION
// ============================================================================
/*
    THE ? OPERATOR AND ERROR CONVERSION:
    --------------------------------------------

    * The ? operator does two things:
        1. If Ok(v) -> unwraps and continues
        2. If Err(e) -> calls From::from(e) and returns

    validate_age(age)?  is equivalent to:
    match validate_age(age) {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    }

    * For it to work: impl From<SourceError> for DestinationError

    * ALTERNATIVE: map_err for explicit conversion
        file.read().map_err(|e| MyError::Io(e.to_string()))?;
*/
#[cfg(test)]
mod conversion_propagation {
    use std::num::ParseIntError;

    #[test]
    pub fn demo_question_operator() {
        fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
            // If parse fails, returns the error automatically
            let n: i32 = s.parse()?;
            Ok(n * 2)
        }

        match parse_and_double("10") {
            Ok(n) => println!("     Success: {}", n),
            Err(e) => println!("     Error: {}", e),
        }

        match parse_and_double("no number") {
            Ok(n) => println!("     Success: {}", n),
            Err(e) => println!("     Expected error: {}", e),
        }
    }
}

// ============================================================================
// 6. THISERROR
// ============================================================================
/*
    THISERROR: REDUCE BOILERPLATE:
    --------------------------------------------

    * What does it generate automatically?
        * impl Display         -> from #[error("message")]
        * impl Error           -> with automatic source() if #[source]
        * impl From<E>         -> from #[from] for automatic conversion

    * ATTRIBUTES:
        #[error("...")]   -> Defines the Display message
        #[from]           -> Generates impl From<T> (allows using ?)
        #[source]         -> Marks the field as source() (error cause)
        {0}, {1}          -> Interpolates fields by position
        {field}           -> Interpolates fields by name
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
            #[error("Parse int error: {0}")] // {0} = first positional field
            ParseInt(#[from] ParseIntError), // variant with positional field

            #[error("Query failed: {query}")] // {query} = named field
            QueryFailed {
                // variant with named fields
                query: String,
                #[source] // marks error cause
                cause: io::Error,
            },
        }

        fn db_operation() -> Result<(), DatabaseError> {
            let _num: u32 = "abc".parse()?; // converts ParseIntError to DatabaseError

            // Simulate a query error
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
    ANYHOW: ADDING CONTEXT:
    --------------------------------------------

    * .context("msg")      -> Always evaluates the string
    * .with_context(|| ..) -> Lazy, only evaluates on error
    * bail!("msg")         -> Creates error and returns immediately

    * Each .context() WRAPS the previous error.
    * Multiple contexts are stacked.
*/
#[cfg(test)]
mod anyhow {
    use anyhow::{Context, Result as AnyResult, bail};
    use std::num::ParseIntError;
    use std::str::Utf8Error;

    #[test]
    pub fn anyhow_demo() {
        fn load_port(path: &str) -> AnyResult<u16> {
            // Simulate file reading
            let contents = if path.contains("no_existe") {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                ))
            } else {
                Ok("8080")
            };

            let contents =
                contents.with_context(|| format!("Reading port file: {}", path))?;

            let port: u16 = contents
                .trim()
                .parse()
                .context("Parsing port as u16")?;

            if port == 0 {
                bail!("Port cannot be 0");
            }
            Ok(port)
        }

        // Show the error
        match load_port("/tmp/no_existe") {
            Ok(p) => println!("     Port loaded: {}", p),
            Err(e) => println!("     Error loading port: {}", e),
        }

        // List causes with chain()
        // Simulate a parse error to see the chain
        fn fail_parse() -> AnyResult<()> {
            let _n: i32 = "abc".parse().context("Parsing integer")?;
            Ok(())
        }

        if let Err(e) = fail_parse() {
            println!("     Error chain:");
            for (i, cause) in e.chain().enumerate() {
                println!("       [{}]: {}", i, cause);
            }
        }

        // Access the root cause
        if let Err(e) = fail_parse() {
            if let Some(root) = e.chain().last() {
                if let Some(parse_err) = root.downcast_ref::<ParseIntError>() {
                    println!("     Root cause is ParseIntError: {}", parse_err);
                } else if let Some(utf8_err) = root.downcast_ref::<Utf8Error>() {
                    println!("     Root cause is Utf8Error: {}", utf8_err);
                }
            }
        }
    }
}

// ============================================================================
// 8. RUST_BACKTRACE
// ============================================================================
/*
    WHEN IS A BACKTRACE CAPTURED?
    --------------------------------------------

    Situation                    | Backtrace with RUST_BACKTRACE=1
    -----------------------------|-------------------------------
    panic!()                     | ✅ Always
    Result::Err (normal)         | ❌ No
    Box<dyn Error>               | ❌ No
    thiserror                    | ❌ No
    anyhow::Error                | ✅ Yes

    RUST_BACKTRACE values:
        0    -> No backtrace
        1    -> Summary backtrace
        full -> Full backtrace
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

        // Capture manually:
        let _err = MyError::Critical {
            backtrace: Backtrace::capture().to_string(),
        };
    }
}
