// Punto de entrada del módulo modules_demo
// Demuestra diferentes estrategias de organización

pub mod domain;
pub mod hybrid;
pub mod monolithic;

/*
RESUMEN DE ESTRATEGIAS:

┌─────────────────────────────────────────────────────────────────┐
│                     MONOLITHIC (1 archivo)                       │
├─────────────────────────────────────────────────────────────────┤
│ monolithic.rs                                                    │
│   ├── User, UserRepo, UserService                               │
│   ├── Order, OrderRepo, OrderService                            │
│   └── Payment, PaymentRepo, PaymentService                      │
│                                                                  │
│ ✓ Simple para código pequeño (<200 líneas)                      │
│ ✗ No escala, merge conflicts, acoplamiento                      │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                  DOMAIN (Por Feature/Vertical)                   │
├─────────────────────────────────────────────────────────────────┤
│ domain/                                                          │
│   ├── user.rs    → User + UserRepo + UserService                │
│   ├── order.rs   → Order + OrderRepo + OrderService             │
│   └── payment.rs → Payment + PaymentRepo + PaymentService       │
│                                                                  │
│ ✓ Alta cohesión, bajo acoplamiento                              │
│ ✓ Ideal para microservicios/DDD                                 │
│ ✗ Puede duplicar código común                                   │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│              HYBRID (Dominio + Capas Internas)                   │
├─────────────────────────────────────────────────────────────────┤
│ hybrid/                                                          │
│   └── user/                                                      │
│       ├── model.rs      → User struct                            │
│       ├── repository.rs → UserRepository                         │
│       └── service.rs    → UserService                            │
│                                                                  │
│ ✓ Mejor separación de responsabilidades                         │
│ ✓ Testabilidad máxima                                            │
│ ✓ Escalable para proyectos grandes                              │
│ ✗ Más archivos, overhead inicial                                │
└─────────────────────────────────────────────────────────────────┘

GUÍA DE DECISIÓN:

Tamaño del proyecto:
- <200 líneas      → monolithic
- 200-500 líneas   → domain
- >500 líneas      → hybrid

Equipo:
- 1 desarrollador  → monolithic o domain
- 2-5 devs         → domain
- >5 devs          → hybrid

Complejidad:
- CRUD simple      → domain
- Lógica compleja  → hybrid
- Microservicios   → domain

REGLAS GENERALES:

1. EMPEZAR SIMPLE
   - Comenzar con domain/
   - Migrar a hybrid/ cuando un dominio crece >300 líneas

2. PRIVADO POR DEFECTO
   - Solo exponer API pública en mod.rs
   - Detalles internos quedan privados

3. UN CONCEPTO = UN ARCHIVO
   - User en user.rs o user/model.rs
   - No mezclar User y Order en el mismo archivo

4. TESTS JUNTO AL CÓDIGO
   - #[cfg(test)] mod tests en el mismo archivo
   - Tests de integración en tests/

5. RE-EXPORTS LIMPIOS
   - mod.rs hace re-exports para API limpia
   - use crate::domain::User; (no domain::user::User)
*/
