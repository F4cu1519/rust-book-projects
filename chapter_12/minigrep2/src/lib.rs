use std::error::Error;
use std::fs;
use std::env;

// Config agrupa toda la configuración del programa en un solo lugar.
// Al tener un struct dedicado, queda claro qué datos son "configuración"
// y qué datos son parte de la lógica del programa.
// Los campos son pub porque main.rs necesita acceder a ellos.
pub struct Config {
    pub query: String,
    pub file_path: String,
    // Controla si la búsqueda ignora mayúsculas/minúsculas.
    // Se activa mediante la variable de entorno IGNORE_CASE.
    pub ignore_case: bool,
}

impl Config {
    // Usamos build() en lugar de new() porque en Rust la convención es que
    // new() nunca falla. build() puede fallar y lo señala devolviendo Result.
    //
    // Recibe un slice de Strings (los argumentos del programa) y devuelve:
    //   - Ok(Config) si hay suficientes argumentos
    //   - Err(&'static str) con un mensaje de error si faltan argumentos
    //
    // &'static str significa que el string de error está hardcodeado en
    // el binario y vive durante toda la ejecución del programa.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Verificamos antes de acceder a los índices para evitar un pánico
        // confuso de "index out of bounds". Así damos un mensaje claro.
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Usamos clone() para crear Strings propios dentro de Config.
        // No podemos usar referencias porque args pertenece a main() y
        // Config podría necesitar vivir más que esa función.
        // En el Cap. 13 veremos formas más eficientes usando iteradores.
        let query = args[1].clone();
        let file_path = args[2].clone();

        // env::var() devuelve Ok(valor) si la variable está definida,
        // o Err si no lo está. Con is_ok() solo nos importa si existe,
        // no su valor concreto. Cualquier valor activa el modo insensible.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// run() contiene toda la lógica principal del programa, separada de main()
// para que pueda ser testeada de forma independiente y para que main()
// quede limpio y con una sola responsabilidad.
//
// Devuelve Result<(), Box<dyn Error>>:
//   - () en el caso exitoso (no hay valor útil que devolver, solo "éxito")
//   - Box<dyn Error> en caso de error: un puntero a CUALQUIER tipo que
//     implemente el trait Error. "dyn" significa que el tipo exacto se
//     resuelve en tiempo de ejecución, dándonos flexibilidad para devolver
//     distintos tipos de error según la situación.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // El operador ? reemplaza al .expect() que usábamos antes.
    // Si read_to_string falla, en lugar de entrar en pánico, ? devuelve
    // el error automáticamente al llamador (main) para que lo maneje.
    // Es azúcar sintáctica para:
    //   match resultado { Ok(v) => v, Err(e) => return Err(e) }
    let contents = fs::read_to_string(config.file_path)?;

    // Elegimos qué función de búsqueda usar según la configuración.
    // La decisión se toma aquí una sola vez; el resto del código
    // simplemente usa `results` sin saber cómo se obtuvo.
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // Ok(()) indica éxito. El () es el tipo unitario: no tenemos ningún
    // valor útil que devolver, solo queremos señalar que todo salió bien.
    Ok(())
}

// La anotación de lifetime 'a le dice al compilador que las referencias
// en el Vec devuelto viven tanto como el parámetro `contents`.
// Es decir: devolvemos slices del string original, no copias nuevas.
// Rust necesita esto para garantizar que esas referencias nunca apunten
// a memoria que ya fue liberada.
//
// Si no anotáramos 'a, Rust no sabría si los slices vienen de `query`
// o de `contents`, y no podría verificar la seguridad de memoria.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // lines() devuelve un iterador sobre cada línea del string.
    // contains() verifica si la línea contiene la cadena de búsqueda.
    for line in contents.lines() {
        if line.contains(query) {
            // Guardamos la línea original (es un slice de `contents`)
            results.push(line);
        }
    }

    results
}

// Misma lógica que search(), pero convierte a minúsculas antes de comparar
// para ignorar la diferencia entre mayúsculas y minúsculas.
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // to_lowercase() devuelve un String NUEVO (no un slice) porque necesita
    // asignar memoria para los caracteres convertidos. Por eso `query` aquí
    // es un String, no un &str. Al pasarlo a contains() añadimos & para
    // convertirlo en un slice de nuevo, que es lo que contains() espera.
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // Convertimos la línea a minúsculas SOLO para la comparación.
        // La guardamos en results con sus mayúsculas originales intactas,
        // porque queremos mostrar el texto tal como aparece en el archivo.
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// El módulo de tests solo se compila cuando ejecutamos `cargo test`,
// gracias al atributo #[cfg(test)]. Esto evita incluir código de test
// en el binario final.
#[cfg(test)]
mod tests {
    // Traemos al ámbito todo lo definido en el módulo padre (lib.rs)
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // "Duct tape." tiene D mayúscula: NO debe aparecer en los resultados
        // de una búsqueda sensible a mayúsculas. Esto confirma que search()
        // distingue correctamente entre "duct" y "Duct".
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        // La consulta tiene mayúsculas y minúsculas mezcladas a propósito
        // para verificar que la función las ignora correctamente.
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        // Debe encontrar "Rust:" (empieza con mayúscula) y
        // "Trust me." (contiene "rust" dentro de "Trust")
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}