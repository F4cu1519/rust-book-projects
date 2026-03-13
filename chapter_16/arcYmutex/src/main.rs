// ============================================================================
// CAPÍTULO 16.3 — Concurrencia con Estado Compartido
// ============================================================================
//
// TEORÍA GENERAL
// --------------
// Mientras los canales (cap. 16.2) transfieren la propiedad del dato de un
// hilo a otro (single ownership), la memoria compartida permite que múltiples
// hilos accedan al MISMO dato al mismo tiempo (multiple ownership).
// Esto requiere sincronización explícita para evitar data races.
//
// La herramienta principal es Mutex<T> combinada con Arc<T>.
//
// ============================================================================
// POR QUÉ Mutex<T> SOLO NO ALCANZA EN MÚLTIPLES HILOS
// ----------------------------------------------------
// Si intentás mover un Mutex<T> a múltiples hilos directamente:
//
//   let counter = Mutex::new(0);
//   for _ in 0..3 {
//       thread::spawn(move || { counter.lock()... });
//   }
//
// El compilador falla con E0382: "borrow of moved value".
// En la primera iteración, counter se MUEVE al closure del hilo.
// En la segunda, ya no existe en el scope del main.
// Rust no permite mover un valor a más de un dueño.
//
// ============================================================================
// POR QUÉ Rc<Mutex<T>> TAMPOCO FUNCIONA
// --------------------------------------
// Rc<T> permite múltiples dueños contando referencias, pero NO es thread-safe.
// Su contador de referencias usa operaciones normales (no atómicas).
// Si dos hilos clonan o descartan un Rc<T> al mismo tiempo, el contador
// puede corromperse → memory leak o use-after-free.
//
// El compilador falla con E0277:
//   "Rc<Mutex<i32>> cannot be sent between threads safely"
//   "the trait Send is not implemented for Rc<Mutex<i32>>"
//
// Send es un trait marcador que certifica que un tipo es seguro de transferir
// entre hilos. Rc<T> no lo implementa deliberadamente. Arc<T> sí.
//
// ============================================================================
// LA SOLUCIÓN: Arc<Mutex<T>>
// --------------------------
// Arc<T> (Atomically Reference Counted) es idéntico a Rc<T> en su API,
// pero usa operaciones atómicas del procesador para el contador de referencias.
// Las operaciones atómicas son indivisibles: ningún hilo puede interrumpirlas
// a mitad, garantizando un conteo siempre correcto.
//
//   Arc  → múltiples dueños del mismo dato (thread-safe)
//   Mutex → acceso exclusivo al dato de a un hilo por vez
//
// Arc::clone() NO copia el dato: solo incrementa el contador atómico.
// Todos los clones apuntan al mismo Mutex en memoria.
//
// ============================================================================
// CÓMO FUNCIONA MutexGuard Y DROP
// --------------------------------
// .lock() devuelve un MutexGuard<T>: un smart pointer que implementa:
//   - Deref    → para acceder al dato interior (*guard)
//   - Drop     → para liberar el lock automáticamente al salir de scope
//
// No hay unlock() manual. Es imposible olvidarse de liberar el lock.
// También podés llamar drop(guard) explícitamente para liberarlo antes
// de que salga de scope, útil cuando no querés tener el lock ocupado
// más tiempo del necesario.
//
// ============================================================================
// EL RIESGO: DEADLOCK
// -------------------
// Rust no puede protegerte de errores lógicos como el deadlock:
// ocurre cuando dos hilos se esperan mutuamente para siempre.
//
//   Hilo A tiene Lock 1, espera Lock 2
//   Hilo B tiene Lock 2, espera Lock 1  → ninguno avanza jamás
//
// Solución: adquirir siempre los locks en el mismo orden en todos los hilos.
// ============================================================================

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Ejemplo 1: Mutex básico (un solo hilo) ===");
    ejemplo_1_mutex_basico();

    println!("\n=== Ejemplo 2: Arc<Mutex<T>> con 3 hilos ===");
    ejemplo_2_arc_mutex();

    println!("\n=== Ejemplo 3: 10 hilos × 100 incrementos ===");
    ejemplo_3_carga_alta();

    println!("\n=== Ejemplo 4: drop() explícito del MutexGuard ===");
    ejemplo_4_drop_explicito();
}

// Mutex en un solo hilo — demuestra la API básica
fn ejemplo_1_mutex_basico() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("  dentro del scope: valor = {}", *num);
    } // MutexGuard sale de scope → Drop libera el lock automáticamente

    println!("  fuera del scope:  {:?}", m);
}

// Arc<Mutex<T>> con múltiples hilos — la solución correcta
fn ejemplo_2_arc_mutex() {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for id in 1..=3 {
        let contador = Arc::clone(&contador); // incrementa el contador atómico, no copia el dato
        let h = thread::spawn(move || {
            let mut num = contador.lock().unwrap();
            *num += 1;
            println!("  hilo {} → contador = {}", id, *num);
        }); // num sale de scope → lock liberado → siguiente hilo puede adquirirlo
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("  resultado final: {}", *contador.lock().unwrap()); // siempre 3
}

// Prueba de correctitud: sin Mutex habría data races y el resultado sería < 1000
fn ejemplo_3_carga_alta() {
    let contador = Arc::new(Mutex::new(0_i32));
    let mut handles = vec![];

    for id in 1..=10 {
        let contador = Arc::clone(&contador);
        let h = thread::spawn(move || {
            for _ in 0..100 {
                *contador.lock().unwrap() += 1;
            }
            println!("  hilo {} terminó", id);
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    let val = *contador.lock().unwrap();
    println!("  10 hilos × 100 = {}  {}", val, if val == 1000 { "✓" } else { "✗" });
}

// drop() explícito: liberar el lock antes de que salga de scope
fn ejemplo_4_drop_explicito() {
    let datos = Arc::new(Mutex::new(vec![1, 2, 3]));

    let mut guard = datos.lock().unwrap();
    guard.push(4);
    println!("  con lock:  {:?}", *guard);
    drop(guard); // libera el lock explícitamente — útil para no bloquearlo más de lo necesario

    let guard2 = datos.lock().unwrap(); // podemos re-adquirir inmediatamente
    println!("  sin lock previo: {:?}", *guard2);
} // guard2 libera el lock por Drop al salir de scope