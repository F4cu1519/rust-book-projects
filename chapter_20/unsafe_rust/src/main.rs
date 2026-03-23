// =============================================================================
// GUÍA COMPLETA DE RUST INSEGURO (unsafe Rust)
// =============================================================================
// Este archivo explica paso a paso los 5 "superpoderes" de unsafe Rust:
//   1. Desreferenciar punteros crudos (raw pointers)
//   2. Llamar funciones inseguras
//   3. Acceder/modificar variables estáticas mutables
//   4. Implementar traits inseguros
//   5. Acceder a campos de unions
//
// REGLA DE ORO: unsafe no desactiva el borrow checker ni las demás
// verificaciones de Rust. Solo habilita estas 5 operaciones específicas.
// Tú asumes la responsabilidad de la seguridad de memoria.
// =============================================================================

use std::slice;

fn main() {
    superpoder_1_punteros_crudos();
    superpoder_2_funciones_inseguras();
    superpoder_3_estaticas_mutables();
    superpoder_4_traits_inseguros();
    superpoder_5_unions();
    bonus_abstraccion_segura();
    bonus_extern_c();
}

// =============================================================================
// SUPERPODER 1: DESREFERENCIAR PUNTEROS CRUDOS (Raw Pointers)
// =============================================================================
//
// ¿QUÉ ES? Un puntero crudo es una dirección de memoria "desnuda", sin las
// garantías que dan las referencias (&T) o los smart pointers (Box, Rc, etc.).
//
// ¿PARA QUÉ SIRVE?
//   - Interactuar con código C (FFI)
//   - Construir estructuras de datos de bajo nivel (listas enlazadas, arenas)
//   - Optimizaciones donde el borrow checker es demasiado conservador
//
// TIPOS:
//   *const T  →  puntero crudo inmutable (como *const en C)
//   *mut T    →  puntero crudo mutable   (como *mut  en C)
//
// DIFERENCIAS con referencias normales:
//   - Pueden ser nulos
//   - No garantizan apuntar a memoria válida
//   - Pueden existir varios punteros mutables al mismo dato (¡peligro de data race!)
//   - No implementan limpieza automática (no son RAII)
//
// REGLA: Crear un puntero crudo es SEGURO. Desreferenciarlo (*ptr) es INSEGURO.
// =============================================================================
fn superpoder_1_punteros_crudos() {
    println!("\n=== SUPERPODER 1: Punteros Crudos ===");

    let mut num = 42;

    // --- Forma 1: operadores de préstamo crudo (recomendada desde Rust 1.82) ---
    // &raw const  →  crea *const T
    // &raw mut    →  crea *mut T
    // Estas expresiones son seguras; solo la desreferenciación es unsafe.
    let r_inmutable: *const i32 = &raw const num;
    let r_mutable:   *mut   i32 = &raw mut   num;

    // --- Forma 2: cast explícito con 'as' (útil para direcciones arbitrarias) ---
    // ⚠️  Peligroso: no hay garantía de que 0x12345 sea memoria válida.
    let direccion_arbitraria = 0x12345usize;
    let _puntero_peligroso = direccion_arbitraria as *const i32;
    // No lo desreferenciamos porque causaría undefined behavior / segfault.

    // --- Desreferenciar dentro de bloque unsafe ---
    // El bloque unsafe le dice al compilador: "yo sé lo que hago aquí".
    unsafe {
        // *r_inmutable lee el valor apuntado
        println!("Valor via puntero inmutable: {}", *r_inmutable);

        // *r_mutable escribe en la dirección apuntada
        *r_mutable = 100;
        println!("Valor tras escritura mutable: {}", *r_mutable);
    }

    // Rust permite tener *const y *mut al mismo dato simultáneamente,
    // cosa que con referencias (&T y &mut T) NO está permitida.
    // Esto es útil, pero requiere que tú garantices que no hay carreras de datos.
    let _ambos_al_mismo_tiempo: (*const i32, *mut i32) = (r_inmutable, r_mutable);
    println!("(Se crearon puntero inmutable y mutable al mismo dato — permitido con raw pointers)");
}

// =============================================================================
// SUPERPODER 2: LLAMAR FUNCIONES INSEGURAS
// =============================================================================
//
// ¿QUÉ ES? Una función marcada con `unsafe fn` tiene requisitos que el
// compilador NO puede verificar. Tú debes leer su documentación y cumplirlos.
//
// ¿PARA QUÉ SIRVE?
//   - Encapsular operaciones de bajo nivel con contratos explícitos
//   - Documentar claramente qué precondiciones debe cumplir el llamador
//
// FORMA DE USO:
//   unsafe fn mi_funcion() { ... }   ← definición
//   unsafe { mi_funcion(); }         ← llamada (debe estar en bloque unsafe)
//
// CONVENCIÓN: Escribe un comentario "// SAFETY: ..." antes de cada bloque
// unsafe explicando POR QUÉ es seguro en ese contexto.
// =============================================================================

// Función insegura simple.
// CONTRATO: el puntero 'ptr' debe ser válido y apuntar a un i32 inicializado.
unsafe fn leer_puntero(ptr: *const i32) -> i32 {
    // Para realizar operaciones inseguras DENTRO de una función insegura,
    // aún necesitas un bloque unsafe explícito (el compilador te avisará si olvidas).
    unsafe { *ptr }
}

fn superpoder_2_funciones_inseguras() {
    println!("\n=== SUPERPODER 2: Funciones Inseguras ===");

    let valor = 77i32;
    let ptr = &raw const valor;

    // Para llamar una función insegura necesitas un bloque unsafe.
    // SAFETY: 'ptr' viene de una referencia válida a 'valor', que vive
    //         en este scope, por lo que el puntero es válido.
    let resultado = unsafe { leer_puntero(ptr) };
    println!("Valor leído con función insegura: {}", resultado);

    // Intentar llamarla SIN unsafe generaría el error:
    //   error[E0133]: call to unsafe function is unsafe and requires unsafe block
    // let _ = leer_puntero(ptr);  // ← NO compila
}

// =============================================================================
// SUPERPODER 3: ACCEDER / MODIFICAR VARIABLES ESTÁTICAS MUTABLES
// =============================================================================
//
// ¿QUÉ ES? Las variables globales en Rust se llaman `static`. Las inmutables
// son seguras, pero las MUTABLES son inseguras porque varios hilos podrían
// acceder a ellas simultáneamente causando carreras de datos.
//
// ¿PARA QUÉ SIRVE?
//   - Contadores globales simples (en programas de un solo hilo)
//   - Estado global en sistemas embebidos donde no hay threading
//   - Interfaces con código C que usa variables globales
//
// CONVENCIÓN DE NOMBRES: SCREAMING_SNAKE_CASE (todo mayúsculas con guiones bajos)
//
// REGLA: Leer o escribir una `static mut` siempre requiere bloque unsafe.
//        Usa Mutex<T> o AtomicXxx cuando haya múltiples hilos.
// =============================================================================

// Variable estática mutable global.
// ⚠️  Solo debe usarse desde un único hilo.
static mut CONTADOR: u32 = 0;

/// Incrementa el contador global.
///
/// # Safety
/// Debe llamarse desde un único hilo a la vez. Llamarla concurrentemente
/// desde múltiples hilos produce comportamiento indefinido (data race).
unsafe fn incrementar_contador(inc: u32) {
    unsafe {
        CONTADOR += inc;
    }
}

fn superpoder_3_estaticas_mutables() {
    println!("\n=== SUPERPODER 3: Variables Estáticas Mutables ===");

    // SAFETY: Este programa es de un solo hilo; no hay riesgo de data race.
    unsafe {
        incrementar_contador(5);
        incrementar_contador(3);
        // Leemos con un puntero crudo para evitar crear una referencia a static mut
        // (el compilador nos obligaría a usar #[allow(static_mut_refs)] de lo contrario).
        println!("CONTADOR = {}", *(&raw const CONTADOR));
    }
}

// =============================================================================
// SUPERPODER 4: IMPLEMENTAR TRAITS INSEGUROS
// =============================================================================
//
// ¿QUÉ ES? Un trait marcado con `unsafe trait` tiene invariantes que el
// compilador no puede verificar automáticamente. Al implementarlo con
// `unsafe impl`, prometes que cumplirás esos invariantes manualmente.
//
// ¿PARA QUÉ SIRVE?
//   - Implementar `Send` o `Sync` para tipos que contienen punteros crudos
//   - Crear traits de bajo nivel con contratos de seguridad complejos
//
// EJEMPLOS DE LA STD:
//   Send  → el tipo puede transferirse entre hilos de forma segura
//   Sync  → el tipo puede ser compartido entre hilos (&T puede enviarse a otro hilo)
//   El compilador los implementa automáticamente cuando puede verificarlo;
//   si tu tipo tiene raw pointers, debes implementarlos manualmente con unsafe impl.
// =============================================================================

/// Trait inseguro: quien lo implemente debe garantizar que `es_valido()`
/// nunca cause undefined behavior para ningún valor del tipo.
unsafe trait Validable {
    fn es_valido(&self) -> bool;
}

struct MiWrapper(i32);

// SAFETY: Para i32, cualquier valor es un entero válido; no hay UB posible.
unsafe impl Validable for MiWrapper {
    fn es_valido(&self) -> bool {
        self.0 >= 0
    }
}

fn superpoder_4_traits_inseguros() {
    println!("\n=== SUPERPODER 4: Traits Inseguros ===");

    let positivo = MiWrapper(10);
    let negativo = MiWrapper(-5);

    // Llamar métodos del trait es perfectamente seguro una vez implementado.
    println!("¿10 es válido?  {}", positivo.es_valido());
    println!("¿-5 es válido?  {}", negativo.es_valido());
}

// =============================================================================
// SUPERPODER 5: ACCEDER A CAMPOS DE UNIONS
// =============================================================================
//
// ¿QUÉ ES? Una `union` es como un `struct`, pero todos sus campos comparten
// la MISMA dirección de memoria. Solo uno de ellos contiene datos válidos
// en un momento dado.
//
// ¿PARA QUÉ SIRVE?
//   - Interoperar con unions de C (FFI)
//   - Implementar tipos de suma de bajo nivel sin overhead
//   - Reinterpretar bytes entre tipos (type punning)
//
// ¿POR QUÉ ES INSEGURO?
//   Rust no sabe qué campo está "activo" en un momento dado.
//   Si escribiste un f32 y lees como u32, obtienes los bytes crudos del float,
//   lo cual puede ser perfectamente válido o basura dependiendo del contexto.
// =============================================================================

// Union que puede contener un entero O un flotante (mismo espacio de memoria).
union EnteroOFlotante {
    entero:  i32,
    flotante: f32,
}

fn superpoder_5_unions() {
    println!("\n=== SUPERPODER 5: Unions ===");

    // Crear una union e inicializar UN campo:
    let u = EnteroOFlotante { entero: 42 };

    // Leer el campo activo es unsafe porque Rust no puede verificar cuál es.
    // SAFETY: Acabamos de asignar el campo 'entero', así que leerlo es válido.
    unsafe {
        println!("Como entero:  {}", u.entero);
        // Leer 'flotante' reinterpreta los mismos bytes como f32 — válido pero
        // el valor numérico carece de significado semántico aquí.
        println!("Como flotante (reinterpretación de bytes): {}", u.flotante);
    }

    // Caso de uso real: union para FFI con C
    // En C es común: union Data { int i; float f; char bytes[4]; };
    // En Rust lo representarías igual y accederías con unsafe.
}

// =============================================================================
// BONUS A: ABSTRACCIÓN SEGURA SOBRE CÓDIGO INSEGURO
// =============================================================================
//
// PATRÓN RECOMENDADO: encapsula el código inseguro dentro de una función
// pública segura, de modo que los usuarios externos nunca necesiten escribir
// unsafe por su cuenta.
//
// Ejemplo: implementación simplificada de split_at_mut
//   - Recibe un slice mutable y un índice
//   - Devuelve dos sub-slices mutables que no se solapan
//   - Internamente necesita unsafe porque el borrow checker no entiende
//     que los dos sub-slices son disjuntos
// =============================================================================
fn split_at_mut_manual(valores: &mut [i32], medio: usize) -> (&mut [i32], &mut [i32]) {
    let len = valores.len();
    assert!(medio <= len, "índice fuera de rango");

    let ptr = valores.as_mut_ptr(); // obtiene *mut i32 al primer elemento

    // SAFETY:
    //   - 'ptr' proviene de un slice válido de longitud 'len'
    //   - 'medio' <= 'len' (garantizado por el assert anterior)
    //   - Los dos rangos [0, medio) y [medio, len) son disjuntos,
    //     por lo que no hay aliasing mutable ilegal.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, medio),               // [0..medio]
            slice::from_raw_parts_mut(ptr.add(medio), len - medio), // [medio..len]
        )
    }
}

fn bonus_abstraccion_segura() {
    println!("\n=== BONUS A: Abstracción Segura sobre Código Inseguro ===");

    let mut v = vec![10, 20, 30, 40, 50, 60];

    // El llamador usa la función como si fuera completamente segura — lo es.
    let (izquierda, derecha) = split_at_mut_manual(&mut v, 3);
    println!("Izquierda: {:?}", izquierda); // [10, 20, 30]
    println!("Derecha:   {:?}", derecha);   // [40, 50, 60]

    // Podemos modificar ambas mitades de forma independiente:
    izquierda[0] = 99;
    derecha[2]   = 77;
    println!("Tras modificar: izq={:?}, der={:?}", izquierda, derecha);
}

// =============================================================================
// BONUS B: USO DE extern "C" PARA LLAMAR FUNCIONES DE C
// =============================================================================
//
// ¿QUÉ ES FFI? Foreign Function Interface — mecanismo para llamar funciones
// de otro lenguaje (normalmente C) desde Rust.
//
// ¿CÓMO FUNCIONA?
//   1. Declara el bloque  unsafe extern "C" { ... }
//   2. Lista las funciones con sus firmas exactas
//   3. Llámalas dentro de bloques unsafe
//
// ¿POR QUÉ ES INSEGURO?
//   - C no tiene las garantías de Rust (sin borrow checker, sin lifetimes)
//   - Rust no puede verificar que la función C cumpla sus contratos
//   - Puedes pasar un puntero nulo por accidente, etc.
//
// KEYWORD 'safe' (Rust 2024+): puedes marcar funciones FFI específicas como
// `safe` dentro del bloque `unsafe extern "C"` si sabes que no tienen
// condiciones de seguridad especiales (ej: abs, strlen, etc.).
// =============================================================================

// Declaramos funciones de la biblioteca estándar de C.
// El ABI "C" define la convención de llamada (cómo se pasan argumentos, etc.)
unsafe extern "C" {
    // abs() no tiene condiciones de unsafe: cualquier i32 es válido.
    safe fn abs(input: i32) -> i32;

    // strlen() SÍ tiene condición: el puntero debe ser válido y null-terminated.
    fn strlen(s: *const u8) -> usize;
}

fn bonus_extern_c() {
    println!("\n=== BONUS B: Llamadas a Funciones C via extern ===");

    // abs() fue marcada 'safe', así que NO necesita bloque unsafe.
    println!("abs(-42) = {}", abs(-42));
    println!("abs( 10) = {}", abs(10));

    // strlen() NO fue marcada safe, así que sí necesita unsafe.
    let texto = b"Hola, mundo!\0"; // slice de bytes con null-terminator
    // SAFETY: 'texto' es un slice válido con null-terminator al final.
    let longitud = unsafe { strlen(texto.as_ptr()) };
    println!("strlen('Hola, mundo!') = {}", longitud);
}

// =============================================================================
// RESUMEN RÁPIDO
// =============================================================================
//
//  Superpoder               │ Cuándo usarlo
// ─────────────────────────┼──────────────────────────────────────────────────
//  *const T / *mut T        │ FFI, estructuras de bajo nivel, optimizaciones
//  unsafe fn                │ Funciones con contratos que Rust no puede verificar
//  static mut               │ Estado global en sistemas single-thread o embebidos
//  unsafe trait/impl        │ Send/Sync manual, traits con invariantes complejos
//  union                    │ Interop con C, type punning de bajo nivel
// ─────────────────────────┴──────────────────────────────────────────────────
//
//  HERRAMIENTA RECOMENDADA: Miri (interprete de Rust que detecta UB en runtime)
//    cargo +nightly miri run
//    cargo +nightly miri test
//
//  LECTURA ADICIONAL: https://doc.rust-lang.org/nomicon/  (The Rustonomicon)
// =============================================================================