// Traemos el trait Error de la biblioteca estándar, necesario para usar
// Box<dyn Error> como tipo de retorno en run()
use std::error::Error;

// Traemos fs para leer archivos del sistema
use std::fs;

// El struct Config agrupa las variables de configuración del programa.
// Antes eran variables sueltas en main(), ahora tienen un lugar claro
// y un nombre que comunica su propósito: son la "configuración".
// Los campos son pub porque main.rs necesita acceder a ellos directamente.
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // build() es un constructor asociado a Config (se llama con Config::build).
    // Recibe un slice de Strings (los argumentos) y devuelve un Result:
    //   - Ok(Config) si todo salió bien
    //   - Err(&'static str) si faltan argumentos
    //
    // &'static str significa que el string de error vive durante todo
    // el programa (es un literal hardcodeado en el código fuente).
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Verificamos que haya suficientes argumentos antes de intentar
        // acceder a los índices 1 y 2. Sin esta verificación, Rust
        // entraría en pánico con un error confuso de "index out of bounds".
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Usamos clone() para crear copias propias de los strings.
        // No podemos simplemente tomar referencias porque args pertenece
        // a main() y Config podría vivir más que esa referencia.
        // En el Capítulo 13 veremos formas más eficientes de hacer esto.
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// run() contiene toda la lógica principal del programa.
// Separándola de main() logramos dos cosas:
//   1. main() queda limpio y fácil de leer
//   2. run() puede ser testeada de forma independiente
//
// El tipo de retorno Result<(), Box<dyn Error>> significa:
//   - En éxito devuelve () (el tipo unitario, es decir "nada útil")
//   - En error devuelve Box<dyn Error>: un puntero a CUALQUIER tipo
//     que implemente el trait Error. El "dyn" indica que el tipo exacto
//     se resuelve en tiempo de ejecución (es dinámico), lo que nos da
//     flexibilidad para devolver distintos tipos de error según el caso.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // El operador ? reemplaza al .expect() que teníamos antes.
    // Si read_to_string falla, en lugar de entrar en pánico, ? devuelve
    // el error automáticamente al llamador (main), que lo manejará.
    // Es azúcar sintáctica para: match result { Ok(v) => v, Err(e) => return Err(e) }
    let contents = fs::read_to_string(config.file_path)?;

    // Iteramos sobre las líneas que coinciden con la búsqueda e imprimimos
    // cada una. search() todavía no está implementada, pero ya definimos
    // cómo la vamos a usar.
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // Retornamos Ok(()) para indicar que todo salió bien.
    // El () dentro del Ok es el tipo unitario: no tenemos ningún valor
    // útil que devolver, solo queremos señalar "éxito".
    Ok(())
}

// La anotación de lifetime 'a indica que el vector devuelto contiene
// referencias que viven tanto como el parámetro contents.
// Es decir: las líneas que devolvemos son fragmentos del string contents,
// no copias nuevas. Rust necesita esta información para garantizar que
// esas referencias no apunten a memoria liberada.
//
// Por ahora el cuerpo usa unimplemented!(), que es un macro que entra en
// pánico con el mensaje "not implemented". Lo completaremos en el próximo
// capítulo usando tests para guiar la implementación.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!()
}