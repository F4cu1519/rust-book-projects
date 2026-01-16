use std::io;

fn leer_string() -> Result<String, io::Error> {
    // Usa Result para manejar errores sin panic

    // Creamos un String vacío que va a ser el buffer de entrada
    // Este String es el dueño de la memoria 
    let mut input = String::new();

    // Leemos desde stdin y escribimos dentro de `input`
    // read_line agrega un '\n' al final
    // El operador `?` propaga el error si falla
    io::stdin().read_line(&mut input)?;

    // input.trim():
    // - devuelve un &str (slice)
    // - NO es dueño de la memoria
    // - solo apunta a `input`
    //
    // to_string():
    // - crea un NUEVO String en el heap
    // - copia el texto sin espacios ni '\n'
    // - ahora el caller es dueño del String
    //
    // Esto es obligatorio porque `input` muere al salir de la función
    Ok(input.trim().to_string()) 
}


fn leer_i32() -> Result<i32, Box<dyn std::error::Error>> {

    let input = leer_string()?;
    let numero: i32 = input.parse()?;
    Ok(numero)
}

// Usamos Box<dyn std::error::Error> por que read_line devuelve io::Error 
// parse devuelve ParseIntError necesitamos un solo tipo de error Box<dyn Error> unifica ambos.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ingrese un numero entero: ");
    let n = leer_i32()?;
    println!("Número ingresado: {}", n);
    println!("doble {}", n*2);
    Ok(())
}
