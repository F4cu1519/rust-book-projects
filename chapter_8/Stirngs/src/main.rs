fn main() {
    // El guion bajo (_) al inicio del nombre indica:
    // "sé que esta variable no se usa, y está bien".
    // Evita el warning del compilador por variable no utilizada.
    let _s1 = String::new();

    // Esto NO es un String, es un &str (string slice).
    // Vive en el binario del programa y es inmutable.
    let s1 = "hola1";
    
    // String::from crea un String (heap, dueño de los datos).
    let s2 = String::from("hola2");

    // to_string() convierte algo que implementa Display en String.
    // En la práctica, es equivalente a String::from para literales.
    let s3 = "hola3".to_string();

    // format! crea un String nuevo sin mover (consumir) s1, s2 ni s3.
    // Es como println!, pero devuelve un String.
    let s = format!("{s1}--{s2}--{s3}");

    println!("{s}");
    // s1 sigue existiendo porque format! solo tomó referencias
    println!("{s1}");

    // String mutable
    let mut s4 = "hola".to_string();

    // push_str agrega un &str al final del String
    s4.push_str(", mundo");
    println!("{s4}");

    let mut s5 = "chau".to_string();
    let s6 = ", mundo".to_string();

    // push_str NO toma ownership, espera un &str
    // Por eso hay que pasar una referencia: &s6
    s5.push_str(&s6);
    println!("{s5}");
    // s6 sigue existiendo porque no fue movido
    println!("{s6}");

    let s7 = String::from("Hello, ");
    let s8 = String::from("world!");

    // El operador +:
    // - MUEVE s7 (deja de existir)
    // - TOMA una referencia a s8 (&s8)
    let s9 = s7 + &s8;

    println!("{s9}");
    // s8 sigue existiendo
    println!("{s8}");
    // println!("{s7}");
    // ❌ No compila:
    // s7 fue movido al crear s9, ya no es válido.

    // --- STRINGS Y UTF-8 ---
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // ❌ No se permite indexar strings en Rust
    // porque UTF-8 usa varios bytes por carácter.

    // chars(): itera por caracteres Unicode
    for c in "Зд".chars() {
        println!("{c}");
    }

    // bytes(): itera por bytes crudos (u8)
    for c in "Зд".bytes() {
        println!("{c}");
    }
}
