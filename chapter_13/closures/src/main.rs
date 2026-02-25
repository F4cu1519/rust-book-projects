fn main() {
    println!("===== 1) CAPTURA DEL ENTORNO =====");
    captura_entorno();

    println!("\n===== 2) CUANDO NECESITAS mut =====");
    closure_mut();

    println!("\n===== 3) CUANDO NECESITAS move =====");
    closure_move();

    println!("\n===== 4) Fn vs FnMut vs FnOnce =====");
    traits_closures();
}

// -------------------------------------------------
// 1) CAPTURAR VARIABLES DEL ENTORNO
// -------------------------------------------------
fn captura_entorno() {
    let x = 10;

    // El closure usa "x" sin recibirlo como parámetro
    // Rust lo captura automáticamente por referencia inmutable
    let suma = |y| x + y;

    println!("10 + 5 = {}", suma(5));

    // x sigue siendo usable porque solo fue prestado
    println!("x sigue valiendo {}", x);
}

// -------------------------------------------------
// 2) CUANDO EL CLOSURE NECESITA SER mut
// -------------------------------------------------
fn closure_mut() {
    let mut contador = 0;

    // Como el closure modifica "contador",
    // lo captura por referencia mutable
    let mut incrementar = || {
        contador += 1;
        println!("contador: {}", contador);
    };

    incrementar();
    incrementar();

    // Ya no podemos usar "contador" mientras el closure exista
    // porque está prestado de forma mutable
}

// -------------------------------------------------
// 3) CUANDO NECESITAS move
// -------------------------------------------------
fn closure_move() {
    let texto = String::from("hola");

    // move obliga al closure a tomar ownership
    let consumir = move || {
        println!("Texto dentro del closure: {}", texto);
    };

    consumir();

    // println!("{}", texto);
    // ↑ Esto NO compilaría porque "texto" fue movido
}

// -------------------------------------------------
// 4) Fn vs FnMut vs FnOnce
// -------------------------------------------------
fn traits_closures() {
    // -------- Fn --------
    // No modifica ni consume variables capturadas
    let x = 5;
    let solo_lee = |y| x + y;
    ejecutar_fn(solo_lee);

    // -------- FnMut --------
    let mut valor = 0;
    let mut modifica = || {
        valor += 1;
        valor
    };
    ejecutar_fnmut(&mut modifica);

    // -------- FnOnce --------
    let s = String::from("Rust");
    let consume = move || {
        println!("Consumido: {}", s);
    };
    ejecutar_fnonce(consume);
}

// Función que acepta closures que implementen Fn
fn ejecutar_fn<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    println!("Fn resultado: {}", f(10));
}

// Función que acepta closures que implementen FnMut
fn ejecutar_fnmut<F>(f: &mut F)
where
    F: FnMut() -> i32,
{
    println!("FnMut resultado: {}", f());
}

// Función que acepta closures que implementen FnOnce
fn ejecutar_fnonce<F>(f: F)
where
    F: FnOnce(),
{
    f();
}