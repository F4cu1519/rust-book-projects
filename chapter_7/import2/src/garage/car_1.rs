
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

