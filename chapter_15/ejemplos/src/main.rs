// ============================================================
//   PUNTEROS INTELIGENTES EN RUST
//   Box<T> | Deref | Drop
// ============================================================

use std::ops::Deref;


// ============================================================
// 1. Box<T>
//
// Box guarda el dato en el HEAP en lugar del STACK.
// Es el DUEÑO del dato — cuando Box muere, el heap se libera.
//
//   STACK           HEAP
//   [ ptr ] ──────▶ [ dato ]
// ============================================================

fn ejemplo_box_basico() {
    println!("\n--- Box básico ---");

    let x = 5;           // vive en el STACK
    let b = Box::new(5); // vive en el HEAP

    println!("x = {x}"); // 5
    println!("b = {b}"); // 5 (Box se usa igual que una variable normal)

    // Al terminar esta función, b sale del ámbito
    // → Drop se ejecuta automáticamente
    // → el heap se libera 🧹
}


fn ejemplo_box_transferir_propiedad() {
    println!("\n--- Box: transferir propiedad ---");

    let a = Box::new(100);
    let b = a; // la propiedad se MUEVE a b, el heap no se copia

    // println!("{a}"); // ❌ a ya no es válido
    println!("b = {b}"); // ✅ solo b es el dueño ahora
}


// Box permite retornar datos desde una función sin copiarlos.
// & no puede hacer esto porque el dato moriría con el stack.
fn crear_dato() -> Box<i32> {
    Box::new(42)
    // el stack de esta función muere, pero Box se mueve afuera
    // el heap sigue vivo ✅
}

fn ejemplo_box_retorno() {
    println!("\n--- Box: retorno desde función ---");

    let dato = crear_dato();
    println!("dato = {dato}"); // 42
}


// ============================================================
// 2. Box con tipos recursivos
//
// Sin Box, Rust no puede calcular el tamaño de un tipo
// que se contiene a sí mismo — sería infinito.
//
// Box soluciona esto porque un puntero SIEMPRE mide lo mismo
// (8 bytes), sin importar qué tan grande sea lo que apunta.
//
//   Nodo = i32 (4 bytes) + Box (8 bytes) = 12 bytes ✅
// ============================================================

enum Lista {
    Nodo(i32, Box<Lista>),
    Vacia,
}

fn ejemplo_lista_recursiva() {
    println!("\n--- Lista recursiva con Box ---");

    // [1] → [2] → [3] → Vacia
    let lista = Lista::Nodo(1,
        Box::new(Lista::Nodo(2,
            Box::new(Lista::Nodo(3,
                Box::new(Lista::Vacia))))));

    // recorrer la lista
    let mut actual = &lista;
    loop {
        match actual {
            Lista::Nodo(valor, siguiente) => {
                print!("[{valor}] → ");
                actual = siguiente;
            }
            Lista::Vacia => {
                println!("Vacia");
                break;
            }
        }
    }
    // cuando lista sale del ámbito, Drop se llama en cadena:
    // Nodo(1) → Nodo(2) → Nodo(3) → Vacia 🧹
}


// ============================================================
// 3. Deref
//
// El trait Deref permite usar * para "seguir" el puntero
// hasta el valor real. Box lo implementa automáticamente.
//
// Cuando escribes *y, Rust ejecuta internamente:
//   *(y.deref())
// ============================================================

fn ejemplo_deref_basico() {
    println!("\n--- Deref básico ---");

    let x = 5;
    let y = &x;        // referencia normal
    let b = Box::new(5); // Box

    // * "sigue" el puntero hasta el valor
    assert_eq!(5, x);
    assert_eq!(5, *y); // desreferencia de &
    assert_eq!(5, *b); // desreferencia de Box (usa Deref internamente)

    println!("x={x}, *y={}, *b={}", *y, *b);
}


// Creamos nuestro propio Box para entender cómo funciona Deref
struct MiBox<T>(T);

impl<T> MiBox<T> {
    fn new(x: T) -> MiBox<T> {
        MiBox(x)
    }
}

// Sin esto, * no funcionaría en MiBox
impl<T> Deref for MiBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // devuelve referencia al primer elemento
    }
}

fn ejemplo_deref_propio() {
    println!("\n--- Deref con MiBox propio ---");

    let x = 5;
    let y = MiBox::new(x);

    assert_eq!(5, *y); // Rust ejecuta *(y.deref()) internamente
    println!("*y = {}", *y);
}


// ============================================================
// Deref Coercion
//
// Rust convierte tipos automáticamente cuando los tipos no
// coinciden, aplicando deref las veces que haga falta.
//
//   &MiBox<String> → deref → &String → deref → &str ✅
//
// Todo en tiempo de compilación — sin costo en ejecución.
// ============================================================

fn saludar(nombre: &str) {
    println!("Hola, {nombre}!");
}

fn ejemplo_deref_coercion() {
    println!("\n--- Deref coercion ---");

    let nombre = MiBox::new(String::from("Rust"));

    // &MiBox<String> se convierte automáticamente a &str
    saludar(&nombre); // ✅ sin conversión manual

    // sin deref coercion tendrías que escribir:
    saludar(&(*nombre)[..]); // 😱 mucho más difícil de leer
}


// ============================================================
// 4. Drop
//
// Drop te permite definir qué pasa cuando un valor sale del
// ámbito. Rust lo llama automáticamente — nunca olvida.
//
// El sistema de propiedad garantiza que drop se llame
// exactamente UNA vez. Nunca dos veces, nunca cero.
// ============================================================

struct Recurso {
    nombre: String,
}

impl Drop for Recurso {
    fn drop(&mut self) {
        // aquí liberarías archivos, conexiones, locks, etc.
        println!("🧹 Limpiando recurso: '{}'", self.nombre);
    }
}

fn ejemplo_drop_automatico() {
    println!("\n--- Drop automático ---");

    let a = Recurso { nombre: String::from("A") };
    let b = Recurso { nombre: String::from("B") };
    let c = Recurso { nombre: String::from("C") };

    println!("Recursos creados: A, B, C");

    // al salir del ámbito se limpian en orden INVERSO: C, B, A
}


fn ejemplo_drop_anticipado() {
    println!("\n--- Drop anticipado con drop() ---");

    let lock = Recurso { nombre: String::from("lock de base de datos") };
    println!("Lock adquirido");

    // liberamos el lock antes de terminar la función
    // para que otro código pueda usarlo
    drop(lock); // ✅ forma correcta de forzar limpieza anticipada
    // lock.drop() ❌ prohibido — Rust lo llamaría dos veces

    println!("Lock liberado — otro código puede continuar");
    println!("Haciendo otras cosas...");
}


// ============================================================
// MAIN
// ============================================================

fn main() {
    println!("=== PUNTEROS INTELIGENTES EN RUST ===");

    // Box
    ejemplo_box_basico();
    ejemplo_box_transferir_propiedad();
    ejemplo_box_retorno();
    ejemplo_lista_recursiva();

    // Deref
    ejemplo_deref_basico();
    ejemplo_deref_propio();
    ejemplo_deref_coercion();

    // Drop
    ejemplo_drop_automatico();
    ejemplo_drop_anticipado();

    println!("\n=== FIN ===");
}