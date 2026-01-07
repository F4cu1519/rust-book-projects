
// La mejor manera de recorrer una lista en Rust es utilizando un bucle `for`, pero para probar todos
// los bucles, implementaremos `while` y `loop` también.
use std::io;
use rand::Rng;
use std::io::Write;


fn recorrer_con_for(list: &[i32]){
    for elemento in list { 
        println!("Elemento 1: {}", elemento);
    }
}

fn recorrer_con_while(list: &[i32]){
    let mut i = 0;
    while list.len() > i {
        println!("Elemento 1: {}", list[i]);
        i += 1;
    }
}

fn recorrer_con_loop(list: &[i32]){
    let mut i = 0;
    loop {
        if i >= list.len() {
            break;
        }
        println!("Elemento 1: {}", list[i]);
        i += 1;
    }
}

fn main() {
    'principal: loop{ // esto es etiquetado para despues referenciar a ese loop
        print!("Cuantos elementos quieres en la lista? ");
        io::stdout().flush().unwrap(); // Asegura que el prompt se imprima antes de leer la entrada
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Por favor ingresa un numero valido");
        let longitud : usize = input.trim().parse().expect("Número inválido"); // usize por que es el formato que dsp me da el len8)

        let mut list: Vec<i32> = Vec::new();
        while longitud > list.len(){
            let random = rand::thread_rng().gen_range(1..=100);
            list.push(random);
        };

        print!("Como la quiere recorrer (Loop '1', While '2' y For '3'): ");
        io::stdout().flush().unwrap();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Por favor ingrese un numero del 1 al 3");
        let opcion : i32 = input2.trim().parse().expect("Ingrese 1, 2 o 3");
        match opcion {
            1 => recorrer_con_for(&list),
            2 => recorrer_con_while(&list),
            3 => recorrer_con_loop(&list),
            _ => println!("Opción inválida"),
        }

        loop {
            let mut entrada = String::new();

            println!("¿Cansado? (y/n)");

            io::stdin().read_line(&mut entrada).unwrap();
            let respuesta = entrada.trim().to_lowercase();

            if respuesta == "y" {
                println!("Nos vemos!!");
                break 'principal; // salimos del loop principal
            } else if respuesta == "n" {
                println!("Sigamos entonces!");
                break; // salimos del loop actual
            } else {
                println!("Entrada inválida, por favor use y/n");
                // no hay break → se repite la pregunta
            }
                
        }   

    }
}