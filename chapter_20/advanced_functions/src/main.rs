// =============================================================================
//  ADVANCED FUNCTIONS AND CLOSURES EN RUST — Explicación + ejemplos reales
//  Basado en: doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html
// =============================================================================
//
//  Contenido:
//    1. Function Pointers (fn)
//    2. fn vs Fn — la diferencia clave
//    3. Enum variants como function pointers
//    4. Retornar closures con impl Trait
//    5. Por qué impl Trait falla con múltiples closures
//    6. Solución: Box<dyn Fn(...)>
//    7. Ejemplos reales: fábricas de funciones
//
// =============================================================================


// =============================================================================
// 1. FUNCTION POINTERS
// =============================================================================
//
// Además de pasar closures a funciones, podés pasar funciones nombradas.
// Las funciones tienen el tipo 'fn' (minúscula) llamado "function pointer".
//
// Sintaxis: fn(TipoArgumento) -> TipoRetorno

fn sumar_uno(x: i32) -> i32 {
    x + 1
}

fn multiplicar_por_dos(x: i32) -> i32 {
    x * 2
}

// Esta función recibe un function pointer como parámetro.
// 'f: fn(i32) -> i32' significa: "f es una función que toma i32 y devuelve i32"
fn aplicar_dos_veces(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg) // llama a f dos veces y suma los resultados
}


// =============================================================================
// 2. fn (TIPO) vs Fn (TRAIT) — LA DIFERENCIA CLAVE
// =============================================================================
//
// fn  → tipo concreto. Solo acepta funciones nombradas.
// Fn  → trait. Acepta funciones nombradas Y closures.
//
// Los function pointers (fn) implementan Fn, FnMut y FnOnce.
// Eso significa que podés pasar fn donde se espera Fn, pero NO al revés.
//
// REGLA PRÁCTICA:
//   Casi siempre usá 'impl Fn(...)' porque acepta todo.
//   Solo usá 'fn' cuando hablás con código C (que no tiene closures).

// Con impl Fn → acepta tanto funciones nombradas como closures
fn aplicar_impl(f: impl Fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}

// Con fn → solo acepta funciones nombradas, NO closures que capturen variables
fn aplicar_fn(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}


// =============================================================================
// 3. ENUM VARIANTS COMO FUNCTION POINTERS
// =============================================================================
//
// Cuando definís un enum con datos, el nombre de cada variante
// también funciona como una función: fn(T) -> Enum
//
// Estado::Valor actúa como fn(u32) -> Estado
// Podés pasarlo directo a .map() sin necesidad de escribir una closure.

#[derive(Debug)]
enum Estado {
    Activo(u32),
    Inactivo,
}


// =============================================================================
// 4. RETORNAR CLOSURES CON impl Trait
// =============================================================================
//
// Una closure no tiene un tipo concreto que puedas nombrar.
// Para retornarla, usás 'impl Fn(...)'.
//
// Leé la firma así:
//   fn nombre() -> impl Fn(i32) -> i32
//   "nombre es una función que devuelve algo que se puede llamar con i32"
//
// El cuerpo simplemente devuelve la closure:
//   { |x| x + 1 }

fn crear_sumador_fijo() -> impl Fn(i32) -> i32 {
    |x| x + 1 // fabrica y devuelve esta closure
}

// Con 'move', la closure captura variables del entorno y las lleva adentro.
// Cada llamada a 'sumador(n)' fabrica una closure con su propio 'n' capturado.
fn sumador(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // 'n' queda capturado dentro de la closure
}


// =============================================================================
// 5. POR QUÉ impl Trait FALLA CON MÚLTIPLES CLOSURES
// =============================================================================
//
// Cada función que devuelve 'impl Fn(i32) -> i32' crea un "opaque type":
// un tipo único e interno que el compilador conoce pero vos no podés nombrar.
//
// Aunque dos funciones tengan la misma firma 'impl Fn(i32) -> i32',
// sus opaque types son DISTINTOS para el compilador.
//
// Por eso NO podés meter ambas en un Vec:
//
//   fn a() -> impl Fn(i32) -> i32 { |x| x + 1 }
//   fn b() -> impl Fn(i32) -> i32 { move |x| x + 10 }
//   let v = vec![a(), b()]; // ERROR: tipos distintos aunque la firma sea igual
//
// El compilador lo rechaza con:
//   "expected opaque type, found a different opaque type"


// =============================================================================
// 6. SOLUCIÓN: Box<dyn Fn(...)>
// =============================================================================
//
// Cuando necesitás mezclar distintas closures (en un Vec, o retornarlas
// de distintas funciones), usás un trait object: Box<dyn Fn(...)>
//
// Box<dyn Fn(i32) -> i32>:
//   - Box   → la closure vive en el heap
//   - dyn   → el tipo concreto se resuelve en runtime (dispatch dinámico)
//   - Fn... → el trait que tiene que implementar
//
// Todas las Box<dyn Fn(i32) -> i32> tienen el MISMO tipo para el compilador,
// aunque adentro haya closures distintas. Por eso funcionan en un Vec.

fn closure_suma(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}

fn closure_multiplica(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * n)
}


// =============================================================================
// 7. EJEMPLOS REALES: FÁBRICAS DE FUNCIONES
// =============================================================================
//
// El patrón "función que devuelve función" sirve para crear comportamiento
// reutilizable y configurable sin repetir código.

// EJEMPLO A: Filtros configurables
// En vez de hacer fn mayor_de_18, fn mayor_de_21, fn mayor_de_65...
// hacés una sola fábrica:
fn mayor_que(minimo: i32) -> impl Fn(i32) -> bool {
    move |valor| valor > minimo
}

// EJEMPLO B: Validadores de texto
// Cada validador es una closure con su propia configuración capturada.
fn longitud_minima(min: usize) -> impl Fn(&str) -> bool {
    move |texto| texto.len() >= min
}

fn debe_contener(caracter: char) -> impl Fn(&str) -> bool {
    move |texto| texto.contains(caracter)
}

// EJEMPLO C: Transformadores de pipeline
// Podés componer transformaciones aplicándolas en secuencia.
fn con_prefijo(prefijo: &'static str) -> impl Fn(&str) -> String {
    move |texto| format!("{}{}", prefijo, texto)
}

fn con_sufijo(sufijo: &'static str) -> impl Fn(&str) -> String {
    move |texto| format!("{}{}", texto, sufijo)
}

// EJEMPLO D: Vec de operaciones (necesita Box<dyn Fn>)
// Cuando querés guardar varias closures distintas juntas.
fn construir_pipeline(operaciones: Vec<Box<dyn Fn(i32) -> i32>>) -> impl Fn(i32) -> i32 {
    // Devuelve una closure que aplica todas las operaciones en secuencia
    move |mut valor| {
        for op in &operaciones {
            valor = op(valor);
        }
        valor
    }
}


// =============================================================================
// MAIN: demos de todo lo anterior
// =============================================================================

fn main() {

    // --- 1. FUNCTION POINTERS ---
    println!("=== 1. FUNCTION POINTERS ===");

    // Pasamos 'sumar_uno' (función nombrada) como argumento
    let resultado = aplicar_dos_veces(sumar_uno, 5);
    println!("sumar_uno aplicado dos veces a 5: {}", resultado); // 12

    let resultado = aplicar_dos_veces(multiplicar_por_dos, 3);
    println!("multiplicar_por_dos aplicado dos veces a 3: {}", resultado); // 12


    // --- 2. fn vs Fn ---
    println!("\n=== 2. fn vs Fn ===");

    // impl Fn acepta funciones nombradas:
    println!("impl Fn con función nombrada: {}", aplicar_impl(sumar_uno, 10)); // 11

    // impl Fn también acepta closures (con o sin captura):
    let extra = 100;
    println!("impl Fn con closure: {}", aplicar_impl(|x| x + extra, 10)); // 110

    // fn solo acepta funciones nombradas:
    println!("fn con función nombrada: {}", aplicar_fn(sumar_uno, 10)); // 11

    // Esto NO compilaría porque la closure captura 'extra':
    // aplicar_fn(|x| x + extra, 10); // ERROR


    // --- 3. ENUM VARIANTS COMO FUNCTION POINTERS ---
    println!("\n=== 3. ENUM VARIANTS ===");

    // Estado::Activo funciona como fn(u32) -> Estado
    // Las dos líneas son equivalentes:
    let con_closure: Vec<Estado> = (0u32..4).map(|n| Estado::Activo(n)).collect();
    let con_variant: Vec<Estado> = (0u32..4).map(Estado::Activo).collect();

    println!("Con closure:  {:?}", con_closure);
    println!("Con variant:  {:?}", con_variant);
    // Mismo resultado, mismo código compilado. Elegí el que sea más claro.


    // --- 4. RETORNAR CLOSURES ---
    println!("\n=== 4. RETORNAR CLOSURES ===");

    let f = crear_sumador_fijo(); // f es la closure |x| x + 1
    println!("f(10) = {}", f(10)); // 11
    println!("f(20) = {}", f(20)); // 21

    // Cada llamada a sumador() fabrica una closure independiente
    let sumar_5  = sumador(5);
    let sumar_50 = sumador(50);

    println!("sumar_5(10)  = {}", sumar_5(10));  // 15
    println!("sumar_50(10) = {}", sumar_50(10)); // 60
    // Cada una tiene su propio 'n' capturado, son independientes


    // --- 5 y 6. Box<dyn Fn> ---
    println!("\n=== 5 y 6. Box<dyn Fn> ===");

    // Podemos mezclar closures distintas en un Vec porque todas son Box<dyn Fn>
    let operaciones: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        closure_suma(10),
        closure_multiplica(3),
        closure_suma(1),
        closure_multiplica(2),
    ];

    for (i, op) in operaciones.iter().enumerate() {
        println!("operacion[{}](5) = {}", i, op(5));
    }
    // operacion[0](5) = 15  (5 + 10)
    // operacion[1](5) = 15  (5 * 3)
    // operacion[2](5) = 6   (5 + 1)
    // operacion[3](5) = 10  (5 * 2)


    // --- 7. EJEMPLOS REALES ---
    println!("\n=== 7. EJEMPLOS REALES ===");

    // FILTROS
    let es_adulto   = mayor_que(17);
    let es_jubilado = mayor_que(64);

    let edades = vec![10, 20, 30, 65, 70];
    let adultos:   Vec<i32> = edades.iter().copied().filter(|&e| es_adulto(e)).collect();
    let jubilados: Vec<i32> = edades.iter().copied().filter(|&e| es_jubilado(e)).collect();

    println!("Adultos:   {:?}", adultos);   // [20, 30, 65, 70]
    println!("Jubilados: {:?}", jubilados); // [65, 70]

    // VALIDADORES
    let min_8_chars   = longitud_minima(8);
    let tiene_numeral = debe_contener('#');

    let password = "segura#123";
    println!("\nPassword '{}' válida por longitud: {}", password, min_8_chars(password));   // true
    println!("Password '{}' tiene '#':            {}", password, tiene_numeral(password)); // true

    let mala = "abc";
    println!("Password '{}' válida por longitud: {}", mala, min_8_chars(mala));   // false
    println!("Password '{}' tiene '#':            {}", mala, tiene_numeral(mala)); // false

    // TRANSFORMADORES
    let agregar_hola  = con_prefijo("Hola, ");
    let agregar_signo = con_sufijo("!");

    let nombres = vec!["Ana", "Bruno", "Carla"];
    for nombre in &nombres {
        let saludo = agregar_signo(&agregar_hola(nombre));
        println!("{}", saludo); // Hola, Ana!  Hola, Bruno!  Hola, Carla!
    }

    // PIPELINE: Vec de operaciones aplicadas en secuencia
    println!("\nPipeline:");
    let pipeline = construir_pipeline(vec![
        Box::new(|x| x + 10),   // paso 1: sumar 10
        Box::new(|x| x * 2),    // paso 2: multiplicar por 2
        Box::new(|x| x - 3),    // paso 3: restar 3
    ]);

    println!("pipeline(5) = {}", pipeline(5));
    // paso 1: 5  + 10 = 15
    // paso 2: 15 *  2 = 30
    // paso 3: 30 -  3 = 27
}
