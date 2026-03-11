// ============================================================
// Cap. 16.1 — Usar Hilos para Ejecutar Código Simultáneamente
// Basado en: doc.rust-lang.org/book/ch16-01-threads.html
// ============================================================

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ----------------------------------------------------------
// 1. Crear un hilo básico con thread::spawn
//    El hilo creado puede terminar antes si el hilo principal
//    termina primero.
// ----------------------------------------------------------
pub fn ejemplo_spawn_basico() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hola número {i} desde el hilo creado!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hola número {i} desde el hilo principal!");
        thread::sleep(Duration::from_millis(1));
    }
    // NOTA: el hilo creado puede no terminar, ya que cuando
    // el hilo principal termina, todos los hilos se detienen.
}

// ----------------------------------------------------------
// 2. Usar JoinHandle para esperar que el hilo termine.
//    join() bloquea el hilo actual hasta que el otro termina.
// ----------------------------------------------------------
pub fn ejemplo_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hola número {i} desde el hilo creado!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hola número {i} desde el hilo principal!");
        thread::sleep(Duration::from_millis(1));
    }

    // Espera a que el hilo creado termine antes de continuar
    handle.join().unwrap();
}

// ----------------------------------------------------------
// 3. join() ANTES del for: los hilos corren de forma
//    secuencial (sin intercalado).
// ----------------------------------------------------------
pub fn ejemplo_join_antes_del_for() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hola número {i} desde el hilo creado!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Al llamar join aquí, el hilo principal espera a que
    // el hilo creado termine ANTES de ejecutar su for.
    handle.join().unwrap();

    for i in 1..5 {
        println!("hola número {i} desde el hilo principal!");
        thread::sleep(Duration::from_millis(1));
    }
}

// ----------------------------------------------------------
// 4. Closure con `move`: transfiere ownership al hilo.
//    Sin move, Rust solo prestaría `v` y no puede garantizar
//    que la referencia sea válida durante toda la vida del hilo.
// ----------------------------------------------------------
pub fn ejemplo_move_closure() {
    let v = vec![1, 2, 3];

    // `move` fuerza al cierre a tomar ownership de `v`
    let handle = thread::spawn(move || {
        println!("Aquí está el vector: {v:?}");
        // `v` ahora pertenece a este hilo
    });

    // No podemos usar `v` aquí, ya fue movido al hilo
    // drop(v); // <- esto NO compilaría

    handle.join().unwrap();
}

// ----------------------------------------------------------
// 5. Retornar un valor desde un hilo mediante JoinHandle.
//    join() devuelve Result<T, E> con el valor retornado
//    por el cierre.
// ----------------------------------------------------------
pub fn ejemplo_retorno_desde_hilo() -> i32 {
    let handle = thread::spawn(|| {
        // Simulamos un cómputo
        42
    });

    // join() retorna Ok(valor) si el hilo terminó bien
    handle.join().unwrap()
}

// ----------------------------------------------------------
// 6. Múltiples hilos con Arc<Mutex<T>> para compartir estado.
//    Arc = puntero de referencia contada atómicamente.
//    Mutex = garantiza acceso exclusivo al dato compartido.
// ----------------------------------------------------------
pub fn ejemplo_multiples_hilos_con_mutex() -> i32 {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contador_clone = Arc::clone(&contador);
        let h = thread::spawn(move || {
            let mut num = contador_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    *contador.lock().unwrap()
}

// ============================================================
// PRUEBAS
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // ----------------------------------------------------------
    // Prueba 1: un hilo creado puede ejecutar código
    // ----------------------------------------------------------
    #[test]
    fn test_hilo_se_ejecuta() {
        let ejecutado = Arc::new(Mutex::new(false));
        let ejecutado_clone = Arc::clone(&ejecutado);

        let handle = thread::spawn(move || {
            let mut val = ejecutado_clone.lock().unwrap();
            *val = true;
        });

        handle.join().unwrap();
        assert!(*ejecutado.lock().unwrap(), "El hilo debería haberse ejecutado");
    }

    // ----------------------------------------------------------
    // Prueba 2: join() garantiza que el hilo termina
    // ----------------------------------------------------------
    #[test]
    fn test_join_espera_al_hilo() {
        let resultado = Arc::new(Mutex::new(0));
        let resultado_clone = Arc::clone(&resultado);

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            let mut val = resultado_clone.lock().unwrap();
            *val = 99;
        });

        handle.join().unwrap();
        assert_eq!(*resultado.lock().unwrap(), 99, "El valor debe ser 99 tras join()");
    }

    // ----------------------------------------------------------
    // Prueba 3: retorno de valor desde un hilo
    // ----------------------------------------------------------
    #[test]
    fn test_retorno_desde_hilo() {
        let valor = ejemplo_retorno_desde_hilo();
        assert_eq!(valor, 42, "El hilo debe retornar 42");
    }

    // ----------------------------------------------------------
    // Prueba 4: move closure transfiere ownership
    // ----------------------------------------------------------
    #[test]
    fn test_move_closure_toma_ownership() {
        let datos = vec![10, 20, 30];
        let suma_esperada: i32 = datos.iter().sum();

        let handle = thread::spawn(move || {
            // `datos` fue movido a este hilo
            datos.iter().sum::<i32>()
        });

        let suma: i32 = handle.join().unwrap();
        assert_eq!(suma, suma_esperada, "La suma debe coincidir");
    }

    // ----------------------------------------------------------
    // Prueba 5: múltiples hilos con Arc<Mutex<T>>
    // ----------------------------------------------------------
    #[test]
    fn test_multiples_hilos_mutex() {
        let total = ejemplo_multiples_hilos_con_mutex();
        assert_eq!(total, 10, "10 hilos deben incrementar el contador a 10");
    }

    // ----------------------------------------------------------
    // Prueba 6: los hilos corren concurrentemente (interleaving)
    // ----------------------------------------------------------
    #[test]
    fn test_hilos_concurrentes() {
        let log = Arc::new(Mutex::new(vec![]));

        let mut handles = vec![];
        for id in 0..5 {
            let log_clone = Arc::clone(&log);
            let h = thread::spawn(move || {
                log_clone.lock().unwrap().push(id);
            });
            handles.push(h);
        }

        for h in handles {
            h.join().unwrap();
        }

        let mut ids = log.lock().unwrap().clone();
        ids.sort();
        assert_eq!(ids, vec![0, 1, 2, 3, 4], "Todos los hilos deben registrarse");
    }

    // ----------------------------------------------------------
    // Prueba 7: join() antes del for produce ejecución secuencial
    // ----------------------------------------------------------
    #[test]
    fn test_join_antes_produce_secuencial() {
        let orden = Arc::new(Mutex::new(vec![]));

        let orden_clone = Arc::clone(&orden);
        let handle = thread::spawn(move || {
            orden_clone.lock().unwrap().push("hilo");
        });

        handle.join().unwrap(); // esperamos ANTES de continuar

        orden.lock().unwrap().push("principal");

        let resultado = orden.lock().unwrap().clone();
        assert_eq!(
            resultado,
            vec!["hilo", "principal"],
            "Con join() antes, el hilo termina primero"
        );
    }

    // ----------------------------------------------------------
    // Prueba 8: sin join(), el hilo principal puede terminar antes
    // ----------------------------------------------------------
    #[test]
    fn test_sin_join_puede_no_ejecutarse() {
        let ejecutado = Arc::new(Mutex::new(false));
        let ejecutado_clone = Arc::clone(&ejecutado);

        // Hilo con sleep largo: sin join puede no completarse
        let _handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(60)); // muy largo
            *ejecutado_clone.lock().unwrap() = true;
        });

        // No llamamos join(), el hilo no habrá terminado aún
        assert!(
            !*ejecutado.lock().unwrap(),
            "El hilo con sleep largo no debería haber terminado"
        );
        // _handle se descarta aquí; el hilo se detiene con el test
    }
}