// ─────────────────────────────────────────────────────────────────────────────
// src/main.rs
// ─────────────────────────────────────────────────────────────────────────────

// `use` trae nombres al scope actual para no tener que escribir la ruta completa.
// `hello::ThreadPool` → "del crate llamado `hello` (nuestro src/lib.rs), traé ThreadPool"
use part2::ThreadPool;

// Importaciones múltiples con llaves anidadas — syntax sugar de Rust.
// En lugar de 5 líneas `use` separadas, agrupamos todo del crate `std`.
use std::{
    // fs: módulo de sistema de archivos. Usamos fs::read_to_string()
    fs,
    // BufReader: lector con buffer para no ir byte a byte al SO
    // prelude::*: trae al scope los traits Read, Write, BufRead — necesarios
    //             para que .lines() y .write_all() existan en los tipos
    io::{BufReader, prelude::*},
    // TcpListener: para escuchar conexiones entrantes en un puerto
    // TcpStream:   representa una conexión TCP activa (bidireccional)
    net::{TcpListener, TcpStream},
    // thread: para crear hilos y hacer sleep
    thread,
    // Duration: representa una cantidad de tiempo (usada con thread::sleep)
    time::Duration,
};

fn main() {
    // TcpListener::new() no existe — se usa bind() que vincula al puerto.
    // "127.0.0.1:7878" → loopback (esta máquina), puerto 7878.
    // bind() devuelve Result<TcpListener, Error>.
    // .unwrap() → si Ok(listener) lo desempaqueta; si Err hace panic.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // ThreadPool::new(4) → crea el pool con exactamente 4 hilos internos.
    // El tipo ThreadPool está definido en src/lib.rs.
    // 4 es de tipo usize (entero sin signo, tamaño de puntero en la plataforma).
    let pool = ThreadPool::new(4);

    // listener.incoming() devuelve un iterador INFINITO de Result<TcpStream>.
    // Cada iteración = un intento de conexión entrante (no necesariamente exitoso).
    // El `for` llama .next() implícitamente en cada vuelta.
    for stream in listener.incoming() {

        // stream aquí es Result<TcpStream, Error>.
        // .unwrap() → desempaqueta el TcpStream o hace panic si la conexión falló.
        // Shadowing: reutilizamos el nombre `stream` para el TcpStream desempaquetado.
        let stream = stream.unwrap();

        // pool.execute() recibe un closure y lo encola para que un Worker lo ejecute.
        // || { ... } → closure sin parámetros.
        // `stream` se MUEVE al closure (Rust lo infiere porque TcpStream no es Copy).
        // El closure implementa FnOnce + Send + 'static (verificado por el compilador).
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// `mut stream` porque vamos a ESCRIBIR sobre él (enviar la respuesta HTTP).
// TcpStream es bidireccional: implementa tanto Read como Write.
// El ownership de stream se transfiere a esta función.
fn handle_connection(mut stream: TcpStream) {

    // BufReader::new(&stream) envuelve el stream en un lector con buffer.
    // &stream → referencia inmutable (solo lectura).
    // Sin buffer: cada .read() haría una syscall al SO — muy lento.
    // Con buffer: BufReader lee bloques grandes y los sirve desde memoria RAM.
    // Nota: tomamos &stream (no stream) para poder usar stream más adelante al escribir.
    let buf_reader = BufReader::new(&stream);

    // .lines() → iterador de Result<String, Error>, una por línea del stream.
    //            Se divide en cada byte '\n'. Viene del trait BufRead (importado con prelude::*).
    // .next()  → toma SOLO el primer elemento del iterador (la request line HTTP).
    //            Devuelve Option<Result<String, Error>>.
    // .unwrap() (primero) → desempaqueta el Option. None si el iterador estaba vacío → panic.
    // .unwrap() (segundo) → desempaqueta el Result. Err si hubo error de lectura → panic.
    // Resultado final: String con algo como "GET / HTTP/1.1"
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // `match` es exhaustivo: el compilador exige cubrir todos los casos posibles.
    // &request_line[..] → convierte String a &str (slice de string).
    //                     Necesario porque match no hace auto-deref como el operador ==.
    // Desestructuramos la tupla retornada directamente en dos variables.
    let (status_line, filename) = match &request_line[..] {

        // Brazo 1: petición GET a la raíz /
        // => devuelve una tupla de dos &str literales
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),

        // Brazo 2: petición GET a /sleep — simula trabajo pesado
        "GET /sleep HTTP/1.1" => {
            // thread::sleep() pausa ESTE hilo (el Worker que tomó el job).
            // Los otros 3 Workers del pool siguen disponibles durante estos 5 segundos.
            // Duration::from_secs(5) construye un valor Duration de 5 segundos.
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
            // El bloque retorna la tupla como su última expresión (sin punto y coma)
        }

        // Brazo 3: _ es el patrón "cualquier otra cosa" — equivale al else.
        // Cubre todos los demás paths: /foo, /bar, POST /, etc.
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // fs::read_to_string() lee el archivo completo a un String.
    // Hace todas las syscalls necesarias internamente.
    // Devuelve Result<String, io::Error>.
    // .unwrap() → panic si el archivo no existe o no se puede leer.
    // `filename` viene del match de arriba ("hello.html" o "404.html").
    let contents = fs::read_to_string(filename).unwrap();

    // .len() en un String devuelve la cantidad de BYTES (no caracteres Unicode).
    // HTTP requiere Content-Length en bytes, así que esto es correcto.
    let length = contents.len();

    // format!() construye un String nuevo sin hacer print.
    // Interpolamos las variables con {variable} dentro del string.
    // El formato HTTP es estricto:
    //   línea 1: status line
    //   \r\n: CRLF (Carriage Return + Line Feed) — fin de línea en HTTP
    //   Content-Length: {length}: header que dice cuántos bytes tiene el body
    //   \r\n: fin del header
    //   \r\n: línea vacía que SEPARA headers del body (obligatorio en HTTP)
    //   {contents}: el HTML como body
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // .as_bytes() convierte el &str a &[u8] (slice de bytes crudos).
    //             TCP trabaja con bytes, no con texto — esta conversión es obligatoria.
    // .write_all() envía TODOS los bytes por la conexión TCP.
    //              A diferencia de write(), garantiza que no envía parcialmente.
    //              Viene del trait Write (importado con prelude::*).
    //              Devuelve Result<(), Error>.
    // .unwrap() → panic si la escritura falla (ej: cliente cerró la conexión).
    stream.write_all(response.as_bytes()).unwrap();

    // Al salir de la función, `stream` se dropea → la conexión TCP se cierra.
    // Rust llama automáticamente a Drop::drop() — no hay que cerrar manualmente.
}