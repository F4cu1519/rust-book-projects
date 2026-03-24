 // =============================================================================
// CAPÍTULO 19: PATRONES Y COINCIDENCIA (Patterns and Matching)
// =============================================================================
//
// Los patrones son una sintaxis especial en Rust para hacer coincidir la
// estructura de tipos, tanto complejos como simples. Un patrón puede consistir
// en combinaciones de:
//
//   - Literales
//   - Arrays, enums, structs o tuplas desestructurados
//   - Variables
//   - Comodines (wildcards)
//   - Marcadores de posición (placeholders)
//
// Ejemplos de patrones válidos: `x`, `(a, 3)`, `Some(Color::Red)`
//
// Para usar un patrón, lo comparamos con un valor. Si coincide, usamos las
// partes del valor en nuestro código. Si no coincide, el código asociado
// al patrón no se ejecuta.
// =============================================================================


// =============================================================================
// SECCIÓN 19-1: TODOS LOS LUGARES DONDE SE PUEDEN USAR PATRONES
// =============================================================================


// -----------------------------------------------------------------------------
// 1. BRAZOS DE `match`
// -----------------------------------------------------------------------------
//
// Las expresiones `match` deben ser EXHAUSTIVAS: todas las posibilidades del
// valor deben estar contempladas. El patrón `_` coincide con cualquier cosa
// pero nunca se enlaza a una variable; se usa como captura total (catch-all).
//
//   match VALUE {
//       PATTERN => EXPRESSION,
//       PATTERN => EXPRESSION,
//       _       => EXPRESSION,  // catch-all
//   }

fn ejemplo_match(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// -----------------------------------------------------------------------------
// 2. SENTENCIAS `let`
// -----------------------------------------------------------------------------
//
// Cada vez que escribís `let x = 5;` estás usando un patrón. Formalmente:
//
//   let PATTERN = EXPRESSION;
//
// El nombre de variable es la forma más simple de patrón: significa
// "enlaza lo que coincida aquí a esta variable, sea cual sea el valor".
//
// También podemos desestructurar tuplas con `let`:

fn ejemplo_let() {
    // Patrón simple
    let x = 5;
    println!("x = {x}");

    // Desestructuración de tupla (Listado 19-1)
    let (a, b, c) = (1, 2, 3);
    println!("a={a}, b={b}, c={c}");

    // Si el número de elementos no coincide → error de compilación:
    // let (x, y) = (1, 2, 3); // ERROR: mismatched types
}


// -----------------------------------------------------------------------------
// 3. EXPRESIONES CONDICIONALES `if let`
// -----------------------------------------------------------------------------
//
// `if let` es una forma más corta de un `match` que maneja un solo caso.
// Se puede combinar con `else if` y `else if let` para mayor flexibilidad.
//
// ATENCIÓN: el compilador NO verifica exhaustividad en `if let`,
// a diferencia de `match`. Si omitís el `else` final y dejás casos sin
// manejar, el compilador NO te avisará del posible bug lógico.
//
// También puede introducir nuevas variables que ocultan (shadow) las
// existentes, igual que los brazos de `match`.

fn ejemplo_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Usando tu color favorito, {color}, como fondo");
    } else if is_tuesday {
        println!("¡El martes es día verde!");
    } else if let Ok(age) = age {
        // `age` aquí es una NUEVA variable que oculta la anterior.
        // Por eso `if age > 30` debe ir dentro de este bloque:
        // no podemos escribir `if let Ok(age) = age && age > 30`.
        if age > 30 {
            println!("Usando púrpura como color de fondo");
        } else {
            println!("Usando naranja como color de fondo");
        }
    } else {
        println!("Usando azul como color de fondo");
    }
}


// -----------------------------------------------------------------------------
// 4. BUCLES CONDICIONALES `while let`
// -----------------------------------------------------------------------------
//
// Similar a `if let`, `while let` ejecuta el bucle mientras el patrón
// siga coincidiendo. Muy útil con canales (channels) entre hilos:
// `recv()` retorna Ok(value) mientras lleguen mensajes, y Err cuando
// el emisor se desconecta.

fn ejemplo_while_let() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    // El bucle se detiene automáticamente cuando el emisor se desconecta
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
}


// -----------------------------------------------------------------------------
// 5. BUCLES `for`
// -----------------------------------------------------------------------------
//
// En `for x in y`, el `x` es un patrón. Podemos desestructurar tuplas
// directamente en el `for`, por ejemplo al usar `.enumerate()`.

fn ejemplo_for() {
    let v = vec!['a', 'b', 'c'];

    // `(index, value)` es un patrón que desestructura la tupla
    // producida por `.enumerate()`
    for (index, value) in v.iter().enumerate() {
        println!("{value} está en el índice {index}");
    }
}


// -----------------------------------------------------------------------------
// 6. PARÁMETROS DE FUNCIÓN
// -----------------------------------------------------------------------------
//
// Los parámetros de función también son patrones. En `fn foo(x: i32)`,
// el `x` es un patrón. Podemos desestructurar tuplas directamente
// en la firma de la función.

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Ubicación actual: ({x}, {y})");
}

// También funciona con closures, ya que son similares a las funciones.


// =============================================================================
// NOTA FINAL: PATRONES REFUTABLES E IRREFUTABLES
// =============================================================================
//
// Los patrones no se comportan igual en todos los contextos:
//
//   - IRREFUTABLES: deben coincidir siempre. Se usan en `let`, parámetros
//     de función y bucles `for`. Ejemplo: `let x = 5;` — `x` siempre coincide.
//
//   - REFUTABLES: pueden NO coincidir con algún valor posible. Se usan en
//     `if let`, `while let` y brazos de `match`.
//     Ejemplo: `if let Some(x) = valor` — podría ser `None` y no coincidir.
//
// El compilador de Rust te avisará si usás un patrón refutable donde se
// espera uno irrefutable, o viceversa.
// =============================================================================


fn main() {
    println!("=== match ===");
    println!("{:?}", ejemplo_match(Some(41)));
    println!("{:?}", ejemplo_match(None));

    println!("\n=== let ===");
    ejemplo_let();

    println!("\n=== if let ===");
    ejemplo_if_let();

    println!("\n=== while let ===");
    ejemplo_while_let();

    println!("\n=== for ===");
    ejemplo_for();

    println!("\n=== parámetros de función ===");
    let point = (3, 5);
    print_coordinates(&point);
}