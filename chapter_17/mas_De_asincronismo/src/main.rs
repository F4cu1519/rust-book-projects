// ============================================================
//  ejemplo_async.rs
//  Temas cubiertos:
//    1. yield_now        → ceder el control al runtime
//    2. timeout          → abstracción async propia con select
//    3. Streams          → iteración asíncrona de valores
// ============================================================


use std::time::Duration;
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        println!("========================================");
        println!(" 1. yield_now — Multitarea Cooperativa");
        println!("========================================\n");
        demo_yield_now().await;

        println!("\n========================================");
        println!(" 2. timeout  — Abstracción Async Propia");
        println!("========================================\n");
        demo_timeout().await;

        println!("\n========================================");
        println!(" 3. Streams  — Iteración Asíncrona");
        println!("========================================\n");
        demo_streams().await;
    });
}

// ============================================================
// 1. yield_now
//
//    Simulamos dos tareas "pesadas" (CPU-bound).
//    Sin yield_now, la tarea A bloquearía a la tarea B
//    hasta terminar. Con yield_now, se intercalan.
// ============================================================

async fn demo_yield_now() {
    let tarea_a = async {
        for i in 1..=4 {
            trabajo_pesado("A", i, 30);
            trpl::yield_now().await; // ← cede el control entre cada paso
        }
        println!("[A] terminó");
    };

    let tarea_b = async {
        for i in 1..=4 {
            trabajo_pesado("B", i, 20);
            trpl::yield_now().await; // ← cede el control entre cada paso
        }
        println!("[B] terminó");
    };

    // join corre ambas concurrentemente en un solo hilo
    trpl::join(tarea_a, tarea_b).await;
}

// Simula trabajo sincrónico bloqueante (como cálculo CPU-intensivo)
fn trabajo_pesado(nombre: &str, paso: u32, ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
    println!("[{nombre}] paso {paso} completado ({ms}ms de trabajo)");
}

// ============================================================
// 2. timeout
//
//    Función async propia que "compite" un futuro contra
//    un temporizador usando trpl::select.
//    Retorna Ok(valor) si termina a tiempo, Err si no.
// ============================================================

async fn demo_timeout() {
    // Caso 1: operación que termina DENTRO del tiempo límite
    let operacion_rapida = async {
        trpl::sleep(Duration::from_millis(200)).await;
        "¡resultado listo!"
    };

    match timeout(operacion_rapida, Duration::from_millis(500)).await {
        Ok(valor) => println!("Éxito: '{valor}'"),
        Err(d)    => println!("Tiempo agotado después de {}ms", d.as_millis()),
    }

    // Caso 2: operación que tarda MÁS que el tiempo límite
    let operacion_lenta = async {
        trpl::sleep(Duration::from_secs(5)).await;
        "este valor nunca llega"
    };

    match timeout(operacion_lenta, Duration::from_millis(300)).await {
        Ok(valor) => println!("Éxito: '{valor}'"),
        Err(d)    => println!("Tiempo agotado después de {}ms", d.as_millis()),
    }
}

// Abstracción async propia: compite `futuro` contra un temporizador.
// - Ok(valor)    si el futuro termina primero
// - Err(duracion) si vence el tiempo límite
async fn timeout<F: Future>(
    futuro: F,
    tiempo_limite: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(futuro, trpl::sleep(tiempo_limite)).await {
        trpl::Either::Left(valor) => Ok(valor),   // el futuro ganó la carrera
        trpl::Either::Right(_)    => Err(tiempo_limite), // el timer ganó
    }
}

// ============================================================
// 3. Streams
//
//    Tres ejemplos progresivos:
//      a) Stream básico desde un iterador
//      b) Stream con transformaciones encadenadas (map + filter)
//      c) Stream desde un canal async (datos que llegan con el tiempo)
// ============================================================

async fn demo_streams() {
    // --- 3a. Stream básico ---
    println!("-- 3a. Stream básico desde iterador --");
    {
        let numeros = [10, 20, 30, 40, 50];
        let mut stream = trpl::stream_from_iter(numeros);

        while let Some(n) = stream.next().await {
            println!("  recibido: {n}");
        }
        //v Esto se descompone en:
        //v     1. stream.next()       → crea un Future que llamará poll_next internamente
        //v     2. .await              → cede el control al runtime hasta que haya un elemento
        //v     3. Some(value)         → si llegó un elemento, entramos al cuerpo del bucle
        //v     4. None                → si el stream terminó, salimos del bucle 
    }

    // --- 3b. Stream con map + filter encadenados ---
    println!("\n-- 3b. Stream con map y filter --");
    {
        let mut stream = trpl::stream_from_iter(1u32..=10)
            .map(|n| n * n)          // eleva al cuadrado cada número
            .filter(|n| n % 2 == 0); // solo los cuadrados pares

        while let Some(n) = stream.next().await {
            println!("  cuadrado par: {n}");
        }
    }

    // --- 3c. Stream desde canal async (simula datos "en tiempo real") ---
    println!("\n-- 3c. Stream desde canal async --");
    {
        let (tx, mut rx) = trpl::channel();

        // Tarea productora: envía temperaturas cada 100ms
        let productor = async move {
            let temperaturas = [36.5, 37.1, 38.0, 36.8, 39.2];
            for temp in temperaturas {
                tx.send(temp).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
            // Al salir, tx se descarta → canal cerrado → stream termina
        };

        // Tarea consumidora: procesa cada temperatura al llegar
        let consumidor = async {
            while let Some(temp) = rx.recv().await {
                let estado = if temp >= 38.0 { "⚠ fiebre" } else { "✓ normal" };
                println!("  temperatura: {temp:.1}°C — {estado}");
            }
            println!("  [canal cerrado, stream terminado]");
        };

        // Ambas tareas corren concurrentemente
        trpl::join(productor, consumidor).await;
    }
}