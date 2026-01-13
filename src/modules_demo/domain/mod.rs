// Módulo domain: organización por dominio/feature
// Cada submódulo es independiente y auto-contenido

pub mod order;
pub mod user;

// Re-exports para API más limpia
pub use order::{Order, OrderItem, OrderService, OrderStatus};
pub use user::{User, UserService};

/*
VENTAJAS DE mod.rs:

1. PUNTO DE ENTRADA CLARO
   - Un solo lugar para ver qué contiene el módulo
   - `use modules_demo::domain::User` vs `use modules_demo::domain::user::User`

2. RE-EXPORTS SELECTIVOS
   - Podemos ocultar detalles internos (repositories)
   - Solo exponemos la API pública (User, UserService)

3. DOCUMENTACIÓN CENTRALIZADA
   - Documentar el módulo completo aquí

4. CONTROL DE VISIBILIDAD
   - Decidir qué es público vs privado del módulo
*/
