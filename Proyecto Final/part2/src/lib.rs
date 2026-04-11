// ─────────────────────────────────────────────────────────────────────────────
// src/lib.rs
// ─────────────────────────────────────────────────────────────────────────────

use std::{
    sync::{
        // Arc (Atomic Reference Counting): puntero inteligente para ownership
        //     COMPARTIDO entre múltiples hilos.
        //     Lleva un contador interno de cuántos Arc apuntan al mismo dato.
        //     Cuando el contador llega a 0, el dato se libera (Drop automático).
        //     Arc::clone() NO clona el dato — solo incrementa el contador atómicamente.
        Arc,
        // Mutex (Mutual Exclusion): garantiza acceso EXCLUSIVO a un dato.
        //     Solo un hilo puede tener el lock en un momento dado.
        //     Otros hilos que intenten hacer .lock() quedan BLOQUEADOS hasta que
        //     el primero libere el lock (al dropear el MutexGuard).
        Mutex,
        // mpsc: Multiple Producer Single Consumer.
        //       Múltiples hilos pueden enviar (Sender), uno solo puede recibir (Receiver).
        //       Es el canal de comunicación entre el pool y los Workers.
        mpsc,
    },
    // thread: para thread::spawn() y thread::JoinHandle
    thread,
};

// Type alias: le damos un nombre corto a un tipo complejo.
// Box<dyn FnOnce() + Send + 'static> significa:
//   Box<...>    → puntero en el heap con tamaño conocido en compile time.
//                 Necesario porque cada closure tiene un tipo DISTINTO e inconocible.
//   dyn         → "dispatch dinámico" — el tipo concreto se resuelve en runtime.
//   FnOnce()    → el closure se puede llamar UNA sola vez (consume sus capturas).
//                 Es el menos restrictivo de los 3 traits de closure (Fn, FnMut, FnOnce).
//   + Send      → el closure puede MOVERSE entre hilos de forma segura.
//   + 'static   → el closure no contiene referencias con lifetime acotado.
//                 Es necesario porque el hilo puede vivir más que el scope donde fue creado.
type Job = Box<dyn FnOnce() + Send + 'static>;

// `pub` → visible desde main.rs (y desde cualquier código externo al crate).
pub struct ThreadPool {
    // Vec<Worker>: vector dinámico de Workers.
    // El ThreadPool es dueño (owner) de todos sus Workers.
    workers: Vec<Worker>,

    // mpsc::Sender<Job>: el extremo de envío del canal.
    // ThreadPool lo usa en execute() para encolar Jobs.
    // Cuando ThreadPool se dropea, este Sender se cierra →
    //   los Workers en recv() recibirán Err → pueden saber que deben terminar.
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Crea un nuevo ThreadPool con `size` hilos.
    ///
    /// # Panics
    /// Hace panic si `size` es 0.
    pub fn new(size: usize) -> ThreadPool {

        // assert! es una macro que hace panic si la condición es false.
        // Equivale a: if !(size > 0) { panic!(...) }
        // Un pool de 0 hilos no puede ejecutar nada → error irrecuperable.
        assert!(size > 0);

        // mpsc::channel() crea el canal y devuelve una tupla (Sender, Receiver).
        // Desestructuramos directamente con pattern matching en el let.
        // El canal es genérico: como Job es nuestro type alias, Rust infiere
        // que es mpsc::channel::<Job>().
        let (sender, receiver) = mpsc::channel();

        // Envolvemos receiver en Arc<Mutex<>>:
        //   Mutex<receiver> → solo un Worker puede llamar .recv() a la vez.
        //   Arc<...>        → múltiples Workers pueden tener un puntero al mismo Mutex.
        // Sin Arc: no podríamos darle el receiver a más de un Worker (violaría ownership).
        // Sin Mutex: múltiples hilos accediendo a receiver simultáneamente = data race.
        let receiver = Arc::new(Mutex::new(receiver));
        // `receiver` ahora es de tipo Arc<Mutex<mpsc::Receiver<Job>>>

        // Vec::with_capacity(size): crea un Vec vacío pero con memoria pre-reservada
        // para `size` elementos. Evita re-allocations al hacer push().
        // Es una optimización respecto a Vec::new() cuando sabemos el tamaño final.
        let mut workers = Vec::with_capacity(size);

        // Creamos exactamente `size` Workers, numerados 0..size (exclusive).
        for id in 0..size {
            // Arc::clone(&receiver): NO clona el dato subyacente.
            //   Solo incrementa el contador de referencias atómicamente.
            //   Cada Worker recibe su propio Arc que apunta al MISMO Mutex<Receiver>.
            //   El Arc original y todos los clones apuntan al mismo dato en heap.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Construimos el ThreadPool con sintaxis de inicialización de struct.
        // `workers` y `sender` son los campos definidos arriba.
        ThreadPool { workers, sender }
    }

    // Método genérico: F es un tipo que se determina en cada llamada.
    // &self → referencia inmutable al pool (no consume ni muta el pool).
    // where clause: restricciones sobre F separadas del nombre del método,
    //               más legible que ponerlas inline.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Envolvemos el closure en Box para convertirlo al tipo Job.
        // Box::new(f) mueve f al heap y devuelve un puntero con tamaño fijo.
        // Ahora `job` es de tipo Box<dyn FnOnce() + Send + 'static> = Job.
        let job = Box::new(f);

        // self.sender.send(job) envía el Job por el canal.
        // Devuelve Result<(), SendError<Job>>.
        // .unwrap() → solo falla si el Receiver se cerró (todos los Workers terminaron).
        //             Durante operación normal nunca ocurre, por eso usamos unwrap.
        self.sender.send(job).unwrap();
    }
}

// `struct Worker` es privada (sin `pub`) — detalle interno del pool.
// main.rs no necesita saber que Worker existe.
struct Worker {
    // id: para identificar el Worker en logs/debug.
    // En producción sería útil para métricas y trazabilidad.
    id: usize,

    // JoinHandle<()>: handle al hilo del SO creado por thread::spawn.
    // El () indica que el hilo no retorna ningún valor.
    // Guardarlo evita que el hilo sea "detached" (daemon).
    // Con JoinHandle podríamos llamar .join() para esperar que termine.
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // Privado también — solo ThreadPool::new() llama a esto.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        // thread::spawn() crea un nuevo hilo del SO y ejecuta el closure en él.
        // Devuelve JoinHandle<T> donde T es lo que retorna el closure.
        // Como nuestro loop es infinito, el closure retorna () implícitamente.
        //
        // `move` → el closure TOMA OWNERSHIP de las variables que captura.
        //          Sin `move`, `receiver` sería una referencia que podría
        //          vivir menos que el hilo → el compilador rechazaría el código.
        //          Con `move`, el Arc se mueve al closure → el hilo es dueño de su Arc.
        let thread = thread::spawn(move || {

            // Loop infinito: el Worker siempre está listo para el próximo Job.
            // Solo termina si recv() devuelve Err (el canal se cerró).
            // Con unwrap() hacemos panic en ese caso — en producción manejaríamos el error.
            loop {
                // Esta línea tiene 3 operaciones encadenadas — las analizamos de izquierda a derecha:
                //
                // receiver.lock()
                //   → intenta adquirir el Mutex. Si otro Worker lo tiene, BLOQUEA este hilo
                //     hasta que se libere. Devuelve Result<MutexGuard<Receiver<Job>>, PoisonError>.
                //   → PoisonError ocurre si un hilo hizo panic MIENTRAS tenía el lock.
                //
                // .unwrap()
                //   → desempaqueta el MutexGuard o hace panic si el Mutex está envenenado.
                //   → MutexGuard<T> implementa Deref → podemos llamar métodos de Receiver sobre él.
                //
                // .recv()
                //   → BLOQUEA el hilo hasta que llegue un Job en el canal.
                //   → Devuelve Ok(Job) si recibió algo, o Err si el Sender se cerró.
                //   → Esta es la "sala de espera" del Worker — aquí pasa la mayor parte del tiempo.
                //
                // .unwrap()
                //   → desempaqueta el Job o hace panic si el canal se cerró.
                //
                // CLAVE — por qué usamos `let` y no `while let`:
                //   Con `let`, el MutexGuard retornado por .lock() se DROPEA
                //   al final de esta sentencia (antes del ;).
                //   Cuando MutexGuard se dropea → el Mutex se LIBERA automáticamente.
                //   Así, mientras job() se ejecuta, el Mutex está LIBRE y
                //   otros Workers pueden tomar el lock y recibir su propio Job.
                //
                //   Con `while let Ok(job) = receiver.lock()...recv()`:
                //   el MutexGuard viviría hasta el final del bloque while,
                //   manteniendo el lock DURANTE toda la ejecución de job().
                //   → los otros Workers quedarían bloqueados → comportamiento serial. ❌
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} ejecutando job.");

                // Llamamos al closure. La sintaxis job() funciona porque
                // Box<dyn FnOnce()> implementa el trait FnOnce gracias a una
                // impl especial en la biblioteca estándar de Rust.
                // Esto CONSUME el Job (FnOnce → se llama una sola vez).
                // Después de esta línea, job se dropea y la memoria en heap se libera.
                job();

                // Aquí termina el loop body. Volvemos al inicio del loop
                // y el Worker se bloquea nuevamente en .recv() esperando el próximo Job.
            }
        });

        // Construimos y retornamos el Worker.
        // La función retorna Worker implícitamente (sin return, sin punto y coma).
        Worker { id, thread }
    }
}