// Módulo hybrid: submódulos organizados por dominio, con capas internas

pub mod user;

// Re-exports
pub use user::{User, UserService};

/*
ESTRATEGIA HÍBRIDA:

Estructura:
hybrid/
├── mod.rs           ← Punto de entrada
└── user/
    ├── mod.rs       ← Re-exports del dominio
    ├── model.rs     ← Estructuras de datos
    ├── repository.rs ← Persistencia
    └── service.rs   ← Lógica de negocio

Uso desde fuera:
```rust
use modules_demo::hybrid::User;         // Limpio
use modules_demo::hybrid::UserService;  // Limpio

// NO expuesto:
// use modules_demo::hybrid::user::repository::UserRepository;  ✗
```

CUÁNDO USAR CADA ESTRATEGIA:

1. MONOLÍTICO (1 archivo)
   ✓ <200 líneas
   ✓ Scripts/prototipos
   ✓ Single purpose

2. POR DOMINIO (domain/)
   ✓ Features independientes
   ✓ Microservicios
   ✓ 200-500 líneas por dominio

3. HÍBRIDO (hybrid/)
   ✓ >500 líneas por dominio
   ✓ Múltiples capas (model, service, repo)
   ✓ Proyectos grandes
   ✓ Equipos grandes
*/
