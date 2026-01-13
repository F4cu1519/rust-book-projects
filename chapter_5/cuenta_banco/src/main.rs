use std::io;

struct Cuenta {
    nombre: String,
    saldo: i32,
 }

impl Cuenta { 
    fn deopsitar(&mut self, monto: i32 ){

    }
    fn retirar(&mut self, monto: i32 ) -> bool{

    }
    fn ver_saldo(&self){

    }
}

fn main() {
    let op : usize;
    let mut cuenta = Cuenta {
        nombre: String::from("Usuario"),
        saldo: 0,
    };
    loop {
        println("Que operacion desea realizar");
        println("(1. deopsitar, 2. retirar, 3. ver saldo):");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error de lectura");
            continue;
        }
        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= 3 => {
                op = n;
                break;
            }
            _ => {
                println!("Ingrese un número válido entre 1 y 3");
            }
        }
    }
    if op == 1 { 
        
    } else if op == 2 {

    } else {

    }
}
