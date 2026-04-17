// ─────────────────────────────────────────────────────────────────────────────
// src/main.rs
// ─────────────────────────────────────────────────────────────────────────────

use Ultima_Version::ThreadPool;

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // TcpListener::bind() vincula a un puerto y devuelve Result.
    // "127.0.0.1:7878" → loopback (localhost), puerto 7878.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // ThreadPool::new(4) crea un pool con exactamente 4 hilos internos (Workers).
    // El tipo ThreadPool está definido en src/lib.rs.
    let pool = ThreadPool::new(4);

    // GRACEFUL SHUTDOWN: .take(2)
    // listener.incoming() devuelve un iterador infinito de Result<TcpStream>.
    // .take(2) limita el iterador a SOLO 2 elementos.
    //
    // Comportamiento:
    //   - Iteración 1: primera conexión entra
    //   - Iteración 2: segunda conexión entra
    //   - Iteración 3: el iterador se PARA (take(2) se agotó)
    //   - El for loop se sale automáticamente
    //   - Continuamos con println!("Apagando.")
    //   - Al final de main(), `pool` se DROPEA
    //   - Esto ACTIVA el impl Drop que definimos en lib.rs
    //
    // Sin .take(2), el servidor escucharía conexiones infinitamente.
    // Con .take(2), demostramos que el apagado graceful funciona.
    for stream in listener.incoming().take(2) {

        let stream = stream.unwrap();

        // pool.execute() toma el closure y lo encola en el canal MPSC.
        // El closure se MUEVE al heap (envuelto en Box) y entra en la cola.
        // Uno de los 4 Workers lo recogerá y ejecutará en su hilo.
        //
        // El ownership de `stream` se MUEVE al closure porque TcpStream no es Copy.
        // Una vez ejecutado el closure, stream se dropea (conexión TCP se cierra).
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // Si llegamos aquí es porque .take(2) agotó las iteraciones.
    // El for loop se salió naturalmente.
    // Ahora sale esta línea y termina main().
    println!("Apagando.");

    // ⚠️ CLAVE DEL GRACEFUL SHUTDOWN ⚠️
    // Al final de main(), `pool` se va del scope.
    // Rust llama automáticamente a Drop::drop(&mut pool).
    //
    // En Drop implementamos:
    //   1. drop(self.sender.take())
    //      → El Sender se CIERRA.
    //      → Los 4 Workers que estaban en .recv() reciben Err.
    //      → Ven el error y hacen break del loop infinito.
    //   
    //   2. for worker in self.workers.drain(..)
    //      → Iteramos sobre los Workers (extrayéndolos del Vec).
    //      → Llamamos worker.thread.take() para sacar el JoinHandle de su Option.
    //      → Llamamos .join() en el JoinHandle → ESPERAMOS a que el hilo termine.
    //   
    // Resultado: el programa no termina hasta que:
    //   - Todos los Jobs en progreso se completen
    //   - Todos los Workers hayan salido de su loop
    //   - Todos los hilos se hayan "unido" al hilo principal
    //
    // NO es un apagado abrupto. Es GRACEFUL.
}

// `mut stream` porque escribimos en él (enviar respuesta HTTP).
fn handle_connection(mut stream: TcpStream) {

    // BufReader envuelve el stream en un lector con buffer de RAM.
    // Sin esto, cada lectura sería una syscall al SO (lentísimo).
    // Con buffer: lee bloques, sirve desde RAM, muchas menos syscalls.
    let buf_reader = BufReader::new(&stream);

    // .lines() → iterador de Result<String> (línea por línea del stream HTTP)
    // .next() → toma el PRIMER elemento (la línea de request HTTP)
    // .unwrap().unwrap() → desempaqueta Option y Result
    // Resultado: String como "GET / HTTP/1.1"
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Match desestructura la tupla (status_line, filename) según la petición.
    // &request_line[..] convierte String a &str para hacer pattern matching.
    let (status_line, filename) = match &request_line[..] {

        // Petición GET a / (raíz)
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),

        // Petición GET a /sleep
        // Simulamos trabajo pesado (5 segundos de bloqueo).
        // IMPORTANTE: esto BLOQUEA SOLO este Worker.
        // Los otros 3 Workers siguen disponibles para servir otros requests.
        // Eso es el poder del thread pool — paralelismo.
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }

        // Cualquier otra petición → 404
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Lee el archivo completo a un String.
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Construye la respuesta HTTP con formato estricto:
    //   status_line (ej: "HTTP/1.1 200 OK")
    //   \r\n (CRLF — separador de línea en HTTP)
    //   Content-Length: {bytes}
    //   \r\n
    //   \r\n (línea vacía que SEPARA headers del body)
    //   {body}
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // .as_bytes() → convierte &String a &[u8] (slice de bytes)
    // TCP trabaja con bytes crudos, no con Unicode/Strings.
    // .write_all() → envía TODOS los bytes por el socket TCP.
    //                A diferencia de write(), garantiza envío completo.
    stream.write_all(response.as_bytes()).unwrap();

    // Al salir de esta función, `stream` se dropea → la conexión TCP se cierra.
    // Rust llama automáticamente a Drop::drop() en TcpStream.
    // No hay que cerrar manualmente — eso es RAII (Resource Acquisition Is Initialization).
}