/*
BYTE:
  Unidad básica de memoria (8 bits en sistemas modernos)

WORD:
  Word (palabra): Tamaño “natural” con el que la CPU opera y mueve datos de forma eficiente; coincide con el ancho de los registros generales y del bus de datos. Los punteros también tienen el tamaño de una palabra.
  En 16-bit: 1 word = 2 bytes
  En 32-bit: 1 word = 4 bytes
  En 64-bit: 1 word = 8 bytes

ADDRESS:
  El número que identifica cada byte en memoria

USIZE:
  tipo entero sin signo cuyo tamaño depende de la arquitectura = 1 word

ISIZE:
  tipo entero con signo cuyo tamaño depende de la arquitectura = 1 word

STACK:
  region de memoria para variables de tamaño conocido y vida corta

HEAP:
  region de memoria para datos de tamaño variable y vida larga

PADDING:
  bytes añadidos automáticamente entre variables o campos de struct para cumplir reglas de alineación y optimizar acceso, el padding se llena hasta llenar el word.

DIRECCIONES DE MEMORIA:
  Dirección  │  Contenido (byte)
  ───────────┼──────────────────
  0x1000     │  10101101  (byte 0) (valores: 0-255)
  0x1001     │  11010010  (byte 1)
  0x1002     │  01001110  (byte 2)
  0x1003     │  00110101  (byte 3)
...

*/
/*
STACK LAYOUT - SISTEMA 64-BIT CON ALINEACION Y PADDING DETALLADO
═════════════════════════════════════════════════════════════════════════════

ALINEACION:
  Cada tipo de dato debe empezar en una dirección múltiplo de su tamaño,
  para que la CPU pueda acceder a él eficientemente.
  Se agrega padding (bytes vacíos) entre variables si es necesario, para cumplir
  esta regla.

REGLA DE ALINEACION:
  - bool/u8:   debe estar en dirección múltiplo de 1 (0.1000, 0x1001, 0x1002, ...)
  - u32:       debe estar en dirección múltiplo de 4 (0x1000, 0x1004, 0x1008, ...)
  - u64:       debe estar en dirección múltiplo de 8 (0x1000, 0x1008, 0x1010, ...)
  - &T, *T:    debe estar en dirección múltiplo de 8 (0x1000, 0x1008, 0x1010, ...)
  - struct:    alineación = mayor alineación de sus campos

VARIABLES INDIVIDUALES EN STACK:
  Cuando tienes variables sueltas (no en struct), cada una se alinea a un word completo para optimizar acceso:
  let a: bool = true;  // 1 byte + 7 padding = 8 bytes (1 word)
  let b: bool = false; // 1 byte + 7 padding = 8 bytes (1 word)
  let c: bool = true;  // 1 byte + 7 padding = 8 bytes (1 word)

  en un struct, están juntos y compactos:
  struct ThreeBools { a: bool, b: bool, c: bool }
  let s = ThreeBools { a: true, b: false, c: true }; // Total: 3 bytes (no 24)
═════════════════════════════════════════════════════════════════════════════


Word │ Dirección         │ Byte │ Contenido      │ Variable
─────┼───────────────────┼──────┼────────────────┼──────────────────────────

bool:
  1  │ 0x7fff...632      │ 0    │ 0x01           │ bool flag = true
     │ 0x7fff...633      │ 1    │ 0x00 (padding) │
     │ 0x7fff...634      │ 2    │ 0x00 (padding) │ [7 bytes de padding
     │ 0x7fff...635      │ 3    │ 0x00 (padding) │  para alineación]
     │ 0x7fff...636      │ 4    │ 0x00 (padding) │
     │ 0x7fff...637      │ 5    │ 0x00 (padding) │
     │ 0x7fff...638      │ 6    │ 0x00 (padding) │
     │ 0x7fff...639      │ 7    │ 0x00 (padding) │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (bool ocupa 1, padding 7 → total 1 word)

u32:
  3  │ 0x7fff...648      │ 0    │ 0x0C           │ u32 small_num = 300
     │ 0x7fff...649      │ 1    │ 0x01           │ (little-endian: 0x0000012C)
     │ 0x7fff...650      │ 2    │ 0x00           │
     │ 0x7fff...651      │ 3    │ 0x00           │
     │ 0x7fff...652      │ 4    │ 0x00 (padding) │ [4 bytes padding]
     │ 0x7fff...653      │ 5    │ 0x00 (padding) │
     │ 0x7fff...654      │ 6    │ 0x00 (padding) │
     │ 0x7fff...655      │ 7    │ 0x00 (padding) │

// con OPTIMIZACION: u8 + u32 Compilador optimiza (empaqueta múltiples variables)
u8 + u32 empaquetados:
  2  │ 0x7fff...640      │ 0    │ 0xFF           │ u8 byte_val = 255
     │ 0x7fff...641      │ 1    │ 0x00 (padding) │ [3 bytes padding porque
     │ 0x7fff...642      │ 2    │ 0x00 (padding) │  u32 necesita alineación 4]
     │ 0x7fff...643      │ 3    │ 0x00 (padding) │
     │ 0x7fff...644      │ 4    │ 0x0C           │ u32 small_num = 300 (cabe en mismo word)
     │ 0x7fff...645      │ 5    │ 0x01           │
     │ 0x7fff...646      │ 6    │ 0x00           │
     │ 0x7fff...647      │ 7    │ 0x00           │
     │                   │      │ ─── OPTIMIZACIÓN ──│ 2 variables en 1 word

u64:
  4  │ 0x7fff...656      │ 0    │ 0xFF           │ u64 big_num = u64::MAX
     │ 0x7fff...657      │ 1    │ 0xFF           │ (8 bytes consecutivos
     │ 0x7fff...658      │ 2    │ 0xFF           │  sin padding)
     │ 0x7fff...659      │ 3    │ 0xFF           │
     │ 0x7fff...660      │ 4    │ 0xFF           │
     │ 0x7fff...661      │ 5    │ 0xFF           │
     │ 0x7fff...662      │ 6    │ 0xFF           │
     │ 0x7fff...663      │ 7    │ 0xFF           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (u64 ocupa 8 bytes exactos = 1 word)

[u32; 2]:
  5  │ 0x7fff...664      │ 0    │ 0x04           │ array[0]: u32 = 16909060
     │ 0x7fff...665      │ 1    │ 0x03           │ (little-endian: 0x01020304)
     │ 0x7fff...666      │ 2    │ 0x02           │
     │ 0x7fff...667      │ 3    │ 0x01           │
     │ 0x7fff...668      │ 4    │ 0x08           │ array[1]: u32 = 84281096
     │ 0x7fff...669      │ 5    │ 0x07           │ (little-endian: 0x05060708)
     │ 0x7fff...670      │ 6    │ 0x06           │
     │ 0x7fff...671      │ 7    │ 0x05           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 4 ──│ (cada u32 alineado a 4 = 1 word)

Box<i32>:
  6  │ 0x7fff...672      │ 0    │ 0xCD           │ Box<i32> ptr: 4328730445
     │ 0x7fff...673      │ 1    │ 0xAB           │ (little-endian: 0x000000010203ABCD)
     │ 0x7fff...674      │ 2    │ 0x03           │ NO hay padding porque empieza en
     │ 0x7fff...675      │ 3    │ 0x01           │ múltiplo de 8 (byte 0 de word 6)
     │ 0x7fff...676      │ 4    │ 0x00           │
     │ 0x7fff...677      │ 5    │ 0x00           │
     │ 0x7fff...678      │ 6    │ 0x00           │
     │ 0x7fff...679      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (puntero ocupa 8 bytes = 1 word)

String:
  7  │ 0x7fff...680      │ 0    │ 0xFF           │ String.ptr
     │ 0x7fff...681      │ 1    │ 0x04           │ 4328766719
     │ 0x7fff...682      │ 2    │ 0x02           │ (little-endian)
     │ 0x7fff...683      │ 3    │ 0x01           │
     │ 0x7fff...684      │ 4    │ 0x00           │
     │ 0x7fff...685      │ 5    │ 0x00           │
     │ 0x7fff...686      │ 6    │ 0x00           │
     │ 0x7fff...687      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (puntero ocupa 8 bytes = 1 word)
  8  │ 0x7fff...688      │ 0    │ 0x05           │ String.len: 5
     │ 0x7fff...689      │ 1    │ 0x00           │ (little-endian: 0x0000000000000005)
     │ 0x7fff...690      │ 2    │ 0x00           │
     │ 0x7fff...691      │ 3    │ 0x00           │
     │ 0x7fff...692      │ 4    │ 0x00           │
     │ 0x7fff...693      │ 5    │ 0x00           │
     │ 0x7fff...694      │ 6    │ 0x00           │
     │ 0x7fff...695      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│  (String.len ocupa 8 bytes = 1 word)
  9  │ 0x7fff...696      │ 0    │ 0x05           │ String.cap: 5
     │ 0x7fff...697      │ 1    │ 0x00           │ (little-endian: 0x0000000000000005)
     │ 0x7fff...698      │ 2    │ 0x00           │ (3 campos × 8 bytes = 24 bytes = 3 words)
     │ 0x7fff...699      │ 3    │ 0x00           │
     │ 0x7fff...700      │ 4    │ 0x00           │
     │ 0x7fff...701      │ 5    │ 0x00           │
     │ 0x7fff...702      │ 6    │ 0x00           │
     │ 0x7fff...703      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (String.cap ocupa 8 bytes = 1 word)

&bool:
 10  │ 0x7fff...704      │ 0    │ 0x56           │ &bool ref_to_bool
     │ 0x7fff...705      │ 1    │ 0x06           │ Dirección: 0x7fff0656
     │ 0x7fff...706      │ 2    │ 0x7f           │ (little-endian: 0x00007fff0656)
     │ 0x7fff...707      │ 3    │ 0x00           │
     │ 0x7fff...708      │ 4    │ 0x00           │
     │ 0x7fff...709      │ 5    │ 0x00           │
     │ 0x7fff...710      │ 6    │ 0x00           │
     │ 0x7fff...711      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (referencia ocupa 8 bytes = 1 word)

ptr_next:
 11  │ 0x7fff...712      │ 0    │ 0xAB           │ ptr_next: *const str
     │ 0x7fff...713      │ 1    │ 0xCD           │ (little-endian: 0x00007fffABCD)
     │ 0x7fff...714      │ 2    │ 0x7f           │
     │ 0x7fff...715      │ 3    │ 0x00           │
     │ 0x7fff...716      │ 4    │ 0x00           │
     │ 0x7fff...717      │ 5    │ 0x00           │
     │ 0x7fff...718      │ 6    │ 0x00           │
     │ 0x7fff...719      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (puntero ocupa 8 bytes = 1 word)

&T referencia:
 12  │ 0x7fff...720      │ 0    │ 0x9A           │ &T ref_to_data
     │ 0x7fff...721      │ 1    │ 0x12           │ Dirección: 0x7fff129A
     │ 0x7fff...722      │ 2    │ 0x7f           │ (little-endian: 0x00007fff129A)
     │ 0x7fff...723      │ 3    │ 0x00           │
     │ 0x7fff...724      │ 4    │ 0x00           │
     │ 0x7fff...725      │ 5    │ 0x00           │
     │ 0x7fff...726      │ 6    │ 0x00           │
     │ 0x7fff...727      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│ (referencia ocupa 8 bytes = 1 word)

struct Person {
    id: u32,
    active: bool,
    age: u32,
    name_ptr: *const str,
}

LAYOUT EN MEMORIA - ORDEN ORIGINAL (INEFICIENTE):

 13  │ 0x7fff...728      │ 0    │ 0x65           │ struct Person {
     │ 0x7fff...729      │ 1    │ 0x00           │   id: u32 = 101
     │ 0x7fff...730      │ 2    │ 0x00           │   (little-endian: 0x00000065)
     │ 0x7fff...731      │ 3    │ 0x00           │
     │ 0x7fff...732      │ 4    │ 0x01           │   active: bool = true
     │ 0x7fff...733      │ 5    │ 0x00 (padding) │   [3 bytes padding]
     │ 0x7fff...734      │ 6    │ 0x00 (padding) │
     │ 0x7fff...735      │ 7    │ 0x00 (padding) │
     │                   │      │                │
     │                   │      │ ─ DESPERDICIO ─│ 3 bytes wasted

 14  │ 0x7fff...736      │ 0    │ 0x1E           │   age: u32 = 30
     │ 0x7fff...737      │ 1    │ 0x00           │   (little-endian: 0x0000001E)
     │ 0x7fff...738      │ 2    │ 0x00           │
     │ 0x7fff...739      │ 3    │ 0x00           │
     │ 0x7fff...740      │ 4    │ 0x00 (padding) │   [4 bytes padding]
     │ 0x7fff...741      │ 5    │ 0x00 (padding) │
     │ 0x7fff...742      │ 6    │ 0x00 (padding) │
     │ 0x7fff...743      │ 7    │ 0x00 (padding) │
     │                   │      │                │
     │                   │      │ ─ DESPERDICIO ─│ 4 bytes wasted ← PEOR!

 15  │ 0x7fff...744      │ 0    │ 0x56           │   name_ptr: *const str
     │ 0x7fff...745      │ 1    │ 0x06           │   (little-endian: 0x00007fff0656)
     │ 0x7fff...746      │ 2    │ 0x7f           │
     │ 0x7fff...747      │ 3    │ 0x00           │
     │ 0x7fff...748      │ 4    │ 0x00           │
     │ 0x7fff...749      │ 5    │ 0x00           │
     │ 0x7fff...750      │ 6    │ 0x00           │
     │ 0x7fff...751      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─── ALINEACION 8 ──│
     │                   │      │                │ Total: 4+1+3+4+4+8 = 24 bytes
     │                   │      │                │ DESPERDICIO: 3+4 = 7 bytes

 16  │ 0x7fff...752      │ 0    │ 0x00 (padding) │ [padding final para próxima variable]
     │ 0x7fff...753      │ 1    │ 0x00 (padding) │
     │ 0x7fff...754      │ 2    │ 0x00 (padding) │
     │ 0x7fff...755      │ 3    │ 0x00 (padding) │
     │ 0x7fff...756      │ 4    │ 0x00 (padding) │
     │ 0x7fff...757      │ 5    │ 0x00 (padding) │
     │ 0x7fff...758      │ 6    │ 0x00 (padding) │
     │ 0x7fff...759      │ 7    │ 0x00 (padding) │
     │                   │      │                │
     │                   │      ├────────────────┬ [ESPACIO DISPONIBLE]
     │                   │      │
  ∞  │ 0x7fff...3040     │      │ ← Límite del stack

═════════════════════════════════════════════════════════════════════════════
LAYOUT OPTIMIZADO - REORDEN DE CAMPOS (EFICIENTE):

struct PersonOptimized {
    name_ptr: *const str,  // 8 bytes, alineación 8 → offset 0
    id: u32,               // 4 bytes, alineación 4 → offset 8
    age: u32,              // 4 bytes, alineación 4 → offset 12
    active: bool,          // 1 byte, alineación 1 → offset 16
}

Word │ Dirección         │ Byte │ Contenido      │ Campo
─────┼───────────────────┼──────┼────────────────┼──────────────────────────

 13  │ 0x7fff...728      │ 0    │ 0x56           │ name_ptr: *const str
     │ 0x7fff...729      │ 1    │ 0x06           │ (little-endian: 0x00007fff0656)
     │ 0x7fff...730      │ 2    │ 0x7f           │
     │ 0x7fff...731      │ 3    │ 0x00           │
     │ 0x7fff...732      │ 4    │ 0x00           │
     │ 0x7fff...733      │ 5    │ 0x00           │
     │ 0x7fff...734      │ 6    │ 0x00           │
     │ 0x7fff...735      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─ SIN PADDING ─│ 0 bytes wasted ✓

 14  │ 0x7fff...736      │ 0    │ 0x65           │ id: u32 = 101
     │ 0x7fff...737      │ 1    │ 0x00           │ (little-endian: 0x00000065)
     │ 0x7fff...738      │ 2    │ 0x00           │
     │ 0x7fff...739      │ 3    │ 0x00           │
     │ 0x7fff...740      │ 4    │ 0x1E           │ age: u32 = 30
     │ 0x7fff...741      │ 5    │ 0x00           │ (little-endian: 0x0000001E)
     │ 0x7fff...742      │ 6    │ 0x00           │
     │ 0x7fff...743      │ 7    │ 0x00           │
     │                   │      │                │
     │                   │      │ ─ SIN PADDING ─│ 0 bytes wasted ✓

 15  │ 0x7fff...744      │ 0    │ 0x01           │ active: bool = true
     │ 0x7fff...745      │ 1    │ 0x00 (padding) │ [7 bytes padding final]
     │ 0x7fff...746      │ 2    │ 0x00 (padding) │
     │ 0x7fff...747      │ 3    │ 0x00 (padding) │
     │ 0x7fff...748      │ 4    │ 0x00 (padding) │
     │ 0x7fff...749      │ 5    │ 0x00 (padding) │
     │ 0x7fff...750      │ 6    │ 0x00 (padding) │
     │ 0x7fff...751      │ 7    │ 0x00 (padding) │
     │                   │      │                │
     │                   │      │ ─ PADDING FINAL│ Total: 8+4+4+1+7 = 24 bytes
     │                   │      │                │ DESPERDICIO: 7 bytes (al final)



═════════════════════════════════════════════════════════════════════════════
COMPARATIVA VISUAL
═════════════════════════════════════════════════════════════════════════════

ORDEN ORIGINAL (INEFICIENTE):
┌─────────┬──────┬──────────┬─────────┬──────┬──────────┐
│ id (4)  │act(1)│pad(3)    │age (4)  │pad(4)│ptr (8)   │
│ u32     │bool  │ ××××     │ u32     │××××××│*const str│
└─────────┴──────┴──────────┴─────────┴──────┴──────────┘
0       4 5      8         12       16        24
DESPERDICIOS: 3 bytes + 4 bytes = 7 bytes (MAL DISTRIBUIDO)


ORDEN OPTIMIZADO (EFICIENTE):
┌─────────────────┬─────────┬─────────┬──────┬──────────┐
│   ptr (8)       │ id (4)  │age (4)  │act(1)│pad (7)   │
│*const str       │ u32     │ u32     │bool  │××××××××× │
└─────────────────┴─────────┴─────────┴──────┴──────────┘
0                8         12              16        24
DESPERDICIOS: 7 bytes al final (MEJOR DISTRIBUIDO)

✓ VENTAJA: Acceso a id, age, active sin atravesar padding
✗ DESVENTAJA: Padding solo al final (menos importante)

REGLA DE ORO:
  Ordena campos de MAYOR a MENOR alineación:
  - Punteros (8 bytes) primero
  - u64 (8 bytes)
  - u32 (4 bytes)
  - u16 (2 bytes)
  - u8, bool (1 byte) al final

═════════════════════════════════════════════════════════════════════════════

¿POR QUE EXISTE ALINEACION?

La CPU es más eficiente cuando accede a datos alineados:
  - Dirección alineada a 8: puede leer/escribir 8 bytes en UNA operación
  - Dirección NO alineada: requiere DOS operaciones + combinación

Ejemplo:
  ✓ ALINEADO (u64 en dirección 648):
    CPU lee dirección 648 → obtiene 8 bytes en 1 operación

  ✗ NO ALINEADO (u64 en dirección 649):
    CPU lee dirección 649 → obtiene solo parte del dato
    CPU lee dirección 657 → obtiene el resto
    CPU combina ambas partes (más lento)

PADDING ADICIONAL ENTRE VARIABLES:
  El compilador añade padding entre variables para que cada una
  empiece en una dirección alineada según su tamaño.

STRUCT LAYOUT:
  struct Person {
    id: u32,           // @ offset 0 (alineación 4)
    age: u8,           // @ offset 4 (alineación 1)
    active: bool,      // @ offset 5 (alineación 1)
    name_ptr: *const str, // @ offset 8 (alineación 8) ← requiere padding
  }

  MEMORIA:
  [id: 4 bytes] [padding: 3] [age: 1] [active: 1] [padding: 2] [name_ptr: 8]
  = 4 + 3 + 1 + 1 + 2 + 8 = 19 bytes → redondeado a 24 (múltiplo de 8)

REORDENAR CAMPOS REDUCE PADDING:
  struct PersonOptimizado {
    name_ptr: *const str,  // @ offset 0 (8 bytes)
    id: u32,               // @ offset 8 (4 bytes)
    age: u8,               // @ offset 12 (1 byte)
    active: bool,          // @ offset 13 (1 byte)
  }
  = 8 + 4 + 1 + 1 = 14 bytes → redondeado a 16 (mejor)

═════════════════════════════════════════════════════════════════════════════
*/

/*
═════════════════════════════════════════════════════════════════════════════
HEAP
═════════════════════════════════════════════════════════════════════════════


STACK (Contiguo, ordenado):
┌──────┬──────┬──────┬──────┬──────┐
│ var1 │ var2 │ var3 │ var4 │ var5 │ ← Variables pegadas
└──────┴──────┴──────┴──────┴──────┘
0x1000 0x1008 0x1010 0x1018 0x1020  ← Direcciones consecutivas

HEAP (Disperso, fragmentado):
┌──────┐         ┌──────┐               ┌──────┐
│ Box1 │         │ Vec1 │               │ Str1 │
└──────┘         └──────┘               └──────┘
0x2000           0x4000                 0x7000
       ↓ 8 KB ↑          ↓ 12 KB ↑             ↓ 4 KB ↑
              [FREE]              [FREE]              [FREE]

┌──────┐                          ┌──────┐
│ Box2 │                          │ Vec2 │
└──────┘                          └──────┘
0x9000                            0xC000
*/
#[allow(dead_code)]
#[allow(unused_variables)]
fn ejemplo() {
    // STACK: variables contiguas
    let a: u64 = 100; // dirección: 0x7fff0100
    let b: u32 = 200; // dirección: 0x7fff0108 (8 bytes después)
    let c: bool = true; // dirección: 0x7fff010C (4 bytes después)

    // HEAP: disperso
    let box1 = Box::new(42); // heap @ 0x1234567 (aleatorio)
    let string = String::from("hello"); // heap @ 0xABCDEF (aleatorio, lejano)
    let vec = vec![1, 2, 3]; // heap @ 0x98765 (aleatorio, entre los anteriores)

    // El allocator del heap elige dónde poner cada dato
    // dependiendo de qué memoria está disponible
}

/*
Ejemplo de fragmentación y asignacion:

Inicial (vacío):
┌──────────────────────────────────┐
│ [LIBRE]                          │
└──────────────────────────────────┘

Después de: let a = Box::new(10); (4 bytes)
┌─────┬──────────────────────────┐
│ [a] │ [LIBRE]                  │
└─────┴──────────────────────────┘

Después de: let b = Box::new(20); (4 bytes)
┌─────┬─────┬──────────────────────┐
│ [a] │ [b] │ [LIBRE]              │
└─────┴─────┴──────────────────────┘

Después de: drop(a); (libera 'a')
┌─────┬─────┬──────────────────────┐
│[LIBRE]│ [b] │ [LIBRE]              │
└─────┴─────┴──────────────────────┘
      ↑ Hueco disponible para nueva memoria

Después de: let c = Box::new(30); (4 bytes)
┌─────┬─────┬──────────────────────┐
│ [c] │ [b] │ [LIBRE]              │
└─────┴─────┴──────────────────────┘
      ↑ 'c' ocupa el hueco anterior


El heap usa un allocator (gestor de memoria) que:

Mantiene registro de bloques libres y usados
Cuando pides memoria, busca el primer bloque libre lo suficientemente grande
Puede haber "huecos" si liberaste memoria anterior
Nueva memoria va al primer hueco disponible (first-fit) o al mejor hueco (best-fit)
*/

/*
═════════════════════════════════════════════════════════════════════════════
PREASIGNACION DE MEMORIA
═════════════════════════════════════════════════════════════════════════════

Hay 3 escenarios donde se preasigna memoria:

1. ARRAYS ESTÁTICOS [T; N] - Compiletime, Stack
   ───────────────────────────────────────────────

   let arr: [u32; 5] = [1, 2, 3, 4, 5];

   ┌─────┬─────┬─────┬─────┬─────┐
   │  1  │  2  │  3  │  4  │  5  │  ← 5 elementos siempre
   └─────┴─────┴─────┴─────┴─────┘
   Dirección: 0x7fff0100 (stack)
   Tamaño: 5 × 4 bytes = 20 bytes (fijo, conocido en compiletime)

   CARACTERISTICAS:
   - Tamaño FIJO en compiletime [u32; 5] siempre son 5 elementos
   - Ubicación: STACK (contiguo, rápido)
   - Sin padding dinámico
   - No hay "capacity" vs "length" (ambos son 5)


2. VEC SIN PREASIGNACION - Runtime, Heap (crece bajo demanda)
   ────────────────────────────────────────────────────────────

   let mut vec = Vec::new();  // capacity = 0, length = 0
   vec.push(10);              // capacity = 1, length = 1
   vec.push(20);              // capacity = 2, length = 2
   vec.push(30);              // capacity = 4, length = 3 ← REALLOCATION

   PASO 1 (after new):
   Stack:
   ┌───────────────────┐
   │ ptr: null         │  ← sin heap asignado
   │ len: 0            │
   │ cap: 0            │
   └───────────────────┘

   PASO 2 (after push(10)):
   Stack:
   ┌───────────────────┐
   │ ptr: 0x1000       │  ─────┐
   │ len: 1            │       │
   │ cap: 1            │       │
   └───────────────────┘       │
                               ↓ Heap (0x1000):
                               ┌──────┐
                               │ [10] │  1 elemento asignado
                               └──────┘

   PASO 3 (after push(20)):
   Stack:
   ┌───────────────────┐
   │ ptr: 0x1000       │  ─────┐
   │ len: 2            │       │
   │ cap: 2            │       │
   └───────────────────┘       │
                               ↓ Heap (0x1000):
                               ┌──────┬──────┐
                               │ [10] │ [20] │  2 elementos asignados
                               └──────┴──────┘

   PASO 4 (after push(30)):
   El vec necesita crecer: capacity < length
   → REALLOCATE a nuevo bloque (0x2000):

   Stack:
   ┌───────────────────┐
   │ ptr: 0x2000       │  ─────┐ (cambió!)
   │ len: 3            │       │
   │ cap: 4            │       │  IMPORTANTE:
   └───────────────────┘       │  - Copia datos de 0x1000 a 0x2000
                               │  - Libera 0x1000
   VIEJO (0x1000):            │  - Nuevo bloque para 4 elementos
   ┌──────┬──────┐            │
   │[10]  │ [20] │ [LIBERADO] │
   └──────┴──────┘            │
                               ↓ Heap (0x2000):
                               ┌──────┬──────┬──────┬──────┐
                               │ [10] │ [20] │ [30] │ [??] │  4 elementos capacity
                               └──────┴──────┴──────┴──────┘

   PROBLEMA: Cada reallocation es cara (copia todo + libera anterior)


3. VEC CON PREASIGNACION - Runtime, Heap (evita reallocations)
   ─────────────────────────────────────────────────────────────

   let mut vec = Vec::with_capacity(10);
   // capacity = 10, length = 0 (PREASIGNADO)

   Stack:
   ┌───────────────────┐
   │ ptr: 0x1000       │  ─────┐
   │ len: 0            │       │
   │ cap: 10           │       │
   └───────────────────┘       │
                               ↓ Heap (0x1000):
                               ┌────────────────────────────┐
                               │ [??] [??] [??] ... [??]    │  10 elementos SIN INICIALIZAR
                               │ len=0       capacity=10    │
                               └────────────────────────────┘

   AHORA podemos hacer push sin reallocations:

   vec.push(10);
   vec.push(20);
   vec.push(30);

   Stack:
   ┌───────────────────┐
   │ ptr: 0x1000       │  ─────┐
   │ len: 3            │       │
   │ cap: 10           │       │
   └───────────────────┘       │
                               ↓ Heap (0x1000):
                               ┌────────────────────────────┐
                               │ [10] [20] [30] [??] ... [??]
                               │ len=3       capacity=10    │
                               └────────────────────────────┘

   ✓ NO hay reallocations (el espacio ya estaba reservado)
   ✓ Datos en orden
   ✓ Solo se usa 3/10 de la memoria preasignada


4. STRING CON PREASIGNACION - Runtime, Heap
   ──────────────────────────────────────────

   let mut s = String::with_capacity(100);
   // capacity = 100 bytes, length = 0 (string vacío)

   Stack:
   ┌───────────────────┐
   │ ptr: 0x3000       │  ─────┐
   │ len: 0            │       │
   │ cap: 100          │       │
   └───────────────────┘       │
                               ↓ Heap (0x3000):
                               ┌──────────────────────────────┐
                               │ [??] [??] ... [??]           │  100 bytes SIN INICIALIZAR
                               │ len=0       capacity=100      │
                               └──────────────────────────────┘

   s.push_str("Hola");  // len = 4
   s.push_str(" Mundo"); // len = 10

   Stack:
   ┌───────────────────┐
   │ ptr: 0x3000       │  ─────┐
   │ len: 10           │       │
   │ cap: 100          │       │
   └───────────────────┘       │
                               ↓ Heap (0x3000):
                               ┌──────────────────────────────┐
                               │ H o l a   M u n d o [??] ... │  10 bytes usados
                               │ len=10      capacity=100      │
                               └──────────────────────────────┘

   ✓ Sin reallocations
   ✓ Espacio reservado para crecer
   ✓ Solo 10 de 100 bytes usados


DIFERENCIA: CAPACITY vs LENGTH
═════════════════════════════════════════════════════════════════════════════

Vec<T>:

  length (len):     Cuántos elementos INICIALIZADOS tienes
                    Solo estos son accesibles: vec[0..len]

  capacity (cap):   Cuánta memoria TOTAL está asignada en heap
                    Puede ser >= length
                    Reserve para crecer sin reallocation

┌─────────────────────────────────────────┐
│ Vec { ptr, len, cap }                   │
└─────────────────────────────────────────┘

Ejemplo:
  let mut vec: Vec<u32> = Vec::with_capacity(100);
  vec.push(42);

  vec.len()      // 1 (un elemento inicializado)
  vec.capacity() // 100 (espacio total preasignado)

  println!("{:?}", vec);     // [42]
  println!("{}", vec[0]);    // 42 ✓
  println!("{}", vec[1]);    // ✗ PANIC (out of bounds)
                             // length es 1, no puedes acceder vec[1]


MEMORIA USADA vs ASIGNADA
═════════════════════════════════════════════════════════════════════════════

Sin preasignacion:
  let mut vec = Vec::new();
  vec.push(1); vec.push(2); vec.push(3); vec.push(4); vec.push(5);

  Reallocations: 0→1→2→4→8
  Memoria ASIGNADA TOTAL (a lo largo del tiempo): 1+2+4+8 = 15 bytes
  Memoria USADA FINAL: 5×4 = 20 bytes (en el bloque de 32 bytes)
  DESPERDICIO: 12 bytes (32-20)

Con preasignacion:
  let mut vec = Vec::with_capacity(10);
  vec.push(1); vec.push(2); vec.push(3); vec.push(4); vec.push(5);

  Reallocations: 0
  Memoria ASIGNADA: 10×4 = 40 bytes (una sola vez)
  Memoria USADA: 5×4 = 20 bytes
  DESPERDICIO: 20 bytes (40-20)

  ✓ Pero CERO reallocations (mucho más rápido)


CASOS DE USO
═════════════════════════════════════════════════════════════════════════════

USE Vec::with_capacity() CUANDO:
  ✓ Sabes aproximadamente cuántos datos esperas
  ✓ La reallocation es cara (muchos datos)
  ✓ Quieres performance predecible
  ✓ Loop que va a hacer muchos push()

  Ejemplo:
    let mut vec = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
      vec.push(i);  // Sin reallocations
    }

USE Vec::new() CUANDO:
  ✓ No sabes cuántos datos tendrás
  ✓ Los datos son pocos
  ✓ La performance no es crítica

  Ejemplo:
    let mut errors = Vec::new();
    if condition { errors.push("error1"); }
    if other_condition { errors.push("error2"); }


ARRAYS ESTÁTICOS vs VEC
═════════════════════════════════════════════════════════════════════════════

[T; N]:                    Vec<T>:
├─ Tamaño FIJO             ├─ Tamaño VARIABLE
├─ Compiletime             ├─ Runtime
├─ Stack                   ├─ Heap (metadatos en stack)
├─ Rápido acceso           ├─ Acceso por puntero (más lento)
├─ No puede crecer         ├─ Puede crecer/encogerse
└─ Tamaño parte del tipo   └─ Tamaño no parte del tipo

let arr: [u32; 5] = [1,2,3,4,5];  // 5 elementos, eso es TODO
let mut vec: Vec<u32> = Vec::new(); // 0 elementos, puede crecer

═════════════════════════════════════════════════════════════════════════════
*/
