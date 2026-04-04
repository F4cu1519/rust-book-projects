// ===============================
// ADVANCED TYPES IN RUST - FULL EXAMPLE
// ===============================

// 1. TYPE ALIAS
type Kilometers = i32;

fn usar_alias() {
    let distancia: Kilometers = 100;
    let numero: i32 = 50;

    println!("Distancia: {}", distancia);
    println!("Numero: {}", numero);
}

// 2. NEVER TYPE (!)
fn nunca_vuelve() -> ! {
    panic!("Esta funcion nunca retorna");
}

fn ejemplo_never(valor: i32) -> i32 {
    let resultado = match valor {
        1 => 10,
        _ => panic!("Valor no valido"),
    };

    resultado
}

// 3. DYNAMICALLY SIZED TYPES (DST)
fn imprimir_str(s: &str) {
    println!("Texto: {}", s);
}

// 4. TRAIT Sized y ?Sized
fn acepta_sized<T: Sized>(x: T) {
    println!("Valor con tamaño conocido");
}

fn acepta_posiblemente_no_sized<T: ?Sized>(x: &T) {
    println!("Puede ser DST (como str)");
}

// ===============================
// MAIN
// ===============================

fn main() {
    // TYPE ALIAS
    usar_alias();

    // NEVER TYPE (comentado para no romper ejecucion)
    // nunca_vuelve();

    let valor = 1;
    let resultado = ejemplo_never(valor);
    println!("Resultado: {}", resultado);

    // DST
    let texto = "hola mundo";
    imprimir_str(texto);

    // SIZED
    acepta_sized(10);

    // ?SIZED
    acepta_posiblemente_no_sized(texto);
}
