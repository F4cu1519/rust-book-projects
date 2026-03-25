// =============================================================================
// COMUNICACIÓN CON C DESDE RUST — Guía completa
// =============================================================================
//
// HAY DOS CASOS:
//
//  CASO 1: Funciones de la librería estándar de C (libc)
//          → Solo declarás con extern "C", no necesitás nada más.
//
//  CASO 2: Tu propia librería C (archivo .c, .so, .a, .dll)
//          → Necesitás también un build.rs que le diga a Cargo cómo linkear.
//
// =============================================================================


// =============================================================================
// CASO 1: Funciones de la libc (ya disponibles en el sistema)
// =============================================================================
//
// PASO ÚNICO: Declarar las funciones con `unsafe extern "C"`.
// No hay imports, no hay Cargo.toml especial, el linker las encuentra solo.
//
// SINTAXIS:
//   unsafe extern "C" {
//       fn nombre_funcion_en_c(param: tipo_rust) -> tipo_rust;
//   }
//
// TIPOS EQUIVALENTES C ↔ Rust:
//   int       ↔  i32 (o c_int del crate libc)
//   long      ↔  i64
//   float     ↔  f32
//   double    ↔  f64
//   char*     ↔  *const u8  (o *const c_char)
//   void*     ↔  *mut std::ffi::c_void
//   size_t    ↔  usize
//   bool      ↔  u8 (en C no hay bool real, es un entero)
// =============================================================================

unsafe extern "C" {
    // Valor absoluto — sin condiciones de safety, la marcamos 'safe'
    safe fn abs(input: i32) -> i32;

    // Raíz cuadrada de la libm
    fn sqrt(x: f64) -> f64;

    // Longitud de string C (necesita puntero a bytes terminados en '\0')
    fn strlen(s: *const u8) -> usize;

    // Copia de memoria (como memcpy en C)
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

fn ejemplo_libc() {
    println!("--- CASO 1: Funciones de la libc ---");

    // abs() es 'safe', no necesita bloque unsafe
    println!("abs(-99) = {}", abs(-99));

    // sqrt() NO es safe, necesita bloque unsafe
    // SAFETY: 16.0 es un f64 válido y positivo; sqrt es bien definida.
    let raiz = unsafe { sqrt(16.0) };
    println!("sqrt(16.0) = {}", raiz);

    // strlen() necesita un slice de bytes con '\0' al final (null-terminator)
    // En Rust los &str NO tienen '\0', hay que crearlo manualmente con b"..."
    let texto_c = b"hola desde C\0"; // el \0 al final es OBLIGATORIO para strlen
    // SAFETY: texto_c es un slice válido con null-terminator al final.
    let longitud = unsafe { strlen(texto_c.as_ptr()) };
    println!("strlen = {}", longitud); // imprime 13

    // memcpy: copiamos bytes de un buffer a otro
    let origen  = b"RUST";
    let mut destino = [0u8; 4];
    // SAFETY: ambos buffers son válidos y tienen al menos 4 bytes.
    unsafe {
        memcpy(destino.as_mut_ptr(), origen.as_ptr(), 4);
    }
    println!("memcpy resultado: {:?}", std::str::from_utf8(&destino).unwrap());
}


// =============================================================================
// CASO 2: Tu propia librería C
// =============================================================================
//
// Supongamos que tenés este archivo: src/mi_lib.c
//
//   // mi_lib.c
//   #include <stdio.h>
//
//   int sumar(int a, int b) {
//       return a + b;
//   }
//
//   void saludar(const char* nombre) {
//       printf("Hola, %s!\n", nombre);
//   }
//
// PARA LINKEARLA necesitás crear build.rs en la raíz del proyecto:
//
//   // build.rs  (va en la raíz, al lado de Cargo.toml)
//   fn main() {
//       // Le dice a Cargo que compile mi_lib.c
//       cc::Build::new()
//           .file("src/mi_lib.c")
//           .compile("mi_lib");       // genera libmi_lib.a
//   }
//
// Y agregar en Cargo.toml:
//   [build-dependencies]
//   cc = "1.0"        # crate que sabe compilar C
//
// Luego en tu código Rust declarás las funciones igual que antes:
// =============================================================================

// Estas serían las declaraciones para mi_lib.c de arriba
// (comentadas porque no tenemos el .c en este ejemplo)
//
// unsafe extern "C" {
//     fn sumar(a: i32, b: i32) -> i32;
//     fn saludar(nombre: *const u8);
// }
//
// fn ejemplo_libreria_propia() {
//     // SAFETY: los argumentos son enteros válidos.
//     let resultado = unsafe { sumar(10, 32) };
//     println!("sumar(10, 32) = {}", resultado);
//
//     let nombre = b"mundo\0";
//     // SAFETY: nombre es un slice válido con null-terminator.
//     unsafe { saludar(nombre.as_ptr()); }
// }


// =============================================================================
// UNION — ejemplo de uso real en FFI con C
// =============================================================================
//
// En C es muy común pasar unions para representar "uno de varios tipos posibles"
// sin saber cuál hasta runtime. Por ejemplo, una librería de eventos podría tener:
//
//   // C
//   union EventoData {
//       int  tecla;      // si el evento es KeyPress
//       struct { int x; int y; } mouse;  // si el evento es MouseMove
//   };
//
// En Rust lo representás así:
// =============================================================================

// Equivalente Rust de la union de C
// #[repr(C)] le dice al compilador que use el mismo layout de memoria que C
#[repr(C)]
union EventoData {
    tecla: i32,
    // Para structs dentro de union se necesita que sean Copy
    mouse: MousePos,
}

// La struct también necesita #[repr(C)] si va a cruzar la frontera con C
#[repr(C)]
#[derive(Clone, Copy)]
struct MousePos {
    x: i32,
    y: i32,
}

// Enum que indica qué campo de la union está activo
enum TipoEvento {
    Tecla,
    Mouse,
}

fn ejemplo_union_ffi() {
    println!("--- Union en contexto FFI ---");

    // Simulamos recibir un evento de teclado desde C
    let evento = EventoData { tecla: 65 }; // 65 = tecla 'A' en ASCII
    let tipo   = TipoEvento::Tecla;

    match tipo {
        TipoEvento::Tecla => {
            // SAFETY: sabemos que el campo activo es 'tecla' porque tipo == Tecla
            let codigo = unsafe { evento.tecla };
            println!("Tecla presionada: {} ('{}')", codigo, char::from(codigo as u8));
        }
        TipoEvento::Mouse => {
            // SAFETY: sabemos que el campo activo es 'mouse' porque tipo == Mouse
            let pos = unsafe { evento.mouse };
            println!("Mouse en: ({}, {})", pos.x, pos.y);
        }
    }

    // Simulamos un evento de mouse
    let evento2 = EventoData { mouse: MousePos { x: 320, y: 240 } };
    let tipo2   = TipoEvento::Mouse;

    match tipo2 {
        TipoEvento::Mouse => {
            let pos = unsafe { evento2.mouse };
            println!("Mouse movido a: ({}, {})", pos.x, pos.y);
        }
        TipoEvento::Tecla => {
            let k = unsafe { evento2.tecla };
            println!("Tecla: {}", k);
        }
    }
}


// =============================================================================
// UNIÓN vs ENUM — ¿cuándo usar cada uno?
// =============================================================================
//
//  Situación                              │ Usar
// ───────────────────────────────────────┼───────────────────────────────
//  Código puro Rust                       │ enum  (más seguro, idiomático)
//  Interactuar con código C que usa union │ union (necesario para FFI)
//  Type punning / reinterpretar bytes     │ union + unsafe
//  Variantes con tipo conocido en runtime │ union + campo discriminante
//
// En Rust idiomático CASI NUNCA usás union directamente.
// Solo cuando C te lo impone.
// =============================================================================


fn main() {
    ejemplo_libc();
    println!();
    ejemplo_union_ffi();

    println!();
    println!("=== Resumen de cómo llamar a C ===");
    println!("1. Funciones de la libc  → solo extern 'C' {{ ... }}, sin nada más");
    println!("2. Tu propio .c          → build.rs + crate 'cc' en Cargo.toml");
    println!("3. Librería .so/.dll/.a  → build.rs con println!(\"cargo:rustc-link-lib=...\")");
    println!("4. Las funciones siempre se declaran igual con extern 'C'");
    println!("5. Las llamadas siempre van en bloque unsafe (salvo marcadas 'safe')");
}