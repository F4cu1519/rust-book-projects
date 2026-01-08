use std::io;
use std::io::Write;

fn main() {
    println!("------------------------------------------------------------------------------------------");
    // std::stdin.read_line(&mut entrada).except("Failed to read line");
    println!("In this example, you will learn about functions in Rust.");
    print!("Ingrese un número: ");
    io::stdout().flush().unwrap(); // Asegura que el prompt se imprima antes de leer la entrada

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Failed to read line");
    let decimal : i32 = entrada.trim().parse().expect("Please type a number!");
    
    let binary = in_binary(decimal); // dia 6 Al ser un tipo i32 no hay problema al pasarlo, se hace un copy, cosa que no sucederia si fuese un string pues vive en el heap

    println!("The value of the decimal is: {decimal}, in binary is: {binary}");
}

fn in_binary(mut x: i32) -> String {
    if x == 0 {
        return String::from("0")
    }

    let mut b = String::new();
    
    while x >= 1{
        let resto = x % 2;
        x = x / 2;
        let resto : String = resto.to_string();
        b = b + &resto;
    }
    b.chars()        // iterador de caracteres
    .rev()          // los invierte
    .collect()      // los junta en un String
}
