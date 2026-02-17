// ============================================================
// Función 1: versión con panic
// Usada para demostrar #[should_panic]
// ============================================================
fn dividir_panic(a: i32, b: i32) -> i32 {
    if b == 0 {
        // Si el divisor es cero, el programa entra en pánico
        panic!("¡No se puede dividir entre cero!");
    }
    a / b
}

// ============================================================
// Función 2: versión con Result
// Usada para demostrar pruebas con Result<T, E>
// ============================================================
fn dividir_result(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        // Devolvemos un error controlado en lugar de entrar en pánico
        return Err(String::from("No se puede dividir entre cero"));
    }
    Ok(a / b)
}

// ============================================================
// Módulo de pruebas
// Solo se compila cuando se ejecuta `cargo test`
// ============================================================
#[cfg(test)]
mod tests {
    use super::*; // Importamos las funciones del módulo padre

    // ----------------------------------------------------------
    // Prueba con #[should_panic]
    // Verifica que la función entra en pánico cuando b == 0
    // El parámetro `expected` confirma que el mensaje de pánico
    // contiene ese texto específico es mas util cuando puede 
    // fallar mas de una cosa
    // ----------------------------------------------------------
    #[test]
    #[should_panic(expected = "No se puede dividir entre cero")]
    fn test_division_por_cero_panic() {
        dividir_panic(10, 0); // Debe entrar en pánico ✅
    }

    // ----------------------------------------------------------
    // Prueba con Result<T, E> — caso exitoso
    // La función devuelve Ok, usamos `?` para propagar el error
    // automáticamente si algo falla
    // ----------------------------------------------------------
    #[test]
    fn test_division_ok() -> Result<(), String> {
        let resultado = dividir_result(10, 2)?; // Si es Err, la prueba falla sola
        assert_eq!(resultado, 5);
        Ok(()) // Todo salió bien ✅
    }

    // ----------------------------------------------------------
    // Prueba con Result<T, E> — caso de error
    // Verificamos que cuando b == 0 se devuelve un Err
    // No usamos `?` aquí porque queremos inspeccionar el error
    // ----------------------------------------------------------
    #[test]
    fn test_division_error() {
        let resultado = dividir_result(10, 0);
        assert!(resultado.is_err()); // Confirmamos que es un error ✅
    }
}
