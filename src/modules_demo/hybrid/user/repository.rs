// Repository: Capa de persistencia
// Solo se encarga de guardar/recuperar datos

use super::model::User;
use std::collections::HashMap;

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

    pub fn delete(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_find() {
        let mut repo = UserRepository::new();
        let user = User::new(1, "Alice".to_string(), "alice@test.com".to_string());

        repo.save(user.clone()).unwrap();
        let found = repo.find_by_id(1).unwrap();

        assert_eq!(found, &user);
    }

    #[test]
    fn test_find_by_email() {
        let mut repo = UserRepository::new();
        let user = User::new(1, "Bob".to_string(), "bob@test.com".to_string());

        repo.save(user.clone()).unwrap();
        let found = repo.find_by_email("bob@test.com").unwrap();

        assert_eq!(found.name, "Bob");
    }

    #[test]
    fn test_delete() {
        let mut repo = UserRepository::new();
        let user = User::new(1, "Charlie".to_string(), "charlie@test.com".to_string());

        repo.save(user).unwrap();
        let deleted = repo.delete(1).unwrap();

        assert_eq!(deleted.name, "Charlie");
        assert!(repo.find_by_id(1).is_none());
    }
}
