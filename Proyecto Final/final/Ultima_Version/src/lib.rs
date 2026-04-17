// ─────────────────────────────────────────────────────────────────────────────
// src/lib.rs
// ─────────────────────────────────────────────────────────────────────────────

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE ALIAS: Job
// ═══════════════════════════════════════════════════════════════════════════════

// Box<dyn FnOnce() + Send + 'static> es un tipo muy largo.
// type Job = ... nos permite escribir Job en lugar de la ruta completa.
//
// Qué significa cada parte:
//
//   Box<...>
//     → puntero inteligente que apunta a datos en el HEAP.
//     → Necesario porque cada closure tiene un TIPO DISTINTO.
//     → Rust necesita un tamaño conocido en compile-time, y Box proporciona
//       un puntero de tamaño fijo.
//
//   dyn FnOnce()
//     → "dispatch dinámico" — el tipo concreto del closure se decide en RUNTIME.
//     → FnOnce significa que el closure se puede llamar EXACTAMENTE UNA VEZ.
//     → () sin parámetros → el closure no toma argumentos.
//     → Comparación:
//       - Fn() → se puede llamar múltiples veces, sin mutar capturas
//       - FnMut() → se puede llamar múltiples veces, puede mutar capturas
//       - FnOnce() → se llama una sola vez, CONSUME las capturas
//       FnOnce es el menos restrictivo (acepta todos los closures).
//
//   + Send
//     → trait marker que garantiza que el closure es "thread-safe" para ENVIAR
//       entre hilos.
//     → Necesario porque el Job viajará por el canal MPSC a otro hilo.
//
//   + 'static
//     → el closure NO contiene referencias con lifetime acotado.
//     → Solo puede referenciar datos 'static (constantes, heap, etc).
//     → Necesario porque el Job puede ejecutarse mucho después de que el
//       scope donde se creó haya terminado.
//
// Ejemplo de closure que NO sería Send + 'static:
//   let s = "hola";           // &str local
//   let f = || println!("{s}");  // captura &s — tiene lifetime acotado
//   // f no implementa 'static porque &s no vive para siempre
//
type Job = Box<dyn FnOnce() + Send + 'static>;

// ═══════════════════════════════════════════════════════════════════════════════
// STRUCT: ThreadPool
// ═══════════════════════════════════════════════════════════════════════════════

pub struct ThreadPool {
    // Vec<Worker>: vector dinámico de Workers.
    // El ThreadPool es el DUEÑO de todos los Workers.
    // Cuando ThreadPool se dropea, automáticamente se dropean todos sus Workers.
    workers: Vec<Worker>,

    // ⚠️ GRACEFUL SHUTDOWN CHANGE #1 ⚠️
    // ANTES: sender: mpsc::Sender<Job>
    // AHORA: sender: Option<mpsc::Sender<Job>>
    //
    // ¿Por qué Option?
    //   - En el método Drop, necesitamos hacer drop(sender) explícitamente.
    //   - drop() consume el valor (toma ownership).
    //   - Si sender fuese mpsc::Sender<Job> directamente, no podríamos 
    //     "sacarlo" del campo sin mover de un &mut self.
    //   - Con Option<Sender>, podemos usar .take() para extraer el Sender.
    //   - .take() reemplaza el interior del Option con None y retorna el valor anterior.
    //
    // Analógía: es como tener una caja (Option).
    //   - Some(sender) → la caja tiene algo adentro.
    //   - .take() → abrimos la caja, extraemos el contenido, cerramos la caja vacía.
    //   - None → la caja ahora está vacía.
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Crea un nuevo ThreadPool con `size` hilos internos (Workers).
    ///
    /// # Panics
    /// Hace panic si `size` es 0.
    ///
    /// # Ejemplo
    /// ```
    /// let pool = ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool size debe ser mayor que 0");

        // mpsc::channel() crea un CANAL de comunicación entre hilos.
        // Devuelve (Sender, Receiver).
        // 
        // MPSC = Multiple Producers, Single Consumer
        //   - Múltiples hilos pueden llamar .send() (Sender clonado).
        //   - Solo UN hilo puede llamar .recv() (el Receiver original).
        //
        // En nuestro caso:
        //   - Main + cualquier código que tenga un Sender → PRODUCTORES (envían Jobs)
        //   - Workers → CONSUMIDOR (reciben Jobs)
        //
        // El canal es genérico: Rust infiere mpsc::channel::<Job>()
        // basándose en cómo lo usamos.
        let (sender, receiver) = mpsc::channel();

        // Envolvemos el receiver en Arc<Mutex<>>:
        //
        // Arc = Atomic Reference Counting (puntero inteligente compartido)
        //   - Permite que múltiples propietarios apunten al MISMO dato.
        //   - Lleva un contador de cuántos Arc apuntan a él.
        //   - Cuando el contador llega a 0, se libera automáticamente (Drop).
        //   - Arc::clone() NO clona el dato — solo incrementa el contador.
        //   - Atómico = thread-safe (usa instrucciones CPU atómicas).
        //
        // Mutex = Mutual Exclusion (candado exclusivo)
        //   - Garantiza acceso EXCLUSIVO al receiver.
        //   - Solo UN hilo puede tener el lock en un momento.
        //   - .lock() devuelve un guard que libera el lock cuando se dropea.
        //   - Los demás hilos que llamen .lock() quedan BLOQUEADOS hasta que 
        //     se libere.
        //
        // ¿Por qué ambos?
        //   - Arc: para que múltiples Workers tengan un pointer al mismo receiver.
        //   - Mutex: para que solo UN Worker pueda llamar .recv() a la vez
        //     (evita data races).
        //
        // Sin Arc: no podríamos darle receiver a más de un Worker (violaría ownership).
        // Sin Mutex: múltiples hilos llamando .recv() simultáneamente = undefined behavior.
        let receiver = Arc::new(Mutex::new(receiver));

        // Vec::with_capacity(size) crea un vector vacío con memoria
        // pre-reservada para `size` elementos.
        //
        // Sin with_capacity:
        //   Vec::new() crea un vector vacío.
        //   Cuando hacemos push() y se llena, Rust reasloca toda la memoria.
        //   Caros: buscar espacio, copiar datos.
        //
        // Con with_capacity:
        //   Rust reserva de una sola vez la memoria para `size` elementos.
        //   Los push() subsiguientes NO requieren realocación.
        //   Eficiente: menos syscalls, menos copia.
        //
        // Nos importa porque los Workers están en el heap y queremos
        // evitar realocaciones innecesarias.
        let mut workers = Vec::with_capacity(size);

        // Creamos exactamente `size` Workers.
        // Rango 0..size es EXCLUSIVO en el extremo: crea [0, 1, 2, ..., size-1]
        for id in 0..size {
            // Arc::clone(&receiver) incrementa el contador de referencias.
            // NO clona el dato subyacente.
            //
            // Visualización de memoria:
            //   Heap: [Mutex<Receiver>] ← el dato real está una sola vez
            //   Stack: receiver (Arc) → Heap (contador: 1)
            //   Stack: Arc(clone1) → Heap (contador: 2)
            //   Stack: Arc(clone2) → Heap (contador: 3)
            //   ...
            //   Stack: Arc(clone4) → Heap (contador: 4)
            //
            // Cuando cada clone se dropea, el contador decrementa.
            // Cuando contador llega a 0, la memoria en Heap se libera.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Construimos ThreadPool con sintaxis de inicialización.
        // Asignamos Some(sender) porque ahora sender es Option<...>.
        // Explicaremos por qué Some() al dropear el pool.
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Encola un closure para que un Worker lo ejecute.
    ///
    /// # Genéricos
    /// F: el tipo del closure (determinado automáticamente en cada llamada).
    ///
    /// # where clause
    /// Las restricciones sobre F se escriben en el where clause para legibilidad.
    ///   F: FnOnce() → el closure se puede llamar una sola vez.
    ///   + Send → es seguro enviar entre hilos.
    ///   + 'static → no contiene referencias con lifetime acotado.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Box::new(f) envuelve el closure en una caja (Box) y lo coloca en el heap.
        // Esto lo convierte al tipo Job (nuestro type alias).
        //
        // ¿Por qué Box?
        //   - Cada closure tiene un TIPO ÚNICO determinado por el compilador.
        //   - Rust necesita conocer el tamaño de todo en compile-time.
        //   - dyn FnOnce() es "tamaño desconocido", por eso necesitamos un puntero.
        //   - Box es un puntero de tamaño FIJO que puede apuntar a cualquier closure.
        //
        // ¿Por qué heap?
        //   - Los closures pueden capturar variables de tamaño variable.
        //   - No sabemos cuánto espacio ocupa cada closure en compile-time.
        //   - El heap es perfecto para datos de tamaño dinámico.
        let job = Box::new(f);

        // self.sender.as_ref().unwrap().send(job)
        //
        // self.sender es Option<Sender>.
        // .as_ref() convierte Option<Sender> → Option<&Sender>.
        //   - Some(s) → Some(&s)
        //   - None → None
        //   Permite obtener una referencia sin consumir el Option.
        //
        // .unwrap() desempaqueta el Some(&Sender) o hace panic si es None.
        //   - En operación normal, sender SIEMPRE es Some.
        //   - Solo sería None después de que Drop lo convirtió en None,
        //     y nadie debería llamar execute() después de que ThreadPool se dropea.
        //   - Por eso unwrap() es seguro aquí.
        //
        // .send(job) envía el Job por el canal al Receiver (los Workers).
        // Devuelve Result<(), SendError<Job>>.
        //   - Ok(()) si el job fue encolado exitosamente.
        //   - Err si el Receiver fue dropeado (todos los Workers terminaron).
        //
        // .unwrap() desempaqueta el Ok o hace panic si es Err.
        //   - Durante operación normal nunca falla.
        //   - En producción, manejaríamos el error en lugar de panic.
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// IMPL DROP: Graceful Shutdown
// ═══════════════════════════════════════════════════════════════════════════════

// ⚠️ GRACEFUL SHUTDOWN CHANGE #2 ⚠️
// Implementamos el trait Drop para ThreadPool.
//
// Drop es un trait especial que Rust llama automáticamente cuando un valor
// se va del scope.
//
// Nuestro plan de apagado graceful:
//   1. Cerrar el canal Sender.
//      → Los Workers en .recv() recibirán error.
//      → Verán el error y saldrán del loop infinito.
//   2. Esperar a que cada Worker termine su hilo.
//      → Llamamos .join() en cada JoinHandle.
//      → Esto BLOQUEA el hilo principal hasta que el Worker termine.
//   3. Cuando todos los Workers han sido "joinados", el programa puede terminar.
//
// ¿Por qué Drop y no destructor manual?
//   - Rust obliga a limpiar resources automáticamente.
//   - Drop implementa el patrón RAII (Resource Acquisition Is Initialization).
//   - Es imposible olvidar limpiar — Rust lo hace por ti.
//   - Es explícito — cualquiera que lea el código ve qué pasa al dropear.

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // PASO 1: Cerrar el canal Sender.
        //
        // drop(self.sender.take())
        //
        // self.sender es Option<Sender>.
        // .take() extrae el valor del Option y reemplazalo con None.
        //   - Some(s) → retorna s y deja None en self.sender
        //   - None → retorna None (no pasa nada)
        //
        // drop(...) consume el valor retornado, invocando Drop en él.
        // Cuando Sender se dropea, el canal se CIERRA.
        //
        // Resultado:
        //   - Los 4 Workers están bloqueados en .recv() esperando un Job.
        //   - Cuando el canal se cierra, .recv() devuelve Err.
        //   - Los Workers ven el Err en el match y hacen break.
        //   - Los hilos de los Workers salen del loop infinito.
        //
        // IMPORTANTE: Este es el MECANISMO CRÍTICO del apagado graceful.
        // Sin cerrar el Sender, los Workers permanecerían bloqueados en .recv()
        // para siempre → el programa nunca terminaría (deadlock).
        drop(self.sender.take());

        println!("Cerrando el servidor. Esperando a que los Workers terminen...");

        // PASO 2: Iterar sobre los Workers y hacer join() en cada uno.
        //
        // self.workers.drain(..)
        //
        // .drain(..) es un método de Vec que:
        //   - Extrae TODOS los elementos del vector (rango .. = todo).
        //   - Los retorna en un iterador.
        //   - Deja el vector vacío después.
        //
        // ¿Por qué drain() y no un for loop regular?
        //   - for worker in &mut self.workers { worker.thread.join() }
        //   - Esto NO funcionaría porque:
        //     - Necesitamos ownership de worker.thread para llamar .join().
        //     - Pero &mut worker solo da una referencia mutable.
        //     - Para obtener ownership, necesitamos extraerlo del Worker.
        //   - drain() extrae el ownership de cada Worker.
        //   - Ahora podemos acceder a worker.thread sin referencias.
        //
        // Iteración:
        //   - Primer worker extraído y entra al bloque { ... }
        //   - Después sale del scope → se dropea automáticamente
        //   - Siguiente worker, repeat.
        for mut worker in self.workers.drain(..) {
            println!("Cerrando Worker {}", worker.id);

            // PASO 2.1: Extraer el JoinHandle del Worker.
            //
            // ⚠️ GRACEFUL SHUTDOWN CHANGE #3 ⚠️
            // ANTES: worker.thread era thread::JoinHandle<()> directamente.
            // AHORA: worker.thread es Option<thread::JoinHandle<()>>.
            //
            // ¿Por qué cambiar a Option?
            //   - .join() consume el JoinHandle (toma ownership).
            //   - En el Worker::new() hacemos worker.thread = thread::spawn().
            //   - Si luego necesitamos hacer .join() en Drop, tenemos un problema:
            //     No podemos mover de un campo si solo tenemos &mut self.
            //   - Solución: usar Option<JoinHandle>.
            //   - .take() extrae el JoinHandle de Some(jh) → Ok(jh) y deja None.
            //
            // if let Some(thread) = worker.thread.take()
            //   - Si worker.thread es Some(jh): entra al bloque con jh.
            //   - Si worker.thread es None: salta el bloque.
            //   - Esto es seguro — no hacemos panic en None.
            //
            // En nuestro caso, thread SIEMPRE será Some porque solo lo
            // convertimos en None aquí en Drop. Pero defensivamente,
            // el if let maneja el caso None sin error.
            if let Some(thread) = worker.thread.take() {
                // PASO 2.2: Hacer join() en el JoinHandle.
                //
                // thread.join() espera a que el hilo del Worker termine.
                //
                // Qué pasa internamente:
                //   - join() bloquea el hilo PRINCIPAL hasta que el Worker termine.
                //   - El Worker está en su loop, esperando .recv().
                //   - Pero ya cerramos el Sender, así que .recv() retorna Err.
                //   - El Worker ve el Err, hace break, sale del loop.
                //   - El hilo del Worker ejecuta el return implícito.
                //   - El Worker termina.
                //   - El join() se desbloquea en el hilo principal.
                //   - Continuamos con el siguiente Worker.
                //
                // Resultado de join():
                //   - Ok(()) si el hilo terminó normalmente.
                //   - Err si el hilo hizo panic.
                //
                // .unwrap() desempaqueta Ok(()) o hace panic si Err.
                //   - En nuestro código, el Worker nunca hace panic (excepto en unwrap()),
                //     así que normalmente retorna Ok.
                //   - En producción, manejaríamos los Err de panic mejor.
                thread.join().unwrap();
            }
        }

        println!("Todos los Workers han terminado. Servidor apagado.");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// STRUCT: Worker
// ═══════════════════════════════════════════════════════════════════════════════

// `struct Worker` es PRIVADA (sin `pub`).
// Es un detalle implementación interna del ThreadPool.
// main.rs nunca interactúa directamente con Worker.
struct Worker {
    // id: identificador del Worker (0, 1, 2, 3 para un pool de 4).
    // Útil para debugging, logging, métricas.
    id: usize,

    // ⚠️ GRACEFUL SHUTDOWN CHANGE #3 ⚠️
    // ANTES: thread: thread::JoinHandle<()>
    // AHORA: thread: Option<thread::JoinHandle<()>>
    //
    // JoinHandle<T> representa un hilo del SO creado por thread::spawn.
    // El <()> significa que el hilo no retorna ningún valor.
    //
    // ¿Por qué cambiar a Option?
    //   - Necesitamos poder hacer .take() en Drop para extraer el JoinHandle.
    //   - Si fuese JoinHandle directamente, estaría "movido" al crear el Worker,
    //     y no podríamos extraerlo después.
    //   - Con Option<JoinHandle>, podemos:
    //     - Crear el Worker: thread = Some(JoinHandle)
    //     - Más tarde en Drop: thread.take() extrae el JoinHandle
    //     - Llamar .join() en el JoinHandle extraído
    //
    // Nota: dentro del loop del Worker, nunca accedemos a self.thread.
    // El loop está en el closure de thread::spawn, que es una closure aislada.
    // Así que cambiar a Option no afecta la lógica del Worker.
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // fn new() es PRIVADA también — solo ThreadPool::new() la llama.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // thread::spawn(move || { ... })
        //
        // thread::spawn crea un NUEVO HILO del sistema operativo.
        //   - El closure se ejecuta EN ESE NUEVO HILO (concurrentemente).
        //   - Devuelve JoinHandle<T> donde T es el tipo de retorno del closure.
        //   - Nuestro closure retorna () implícitamente (loop infinito hasta break).
        //
        // `move || { ... }`
        //   - move captura POR OWNERSHIP (no por referencia).
        //   - Sin move: receiver sería una referencia que puede vivir menos
        //     que el hilo → compilador rechaza (reference outlives hilo).
        //   - Con move: receiver se MUEVE al closure → el hilo es dueño de su Arc.
        //   - El hilo puede vivir más que el scope donde se creó.
        //
        // Implicaciones de `move`:
        //   - El closure no puede acceder a variables locales del scope.
        //   - Solo puede acceder a variables que captura (en este caso, receiver).
        //   - Ideal para hilos que necesitan ejecutarse independientemente.
        let thread = thread::spawn(move || {
            // ═══════════════════════════════════════════════════════════════════════
            // LOOP INFINITO DEL WORKER
            // ═══════════════════════════════════════════════════════════════════════

            loop {
                // receiver es Arc<Mutex<mpsc::Receiver<Job>>>.
                // receiver.lock()
                //   - Intenta adquirir el Mutex.
                //   - Si otro Worker lo tiene → BLOQUEA este hilo hasta que se libere.
                //   - Devuelve Result<MutexGuard<...>, PoisonError>.
                //   - MutexGuard es un RAII guard que libera el lock cuando se dropea.
                //
                // .unwrap()
                //   - Desempaqueta el Ok(MutexGuard) o panic en Err.
                //   - PoisonError ocurre si otro hilo hizo panic MIENTRAS tenía el lock.
                //   - En nuestro código, nunca pasa, así que unwrap es seguro.
                //   - Resultado: tenemos &Mutex -> podemos acceder a Receiver a través de Deref.
                //
                // .recv()
                //   - Método de Receiver: espera (BLOQUEA) hasta recibir un Job del canal.
                //   - Devuelve Result<Job, RecvError>.
                //   - Ok(Job): recibimos un Job de algún producer (main vía execute()).
                //   - Err: el Sender fue dropeado, no llegarán más Jobs.
                //
                // CLAVE DE TIMING:
                //   - receiver.lock() obtiene el Mutex.
                //   - receiver.lock().unwrap() retorna un MutexGuard.
                //   - .recv() se llama en el MutexGuard.
                //   - La sentencia `let job = ...;` termina (punto y coma).
                //   - Al terminar la sentencia, MutexGuard se DROPEA.
                //   - Cuando MutexGuard se dropea, el Mutex se LIBERA.
                //
                // ¿Por qué es importante?
                //   - El Mutex está BLOQUEADO SOLO durante .recv().
                //   - Una vez que recibimos el Job, liberamos el Mutex.
                //   - Otros Workers pueden ahora tomar el Mutex y llamar .recv().
                //   - Esto es CONCURRENCIA: múltiples Workers hacen trabajo en paralelo.
                //
                // MALA alternativa (no hagas esto):
                //   let guard = receiver.lock().unwrap();
                //   let job = guard.recv().unwrap();
                //   job();  // ← Mutex SIGUE BLOQUEADO mientras ejecutamos el Job
                //   
                // Esto sería SERIAL: solo UN Worker ejecutaría job por vez.
                // Los otros 3 estarían esperando el Mutex. No hay paralelismo.
                //
                // ═══════════════════════════════════════════════════════════════════════
                // ⚠️ GRACEFUL SHUTDOWN CHANGE #4 ⚠️
                // ANTES: let job = receiver.lock().unwrap().recv().unwrap();
                // AHORA: hacemos match sobre el resultado de recv()
                //
                // El cambio es CRÍTICO para graceful shutdown:
                //   - Con .unwrap(): si recv() retorna Err, hacemos panic → crash abrupto.
                //   - Con match: vemos el error y hacemos break → apagado graceful.
                //
                // Razón:
                //   - Cuando cerramos el Sender en Drop, todos los .recv() retornan Err.
                //   - Con unwrap, cada Worker hace panic → multiple panics.
                //   - Con match, cada Worker ve el error, imprime, y sale limpiamente.
                let message = receiver.lock().unwrap().recv();

                // match message:
                //   - Ok(job) rama → recibimos un Job, lo ejecutamos.
                //   - Err(_) rama → recv falló, el Sender se cerró, salimos.
                match message {
                    Ok(job) => {
                        println!("Worker {} recibió un job; ejecutando.", id);

                        // job() llama el closure.
                        // FnOnce() consume el Job — después no puedo volver a llamarlo.
                        // El closure se ejecuta en el contexto de este hilo Worker.
                        //
                        // Ejemplo: si el Job es pool.execute(|| handle_connection(stream)),
                        // aquí se ejecuta handle_connection(stream) en el hilo de este Worker.
                        // Mientras ejecuta, los otros 3 Workers siguen disponibles
                        // para otros Jobs. ← PARALELISMO.
                        job();

                        // job() se dropea automáticamente aquí.
                        // Si job capturaba datos (ej: stream), esos datos se dropean también.
                        // TcpStream::drop() cierra la conexión TCP automáticamente.
                    }

                    Err(_) => {
                        // recv() retornó Err → el Sender fue dropeado.
                        // Esto ocurre cuando Drop::drop(ThreadPool) cierra el Sender.
                        //
                        // Esto es la SEÑAL para terminar.
                        // El Worker sale del loop infinito.
                        println!("Worker {} desconectado; apagándose.", id);
                        break;
                        // Después del break:
                        //   - Salimos del loop infinito.
                        //   - La función (closure) termina.
                        //   - El hilo se termina.
                        //   - El JoinHandle en Drop se desbloquea del .join().
                        //   - Continuamos con el siguiente Worker en Drop.
                    }
                }

                // Aquí termina el body del loop.
                // Volvemos al inicio del loop.
                // Si no hicimos break, continuamos esperando en .recv().
            }
            // Aquí termina el closure (cuando break sale del loop).
            // El hilo termina.
        });

        // Envolvemos el JoinHandle en Some(thread).
        // Más tarde en Drop, haremos thread.take() para extraerlo.
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESUMEN: FLUJO DE GRACEFUL SHUTDOWN
// ═══════════════════════════════════════════════════════════════════════════════

// 1. main() crea un ThreadPool y recibe 2 requests con .take(2).
// 2. Al terminar main(), `pool` sale del scope.
// 3. Rust llama Drop::drop(&mut pool) automáticamente.
// 4. En Drop:
//    a. drop(self.sender.take()) → cierra el canal.
//       - Los 4 Workers en .recv() recibirán Err.
//       - Cada Worker ve Err en el match y hace break.
//       - Los 4 Workers salen del loop, sus hilos terminan.
//    b. for worker in self.workers.drain(..) → itera sobre Workers.
//       - Extrae el JoinHandle de cada Worker.
//       - Llama .join() → espera a que el hilo termine.
//       - Como ya salieron del loop en paso 4.a, el join() se desbloquea rápido.
// 5. Cuando todos los Workers han sido joinados, el programa termina normalmente.
//
// NO es un apagado abrupto (ctrl-C). Es GRACEFUL:
//   ✓ Los hilos salen de sus loops de forma controlada.
//   ✓ Los Jobs en progreso se completan.
//   ✓ Los recursos se limpian adecuadamente (Dealloc, cierre de conexiones).
//   ✓ El programa termina sin undefined behavior.