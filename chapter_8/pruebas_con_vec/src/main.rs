// Importamos Debug para poder imprimir enums con {:?}
#[derive(Debug)]

// En Rust los enums usan PascalCase por convención
enum Familia {
    // Cada variante puede tener datos asociados
    // En este caso usamos i32 para representar la edad
    Nene(i32),
    Nena(i32),
    Hombre(i32),
    Mujer(i32),
}

fn main() {
    // ================================
    // VECTORES Y ARRAYS
    // ================================

    // Vector dinámico vacío
    // Vive en el heap y puede crecer o achicarse
    let vector_vacio: Vec<i32> = Vec::new();

    // Array de tamaño fijo
    // Vive en el stack y su tamaño NO puede cambiar
    let array = [1, 2, 3, 4, 5];

    // Acceso directo por índice (peligroso si se sale de rango)
    println!("El primer elemento del array es: {}", array[0]);

    // ================================
    // VECTOR MUTABLE
    // ================================

    // Vector dinámico con valores iniciales
    // Declarado como mutable para poder modificarlo
    let mut vector2 = vec![5, 6, 7, 8];

    // Acceso directo al primer elemento
    println!("El primer elemento del vector es: {}", vector2[0]);

    // push agrega un elemento al final del vector
    // Puede provocar una realocación en memoria
    vector2.push(9);

    // Accedemos al último elemento (índice 4)
    println!("El último elemento del vector es: {}", vector2[4]);

    // ================================
    // ACCESO SEGURO CON get
    // ================================

    // get devuelve un Option<&T>
    // Some(&valor) si existe
    // None si el índice está fuera de rango
    let medio: Option<&i32> = vector2.get(2);

    // match nos obliga a manejar todos los casos posibles
    match medio {
        // Si existe el valor, lo imprimimos
        Some(valor) => println!("El elemento del medio es: {}", valor),

        // Si no existe, evitamos un panic
        None => println!("El elemento está fuera de rango"),
    }

    // ================================
    // ENUM + VECTOR
    // ================================

    // Vector que contiene distintos tipos de datos
    // gracias al enum Familia
    let familia = vec![
        Familia::Nene(6),
        Familia::Nena(8),
        Familia::Hombre(36),
        Familia::Mujer(35),
    ];

    println!("La familia tiene:");

    // Recorremos el vector usando referencias
    // para no mover los valores
    for persona in &familia {
        // Debug imprime el enum completo
        println!("{:?}", persona);
    }

    // ================================
    // MATCH SOBRE ENUM
    // ================================

    println!("Detalle de la familia:");

    // match nos permite ejecutar lógica distinta
    // según la variante del enum
    for persona in &familia {
        match persona {
            Familia::Nene(edad) => {
                println!("Nene de {} años", edad);
            }
            Familia::Nena(edad) => {
                println!("Nena de {} años", edad);
            }
            Familia::Hombre(edad) => {
                println!("Hombre de {} años", edad);
            }
            Familia::Mujer(edad) => {
                println!("Mujer de {} años", edad);
            }
        }
    }
}
