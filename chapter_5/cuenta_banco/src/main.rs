use std::io;

struct Cuenta {
    nombre: String,
    saldo: i32,
 }

impl Cuenta { 
    fn depositar(&mut self, monto: i32 ){
        self.saldo += monto;
        println!("Su saldo tras la recarga es de : {}", self.saldo);
    }
    fn retirar(&mut self, monto: i32 ) -> bool{
        if monto <= self.saldo {
            self.saldo -= monto;
            println!("Su saldo señor {} tras el retiro es de: {}", self.nombre, self.saldo);
            true
        } else {
            false
        }
    }
    fn ver_saldo(&self){
        println!("Su saldo es de : {}", self.saldo);
    }
}

fn main() {
    let mut op : usize;
    let mut cuenta = Cuenta {
        nombre: String::from("Usuario"),
        saldo: 0,
    };
    loop {

        loop {
            println!("Que operacion desea realizar");
            println!("(1. deopsitar, 2. retirar, 3. ver saldo):");
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
            loop{
                println!("Ingrese el monto a depositar: ");
                let mut deposito = String::new();
                if io::stdin().read_line(&mut deposito).is_err() {
                    println!("Error de lectura");
                    continue;
                }
                match deposito.trim().parse::<i32>() {
                Ok(n) if n > 0 => {
                    cuenta.depositar(n);
                    break;
                }
                _ => {
                    println!("Ingrese un número válido mayor que 0");
                }
            }
            }   

        } else if op == 2 {
            loop{
                println!("Ingrese el monto a retirar: ");
                let mut retiro = String::new();
                if io::stdin().read_line(&mut retiro).is_err() {
                    println!("Error de lectura");
                    continue;
                }
                match retiro.trim().parse::<i32>() {
                    Ok(n) if n > 0 => {
                        cuenta.retirar(n);
                        break;
                    }
                    _ => {
                        println!("Ingrese un número válido mayor que 0");
                    }
                }
            }   
        } else {
            cuenta.ver_saldo();
        }

        println!("Desea continuar? (y or n): ");
        let mut otra = String::new();
        if io::stdin().read_line(&mut otra).is_err() {println!("Error de lectura");}
        match otra.trim() {
            "y" => continue,
            "n" => break,
            _ => println!("Ingrese una de las opciones validas"),
        }
    }
}
