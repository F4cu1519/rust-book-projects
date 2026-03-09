// ============================================================
//   Rc<T> y RefCell<T> en RUST
//
//   Rc<T>      → múltiples dueños, solo lectura
//   RefCell<T> → un dueño, pero permite mutar desde &self
//   Rc<RefCell<T>> → múltiples dueños + mutación controlada
// ============================================================

use std::rc::Rc;
use std::cell::RefCell;


// ============================================================
// PARTE 1: El problema con mut normal
//
// mut funciona perfecto cuando hay UN solo dueño.
// Rust sabe que nadie más tiene el dato, no hay riesgo.
// ============================================================

fn ejemplo_mut_normal() {
    println!("--- mut normal (un solo dueño) ---");

    let mut v = vec![1, 2, 3];
    v.push(4); // ✅ v es el único dueño, puede mutar sin problema

    println!("v = {:?}", v); // [1, 2, 3, 4]
}


// ============================================================
// PARTE 2: El problema con Rc<T>
//
// Rc permite múltiples dueños del mismo dato en el heap.
// Lleva un contador interno: sube con clone, baja con Drop.
// Cuando el contador llega a 0, el dato se libera.
//
// PERO: si varios dueños pudieran mutar al mismo tiempo,
// podrían pisarse entre sí sin saberlo → data race 💥
// Por eso Rc solo permite lectura.
// ============================================================

fn ejemplo_rc_solo_lectura() {
    println!("\n--- Rc<T>: múltiples dueños, solo lectura ---");

    let a = Rc::new(vec![1, 2, 3]);
    let b = Rc::clone(&a); // b también es dueño del mismo vec
    let c = Rc::clone(&a); // c también

    // los tres ven el mismo dato en memoria
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    println!("dueños activos = {}", Rc::strong_count(&a)); // 3

    // a.push(4); // ❌ Rc no permite mutar — varios dueños, riesgo de data race

    // cuando a, b y c salen del ámbito:
    // contador baja de 3 → 2 → 1 → 0 → dato se libera 🧹
}


// ============================================================
// PARTE 3: El problema con &self inmutable
//
// Cuando implementas un trait que dice &self,
// no puedes cambiarlo a &mut self.
// Pero a veces necesitas guardar estado interno.
//
// RefCell resuelve esto: permite mutar adentro
// aunque la firma diga &self.
// Las reglas de préstamo se verifican en RUNTIME,
// no en compilación.
// ============================================================

trait Mensajero {
    fn enviar(&self, msg: &str); // firma fija: &self, no puedes cambiarla
}

struct MockMensajero {
    // Sin RefCell: Vec<String> → no puedes mutar desde &self ❌
    // Con RefCell: RefCell<Vec<String>> → puedes mutar desde &self ✅
    mensajes: RefCell<Vec<String>>,
}

impl MockMensajero {
    fn nuevo() -> MockMensajero {
        MockMensajero {
            mensajes: RefCell::new(vec![]),
        }
    }
}

impl Mensajero for MockMensajero {
    fn enviar(&self, msg: &str) {           // &self inmutable
        self.mensajes                        // pero RefCell permite...
            .borrow_mut()                   // ...pedir acceso mutable
            .push(msg.to_string());         // ✅ y mutar adentro
    }
}

fn ejemplo_refcell_con_self() {
    println!("\n--- RefCell<T>: mutar desde &self ---");

    let mensajero = MockMensajero::nuevo(); // no necesita ser mut

    mensajero.enviar("Hola");       // &self pero muta adentro ✅
    mensajero.enviar("Como estas");
    mensajero.enviar("Chau");

    // borrow() para leer
    println!("mensajes guardados:");
    for m in mensajero.mensajes.borrow().iter() {
        println!("  - {m}");
    }
}


// ============================================================
// PARTE 4: Rc<RefCell<T>> — la combinación más poderosa
//
// Rc   → varios dueños del mismo dato
// RefCell → cualquiera de ellos puede mutarlo
//
// RefCell actúa como semáforo: solo deja pasar a uno a la vez.
// Si dos intentan mutar al mismo tiempo → panic en runtime.
// ============================================================

fn ejemplo_rc_refcell() {
    println!("\n--- Rc<RefCell<T>>: multiples duenos + mutacion ---");

    let dato = Rc::new(RefCell::new(vec![1, 2, 3]));

    let a = Rc::clone(&dato); // a es dueño
    let b = Rc::clone(&dato); // b también es dueño

    println!("dueños activos = {}", Rc::strong_count(&dato)); // 3

    // a muta el vec
    a.borrow_mut().push(4);
    println!("despues de que a agrega 4: {:?}", dato.borrow());

    // b también puede mutarlo
    b.borrow_mut().push(5);
    println!("despues de que b agrega 5: {:?}", dato.borrow());

    // todos ven el mismo vec actualizado
    println!("dato = {:?}", dato.borrow()); // [1, 2, 3, 4, 5]
    println!("a    = {:?}", a.borrow());    // [1, 2, 3, 4, 5]
    println!("b    = {:?}", b.borrow());    // [1, 2, 3, 4, 5]
}


// ============================================================
// PARTE 5: RefCell panics si violas las reglas en runtime
//
// Las reglas de Rust siguen existiendo:
//   - puedes tener MUCHOS borrow() al mismo tiempo
//   - pero solo UN borrow_mut() a la vez
//   - y no puedes mezclar borrow() y borrow_mut()
//
// Si las violas, no hay error de compilación — hay panic.
// ============================================================

fn ejemplo_refcell_panic() {
    println!("\n--- RefCell<T>: las reglas siguen existiendo ---");

    let dato = RefCell::new(vec![1, 2, 3]);

    // ✅ múltiples borrow() al mismo tiempo está bien
    let lectura1 = dato.borrow();
    let lectura2 = dato.borrow();
    println!("dos lecturas a la vez: {:?} y {:?}", lectura1, lectura2);
    drop(lectura1); // liberamos antes de intentar mutar
    drop(lectura2);

    // ✅ un borrow_mut() cuando no hay otros activos
    dato.borrow_mut().push(4);
    println!("despues de mutar: {:?}", dato.borrow());

    // ❌ esto haria panic en runtime:
    // let mut uno = dato.borrow_mut();
    // let mut dos = dato.borrow_mut(); // 💥 already borrowed: BorrowMutError
}


// ============================================================
// MAIN
// ============================================================

fn main() {
    println!("=== Rc<T> y RefCell<T> en RUST ===\n");

    ejemplo_mut_normal();
    ejemplo_rc_solo_lectura();
    ejemplo_refcell_con_self();
    ejemplo_rc_refcell();
    ejemplo_refcell_panic();

    println!("\n=== RESUMEN ===");
    println!("mut              -> un dueno,        puede mutar");
    println!("Rc<T>            -> varios duenos,   solo lectura");
    println!("RefCell<T>       -> un dueno,        muta desde &self");
    println!("Rc<RefCell<T>>   -> varios duenos,   muta con semaforo");
}