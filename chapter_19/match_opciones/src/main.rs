// ============================================================
//  SINTAXIS DE PATRONES EN RUST - Guía completa con ejemplos
// ============================================================

fn main() {
    matching_literales();
    matching_variables_nombradas();
    matching_multiples_patrones();
    matching_rangos();
    destructuring_structs();
    destructuring_enums();
    destructuring_anidado();
    ignorar_valores();
    match_guards();
    at_bindings();
}


// ============================================================
// 1. MATCHING DE LITERALES
//    Lo más simple: comparar contra valores concretos.
// ============================================================
fn matching_literales() {
    println!("\n--- 1. Matching de literales ---");

    let x = 2;

    match x {
        1 => println!("uno"),
        2 => println!("dos"),
        3 => println!("tres"),
        _ => println!("cualquier otro"), // _ es el "default", atrapa todo lo demás
    }
}


// ============================================================
// 2. VARIABLES NOMBRADAS Y SHADOWING
//
//    ⚠️ TRAMPA COMÚN: dentro de un match, si usás el nombre
//    de una variable como patrón, Rust crea una NUEVA variable
//    que tapa (sombrea) a la de afuera. No compara con ella.
// ============================================================
fn matching_variables_nombradas() {
    println!("\n--- 2. Variables nombradas y shadowing ---");

    let x = Some(5);
    let y = 10; // <-- esta y externa

    match x {
        Some(50) => println!("Es 50"),

        // ⚠️ Este 'y' NO es el y = 10 de arriba.
        // Rust crea una nueva variable 'y' que matchea CUALQUIER
        // valor dentro del Some. Entonces captura el 5.
        Some(y) => println!("y interno vale: {y}"), // imprime 5, no 10

        _ => println!("default"),
    }

    // Acá sí es el y original
    println!("y externo sigue siendo: {y}"); // imprime 10
}


// ============================================================
// 3. MÚLTIPLES PATRONES CON |
//    El operador | funciona como OR entre patrones.
// ============================================================
fn matching_multiples_patrones() {
    println!("\n--- 3. Múltiples patrones con | ---");

    let x = 2;

    match x {
        1 | 2 => println!("uno o dos"),   // si x es 1 O 2
        3 | 4 => println!("tres o cuatro"),
        _     => println!("otra cosa"),
    }
}


// ============================================================
// 4. RANGOS CON ..=
//    Permite matchear un rango inclusivo de valores.
//    Solo funciona con números y char.
// ============================================================
fn matching_rangos() {
    println!("\n--- 4. Rangos con ..= ---");

    let numero = 7;

    match numero {
        1..=5   => println!("entre 1 y 5"),
        6..=10  => println!("entre 6 y 10"),  // esto matchea
        11..=20 => println!("entre 11 y 20"),
        _       => println!("fuera de rango"),
    }

    // También funciona con char
    let letra = 'f';

    match letra {
        'a'..='m' => println!("primera mitad del abecedario"), // matchea
        'n'..='z' => println!("segunda mitad del abecedario"),
        _         => println!("no es letra minúscula"),
    }
}


// ============================================================
// 5. DESTRUCTURING DE STRUCTS
//    Permite extraer los campos de un struct directamente
//    en variables con nombre.
// ============================================================
fn destructuring_structs() {
    println!("\n--- 5. Destructuring de structs ---");

    struct Point { x: i32, y: i32 }

    let p = Point { x: 3, y: 7 };

    // Forma larga: renombrar campos a otras variables
    let Point { x: a, y: b } = p;
    println!("a={a}, b={b}"); // a=3, b=7

    let p2 = Point { x: 0, y: 9 };

    // Forma corta (shorthand): las variables toman el mismo nombre que los campos
    let Point { x, y } = p2;
    println!("x={x}, y={y}"); // x=0, y=9

    // Destructuring en match: mezclar literales y variables
    // Podés fijar algunos campos a valores concretos y capturar otros
    let p3 = Point { x: 0, y: 5 };

    match p3 {
        Point { x, y: 0 } => println!("Sobre el eje X en x={x}"),
        // ↑ solo matchea si y == 0, captura x

        Point { x: 0, y } => println!("Sobre el eje Y en y={y}"), // matchea
        // ↑ solo matchea si x == 0, captura y

        Point { x, y } => println!("En ningún eje: ({x}, {y})"),
    }
}


// ============================================================
// 6. DESTRUCTURING DE ENUMS
//    Cada variante del enum puede tener una forma distinta
//    de destructurarse, según cómo fue definida.
// ============================================================
fn destructuring_enums() {
    println!("\n--- 6. Destructuring de enums ---");

    enum Mensaje {
        Salir,                      // sin datos
        Mover { x: i32, y: i32 },  // struct-like
        Escribir(String),           // tuple-like con 1 elemento
        CambiarColor(u8, u8, u8),   // tuple-like con 3 elementos
    }

    let msg = Mensaje::CambiarColor(255, 128, 0);

    match msg {
        Mensaje::Salir => {
            // No hay datos que extraer
            println!("Salir");
        }

        Mensaje::Mover { x, y } => {
            // Destructuring de variante struct-like, igual que un struct normal
            println!("Mover a x={x}, y={y}");
        }

        Mensaje::Escribir(texto) => {
            // La variable 'texto' captura el String interno
            println!("Escribir: {texto}");
        }

        Mensaje::CambiarColor(r, g, b) => {
            // Tres variables capturan los tres u8 de la tupla interna
            println!("Color: r={r}, g={g}, b={b}"); // imprime esto
        }
    }
}


// ============================================================
// 7. DESTRUCTURING ANIDADO
//    Se puede hacer matching de estructuras dentro de otras.
//    Rust "baja" todos los niveles de anidamiento en un solo match.
// ============================================================
fn destructuring_anidado() {
    println!("\n--- 7. Destructuring anidado ---");

    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
    }

    enum Mensaje {
        CambiarColor(Color), // el enum contiene OTRO enum adentro
    }

    let msg = Mensaje::CambiarColor(Color::Hsv(180, 50, 80));

    match msg {
        // Acá hacemos matching de dos niveles a la vez:
        // 1) msg debe ser CambiarColor
        // 2) el Color adentro debe ser Rgb -> extrae r, g, b
        Mensaje::CambiarColor(Color::Rgb(r, g, b)) => {
            println!("RGB: {r}, {g}, {b}");
        }

        // 1) msg debe ser CambiarColor
        // 2) el Color adentro debe ser Hsv -> extrae h, s, v
        Mensaje::CambiarColor(Color::Hsv(h, s, v)) => {
            println!("HSV: {h}, {s}, {v}"); // imprime esto
        }
    }

    // También se pueden mezclar structs y tuplas anidadas
    struct Punto { x: i32, y: i32 }
    let ((pies, pulgadas), Punto { x, y }) = ((5, 11), Punto { x: 3, y: -7 });
    println!("Altura: {pies}'{pulgadas}\", punto: ({x}, {y})");
}


// ============================================================
// 8. IGNORAR VALORES
//    Hay tres formas distintas según qué tan "ignorado" lo querés.
// ============================================================
fn ignorar_valores() {
    println!("\n--- 8. Ignorar valores ---");

    // --- 8a. _ : ignora completamente, NO hace binding ---
    // Útil como comodín final en match, o para ignorar parámetros.
    fn solo_segundo(_: i32, y: i32) {
        println!("Solo uso y={y}");
    }
    solo_segundo(99, 42); // el 99 se descarta totalmente

    // --- 8b. _ dentro de patrones: ignora partes específicas ---
    let numeros = (1, 2, 3, 4, 5);
    match numeros {
        (primero, _, tercero, _, quinto) => {
            println!("Primero={primero}, tercero={tercero}, quinto={quinto}");
            // El segundo y cuarto se ignoran
        }
    }

    // También útil para ignorar el valor dentro de un Some
    // cuando solo te importa *si* hay algo, no *qué* hay:
    let config = Some(String::from("dark_mode"));
    match (config, Some(String::from("light_mode"))) {
        (Some(_), Some(_)) => println!("Ambas configs existen, no sobreescribo"),
        _ => println!("Al menos una es None"),
    }

    // --- 8c. _nombre : suprime warning de "no usada", PERO sí hace binding ---
    // ⚠️ Diferencia sutil con _:
    //   - _  NO mueve el valor (no hace binding)
    //   - _x SÍ mueve el valor (hace binding, solo ignora el warning)
    let _variable_futura = 42; // no da warning aunque no la uses

    // --- 8d. .. : ignora el RESTO de campos o elementos ---

    struct Punto3D { x: i32, y: i32, z: i32 }
    let origen = Punto3D { x: 1, y: 2, z: 3 };

    match origen {
        // Solo me interesa x, ignoro y y z con ..
        Punto3D { x, .. } => println!("x={x}"),
    }

    // Con tuplas, .. puede ir al medio (pero solo UNA vez, no ambiguo)
    let datos = (10, 20, 30, 40, 50);
    match datos {
        (primero, .., ultimo) => println!("primero={primero}, ultimo={ultimo}"),
        //               ↑ ignora 20, 30, 40
    }
}


// ============================================================
// 9. MATCH GUARDS
//    Una condición if adicional en un brazo del match.
//    Se evalúa DESPUÉS de que el patrón matchea.
//    Útil para lógica que no se puede expresar solo con patrones.
// ============================================================
fn match_guards() {
    println!("\n--- 9. Match Guards ---");

    // Caso básico: filtrar pares/impares
    let num = Some(6);
    match num {
        Some(x) if x % 2 == 0 => println!("{x} es par"),   // matchea
        Some(x)               => println!("{x} es impar"),
        None                  => println!("nada"),
    }

    // Caso importante: comparar con variable EXTERNA
    // (solución al problema de shadowing del ejemplo 2)
    let x = Some(5);
    let y = 10; // quiero comparar contra este y externo

    match x {
        Some(50) => println!("Es 50"),

        // Usamos 'n' (nombre distinto) para NO shadowear y,
        // y en el guard comparamos n contra el y externo.
        // El guard tiene acceso a variables del scope exterior.
        Some(n) if n == y => println!("n={n} es igual al y externo={y}"),

        _ => println!("default: x={x:?}"), // imprime esto porque 5 != 10
    }

    // Precedencia de | con guards:
    // El guard se aplica a TODOS los patrones del |, no solo al último
    let x = 4;
    let activo = true;
    match x {
        // Esto es: (1 | 2 | 3 | 4) if activo
        // NO es:   1 | 2 | 3 | (4 if activo)
        1 | 2 | 3 | 4 if activo => println!("x={x} está en 1..4 y activo=true"), // matchea
        _                       => println!("no"),
    }
}


// ============================================================
// 10. AT BINDINGS CON @
//
//     @ resuelve una limitación específica: cuando hacés matching
//     con un rango o patrón complejo, normalmente NO tenés acceso
//     al valor exacto que matcheó. Con @ podés CAPTURARLO y
//     VERIFICARLO al mismo tiempo.
//
//     Sintaxis: variable @ patrón
//
//     Se lee: "capturá el valor en 'variable', PERO solo si cumple 'patrón'"
// ============================================================
fn at_bindings() {
    println!("\n--- 10. @ Bindings ---");

    enum Mensaje {
        Hola { id: i32 },
    }

    let msg = Mensaje::Hola { id: 5 };

    match msg {
        // Sin @, con rango:
        //   Mensaje::Hola { id: 3..=7 } => ...
        //   ✅ verifica que id esté en 3..=7
        //   ❌ pero NO podés usar el valor de id adentro del brazo

        // Sin @, con variable:
        //   Mensaje::Hola { id } => ...
        //   ✅ capturás el valor de id
        //   ❌ pero no verificás nada, matchea CUALQUIER id

        // Con @: lo mejor de los dos mundos
        //   id @ 3..=7  significa:
        //   1. Verificá que el campo id esté en el rango 3..=7
        //   2. Si lo está, guardá ese valor en una variable llamada 'id'
        //   3. Ahora podés usar 'id' en el cuerpo del brazo
        Mensaje::Hola { id: id @ 3..=7 } => {
            // Sabemos que id está en [3,7] Y tenemos el valor exacto
            println!("id en rango [3,7]: {id}"); // imprime "id en rango [3,7]: 5"
        }

        Mensaje::Hola { id: 10..=12 } => {
            // Solo verificación de rango, sin captura
            // Acá NO podemos usar 'id' porque no lo capturamos
            println!("id en rango [10,12] (no sé cuál exactamente)");
        }

        Mensaje::Hola { id } => {
            // Captura sin verificación: cualquier id que no matcheó arriba
            println!("otro id: {id}");
        }
    }

    // Ejemplo extra: @ con múltiples condiciones para ver la diferencia clara
    let numero = 15_u32;

    let descripcion = match numero {
        // Captura el número Y verifica que sea "chico"
        n @ 1..=10 => format!("chico: {n}"),

        // Captura Y verifica que sea "mediano"
        n @ 11..=20 => format!("mediano: {n}"), // matchea, n=15

        // Cualquier otro
        n => format!("grande: {n}"),
    };

    println!("{descripcion}"); // "mediano: 15"
}