/*
panic!
        “Este programa está mal si esto ocurre”

Result + ?
        “Esto puede fallar y alguien puede decidir qué hacer, lo uso solo para una recuperacion del programa”

Con result y usando ? busco que el programa continue su funcionamiento si el error que se dio no permite la
continuacion del programa con su proposito original no tiene sentido no usar panic.
*/

fn main() {
    // =========================================================
    // EJEMPLO 1: panic! por violar una regla interna (bug)
    // =========================================================
    // Esta función define una PRECONDICIÓN:
    // el divisor (b) NUNCA puede ser 0.
    //
    // Si alguien llama a esta función con b = 0,
    // el error NO es del usuario ni del mundo real,
    // es un error del programador.
    //
    // En ese caso, panic! está BIEN.
    println!("Resultado división: {}", dividir(10, 2));

    // =========================================================
    // EJEMPLO 2: panic! por estado imposible
    // =========================================================
    // Este enum define TODOS los estados válidos.
    // Si aparece algo fuera de esto, hay corrupción lógica.
    let estado = Estado::Iniciado;
    procesar_estado(estado);

    // =========================================================
    // EJEMPLO 3: panic! por uso incorrecto de una función
    // =========================================================
    // Esta función SOLO acepta números >= 0.
    // Si alguien rompe ese contrato, es un bug.
    println!("Raíz: {}", raiz_cuadrada(9.0));
}

// =========================================================
// EJEMPLO 1: Regla interna que nunca debe romperse
// =========================================================
fn dividir(a: i32, b: i32) -> i32 {
    // Justificación del panic!:
    // - b == 0 rompe una regla matemática
    // - No hay forma correcta de continuar
    // - Indica un error de programación
    if b == 0 {
        panic!("Error de programación: el divisor nunca puede ser 0");
    }

    a / b
}

// =========================================================
// EJEMPLO 2: Estados imposibles
// =========================================================

// Este enum define un conjunto CERRADO de estados.
// No puede existir ningún otro.
enum Estado {
    Iniciado,
    Finalizado,
}

fn procesar_estado(estado: Estado) {
    match estado {
        Estado::Iniciado => {
            println!("Procesando estado iniciado");
        }
        Estado::Finalizado => {
            println!("Procesando estado finalizado");
        }

        // Este brazo nunca debería ejecutarse.
        // Si ocurre, el programa está lógicamente roto.
        //
        // panic! acá sirve para decir:
        // "Este estado NO debería existir"
        _ => panic!("Estado imposible alcanzado"),
    }
}

// =========================================================
// EJEMPLO 3: Precondición clara de una función
// =========================================================
fn raiz_cuadrada(x: f64) -> f64 {
    // Justificación del panic!:
    // - La función tiene un CONTRATO
    // - El llamador debe respetarlo
    // - Un número negativo rompe ese contrato
    if x < 0.0 {
        panic!("Error de programación: no se permiten números negativos");
    }

    x.sqrt()
}
