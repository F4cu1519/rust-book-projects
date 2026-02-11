// EJEMPLOS PRÁCTICOS DE LIFETIMES EN RUST
// ========================================
// Este archivo contiene ejemplos completos y funcionales de lifetimes

// EJEMPLO 1: Función básica con lifetimes
// ----------------------------------------
// Esta función retorna la cadena más larga
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// EJEMPLO 2: Función que siempre retorna el primer parámetro
// -----------------------------------------------------------
// Solo el primer parámetro necesita estar relacionado con el retorno
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// EJEMPLO 3: Struct con referencias
// ----------------------------------
// ImportantExcerpt no puede vivir más que la referencia que contiene
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Método simple - el lifetime se infiere por las reglas de elisión
    fn level(&self) -> i32 {
        3
    }
    
    // Retorna una referencia con el mismo lifetime que self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// EJEMPLO 4: Múltiples lifetimes
// -------------------------------
#[derive(Debug)]
struct Context<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> Context<'a, 'b> {
    fn get_first(&self) -> &'a str {
        self.first
    }
    
    fn get_second(&self) -> &'b str {
        self.second
    }
}

// EJEMPLO 5: Parser - caso de uso real
// -------------------------------------
struct Parser<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Parser { text, position: 0 }
    }
    
    fn current_char(&self) -> Option<char> {
        self.text.chars().nth(self.position)
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn peek(&self, offset: usize) -> Option<char> {
        self.text.chars().nth(self.position + offset)
    }
    
    fn extract_word(&mut self) -> &'a str {
        let start = self.position;
        
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                break;
            }
            self.advance();
        }
        
        &self.text[start..self.position]
    }
}

// EJEMPLO 6: Iterador personalizado con lifetimes
// ------------------------------------------------
struct Split<'a> {
    remaining: &'a str,
    delimiter: char,
}

impl<'a> Split<'a> {
    fn new(s: &'a str, delimiter: char) -> Self {
        Split {
            remaining: s,
            delimiter,
        }
    }
}

impl<'a> Iterator for Split<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        
        match self.remaining.find(self.delimiter) {
            Some(pos) => {
                let item = &self.remaining[..pos];
                self.remaining = &self.remaining[pos + 1..];
                Some(item)
            }
            None => {
                let item = self.remaining;
                self.remaining = "";
                Some(item)
            }
        }
    }
}

// EJEMPLO 7: Cache con referencias
// ---------------------------------
struct Cache<'a, T> {
    data: &'a T,
    computed: Option<String>,
}

impl<'a, T> Cache<'a, T>
where
    T: std::fmt::Display,
{
    fn new(data: &'a T) -> Self {
        Cache {
            data,
            computed: None,
        }
    }
    
    fn get_or_compute(&mut self) -> &str {
        if self.computed.is_none() {
            self.computed = Some(format!("Cached: {}", self.data));
        }
        self.computed.as_ref().unwrap()
    }
}

// EJEMPLO 8: Función con lifetime bounds
// ---------------------------------------
// T debe vivir al menos tanto como 'a
fn print_ref<'a, T>(item: &'a T)
where
    T: std::fmt::Debug + 'a,
{
    println!("Debug: {:?}", item);
}

// EJEMPLO 9: Struct que guarda referencias a elementos de un Vec
// ---------------------------------------------------------------
struct VecWrapper<'a, T> {
    items: &'a Vec<T>,
}

impl<'a, T> VecWrapper<'a, T> {
    fn new(items: &'a Vec<T>) -> Self {
        VecWrapper { items }
    }
    
    fn get(&self, index: usize) -> Option<&'a T> {
        self.items.get(index)
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

// EJEMPLO 10: Referencias mutables con lifetimes
// -----------------------------------------------
fn modify_string<'a>(s: &'a mut String, addition: &str) -> &'a str {
    s.push_str(addition);
    s.as_str()
}

// EJEMPLO 11: Lifetime elision en acción
// ---------------------------------------
// Estas funciones NO necesitan anotaciones explícitas gracias a las reglas de elisión

// Regla 2: un parámetro de entrada -> mismo lifetime en salida
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// Regla 3: método -> lifetime de self en la salida
impl<'a> ImportantExcerpt<'a> {
    fn get_part(&self) -> &str {
        self.part
    }
}

// EJEMPLO 12: Struct con lifetime estático
// -----------------------------------------
struct Config {
    // 'static significa que la referencia vive para todo el programa
    version: &'static str,
    name: String,
}

impl Config {
    fn new(name: String) -> Self {
        Config {
            version: "1.0.0",  // String literal tiene lifetime 'static
            name,
        }
    }
}

// EJEMPLO 13: Combinando generics, traits y lifetimes
// ----------------------------------------------------
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// EJEMPLO 14: Closures con lifetimes
// -----------------------------------
fn apply_to_string<'a, F>(s: &'a str, f: F) -> String
where
    F: Fn(&'a str) -> String,
{
    f(s)
}

// EJEMPLO 15: Lifetime en enums
// ------------------------------
enum StringOrInt<'a> {
    Str(&'a str),
    Int(i32),
}

impl<'a> StringOrInt<'a> {
    fn display(&self) {
        match self {
            StringOrInt::Str(s) => println!("String: {}", s),
            StringOrInt::Int(i) => println!("Int: {}", i),
        }
    }
}

// FUNCIÓN MAIN CON DEMOSTRACIONES
// ================================
fn main() {
    println!("=== EJEMPLOS DE LIFETIMES EN RUST ===\n");
    
    // Ejemplo 1: longest()
    println!("--- Ejemplo 1: longest() ---");
    let string1 = String::from("long string is long");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
    
    // Demostración de scope válido
    {
        let string3 = String::from("short");
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("Result within scope: {}", result2);
    } // string3 sale del scope, pero está OK porque usamos result2 antes
    
    // Ejemplo 2: first()
    println!("\n--- Ejemplo 2: first() ---");
    let x = "hello";
    let y = "world";
    println!("First: {}", first(x, y));
    
    // Ejemplo 3: ImportantExcerpt
    println!("\n--- Ejemplo 3: ImportantExcerpt ---");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {:?}", excerpt);
    println!("Level: {}", excerpt.level());
    let part = excerpt.announce_and_return_part("Attention!");
    println!("Part: {}", part);
    
    // Ejemplo 4: Context con múltiples lifetimes
    println!("\n--- Ejemplo 4: Multiple lifetimes ---");
    let first = String::from("first string");
    let second = "second string";
    
    let context = Context {
        first: first.as_str(),
        second,
    };
    
    println!("Context: {:?}", context);
    println!("First: {}", context.get_first());
    println!("Second: {}", context.get_second());
    
    // Ejemplo 5: Parser
    println!("\n--- Ejemplo 5: Parser ---");
    let text = "hello world rust";
    let mut parser = Parser::new(text);
    
    println!("Current char: {:?}", parser.current_char());
    let word1 = parser.extract_word();
    println!("First word: {}", word1);
    
    parser.advance(); // Skip space
    let word2 = parser.extract_word();
    println!("Second word: {}", word2);
    
    // Ejemplo 6: Split iterator
    println!("\n--- Ejemplo 6: Split iterator ---");
    let sentence = "rust-is-awesome";
    let split = Split::new(sentence, '-');
    
    print!("Parts: ");
    for part in split {
        print!("{} ", part);
    }
    println!();
    
    // Ejemplo 7: Cache
    println!("\n--- Ejemplo 7: Cache ---");
    let value = 42;
    let mut cache = Cache::new(&value);
    
    println!("First access: {}", cache.get_or_compute());
    println!("Second access (cached): {}", cache.get_or_compute());
    
    // Ejemplo 8: print_ref
    println!("\n--- Ejemplo 8: print_ref ---");
    let numbers = vec![1, 2, 3];
    print_ref(&numbers);
    
    // Ejemplo 9: VecWrapper
    println!("\n--- Ejemplo 9: VecWrapper ---");
    let items = vec![10, 20, 30, 40, 50];
    let wrapper = VecWrapper::new(&items);
    
    println!("Length: {}", wrapper.len());
    println!("Item at 2: {:?}", wrapper.get(2));
    
    // Ejemplo 10: modify_string
    println!("\n--- Ejemplo 10: Mutable references ---");
    let mut greeting = String::from("Hello");
    {
        let result = modify_string(&mut greeting, " World");
        println!("Modified: {}", result);
    }
    println!("After scope: {}", greeting);
    
    // Ejemplo 11: Lifetime elision
    println!("\n--- Ejemplo 11: Lifetime elision ---");
    let sentence = "The quick brown fox";
    let first = first_word(sentence);
    println!("First word: {}", first);
    
    // Ejemplo 12: Static lifetime
    println!("\n--- Ejemplo 12: Static lifetime ---");
    let config = Config::new(String::from("MyApp"));
    println!("Config: {} v{}", config.name, config.version);
    
    // Ejemplo 13: Combinación de generics, traits y lifetimes
    println!("\n--- Ejemplo 13: Complex example ---");
    let s1 = "short";
    let s2 = "longer string";
    let announcement = "Comparing strings";
    
    let result = longest_with_announcement(s1, s2, announcement);
    println!("Result: {}", result);
    
    // Ejemplo 14: Closures con lifetimes
    println!("\n--- Ejemplo 14: Closures ---");
    let text = "rust";
    let uppercase = apply_to_string(text, |s| s.to_uppercase());
    println!("Uppercase: {}", uppercase);
    
    // Ejemplo 15: Enum con lifetimes
    println!("\n--- Ejemplo 15: Enum with lifetimes ---");
    let text = "Hello";
    let str_variant = StringOrInt::Str(text);
    let int_variant = StringOrInt::Int(42);
    
    str_variant.display();
    int_variant.display();
    
    // Demostración de un error común (comentado porque no compila)
    /*
    // ERROR: borrowed value does not live long enough
    let result;
    {
        let temp = String::from("temporary");
        result = longest("long", temp.as_str());
    }
    println!("{}", result); // temp ya no existe!
    */
    
    println!("\n=== FIN DE LOS EJEMPLOS ===");
}

// EJEMPLOS DE PATRONES COMUNES
// =============================

// Patrón 1: Retornar referencia del input más largo
fn get_longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}

// Patrón 2: Struct que mantiene referencia a slice
struct Slice<'a, T> {
    data: &'a [T],
}

impl<'a, T> Slice<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Slice { data }
    }
    
    fn first(&self) -> Option<&'a T> {
        self.data.first()
    }
}

// Patrón 3: Builder pattern con referencias
struct ConfigBuilder<'a> {
    name: Option<&'a str>,
    version: Option<&'a str>,
}

impl<'a> ConfigBuilder<'a> {
    fn new() -> Self {
        ConfigBuilder {
            name: None,
            version: None,
        }
    }
    
    fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
    
    fn version(mut self, version: &'a str) -> Self {
        self.version = Some(version);
        self
    }
}

// EJERCICIOS PROPUESTOS
// ======================
//
// 1. Crea una función que tome dos referencias y retorne la más corta
//
// 2. Implementa un struct `Buffer<'a>` que mantenga una referencia a un array
//    y tenga métodos para acceder a elementos
//
// 3. Crea un iterador personalizado que itere sobre palabras en un string
//
// 4. Implementa un struct `Pair<'a, T>` que contenga dos referencias del mismo tipo
//
// 5. Escribe una función que combine tres strings y retorne una referencia
//    con el lifetime apropiado
//
// 6. Crea un struct `View<'a>` que represente una "vista" de datos sin ownership
