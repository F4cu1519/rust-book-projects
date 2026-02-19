use std::env;
use std::fs;
// Ten en cuenta que `std::env::args` entrará en pánico si algún argumento contiene 
// Unicode inválido. Si tu programa necesita aceptar argumentos con Unicode inválido, 
// usa `std::env::args_os` en su lugar. Esa función devuelve un iterador que produce valores 
// `OsString` en lugar de `String`. Hemos elegido usar `std::env::args` aquí por simplicidad, 
// ya que los valores `OsString` difieren según la plataforma y son más complejos de manejar 
// que los `String`.


fn main() {
    // para que un argumento sea tomado como tal tiene que ser escrito como --
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Primero traemos al ámbito una parte relevante de la biblioteca estándar con una sentencia `use`: 
    // necesitamos `std::fs` para manejar archivos.
    // En `main`, la nueva sentencia `fs::read_to_string` toma `file_path`, abre ese archivo y devuelve 
    // un valor de tipo `std::io::Result<String>` que contiene el contenido del archivo.

    println!("With text:\n{contents}");
}