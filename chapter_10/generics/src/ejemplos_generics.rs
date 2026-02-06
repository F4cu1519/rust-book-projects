// EJEMPLOS PRÁCTICOS DE GENERICS EN RUST
// =======================================
// Este archivo contiene ejemplos completos y funcionales de tipos genéricos

// EJEMPLO 1: Función genérica básica
// -----------------------------------
// Retorna el primer elemento de un slice
fn first<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        Some(&list[0])
    }
}

// EJEMPLO 2: Struct genérico simple
// ----------------------------------
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// EJEMPLO 3: Struct con múltiples parámetros genéricos
// -----------------------------------------------------
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
    
    fn first(&self) -> &T {
        &self.first
    }
    
    fn second(&self) -> &U {
        &self.second
    }
}

// EJEMPLO 4: Enum genérico personalizado
// ---------------------------------------
#[derive(Debug)]
enum MyResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> MyResult<T, E> {
    fn is_success(&self) -> bool {
        match self {
            MyResult::Success(_) => true,
            MyResult::Failure(_) => false,
        }
    }
    
    fn unwrap(self) -> T 
    where 
        E: std::fmt::Debug,
    {
        match self {
            MyResult::Success(val) => val,
            MyResult::Failure(err) => panic!("Called unwrap on Failure: {:?}", err),
        }
    }
}

// EJEMPLO 5: Container genérico con métodos útiles
// -------------------------------------------------
#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
    
    fn set(&mut self, value: T) {
        self.value = value;
    }
    
    fn into_inner(self) -> T {
        self.value
    }
    
    // Método genérico que transforma el contenido
    fn map<U, F>(self, f: F) -> Container<U>
    where
        F: FnOnce(T) -> U,
    {
        Container {
            value: f(self.value),
        }
    }
}

// EJEMPLO 6: Métodos específicos para ciertos tipos
// --------------------------------------------------
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// EJEMPLO 7: Función que intercambia valores
// -------------------------------------------
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

// EJEMPLO 8: Wrapper genérico
// ----------------------------
#[derive(Debug)]
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// Implementación para tipos que pueden clonarse
impl<T: Clone> Wrapper<T> {
    fn duplicate(&self) -> T {
        self.value.clone()
    }
}

// EJEMPLO 9: Stack genérico
// --------------------------
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

// EJEMPLO 10: Mixup de tipos
// ---------------------------
impl<T, U> Point<T>
where
    T: std::fmt::Display,
{
    fn mixup<V, W>(self, other: Point<V>) -> Pair<T, V> {
        Pair {
            first: self.x,
            second: other.x,
        }
    }
}

// FUNCIÓN MAIN CON DEMOSTRACIONES
// ================================
fn main() {
    println!("=== EJEMPLOS DE GENERICS EN RUST ===\n");
    
    // Ejemplo 1: first()
    println!("--- Ejemplo 1: Función first() ---");
    let numbers = vec![1, 2, 3, 4, 5];
    let chars = vec!['a', 'b', 'c'];
    
    println!("Primer número: {:?}", first(&numbers));
    println!("Primer char: {:?}", first(&chars));
    println!("Lista vacía: {:?}", first::<i32>(&[]));
    
    // Ejemplo 2: Point simple
    println!("\n--- Ejemplo 2: Point<T> ---");
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.5, 4.2);
    
    println!("Point de enteros: {:?}", int_point);
    println!("Point de floats: {:?}", float_point);
    println!("Coordenada x: {}", int_point.x());
    
    // Ejemplo 3: Pair con tipos diferentes
    println!("\n--- Ejemplo 3: Pair<T, U> ---");
    let pair = Pair::new(42, "hello");
    println!("Pair original: {:?}", pair);
    
    let swapped = pair.swap();
    println!("Pair invertido: {:?}", swapped);
    
    // Ejemplo 4: MyResult
    println!("\n--- Ejemplo 4: MyResult<T, E> ---");
    let success: MyResult<i32, String> = MyResult::Success(100);
    let failure: MyResult<i32, String> = MyResult::Failure(String::from("Error!"));
    
    println!("¿Es éxito? {}", success.is_success());
    println!("¿Es éxito? {}", failure.is_success());
    println!("Valor: {}", success.unwrap());
    
    // Ejemplo 5: Container con map
    println!("\n--- Ejemplo 5: Container con map() ---");
    let container = Container::new(5);
    println!("Container original: {:?}", container);
    
    let doubled = container.map(|x| x * 2);
    println!("Container duplicado: {:?}", doubled);
    
    let as_string = doubled.map(|x| format!("El número es {}", x));
    println!("Container transformado: {:?}", as_string);
    
    // Ejemplo 6: Método específico para f64
    println!("\n--- Ejemplo 6: Método específico para Point<f64> ---");
    let point = Point::new(3.0, 4.0);
    println!("Distancia desde origen: {}", point.distance_from_origin());
    
    // Ejemplo 7: swap
    println!("\n--- Ejemplo 7: Función swap() ---");
    let (a, b) = (10, 20);
    let (b, a) = swap(a, b);
    println!("Después de swap: a={}, b={}", a, b);
    
    // Ejemplo 8: Wrapper con Clone
    println!("\n--- Ejemplo 8: Wrapper con duplicate() ---");
    let wrapper = Wrapper::new(String::from("Hola"));
    let duplicado = wrapper.duplicate();
    println!("Original: {:?}", wrapper);
    println!("Duplicado: {}", duplicado);
    
    // Ejemplo 9: Stack
    println!("\n--- Ejemplo 9: Stack genérico ---");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("Stack: {:?}", stack);
    println!("Tope: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Después de pop: {:?}", stack);
    
    // Ejemplo 10: Mixup
    println!("\n--- Ejemplo 10: Mixup de Points ---");
    let p1 = Point::new(5, 10);
    let p2 = Point::new("hello", 'c');
    // Descomentando esto causaría un error porque necesitamos Display
    // let mixed = p1.mixup(p2);
    
    // Demostración adicional: Option y Result (tipos genéricos estándar)
    println!("\n--- Bonus: Option y Result de la stdlib ---");
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;
    
    match some_value {
        Some(v) => println!("Hay un valor: {}", v),
        None => println!("No hay valor"),
    }
    
    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("Algo salió mal");
    
    println!("Resultado exitoso: {:?}", ok_result);
    println!("Resultado con error: {:?}", err_result);
    
    println!("\n=== FIN DE LOS EJEMPLOS ===");
}

// EJERCICIOS PROPUESTOS
// ======================
// 
// 1. Modifica Container para que tenga un método `replace` que cambie
//    el valor y devuelva el anterior
//
// 2. Crea un struct `Triple<T, U, V>` con tres valores de tipos diferentes
//
// 3. Implementa un método para Point que calcule la distancia a otro Point
//    (solo para Point<f64>)
//
// 4. Crea un enum `MyOption<T>` similar a Option con métodos útiles
//
// 5. Implementa un método `filter` para Container que devuelva Option<T>
//    basado en una condición
