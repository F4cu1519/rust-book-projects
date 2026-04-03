// =============================================================================
//  ADVANCED TRAITS EN RUST — Explicación completa paso a paso
//  Basado en: doc.rust-lang.org/book/ch20-02-advanced-traits.html
// =============================================================================
//
//  Contenido:
//    1. Tipos Asociados (Associated Types)
//    2. Parámetros Genéricos con Default + Sobrecarga de Operadores
//    3. Métodos con el Mismo Nombre (Disambiguation)
//    4. Supertraits
//    5. Patrón Newtype
//
// =============================================================================


// =============================================================================
// 1. TIPOS ASOCIADOS (Associated Types)
// =============================================================================
//
// Un "tipo asociado" es un nombre placeholder que le damos a un tipo DENTRO
// de un trait. Quien implemente el trait decide qué tipo concreto va ahí.
//
// ¿Por qué no usar genéricos directamente?
//
//   Con genérico:      trait Iterator<T> { fn next(&mut self) -> Option<T>; }
//   Con tipo asociado: trait Iterator    { type Item; fn next... Option<Self::Item>; }
//
// La diferencia clave:
//   - Con genérico → podés implementar Iterator<String> E Iterator<u32> para
//     el mismo tipo. Eso genera ambigüedad al llamar .next().
//   - Con tipo asociado → solo UNA implementación posible por tipo.
//     El compilador siempre sabe qué tipo devuelve .next().

// Definimos nuestro propio trait con tipo asociado:
trait Conversor {
    type Salida; // <-- placeholder: quien implemente decide qué es "Salida"

    fn convertir(&self) -> Self::Salida;
}

struct Celsius(f64);
struct Fahrenheit(f64);

// Acá decidimos: para Celsius, "Salida" es Fahrenheit
impl Conversor for Celsius {
    type Salida = Fahrenheit; // solo se define UNA VEZ, acá

    fn convertir(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

// Si quisiéramos también convertir Fahrenheit → Celsius:
impl Conversor for Fahrenheit {
    type Salida = Celsius;

    fn convertir(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
}

// Ventaja: al llamar .convertir() el compilador SABE exactamente qué tipo
// devuelve, sin que tengamos que anotarlo nosotros en cada uso.


// =============================================================================
// 2. PARÁMETROS GENÉRICOS CON DEFAULT + SOBRECARGA DE OPERADORES
// =============================================================================
//
// Podés darle un valor por defecto a un parámetro genérico:
//
//   trait Add<Rhs = Self> { ... }
//
// Esto dice: "si no especificás Rhs, asumí que es el mismo tipo que Self".
//
// Rust no te deja crear operadores propios, pero sí podés REDEFINIR el
// comportamiento de +, -, *, etc. implementando los traits de std::ops.

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector2D {
    x: f64,
    y: f64,
}

// Implementamos Add para Vector2D.
// Como no especificamos <Rhs>, usa el default: Rhs = Self = Vector2D.
// O sea: estamos sumando Vector2D + Vector2D → Vector2D
impl Add for Vector2D {
    type Output = Vector2D; // tipo asociado: qué devuelve la suma

    fn add(self, otro: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + otro.x,
            y: self.y + otro.y,
        }
    }
}

// Ahora un caso donde SÍ cambiamos Rhs.
// Queremos sumar Vector2D + f64 (escalar), que devuelve Vector2D.
// Para eso especificamos Add<f64> explícitamente.
impl Add<f64> for Vector2D {
    type Output = Vector2D;

    fn add(self, escalar: f64) -> Vector2D {
        Vector2D {
            x: self.x + escalar,
            y: self.y + escalar,
        }
    }
}

// Resultado:
//   v1 + v2          → usa impl Add (Rhs = Vector2D)
//   v1 + 3.0_f64     → usa impl Add<f64>
//
// Los parámetros con default sirven para dos cosas:
//   a) Evitar repetición cuando el caso más común es obvio (Rhs = Self)
//   b) Permitir casos especiales sin romper el caso común


// =============================================================================
// 3. MÉTODOS CON EL MISMO NOMBRE (Disambiguation)
// =============================================================================
//
// En Rust, dos traits distintos pueden tener métodos con el mismo nombre,
// y un tipo puede implementar ambos. Además, el tipo puede tener su propio
// método con ese nombre.
//
// Cuando eso pasa, necesitás decirle explícitamente al compilador cuál querés.

trait Volador {
    fn volar(&self) -> &str;
}

trait Mago {
    fn volar(&self) -> &str;
    fn nombre_cria() -> String; // función asociada (sin &self)
}

struct Humano;

// El tipo tiene su propio método volar:
impl Humano {
    fn volar(&self) -> &str {
        "*agita los brazos frenéticamente*"
    }

    fn nombre_cria() -> String {
        String::from("bebé humano")
    }
}

impl Volador for Humano {
    fn volar(&self) -> &str {
        "Este es tu capitán hablando."
    }
}

impl Mago for Humano {
    fn volar(&self) -> &str {
        "¡Arriba!"
    }

    fn nombre_cria() -> String {
        String::from("aprendiz")
    }
}

// Cómo llamar a cada uno:
//
//   persona.volar()          → llama al del tipo directamente (Humano::volar)
//   Volador::volar(&persona) → llama al de Volador
//   Mago::volar(&persona)    → llama al de Mago
//
// Esto funciona porque &self le da al compilador el tipo concreto (Humano).
//
// PERO con funciones asociadas (sin &self) el compilador no puede deducirlo.
// Necesitás FULLY QUALIFIED SYNTAX:
//
//   <Humano as Mago>::nombre_cria()
//
// Que se lee: "tratá a Humano como si fuera Mago, y llamá nombre_cria"
// La forma general es: <Tipo as Trait>::funcion(args...)


// =============================================================================
// 4. SUPERTRAITS
// =============================================================================
//
// A veces un trait DEPENDE de otro trait para funcionar.
// Podés exigir que quien implemente tu trait también implemente otro.
//
//   trait MiTrait: OtroTrait { ... }
//
// Esto se llama "supertrait": OtroTrait es el supertrait de MiTrait.
// Si intentás implementar MiTrait sin OtroTrait → error de compilación.

use std::fmt;

// OutlinePrint depende de Display.
// Dentro del trait podemos usar .to_string() porque sabemos que Display existe.
trait OutlinePrint: fmt::Display {
    fn imprimir_con_borde(&self) {
        let texto = self.to_string(); // funciona porque Display está garantizado
        let largo = texto.len();
        println!("{}", "*".repeat(largo + 4));
        println!("* {} *", texto);
        println!("{}", "*".repeat(largo + 4));
    }
}

#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

// Primero implementamos Display (el supertrait requerido):
impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Ahora SÍ podemos implementar OutlinePrint, porque Display ya está:
impl OutlinePrint for Punto {}

// Si intentáramos impl OutlinePrint for Punto SIN el Display de arriba,
// el compilador daría error:
//   "Point doesn't implement std::fmt::Display"


// =============================================================================
// 5. PATRÓN NEWTYPE
// =============================================================================
//
// REGLA DEL ORPHAN: en Rust, solo podés implementar un trait sobre un tipo
// si AL MENOS UNO de los dos (trait o tipo) es local a tu crate.
//
// Ejemplo de lo que NO podés hacer directamente:
//   impl fmt::Display for Vec<String> { ... }  // ← ERROR
//   // Display y Vec son ambos de la librería estándar, no son tuyos.
//
// SOLUCIÓN: envolver el tipo externo en un struct propio (newtype).
// El wrapper es local a tu crate → podés implementar cualquier trait en él.
//
// No hay costo en tiempo de ejecución: el compilador elimina el wrapper.

struct ListaDeTextos(Vec<String>); // "newtype": wrapper de Vec<String>

impl fmt::Display for ListaDeTextos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 accede al primer (y único) campo del tuple struct
        write!(f, "[{}]", self.0.join(", "))
    }
}

// DESVENTAJA: ListaDeTextos no hereda los métodos de Vec (push, len, etc.)
// Si los necesitás, tenés dos opciones:
//   a) Implementar Deref para devolver el Vec interno (acceso completo)
//   b) Implementar manualmente solo los métodos que querés exponer

// Implementamos Deref para que ListaDeTextos se comporte como Vec<String>:
use std::ops::Deref;

impl Deref for ListaDeTextos {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.0 // devuelve referencia al Vec interno
    }
}

// Con Deref implementado, podés hacer lista.len(), lista.iter(), etc.
// porque Rust automáticamente desreferencia cuando es necesario.


// =============================================================================
// MAIN: demos de todo lo anterior
// =============================================================================

fn main() {
    println!("=== 1. TIPOS ASOCIADOS ===");
    let agua_hirviendo = Celsius(100.0);
    let en_fahrenheit = agua_hirviendo.convertir();
    println!("100°C = {}°F", en_fahrenheit.0); // 212

    println!("\n=== 2. SOBRECARGA DE OPERADORES ===");
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    let suma = v1 + v2;
    println!("v1 + v2 = {:?}", suma); // x:4, y:6

    let escalado = v1 + 10.0;
    println!("v1 + 10.0 = {:?}", escalado); // x:11, y:12

    println!("\n=== 3. DISAMBIGUATION ===");
    let persona = Humano;

    println!("{}", persona.volar());         // del tipo: *agita los brazos*
    println!("{}", Volador::volar(&persona)); // del trait Volador
    println!("{}", Mago::volar(&persona));    // del trait Mago

    // Para funciones asociadas usamos fully qualified syntax:
    println!("{}", Humano::nombre_cria());           // "bebé humano"
    println!("{}", <Humano as Mago>::nombre_cria()); // "aprendiz"

    println!("\n=== 4. SUPERTRAITS ===");
    let p = Punto { x: 3, y: 7 };
    p.imprimir_con_borde();
    // ********
    // * (3, 7) *
    // ********

    println!("\n=== 5. NEWTYPE PATTERN ===");
    let lista = ListaDeTextos(vec![
        String::from("hola"),
        String::from("mundo"),
        String::from("rust"),
    ]);
    println!("{}", lista); // [hola, mundo, rust]

    // Gracias a Deref, podemos usar métodos de Vec:
    println!("Longitud: {}", lista.len()); // 3
    println!("Primer elemento: {}", lista[0]); // hola
}