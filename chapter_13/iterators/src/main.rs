// =============================================================
// ITERATORS EN RUST — CAPÍTULO 13.2
// Ejemplos del libro "The Rust Programming Language"
// Todos reunidos en un solo archivo y explicados.
// =============================================================

fn main() {
    println!("===== 1) CREAR UN ITERATOR =====");
    crear_iterator();

    println!("\n===== 2) USAR next() DIRECTAMENTE =====");
    usar_next();

    println!("\n===== 3) MÉTODO CONSUMIDOR: sum() =====");
    metodo_sum();

    println!("\n===== 4) ADAPTADOR: map() + collect() =====");
    map_y_collect();

    println!("\n===== 5) filter() CON CAPTURA DE ENTORNO =====");
    filter_ejemplo();
}

// -------------------------------------------------------------
// 1) CREAR UN ITERATOR
// -------------------------------------------------------------
// .iter() crea un iterator sobre referencias inmutables.
// No hace nada todavía (es lazy).
fn crear_iterator() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // El for consume el iterator automáticamente
    for val in v1_iter {
        println!("Valor: {}", val);
    }
}

// -------------------------------------------------------------
// 2) USAR next() DIRECTAMENTE
// -------------------------------------------------------------
// El trait Iterator requiere implementar:
// fn next(&mut self) -> Option<Self::Item>
//
// Cada llamada devuelve Some(valor) hasta que retorna None.
fn usar_next() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // debe ser mutable

    println!("{:?}", v1_iter.next()); // Some(&1)
    println!("{:?}", v1_iter.next()); // Some(&2)
    println!("{:?}", v1_iter.next()); // Some(&3)
    println!("{:?}", v1_iter.next()); // None
}

// -------------------------------------------------------------
// 3) MÉTODO CONSUMIDOR: sum()
// -------------------------------------------------------------
// sum() consume el iterator.
// Después de llamarlo ya no se puede usar.
fn metodo_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    println!("Suma total: {}", total);

    // v1_iter ya no puede usarse aquí
}

// -------------------------------------------------------------
// 4) ADAPTADOR: map() + collect()
// -------------------------------------------------------------
// map() transforma cada elemento pero NO ejecuta nada
// hasta que se consume con collect().
fn map_y_collect() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1
        .iter()
        .map(|x| x + 1) // closure que transforma cada elemento
        .collect();     // consume y crea un nuevo Vec

    println!("Vector transformado: {:?}", v2);
}

// -------------------------------------------------------------
// 5) filter() CON CAPTURA DE ENTORNO
// -------------------------------------------------------------
// filter() recibe un closure.
// Ese closure puede capturar variables del entorno.

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter() // mueve cada Shoe
        .filter(|s| s.size == shoe_size) // captura shoe_size
        .collect()
}
//a
fn filter_ejemplo() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("Zapatos en talle 10: {:?}", in_my_size);
}

// =============================================================
// RESUMEN CONCEPTUAL
// =============================================================
// - Los iterators son lazy (no hacen nada hasta consumirse).
// - .iter() -> referencias inmutables
// - .iter_mut() -> referencias mutables
// - .into_iter() -> mueve los valores
// - next() devuelve Option
// - Métodos consumidores: sum, collect, for
// - Adaptadores: map, filter (devuelven otro iterator)
// =============================================================
