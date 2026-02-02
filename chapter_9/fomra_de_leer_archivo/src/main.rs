use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() {
    // ============================================================
    // FORMA 1: match explícito (la más larga, pero la más clara)
    // ============================================================

    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(f) => {
            println!("Archivo abierto correctamente (FORMA 1)");
            f
        }
        Err(e) => {
            println!("Error al abrir el archivo (FORMA 1): {e}");
            return; // salimos del programa
        }
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => println!("Contenido leído:\n{contents}"),
        Err(e) => {
            println!("Error al leer el archivo: {e}");
            return;
        }
    };

    // ============================================================
    // FORMA 2: if let (más corta, menos detallada)
    // ============================================================

    let mut file = if let Ok(f) = File::open("hello.txt") {
        println!("Archivo abierto correctamente (FORMA 2)");
        f
    } else {
        println!("No se pudo abrir el archivo (FORMA 2)");
        return;
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Error leyendo el archivo: {e}");
        return;
    }

    // ============================================================
    // FORMA 3: unwrap (NO RECOMENDADA en producción)
    // ============================================================

    // unwrap asume que TODO va a salir bien
    // si algo falla, el programa hace panic! y se cae
    let mut file = File::open("hello.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Contenido con unwrap:\n{contents}");

    // ============================================================
    // FORMA 4: expect (unwrap con mensaje personalizado)
    // ============================================================

    let mut file = File::open("hello.txt")
        .expect("No se pudo abrir hello.txt");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("No se pudo leer el contenido del archivo");

    println!("Contenido con expect:\n{contents}");

    // ============================================================
    // FORMA 5: usando ? dentro de una función que devuelve Result
    // ============================================================

    match leer_archivo_con_result() {
        Ok(texto) => println!("Contenido con ?: \n{texto}"),
        Err(e) => println!("Error usando ?: {e}"),
    }

    // ============================================================
    // FORMA 6: la más corta y común (fs::read_to_string)
    // ============================================================

    match fs::read_to_string("hello.txt") {
        Ok(texto) => println!("Contenido con fs::read_to_string:\n{texto}"),
        Err(e) => println!("Error leyendo archivo directamente: {e}"),
    }
}

// ============================================================
// FUNCIÓN AUXILIAR USANDO ?
// ============================================================

// Esta función:
// - Devuelve Result
// - Usa ? para propagar errores
// - Es la forma más idiomática en Rust
fn leer_archivo_con_result() -> Result<String, io::Error> {
    let mut contenido = String::new();

    // Si falla File::open → devuelve Err automáticamente
    File::open("hello.txt")?
        .read_to_string(&mut contenido)?;

    Ok(contenido)
}

// ============================================================
// VARIANTE: main devolviendo Result (muy usada en proyectos reales)
// ============================================================

#[allow(dead_code)]
fn main_con_result() -> Result<(), Box<dyn Error>> {
    let contenido = fs::read_to_string("hello.txt")?;
    println!("{contenido}");
    Ok(())
}
