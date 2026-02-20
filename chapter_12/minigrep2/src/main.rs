// Traemos el módulo env de la biblioteca estándar para leer los argumentos
// de línea de comandos
use std::env;

// Traemos process para poder salir del programa manualmente con un código
// de error, en lugar de depender de panic!
use std::process;

// Traemos la función search y el struct Config desde nuestro propio crate
// de biblioteca (src/lib.rs). Al separar el código en lib.rs, main.rs
// necesita importar explícitamente lo que usa.
use minigrep::Config;
use minigrep::run;

fn main() {
    // Recopilamos todos los argumentos de línea de comandos en un vector.
    // env::args() devuelve un iterador, y collect() lo convierte en Vec<String>.
    // El primer elemento (índice 0) siempre es el nombre del binario.
    let args: Vec<String> = env::args().collect();

    // Config::build reemplaza a Config::new porque la convención en Rust es
    // que new() nunca falla. build() en cambio devuelve un Result, lo que
    // nos permite manejar el error de forma controlada.
    //
    // unwrap_or_else nos permite definir qué hacer si el Result es Err,
    // sin usar panic!. Recibe un cierre (closure) que se ejecuta solo
    // si hay error. El valor interno del Err (nuestro mensaje de texto)
    // llega al cierre como el argumento |err|.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Imprimimos el error con un mensaje claro para el usuario final
        println!("Problem parsing arguments: {err}");

        // Salimos del programa con código 1, que por convención indica
        // que el proceso terminó con un error. Esto es más limpio que
        // panic!, que imprime información técnica innecesaria para el usuario.
        process::exit(1);
    });

    // Usamos if let para manejar solo el caso Err del Result que devuelve run().
    // No usamos unwrap_or_else aquí porque run() devuelve Ok(()) en el caso
    // exitoso, es decir, no hay ningún valor útil que extraer del Ok,
    // solo nos importa saber si hubo un error.
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}