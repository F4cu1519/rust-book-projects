use std::env;
use std::process;

// Importamos Config y run desde nuestro crate de biblioteca (lib.rs).
// Al separar la lógica en lib.rs, main.rs queda como un punto de entrada
// liviano que solo orquesta las piezas, sin contener lógica real.
use minigrep2::Config;
use minigrep2::run;

fn main() {
    // env::args() devuelve un iterador sobre los argumentos de línea de
    // comandos. collect() lo convierte en Vec<String>.
    // El índice 0 siempre es el nombre del binario (ej: "minigrep"),
    // los argumentos del usuario empiezan en el índice 1.
    let args: Vec<String> = env::args().collect();

    // Config::build() devuelve un Result. Usamos unwrap_or_else() para
    // manejar el caso de error sin usar panic!.
    //
    // unwrap_or_else recibe un cierre (closure): una función anónima
    // definida con |err| { ... } que solo se ejecuta si el Result es Err.
    // El valor dentro del Err (nuestro mensaje de texto) llega al cierre
    // como el argumento `err`.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Mensaje claro y amigable para el usuario final,
        // sin el ruido técnico que generaría un panic!
        eprintln!("Problem parsing arguments: {err}");

        // Salimos con código 1, que por convención indica error.
        // Es más limpio que panic!, que imprime stack traces y mensajes
        // internos que confunden a los usuarios.
        process::exit(1);
    });

    // Usamos if let en lugar de unwrap_or_else porque run() devuelve
    // Ok(()) en el caso exitoso: no hay ningún valor útil que extraer,
    // solo nos importa detectar si hubo un error.
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

//    
//    El flujo completo del programa es:
//    ```
//    main()
//      │
//      ├── Lee args de línea de comandos
//      ├── Config::build(&args)  →  lee también IGNORE_CASE del entorno
//      │       └── Err  →  imprime error y process::exit(1)
//      │
//      └── run(config)
//              ├── Lee el archivo con fs::read_to_string
//              ├── Elige search() o search_case_insensitive() según ignore_case
//              ├── Imprime cada línea que coincide
//              └── Err  →  main imprime error y process::exit(1)
//  
//    