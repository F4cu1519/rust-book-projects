use std::io;
use std::fs::File;
use std::io::Read;
use std::error::Error;

// ------------------------------------------------------------
// Función que:
// - Muestra un mensaje
// - Lee una línea desde stdin
// - Devuelve un String o un error
// ------------------------------------------------------------
fn leer_input(mensaje: &str) -> Result<String, io::Error> {
    println!("{mensaje}");

    let mut input = String::new();

    // read_line puede fallar → por eso usamos ?
    io::stdin().read_line(&mut input)?;

    // trim elimina el \n final
    Ok(input.trim().to_string())
}

// ------------------------------------------------------------
// main devuelve Result para poder usar el operador ?
// ------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {

    // 1️⃣ Leemos el nombre del archivo
    // usar ? convierte Result<String, io::Error> → String
    let nombre_archivo = leer_input("Ingrese el nombre del archivo")?;

    // 2️⃣ Abrimos el archivo (puede fallar)
    let mut archivo = File::open(&nombre_archivo)?;

    // 3️⃣ Creamos un String donde guardar el contenido
    let mut contenido = String::new();

    // 4️⃣ Leemos todo el archivo dentro del String
    archivo.read_to_string(&mut contenido)?;

    // 5️⃣ Mostramos el contenido
    println!("\nContenido del archivo:\n{contenido}");

    Ok(())
}
