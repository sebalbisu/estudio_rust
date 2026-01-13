// Dominio: Order
// Todo lo relacionado a órdenes en un solo lugar

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub user_id: u64,
    pub total: f64,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
}

#[derive(Debug, Clone)]
pub struct OrderItem {
    pub product_id: u64,
    pub quantity: u32,
    pub price: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
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

    pub fn find_by_user(&self, user_id: u64) -> Vec<&Order> {
        self.storage
            .values()
            .filter(|o| o.user_id == user_id)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }
}

pub struct OrderService {
    repo: OrderRepository,
}

impl OrderService {
    pub fn new() -> Self {
        Self {
            repo: OrderRepository::new(),
        }
    }

    pub fn create_order(&mut self, user_id: u64, items: Vec<OrderItem>) -> Result<Order, String> {
        if items.is_empty() {
            return Err("Order must have at least one item".to_string());
        }

        let total: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();

        let order = Order {
            id: (self.repo.count() + 1) as u64,
            user_id,
            total,
            items,
            status: OrderStatus::Pending,
        };

        self.repo.save(order.clone())?;
        Ok(order)
    }

    pub fn confirm_order(&mut self, order_id: u64) -> Result<(), String> {
        let order = self
            .repo
            .find_by_id(order_id)
            .ok_or("Order not found")?
            .clone();

        if order.status != OrderStatus::Pending {
            return Err("Order is not in pending status".to_string());
        }

        let updated = Order {
            status: OrderStatus::Confirmed,
            ..order
        };

        self.repo.save(updated)
    }

    pub fn get_user_orders(&self, user_id: u64) -> Vec<&Order> {
        self.repo.find_by_user(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order() {
        let mut service = OrderService::new();
        let items = vec![OrderItem {
            product_id: 1,
            quantity: 2,
            price: 15.0,
        }];

        let order = service.create_order(1, items).unwrap();
        assert_eq!(order.total, 30.0);
        assert_eq!(order.status, OrderStatus::Pending);
    }

    #[test]
    fn test_confirm_order() {
        let mut service = OrderService::new();
        let items = vec![OrderItem {
            product_id: 1,
            quantity: 1,
            price: 10.0,
        }];

        let order = service.create_order(1, items).unwrap();
        service.confirm_order(order.id).unwrap();

        let confirmed = service.repo.find_by_id(order.id).unwrap();
        assert_eq!(confirmed.status, OrderStatus::Confirmed);
    }
}
