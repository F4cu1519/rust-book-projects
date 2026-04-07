// Importamos los módulos necesarios de la biblioteca estándar:
// - fs: para leer archivos del sistema de archivos
// - io::BufReader: para leer el stream con buffer (más eficiente que leer byte a byte)
// - io::prelude::*: trae al scope los traits de I/O como Read y Write, necesarios
//                   para que métodos como write_all y lines estén disponibles
// - net::TcpListener: para escuchar conexiones TCP entrantes
// - net::TcpStream: representa una conexión TCP activa (cliente <-> servidor)
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    // TcpListener::bind() vincula el servidor a la dirección IP y puerto indicados.
    // "bind" viene de "binding to a port" (vincularse a un puerto) — terminología de redes.
    // 127.0.0.1 = localhost (esta máquina), 7878 = puerto elegido arbitrariamente.
    // Devuelve Result<TcpListener, Error>, usamos unwrap() para obtener el valor
    // o hacer panic si falla (ej: el puerto ya está en uso).
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // listener.incoming() devuelve un iterador infinito de Result<TcpStream>.
    // Cada iteración representa un intento de conexión entrante (no necesariamente exitoso).
    for stream in listener.incoming() {
        // Desempaquetamos el Result: si la conexión falló hacemos panic,
        // si tuvo éxito obtenemos el TcpStream (la conexión activa con el cliente).
        let stream = stream.unwrap();

        // Delegamos el manejo de la conexión a una función separada.
        // Buena práctica: separar "aceptar conexión" de "procesar conexión".
        handle_connection(stream);
    }
}

// Recibe `mut stream` porque vamos a escribir sobre él (enviar la respuesta).
// TcpStream es bidireccional: se puede leer (solicitud) y escribir (respuesta).
fn handle_connection(mut stream: TcpStream) {
    // BufReader envuelve el stream para leer con buffer.
    // Sin buffer, cada llamada a read() iría directo al SO — ineficiente.
    // Con buffer, lee bloques grandes y los sirve desde memoria.
    // Recibe &stream (referencia) para no tomar ownership, ya que después
    // necesitamos escribir sobre stream.
    let buf_reader = BufReader::new(&stream);

    // buf_reader.lines() devuelve un iterador de Result<String>, una por línea.
    // .next()   → toma solo la primera línea (la "request line" del HTTP, ej: "GET / HTTP/1.1")
    // .unwrap() → desempaqueta el Option (None si el iterador estaba vacío)
    // .unwrap() → desempaqueta el Result (Err si hubo error de lectura o encoding)
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Analizamos la primera línea de la solicitud HTTP.
    // El formato HTTP es: "MÉTODO /ruta VERSION"
    // Si el cliente pide GET / HTTP/1.1 → responder con 200 OK y hello.html
    if request_line == "GET / HTTP/1.1" {
        // Línea de estado HTTP: versión + código + texto descriptivo
        let status_line = "HTTP/1.1 200 OK";

        // fs::read_to_string() lee todo el archivo y lo devuelve como String.
        // unwrap() hace panic si el archivo no existe o no se puede leer.
        let contents = fs::read_to_string("hello.html").unwrap();

        // Necesitamos el tamaño en bytes para el header Content-Length.
        // El navegador usa este valor para saber cuándo termina el cuerpo.
        let length = contents.len();

        // Armamos la respuesta HTTP completa con format!():
        // - Primera línea: status line
        // - \r\n: fin de línea HTTP (CRLF: Carriage Return + Line Feed)
        // - Content-Length: header obligatorio que indica el tamaño del cuerpo
        // - \r\n\r\n: línea en blanco que separa headers del cuerpo
        // - {contents}: el HTML como cuerpo de la respuesta
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // Convertimos el String a bytes (&[u8]) con as_bytes(),
        // ya que TCP trabaja con bytes crudos, no con texto.
        // write_all() garantiza que se escriban TODOS los bytes (no parcialmente).
        // unwrap() hace panic si la escritura falla.
        stream.write_all(response.as_bytes()).unwrap();

    } else {
        // Cualquier otra ruta (ej: GET /foo, POST /, etc.) → 404 Not Found
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}

// ─── Versión refactorizada (comentada) ────────────────────────────────────────
//
// La lógica es idéntica, pero más concisa: evita repetir el código de
// lectura de archivo y escritura de respuesta usando una tupla.
//
//      fn handle_connection(mut stream: TcpStream) {
//          let buf_reader = BufReader::new(&stream);
//          let request_line = buf_reader.lines().next().unwrap().unwrap();
//
//          // El bloque if/else ahora solo determina los valores que cambian:
//          // status_line y filename. El resto del código es igual para ambos casos.
//          // Rust permite que if/else sea una expresión que retorna un valor,
//          // acá retorna una tupla (&str, &str) que desestructuramos.
//          let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
//              ("HTTP/1.1 200 OK", "hello.html")
//          } else {
//              ("HTTP/1.1 404 NOT FOUND", "404.html")
//          };
//
//          // Este bloque se ejecuta igual para 200 y 404 — sin repetición.
//          let contents = fs::read_to_string(filename).unwrap();
//          let length = contents.len();
//          let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//          stream.write_all(response.as_bytes()).unwrap();
//      }