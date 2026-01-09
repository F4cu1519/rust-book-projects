use std::io;

fn main() {
    println!("Give me the sentence: ");
    let mut input = String::new();
    // Creamos un String vacío en el heap
    // Será el owner del texto que ingrese el usuario
    io::stdin().read_line(&mut input).expect("Error al leer el contenido");

    let word = first_word(&input);
    
    // Pasamos una REFERENCIA a input, no el ownership
    // &input se convierte automáticamente en &str (deref coercion)
    // first_word devuelve un &str (un slice del String input)

    println!("La primera palabra es: {} ", word);
}

fn first_word(s: &str) -> &str {
    // La función recibe un string slice
    // Puede recibir: &String, &str, o un literal
    // No toma ownership, solo presta el dato

    let bytes = s.as_bytes();
    // Convierte el string slice en un slice de bytes (&[u8])
    // Esto permite recorrer el string byte por byte

    for (i, &item) in bytes.iter().enumerate() {
        // iter() recorre los bytes
        // enumerate() agrega el índice
        // i     -> índice del byte
        // item  -> valor del byte (desreferenciado con &item)
        if item == b' ' {
            // b' ' es el byte que representa un espacio en ASCII
            // Cuando encontramos un espacio, termina la primera palabra
            return &s[0..i];
            // Devolvemos un slice del string original
            // Desde el inicio hasta el índice del espacio
            // NO copiamos datos, solo devolvemos una referencia
        }
    }

    &s[..]
    // Si no encontramos ningún espacio:
    // Devolvemos un slice del string completo
}