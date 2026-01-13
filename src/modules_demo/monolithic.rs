// Ejemplo MONOLÍTICO: Todo en un archivo
// ✗ Anti-patrón para código de producción
// ✓ OK para scripts pequeños, demos, prototipos

use std::collections::HashMap;

// ============================================================
// MODELS
// ============================================================

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub user_id: u64,
    pub total: f64,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone)]
pub struct OrderItem {
    pub product_id: u64,
    pub quantity: u32,
    pub price: f64,
}

#[derive(Debug, Clone)]
pub struct Payment {
    pub id: u64,
    pub order_id: u64,
    pub amount: f64,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

// ============================================================
// REPOSITORIES (Data Access)
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

    pub fn find_all(&self) -> Vec<&User> {
        self.storage.values().collect()
    }
}

pub struct OrderRepository {
    storage: HashMap<u64, Order>,
}

impl OrderRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn save(&mut self, order: Order) -> Result<(), String> {
        self.storage.insert(order.id, order);
        Ok(())
    }

    pub fn find_by_id(&self, id: u64) -> Option<&Order> {
        self.storage.get(&id)
    }

    pub fn find_by_user_id(&self, user_id: u64) -> Vec<&Order> {
        self.storage
            .values()
            .filter(|o| o.user_id == user_id)
            .collect()
    }
}

pub struct PaymentRepository {
    storage: HashMap<u64, Payment>,
}

impl PaymentRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn save(&mut self, payment: Payment) -> Result<(), String> {
        self.storage.insert(payment.id, payment);
        Ok(())
    }

    pub fn find_by_order_id(&self, order_id: u64) -> Option<&Payment> {
        self.storage.values().find(|p| p.order_id == order_id)
    }
}

// ============================================================
// SERVICES (Business Logic)
// ============================================================

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub fn create_user(&mut self, name: String, email: String) -> Result<User, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }

        let user = User {
            id: self.repo.storage.len() as u64 + 1,
            name,
            email,
        };

        self.repo.save(user.clone())?;
        Ok(user)
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.repo.find_by_id(id)
    }
}

pub struct OrderService {
    repo: OrderRepository,
}

impl OrderService {
    pub fn new(repo: OrderRepository) -> Self {
        Self { repo }
    }

    pub fn create_order(&mut self, user_id: u64, items: Vec<OrderItem>) -> Result<Order, String> {
        if items.is_empty() {
            return Err("Order must have at least one item".to_string());
        }

        let total: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();

        let order = Order {
            id: self.repo.storage.len() as u64 + 1,
            user_id,
            total,
            items,
        };

        self.repo.save(order.clone())?;
        Ok(order)
    }

    pub fn get_user_orders(&self, user_id: u64) -> Vec<&Order> {
        self.repo.find_by_user_id(user_id)
    }
}

pub struct PaymentService {
    repo: PaymentRepository,
}

impl PaymentService {
    pub fn new(repo: PaymentRepository) -> Self {
        Self { repo }
    }

    pub fn process_payment(&mut self, order_id: u64, amount: f64) -> Result<Payment, String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        let payment = Payment {
            id: self.repo.storage.len() as u64 + 1,
            order_id,
            amount,
            status: PaymentStatus::Completed,
        };

        self.repo.save(payment.clone())?;
        Ok(payment)
    }

    pub fn get_payment_for_order(&self, order_id: u64) -> Option<&Payment> {
        self.repo.find_by_order_id(order_id)
    }
}

// ============================================================
// PROBLEMAS DE ESTE ENFOQUE
// ============================================================

/*
PROBLEMAS:

1. DIFICULTAD DE NAVEGACIÓN
   - 250+ líneas en un archivo
   - Difícil encontrar código específico
   - Scroll infinito

2. MERGE CONFLICTS
   - Múltiples desarrolladores modificando el mismo archivo
   - Alto riesgo de conflictos

3. ACOPLAMIENTO IMPLÍCITO
   - Todo visible para todo
   - Fácil crear dependencias no deseadas
   - Difícil refactorizar

4. TESTS COMPLEJOS
   - Tests de usuario y order en el mismo módulo
   - No se puede testear aisladamente

5. COMPILACIÓN
   - Cambio pequeño requiere recompilar TODO el archivo
   - En archivos grandes (1000+ líneas) es notable

6. VIOLACIÓN DE SINGLE RESPONSIBILITY
   - Este archivo tiene 3 responsabilidades:
     * Gestión de usuarios
     * Gestión de órdenes
     * Gestión de pagos

CUÁNDO ES ACEPTABLE:
- Scripts pequeños (<200 líneas)
- Prototipos rápidos
- Ejemplos/tutoriales
- Single-purpose utilities

SOLUCIÓN:
- Ver domain/, technical/, o hybrid/ para alternativas mejores
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let mut service = UserService::new(UserRepository::new());
        let user = service
            .create_user("John".to_string(), "john@test.com".to_string())
            .unwrap();
        assert_eq!(user.name, "John");
    }

    #[test]
    fn test_create_order() {
        let mut service = OrderService::new(OrderRepository::new());
        let items = vec![OrderItem {
            product_id: 1,
            quantity: 2,
            price: 10.0,
        }];
        let order = service.create_order(1, items).unwrap();
        assert_eq!(order.total, 20.0);
    }

    // Problema: Tests de diferentes dominios mezclados en el mismo módulo
}
