// Estrategia: Organización por DOMINIO (Vertical Slicing)
// Todo lo relacionado a Users está aquí: model, repository, service

use std::collections::HashMap;

// ============================================================
// MODEL
// ============================================================

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

// ============================================================
// REPOSITORY
// ============================================================

pub struct UserRepository {
    storage: HashMap<u64, User>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn save(&mut self, user: User) -> Result<(), String> {
        self.storage.insert(user.id, user);
        Ok(())
    }

    pub fn find_by_id(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn find_by_email(&self, email: &str) -> Option<&User> {
        self.storage.values().find(|u| u.email == email)
    }

    pub fn list_all(&self) -> Vec<&User> {
        self.storage.values().collect()
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }
}

// ============================================================
// SERVICE (Business Logic)
// ============================================================

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            repo: UserRepository::new(),
        }
    }

    pub fn create_user(&mut self, name: String, email: String) -> Result<User, String> {
        // Validación
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }

        // Verificar email único
        if self.repo.find_by_email(&email).is_some() {
            return Err("Email already exists".to_string());
        }

        let user = User {
            id: (self.repo.count() + 1) as u64,
            name,
            email,
        };

        self.repo.save(user.clone())?;
        Ok(user)
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.repo.find_by_id(id)
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        self.repo.list_all()
    }

    pub fn update_email(&mut self, user_id: u64, new_email: String) -> Result<(), String> {
        if !new_email.contains('@') {
            return Err("Invalid email format".to_string());
        }

        let user = self
            .repo
            .find_by_id(user_id)
            .ok_or("User not found")?
            .clone();

        let updated_user = User {
            email: new_email,
            ..user
        };

        self.repo.save(updated_user)
    }
}

// ============================================================
// VENTAJAS DE ESTE ENFOQUE
// ============================================================

/*
VENTAJAS:

1. ALTA COHESIÓN
   - Todo relacionado a User está en un lugar
   - Fácil entender el dominio completo

2. BAJO ACOPLAMIENTO
   - User no depende de Order ni Payment
   - Cambios en User no afectan otros dominios

3. FÁCIL NAVEGACIÓN
   - ¿Dónde está la lógica de usuarios? → domain/user.rs
   - No hay que buscar en múltiples archivos

4. IDEAL PARA MICROSERVICIOS
   - Cada dominio puede convertirse en un servicio independiente
   - Bounded contexts claros

5. TESTS AISLADOS
   - Tests de User no se mezclan con otros dominios
   - Se puede testear el dominio completo

6. ESCALABILIDAD
   - Agregar nuevo dominio = nuevo archivo
   - No afecta dominios existentes

CUÁNDO USAR:
✓ Features independientes
✓ DDD (Domain-Driven Design)
✓ Arquitectura de microservicios
✓ Equipos trabajando en diferentes features

DESVENTAJAS:
✗ Puede duplicar código común (traits, validaciones base)
✗ Si hay mucha lógica compartida entre dominios, puede ser incómodo
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_success() {
        let mut service = UserService::new();
        let user = service
            .create_user("Alice".to_string(), "alice@example.com".to_string())
            .unwrap();

        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.id, 1);
    }

    #[test]
    fn test_create_user_empty_name() {
        let mut service = UserService::new();
        let result = service.create_user("".to_string(), "test@example.com".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_create_user_invalid_email() {
        let mut service = UserService::new();
        let result = service.create_user("Bob".to_string(), "invalid-email".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid email format");
    }

    #[test]
    fn test_create_user_duplicate_email() {
        let mut service = UserService::new();
        service
            .create_user("Alice".to_string(), "alice@test.com".to_string())
            .unwrap();

        let result = service.create_user("Bob".to_string(), "alice@test.com".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email already exists");
    }

    #[test]
    fn test_update_email() {
        let mut service = UserService::new();
        let user = service
            .create_user("Charlie".to_string(), "charlie@old.com".to_string())
            .unwrap();

        service
            .update_email(user.id, "charlie@new.com".to_string())
            .unwrap();

        let updated = service.get_user(user.id).unwrap();
        assert_eq!(updated.email, "charlie@new.com");
    }

    // Ventaja: Todos los tests de User están aquí, aislados de otros dominios
}
