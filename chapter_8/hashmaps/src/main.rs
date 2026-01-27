fn main() {
    // Importamos HashMap desde la librería estándar.
    // HashMap es una estructura que almacena pares clave -> valor.
    use std::collections::HashMap;

    // Creamos un HashMap vacío.
    // La mutabilidad es necesaria porque vamos a insertar elementos.
    let mut scores = HashMap::new();

    // Insertamos una clave String ("Blue") con un valor i32 (10).
    // El HashMap pasa a ser dueño (owner) tanto de la clave como del valor.
    scores.insert(String::from("Blue"), 10);

    // Insertamos otro par clave -> valor.
    // Importante: si usáramos una variable String para la clave,
    // esa variable quedaría movida (moved) al HashMap y ya no sería usable.
    scores.insert(String::from("Yellow"), 50);

    // Creamos una String con el nombre del equipo que queremos buscar.
    let team_name = String::from("Blue");

    // Usamos get para buscar el valor asociado a la clave.
    // get devuelve un Option<&i32>:
    //   - Some(&valor) si existe
    //   - None si no existe
    //
    // copied() transforma Option<&i32> en Option<i32>
    // unwrap_or(0) devuelve el valor si existe o 0 si es None
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Mostramos el puntaje del equipo.
    println!("El puntaje de equipo azul es: {score}");

    // Recorremos el HashMap por referencia (&scores)
    // para no mover sus datos.
    // key es &String y value es &i32
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Imprimimos el HashMap completo usando el formato Debug.
    println!("{scores:?}");

    // Texto de ejemplo para contar palabras.
    let text = "hello world wonderful world";

    // Creamos otro HashMap vacío.
    // En este caso:
    // clave: &str (cada palabra)
    // valor: i32 (contador)
    let mut map = HashMap::new();

    // split_whitespace divide el texto en palabras,
    // separando por espacios en blanco.
    for word in text.split_whitespace() {
        // entry(word):
        // - Si la clave existe, devuelve una referencia al valor
        // - Si no existe, la inserta con el valor dado en or_insert(0)
        //
        // count es &mut i32 (referencia mutable al contador)
        let count = map.entry(word).or_insert(0);

        // Dereferenciamos count y aumentamos el contador en 1
        *count += 1;
    }

    // Mostramos el HashMap final con el conteo de palabras
    println!("{map:?}");
    let _ = main1(); // el _ sirve para que ignorar no sea interpretado como error
}

use std::io;
use std::collections::HashMap;

fn leer_string() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) 
}
fn main1() -> Result<(), io::Error>{
    println!("ingrese una oracion: ");
    let entrada = leer_string()?;
    let mut map = HashMap::new();
    for word in entrada.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
    Ok(())
}