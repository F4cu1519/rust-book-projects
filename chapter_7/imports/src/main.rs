// Se declara un módulo público llamado `garage`.
// Al ser `pub`, puede ser usado desde fuera de este archivo o crate.
pub mod garage {

    // Módulo público `car_1`, que representa un primer auto.
    pub mod car_1 {

        // Función pública que devuelve un booleano.
        // Simula que las luces del auto 1 están encendidas.
        pub fn lights_on() -> bool {
            true
        }

        // Función pública que consulta el estado de las luces
        // de OTRO módulo hermano (car_2).
        pub fn lights_of_a_neigthboor() -> bool {

            // `super` refiere al módulo padre (`garage`)
            // Desde ahí accedemos al módulo hermano `car_2`
            // y traemos la función `lights_on` al scope local.
            use super::car_2::lights_on;

            // Llamamos a la función importada.
            // Devuelve el valor de las luces del car_2.
            lights_on()

            // Otra forma más directa (sin `use`) sería:
            // super::car_2::lights_on()
            //
            // En este caso esa forma suele ser más clara,
            // porque se ve explícitamente de dónde viene la función.
        }
    }

    // Módulo público `car_2`
    pub mod car_2 {

        // Función pública que indica que las luces del auto 2
        // están apagadas.
        pub fn lights_on() -> bool {
            false
        }
    }
}

// Traemos los módulos `car_1` y `car_2` al scope actual.
// `self` se refiere al crate actual.
// Gracias a esto podemos escribir `car_1::lights_on()`
// en lugar de `garage::car_1::lights_on()`.
use self::garage::{car_1, car_2};

fn main() {

    // Mensaje informativo
    println!("Hello, in this program we will demonstrate how to use modules from other files");

    // Llamamos a la función `lights_on` del módulo car_1
    println!("Ligths car 1: {}", car_1::lights_on());

    // Llamamos a la función que consulta las luces del auto vecino (car_2)
    // pero desde car_1, usando `super`
    println!("Ligths car 1: {}", car_1::lights_of_a_neigthboor());

    // Llamamos directamente a la función de car_2
    println!("Ligths car 2: {}", car_2::lights_on());
}
