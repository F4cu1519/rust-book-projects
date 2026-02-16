pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_smaller_a_than_b(a: i32, b: i32) -> bool {
    if a > b { 
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    // La anotación `#[test]`: este atributo indica que es una función de prueba, 
    // por lo que el ejecutor de pruebas sabe que debe tratarla como tal.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_smaller_fn () {
        assert!(is_smaller_a_than_b(1,4), "a no es mas pequeño que b")
    }
}
