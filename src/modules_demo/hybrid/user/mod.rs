// Módulo user: Enfoque híbrido
// Separa responsabilidades técnicas dentro del dominio

pub mod model;
pub mod repository;
pub mod service;

// Re-exports: API pública limpia
pub use model::User;
pub use service::UserService;

// repository::UserRepository no se exporta (implementación interna)

/*
VENTAJAS DEL ENFOQUE HÍBRIDO:

1. SEPARACIÓN DE RESPONSABILIDADES
   - model.rs: Solo datos y validaciones básicas
   - repository.rs: Solo persistencia
   - service.rs: Solo lógica de negocio

2. TESTABILIDAD
   - Cada capa se puede testear independientemente
   - Mock del repository fácil de crear

3. FACILIDAD DE NAVEGACIÓN
   - ¿Dónde está la estructura User? → model.rs
   - ¿Dónde se guarda? → repository.rs
   - ¿Lógica de negocio? → service.rs

4. REUSABILIDAD
   - El modelo puede importarse sin traer dependencias
   - El repository puede reemplazarse (SQL, NoSQL, etc.)

5. ESCALABILIDAD
   - Si model.rs crece, se puede dividir más
   - Fácil agregar nuevas capas (caching, events, etc.)

6. CONTROL DE API
   - mod.rs decide qué es público
   - Repository es internal, solo Service es público

CUÁNDO USAR:
✓ Proyectos medianos/grandes (1000+ líneas por dominio)
✓ Múltiples desarrolladores por feature
✓ Lógica de negocio compleja
✓ Necesitas testear capas independientemente
*/
