use std::io;
use rand::Rng;

fn main() {
    println!("¡Adivina el numero! (Pista esta entre 1 y 100)");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Ingresa tu suposicion:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error al leer la linea");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > secret_number {
            println!("Tu suposicion es muy alta!");
        } else if guess < secret_number {
            println!("Tu suposicion es muy baja!");
        } else {
            println!("¡Felicidades! ¡Lo adivinaste!");
            break;
        }
        println!("Tu suposicion fue : {}", guess);
    }
}
