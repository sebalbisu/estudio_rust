// Model: Solo la estructura de datos
// Separado para reutilización fácil

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }

    // Métodos de validación en el modelo
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    pub fn is_valid_name(name: &str) -> bool {
        !name.is_empty() && name.len() <= 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(User::is_valid_email("test@example.com"));
        assert!(!User::is_valid_email("invalid"));
    }

    #[test]
    fn test_valid_name() {
        assert!(User::is_valid_name("Alice"));
        assert!(!User::is_valid_name(""));
    }
}
