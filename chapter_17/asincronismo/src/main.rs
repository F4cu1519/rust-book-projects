// Importamos la librería estándar para poder leer argumentos
// que se pasan al programa desde la terminal.
//
// Ejemplo:
// cargo run https://rust-lang.org
//
// "https://rust-lang.org" es un argumento.
use std::env;


// Este crate viene del ejemplo del libro de Rust.
// No se usa en producción, es solo educativo.
// Nos da funciones async simples para hacer requests HTTP.
use trpl;


// Este crate sirve para parsear HTML.
// Básicamente permite tratar el HTML como un "árbol"
// y buscar elementos como <title>, <div>, etc.
use scraper::{Html, Selector};



fn main() {

    // -----------------------------
    // LEER ARGUMENTOS DE TERMINAL
    // -----------------------------

    // env::args() devuelve un iterador con los argumentos
    // que se pasaron al ejecutar el programa.
    //
    // collect() convierte ese iterador en un vector Vec<String>.
    //
    // Ejemplo real:
    //
    // cargo run https://rust-lang.org
    //
    // args[0] = nombre del programa
    // args[1] = "https://rust-lang.org"
    let args: Vec<String> = env::args().collect();



    // ------------------------------------
    // EJECUTAR CÓDIGO ASYNC
    // ------------------------------------

    // En Rust no podemos hacer:
    //
    // async fn main()
    //
    // porque el código async necesita un "runtime".
    //
    // block_on crea un runtime pequeño y ejecuta
    // el código async hasta que termine.
    trpl::block_on(async {

        // Tomamos la URL que pasó el usuario.
        // Es el segundo argumento del vector.
        let url = &args[1];



        // ------------------------------------
        // LLAMAR FUNCIÓN ASYNC
        // ------------------------------------

        // page_title(url) devuelve un Future.
        //
        // .await significa:
        // "esperar hasta que el Future termine".
        //
        // Cuando termine devolverá:
        //
        // Some(title) -> encontró título
        // None        -> no encontró título
        match page_title(url).await {

            // Si encontró el título del sitio
            Some(title) => println!("The title for {url} was {title}"),

            // Si el HTML no tenía <title>
            None => println!("{url} had no title"),
        }
    });
}



// ------------------------------------
// FUNCIÓN ASYNC
// ------------------------------------
//
// Esta función obtiene el título de una página web.
//
// async significa que internamente devuelve un Future.
// O sea algo que producirá un resultado en el futuro.
//
// El resultado final será:
//
// Option<String>
//
// Some(String) -> encontró título
// None         -> no encontró título
async fn page_title(url: &str) -> Option<String> {



    // ------------------------------------
    // HACER REQUEST HTTP
    // ------------------------------------

    // trpl::get(url) inicia una request HTTP.
    //
    // Pero es una operación de red, que puede tardar.
    // Por eso devuelve un Future.
    //
    // .await espera hasta que llegue la respuesta.
    let response = trpl::get(url).await;



    // ------------------------------------
    // OBTENER EL HTML COMO TEXTO
    // ------------------------------------

    // La respuesta HTTP contiene muchas cosas:
    //
    // headers
    // status code
    // body
    //
    // .text() obtiene el body como String.
    //
    // También es async porque puede tardar en leerse.
    let response_text = response.text().await;



    // ------------------------------------
    // PARSEAR EL HTML
    // ------------------------------------

    // Aquí convertimos el HTML en una estructura
    // que podemos recorrer como árbol.
    //
    // Algo así conceptualmente:
    //
    // html
    // ├── head
    // │   └── title
    // └── body
    let html = Html::parse_document(&response_text);



    // ------------------------------------
    // CREAR SELECTOR CSS
    // ------------------------------------

    // Esto es como un selector CSS.
    //
    // "title" significa:
    // buscar etiquetas <title>
    //
    // unwrap() significa:
    // "si algo falla, crashea el programa".
    //
    // Aquí lo usamos porque sabemos que el selector es válido.
    let selector = Selector::parse("title").unwrap();



    // ------------------------------------
    // BUSCAR EL <title>
    // ------------------------------------

    // html.select(&selector)
    // devuelve un iterador con todos los <title>.
    //
    // next()
    // toma el primero que encuentre.
    //
    // map(...)
    // transforma ese elemento en el texto del título.
    //
    // inner_html()
    // obtiene el texto dentro de <title>
    html.select(&selector)
        .next()
        .map(|title| title.inner_html())
}