// Service: Lógica de negocio
// Orquesta modelo y repositorio

use super::model::User;
use super::repository::UserRepository;

pub struct UserService {
    repo: UserRepository,
    next_id: u64,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            repo: UserRepository::new(),
            next_id: 1,
        }
    }

    pub fn create_user(&mut self, name: String, email: String) -> Result<User, String> {
        // Validar usando métodos del modelo
        if !User::is_valid_name(&name) {
            return Err("Invalid name".to_string());
        }

        if !User::is_valid_email(&email) {
            return Err("Invalid email format".to_string());
        }

        // Verificar email único (lógica de negocio)
        if self.repo.find_by_email(&email).is_some() {
            return Err("Email already exists".to_string());
        }

        let user = User::new(self.next_id, name, email);
        self.next_id += 1;

        self.repo.save(user.clone())?;
        Ok(user)
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.repo.find_by_id(id)
    }

    pub fn update_email(&mut self, user_id: u64, new_email: String) -> Result<(), String> {
        if !User::is_valid_email(&new_email) {
            return Err("Invalid email format".to_string());
        }

        // Verificar que no existe otro usuario con ese email
        if let Some(existing) = self.repo.find_by_email(&new_email) {
            if existing.id != user_id {
                return Err("Email already in use".to_string());
            }
        }

        let user = self
            .repo
            .find_by_id(user_id)
            .ok_or("User not found")?
            .clone();

        let updated = User::new(user.id, user.name, new_email);
        self.repo.save(updated)
    }

    pub fn delete_user(&mut self, id: u64) -> Result<(), String> {
        self.repo.delete(id).ok_or("User not found".to_string())?;
        Ok(())
    }

    pub fn list_all_users(&self) -> Vec<&User> {
        self.repo.list_all()
    }

    pub fn user_count(&self) -> usize {
        self.repo.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_success() {
        let mut service = UserService::new();
        let user = service
            .create_user("Alice".to_string(), "alice@example.com".to_string())
            .unwrap();

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn test_create_user_duplicate_email() {
        let mut service = UserService::new();
        service
            .create_user("Alice".to_string(), "alice@test.com".to_string())
            .unwrap();

        let result = service.create_user("Bob".to_string(), "alice@test.com".to_string());
        assert!(result.is_err());
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

    #[test]
    fn test_delete_user() {
        let mut service = UserService::new();
        let user = service
            .create_user("David".to_string(), "david@test.com".to_string())
            .unwrap();

        service.delete_user(user.id).unwrap();
        assert!(service.get_user(user.id).is_none());
    }
}
