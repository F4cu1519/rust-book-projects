// EJEMPLOS PRÁCTICOS DE TRAITS EN RUST
// =====================================
// Este archivo contiene ejemplos completos y funcionales de traits

use std::fmt;

// EJEMPLO 1: Trait básico
// ------------------------
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// EJEMPLO 2: Trait con implementación por defecto
// ------------------------------------------------
trait Describable {
    fn name(&self) -> String;
    
    fn describe(&self) -> String {
        format!("This is {}", self.name())
    }
}

struct Book {
    title: String,
    author: String,
}

impl Describable for Book {
    fn name(&self) -> String {
        self.title.clone()
    }
    // describe() usa la implementación por defecto
}

struct Movie {
    title: String,
    director: String,
}

impl Describable for Movie {
    fn name(&self) -> String {
        self.title.clone()
    }
    
    // Sobrescribimos la implementación por defecto
    fn describe(&self) -> String {
        format!("{} is a film directed by {}", self.title, self.director)
    }
}

// EJEMPLO 3: Traits como parámetros
// ----------------------------------
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Sintaxis alternativa con trait bound
fn notify_verbose<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Múltiples trait bounds
fn notify_and_print<T: Summary + fmt::Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item);
}

// EJEMPLO 4: Where clauses
// -------------------------
fn complex_function<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: Summary + fmt::Debug,
{
    format!("t: {}, u: {:?}", t.summarize(), u)
}

// EJEMPLO 5: Trait para comparación personalizada
// ------------------------------------------------
trait Comparable {
    fn is_greater_than(&self, other: &Self) -> bool;
    fn is_equal_to(&self, other: &Self) -> bool;
}

struct Temperature {
    celsius: f64,
}

impl Comparable for Temperature {
    fn is_greater_than(&self, other: &Self) -> bool {
        self.celsius > other.celsius
    }
    
    fn is_equal_to(&self, other: &Self) -> bool {
        (self.celsius - other.celsius).abs() < 0.01
    }
}

// EJEMPLO 6: Trait con tipos asociados
// -------------------------------------
trait Container {
    type Item;
    
    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

struct NumberContainer {
    numbers: Vec<i32>,
}

impl Container for NumberContainer {
    type Item = i32;
    
    fn add(&mut self, item: Self::Item) {
        self.numbers.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.numbers.get(index)
    }
    
    fn len(&self) -> usize {
        self.numbers.len()
    }
}

// EJEMPLO 7: Implementación condicional
// --------------------------------------
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// Este método solo existe si T implementa Display + PartialOrd
impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// EJEMPLO 8: Trait para conversión
// ---------------------------------
trait ToMeters {
    fn to_meters(&self) -> f64;
}

struct Kilometers(f64);
struct Miles(f64);
struct Feet(f64);

impl ToMeters for Kilometers {
    fn to_meters(&self) -> f64 {
        self.0 * 1000.0
    }
}

impl ToMeters for Miles {
    fn to_meters(&self) -> f64 {
        self.0 * 1609.34
    }
}

impl ToMeters for Feet {
    fn to_meters(&self) -> f64 {
        self.0 * 0.3048
    }
}

// EJEMPLO 9: Trait que hereda de otro (supertrait)
// -------------------------------------------------
trait Printable: fmt::Display {
    fn print(&self) {
        println!("{}", self);
    }
}

struct Message {
    content: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message: {}", self.content)
    }
}

impl Printable for Message {}

// EJEMPLO 10: Builder pattern con traits
// ---------------------------------------
trait Builder {
    type Output;
    fn build(self) -> Self::Output;
}

struct PersonBuilder {
    name: Option<String>,
    age: Option<u32>,
    email: Option<String>,
}

struct Person {
    name: String,
    age: u32,
    email: String,
}

impl PersonBuilder {
    fn new() -> Self {
        PersonBuilder {
            name: None,
            age: None,
            email: None,
        }
    }
    
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}

impl Builder for PersonBuilder {
    type Output = Person;
    
    fn build(self) -> Self::Output {
        Person {
            name: self.name.unwrap_or_else(|| String::from("Unknown")),
            age: self.age.unwrap_or(0),
            email: self.email.unwrap_or_else(|| String::from("no-email")),
        }
    }
}

// EJEMPLO 11: Trait para operaciones matemáticas
// -----------------------------------------------
trait Arithmetic {
    fn add(&self, other: &Self) -> Self;
    fn subtract(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Arithmetic for Vector2D {
    fn add(&self, other: &Self) -> Self {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
    fn subtract(&self, other: &Self) -> Self {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// EJEMPLO 12: Retornar tipos que implementan traits
// --------------------------------------------------
fn returns_tweet() -> impl Summary {
    Tweet {
        username: String::from("rustacean"),
        content: String::from("Learning Rust traits!"),
        reply: false,
        retweet: false,
    }
}

// EJEMPLO 13: Trait para validación
// ----------------------------------
trait Validate {
    fn is_valid(&self) -> bool;
    fn validation_errors(&self) -> Vec<String>;
}

struct Email {
    address: String,
}

impl Validate for Email {
    fn is_valid(&self) -> bool {
        self.address.contains('@') && self.address.contains('.')
    }
    
    fn validation_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        if !self.address.contains('@') {
            errors.push(String::from("Email must contain @"));
        }
        if !self.address.contains('.') {
            errors.push(String::from("Email must contain a domain"));
        }
        
        errors
    }
}

// EJEMPLO 14: Trait para serialización simple
// --------------------------------------------
trait Serialize {
    fn to_string(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Serialize for Point {
    fn to_string(&self) -> String {
        format!("Point(x={}, y={})", self.x, self.y)
    }
}

// FUNCIÓN MAIN CON DEMOSTRACIONES
// ================================
fn main() {
    println!("=== EJEMPLOS DE TRAITS EN RUST ===\n");
    
    // Ejemplo 1: Summary trait
    println!("--- Ejemplo 1: Summary trait ---");
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Traits are awesome!"),
        reply: false,
        retweet: false,
    };
    
    let article = NewsArticle {
        headline: String::from("Rust 1.70 Released"),
        location: String::from("Online"),
        author: String::from("Rust Team"),
        content: String::from("Exciting new features..."),
    };
    
    println!("Tweet: {}", tweet.summarize());
    println!("Article: {}", article.summarize());
    
    // Ejemplo 2: Describable con implementación por defecto
    println!("\n--- Ejemplo 2: Describable trait ---");
    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Steve Klabnik"),
    };
    
    let movie = Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };
    
    println!("{}", book.describe());
    println!("{}", movie.describe());
    
    // Ejemplo 3: Traits como parámetros
    println!("\n--- Ejemplo 3: notify() con trait parameter ---");
    notify(&tweet);
    notify_verbose(&article);
    
    // Ejemplo 5: Comparable
    println!("\n--- Ejemplo 5: Comparable trait ---");
    let temp1 = Temperature { celsius: 20.0 };
    let temp2 = Temperature { celsius: 25.0 };
    
    println!("temp2 > temp1? {}", temp2.is_greater_than(&temp1));
    println!("temp1 == temp2? {}", temp1.is_equal_to(&temp2));
    
    // Ejemplo 6: Container con tipos asociados
    println!("\n--- Ejemplo 6: Container trait ---");
    let mut container = NumberContainer {
        numbers: Vec::new(),
    };
    
    container.add(10);
    container.add(20);
    container.add(30);
    
    println!("Container length: {}", container.len());
    println!("Item at index 1: {:?}", container.get(1));
    
    // Ejemplo 7: Implementación condicional
    println!("\n--- Ejemplo 7: Pair con cmp_display ---");
    let pair = Pair::new(10, 20);
    pair.cmp_display();
    
    // Ejemplo 8: ToMeters
    println!("\n--- Ejemplo 8: ToMeters conversions ---");
    let km = Kilometers(5.0);
    let miles = Miles(3.0);
    let feet = Feet(100.0);
    
    println!("5 km = {} meters", km.to_meters());
    println!("3 miles = {} meters", miles.to_meters());
    println!("100 feet = {} meters", feet.to_meters());
    
    // Ejemplo 9: Printable (supertrait)
    println!("\n--- Ejemplo 9: Printable trait ---");
    let msg = Message {
        content: String::from("Hello, Rust!"),
    };
    msg.print();
    
    // Ejemplo 10: Builder pattern
    println!("\n--- Ejemplo 10: Builder pattern ---");
    let person = PersonBuilder::new()
        .name(String::from("Alice"))
        .age(30)
        .email(String::from("alice@example.com"))
        .build();
    
    println!("Built person: {} (age {})", person.name, person.age);
    
    // Ejemplo 11: Arithmetic
    println!("\n--- Ejemplo 11: Arithmetic trait ---");
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    
    let sum = v1.add(&v2);
    let diff = v1.subtract(&v2);
    
    println!("v1 + v2 = {:?}", sum);
    println!("v1 - v2 = {:?}", diff);
    
    // Ejemplo 12: Retornar impl Trait
    println!("\n--- Ejemplo 12: Returning impl Trait ---");
    let returned_tweet = returns_tweet();
    println!("Returned: {}", returned_tweet.summarize());
    
    // Ejemplo 13: Validate
    println!("\n--- Ejemplo 13: Validate trait ---");
    let valid_email = Email {
        address: String::from("user@example.com"),
    };
    let invalid_email = Email {
        address: String::from("invalid-email"),
    };
    
    println!("Valid email? {}", valid_email.is_valid());
    println!("Invalid email? {}", invalid_email.is_valid());
    println!("Errors: {:?}", invalid_email.validation_errors());
    
    // Ejemplo 14: Serialize
    println!("\n--- Ejemplo 14: Serialize trait ---");
    let point = Point { x: 10, y: 20 };
    println!("Serialized: {}", point.to_string());
    
    println!("\n=== FIN DE LOS EJEMPLOS ===");
}

// EJERCICIOS PROPUESTOS
// ======================
//
// 1. Crea un trait `Area` para calcular el área de diferentes formas geométricas
//
// 2. Implementa un trait `Reversible` con un método `reverse` para String y Vec
//
// 3. Crea un trait `Drawable` con un método `draw` e impleméntalo para varios shapes
//
// 4. Implementa From<T> para convertir entre tus propios tipos
//
// 5. Crea un trait `Logger` con diferentes niveles de log (info, warning, error)
//
// 6. Implementa un trait `Cacheable` que determine si un valor debe guardarse en cache
