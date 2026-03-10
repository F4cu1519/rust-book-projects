// ============================================================
//   CICLOS DE REFERENCIA Y Weak<T>
//
//   El problema: Rc<T> nunca libera memoria si hay un ciclo
//   La solución: Weak<T> rompe el ciclo sin afectar al dueño
// ============================================================

use std::rc::{Rc, Weak};
use std::cell::RefCell;


// ============================================================
// PARTE 1: El problema — ciclo de referencia
//
// a apunta a b
// b apunta a a
//
// Ninguno llega a contador 0 → memoria nunca se libera 😱
//
// No lo ejecutamos porque cuelga el programa,
// pero lo mostramos para entender el problema.
// ============================================================

// Si descomentaras esto y lo corrieras, el programa nunca
// liberaría la memoria de a ni de b al terminar:
//
// let a = Rc::new(RefCell::new(None::<Rc<_>>));
// let b = Rc::new(RefCell::new(Some(Rc::clone(&a))));
// *a.borrow_mut() = Some(Rc::clone(&b));
// → a.contador = 2, b.contador = 2
// → al salir: a→1, b→1. Nunca llegan a 0. Fuga de memoria.


// ============================================================
// PARTE 2: La solución — Weak<T>
//
// Weak es una referencia que NO es dueña del dato.
// No sube el strong_count → no impide que se libere.
//
// Para usarla, primero verificas si el dato sigue vivo:
//   weak.upgrade() → Some(rc) si vive, None si fue liberado
// ============================================================

fn ejemplo_weak_basico() {
    println!("--- Weak<T>: referencia sin propiedad ---\n");

    let fuerte = Rc::new(42); // strong_count = 1
    let debil = Rc::downgrade(&fuerte); // weak_count = 1, strong_count sigue en 1

    println!("strong_count = {}", Rc::strong_count(&fuerte)); // 1
    println!("weak_count   = {}", Rc::weak_count(&fuerte));   // 1

    // para usar el Weak, primero verificamos si el dato existe
    match debil.upgrade() {
        Some(valor) => println!("el dato existe: {}", valor), // 42 ✅
        None        => println!("el dato ya fue liberado"),
    }

    // soltamos el dueño fuerte
    drop(fuerte); // strong_count → 0 → dato se libera 🧹
                  // weak_count = 1 pero NO importa para liberar

    // ahora el Weak apunta a memoria liberada
    match debil.upgrade() {
        Some(valor) => println!("el dato existe: {}", valor),
        None        => println!("el dato ya fue liberado ✅"), // esto se imprime
    }
}


// ============================================================
// PARTE 3: Caso real — árbol padre/hijo
//
// Un padre es DUEÑO de sus hijos  → Rc  (fuerte)
// Un hijo CONOCE a su padre       → Weak (débil, sin propiedad)
//
// Si usáramos Rc en ambos lados:
//   padre → hijo  (strong)
//   hijo  → padre (strong)
//   = ciclo = fuga de memoria
//
// Con Weak en el hijo → no hay ciclo → memoria se libera bien
// ============================================================

#[derive(Debug)]
struct Nodo {
    valor: i32,
    padre: RefCell<Weak<Nodo>>,       // débil: conoce al padre pero no lo posee
    hijos: RefCell<Vec<Rc<Nodo>>>,    // fuerte: es dueño de sus hijos
}

fn ejemplo_arbol() {
    println!("\n--- Arbol padre/hijo con Weak<T> ---\n");

    // creamos el hijo (hoja) sin padre todavia
    let hoja = Rc::new(Nodo {
        valor: 3,
        padre: RefCell::new(Weak::new()), // sin padre por ahora
        hijos: RefCell::new(vec![]),
    });

    println!("hoja sin padre:");
    println!("  strong = {}, weak = {}", Rc::strong_count(&hoja), Rc::weak_count(&hoja));
    println!("  padre = {:?}\n", hoja.padre.borrow().upgrade()); // None

    {
        // creamos el padre con la hoja como hijo
        let rama = Rc::new(Nodo {
            valor: 5,
            padre: RefCell::new(Weak::new()),
            hijos: RefCell::new(vec![Rc::clone(&hoja)]), // rama es dueño de hoja
        });

        // le decimos a hoja quién es su padre (con Weak, no Rc)
        *hoja.padre.borrow_mut() = Rc::downgrade(&rama);

        println!("con rama creada:");
        println!("  rama  strong = {}, weak = {}",
            Rc::strong_count(&rama),  // 1: solo lo tiene `rama`
            Rc::weak_count(&rama));   // 1: hoja apunta a rama con Weak

        println!("  hoja  strong = {}, weak = {}",
            Rc::strong_count(&hoja),  // 2: lo tienen `hoja` y `rama.hijos`
            Rc::weak_count(&hoja));   // 0

        println!("  hoja.padre = Some(rama con valor {})\n",
            hoja.padre.borrow().upgrade().unwrap().valor); // 5 ✅

    } // rama sale del ámbito aquí
      // strong_count de rama: 1 → 0 → se libera 🧹
      // weak_count = 1 pero NO importa, igual se libera

    println!("despues de que rama muere:");
    println!("  hoja  strong = {}, weak = {}",
        Rc::strong_count(&hoja), // 1: solo lo tiene `hoja`
        Rc::weak_count(&hoja));  // 0

    // el Weak de hoja.padre ya no apunta a nada válido
    println!("  hoja.padre = {:?}",
        hoja.padre.borrow().upgrade()); // None ✅ sin crash, sin fuga
}


// ============================================================
// MAIN
// ============================================================

fn main() {
    println!("=== CICLOS DE REFERENCIA Y Weak<T> ===\n");

    ejemplo_weak_basico();
    ejemplo_arbol();

    println!("\n=== RESUMEN ===");
    println!("Rc::clone(&a)     → strong_count++  → afecta si se libera");
    println!("Rc::downgrade(&a) → weak_count++    → NO afecta si se libera");
    println!("weak.upgrade()    → Some si vive, None si fue liberado");
    println!("Weak rompe ciclos porque no es dueno del dato");
}
