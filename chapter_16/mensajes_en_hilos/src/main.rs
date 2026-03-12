use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n========================================");
    println!(" EJEMPLOS DE CANALES (mpsc) EN RUST");
    println!("========================================\n");

    ejemplo_1_basico();
    ejemplo_2_ownership();
    ejemplo_3_multiples_valores();
    ejemplo_4_multiples_productores();
    ejemplo_5_try_recv();
}

fn ejemplo_1_basico() {
    println!("--- Ejemplo 1: Canal básico ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mensaje = String::from("¡Hola desde el hilo secundario!");
        tx.send(mensaje).unwrap();
    });

    let recibido = rx.recv().unwrap();
    println!("Recibido: {recibido}\n");
}

fn ejemplo_2_ownership() {
    println!("--- Ejemplo 2: Transferencia de ownership ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("valor transferido");
        tx.send(val).unwrap();
        println!("  [hilo] Valor enviado. Ya no tengo acceso a él.");
    });

    let recibido = rx.recv().unwrap();
    println!("  [main] Tomé ownership del valor: '{recibido}'\n");
}

fn ejemplo_3_multiples_valores() {
    println!("--- Ejemplo 3: Múltiples valores con iterador ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let palabras = vec!["uno", "dos", "tres", "cuatro"];
        for palabra in palabras {
            tx.send(String::from(palabra)).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for recibido in rx {
        println!("  Recibido: {recibido}");
    }
    println!("  [main] Canal cerrado, iteración terminada.\n");
}

fn ejemplo_4_multiples_productores() {
    println!("--- Ejemplo 4: Múltiples productores ---");

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(format!("[Hilo A] mensaje {i}")).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("[Hilo B] mensaje {i}")).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        for i in 1..=3 {
            tx3.send(format!("[Hilo C] mensaje {i}")).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for recibido in rx {
        println!("  {recibido}");
    }
    println!("  [main] Los 3 productores terminaron.\n");
    println!("  ^ Nota: el orden puede variar en cada ejecución.\n");
}

fn ejemplo_5_try_recv() {
    println!("--- Ejemplo 5: try_recv (no bloqueante) ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(300));
        tx.send(String::from("mensaje demorado")).unwrap();
    });

    let mut intentos = 0;
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("  Recibido tras {intentos} intentos: '{msg}'");
                break;
            }
            Err(_) => {
                intentos += 1;
                println!("  Intento {intentos}: sin mensajes aún, haciendo otro trabajo...");
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    println!();
    println!("========================================");
    println!(" Todos los ejemplos completados.");
    println!("========================================\n");
}