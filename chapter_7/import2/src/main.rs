
mod garage;

use garage::{car_1, car_2};

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
