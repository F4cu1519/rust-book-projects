// PROYECTO INTEGRADOR: Generics + Traits + Lifetimes
// ====================================================
// Sistema de Gestión de Tareas (Task Manager)
// Este proyecto combina los tres conceptos principales de Rust

use std::fmt;

// TRAIT: Define comportamiento común para tareas
// -----------------------------------------------
trait Task {
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
    fn is_completed(&self) -> bool;
    fn mark_completed(&mut self);
    
    // Método con implementación por defecto
    fn display(&self) -> String {
        format!(
            "[{}] {} - {}",
            if self.is_completed() { "✓" } else { " " },
            self.get_title(),
            self.get_description()
        )
    }
}

// STRUCT GENÉRICO: TodoTask con prioridad genérica
// -------------------------------------------------
#[derive(Debug)]
struct TodoTask<T> {
    title: String,
    description: String,
    completed: bool,
    priority: T,
}

impl<T> TodoTask<T> {
    fn new(title: String, description: String, priority: T) -> Self {
        TodoTask {
            title,
            description,
            completed: false,
            priority,
        }
    }
    
    fn get_priority(&self) -> &T {
        &self.priority
    }
}

// Implementación del trait Task para TodoTask
impl<T> Task for TodoTask<T> {
    fn get_title(&self) -> &str {
        &self.title
    }
    
    fn get_description(&self) -> &str {
        &self.description
    }
    
    fn is_completed(&self) -> bool {
        self.completed
    }
    
    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

// Implementación de Display solo para TodoTask con prioridades que implementan Display
impl<T: fmt::Display> fmt::Display for TodoTask<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} [Priority: {}]",
            self.display(),
            self.priority
        )
    }
}

// LIFETIMES: TaskManager que mantiene referencias a tareas
// ---------------------------------------------------------
struct TaskManager<'a, T> {
    tasks: Vec<&'a mut TodoTask<T>>,
    name: &'a str,
}

impl<'a, T> TaskManager<'a, T> {
    fn new(name: &'a str) -> Self {
        TaskManager {
            tasks: Vec::new(),
            name,
        }
    }
    
    fn add_task(&mut self, task: &'a mut TodoTask<T>) {
        self.tasks.push(task);
    }
    
    fn get_name(&self) -> &'a str {
        self.name
    }
    
    fn total_tasks(&self) -> usize {
        self.tasks.len()
    }
    
    fn completed_tasks(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_completed()).count()
    }
    
    fn pending_tasks(&self) -> usize {
        self.total_tasks() - self.completed_tasks()
    }
}

// Implementación con trait bound para mostrar todas las tareas
impl<'a, T: fmt::Display> TaskManager<'a, T> {
    fn display_all(&self) {
        println!("\n=== {} ===", self.name);
        println!("Total: {} | Completed: {} | Pending: {}",
                 self.total_tasks(),
                 self.completed_tasks(),
                 self.pending_tasks());
        println!("\nTasks:");
        
        for (i, task) in self.tasks.iter().enumerate() {
            println!("  {}. {}", i + 1, task);
        }
    }
}

// TRAIT ADICIONAL: Prioritizable
// -------------------------------
trait Prioritizable {
    fn priority_level(&self) -> u8;
}

// Enum para prioridades
#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Prioritizable for Priority {
    fn priority_level(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }
}

// FUNCIÓN GENÉRICA con trait bounds
// ----------------------------------
fn find_highest_priority<'a, T>(tasks: &[&'a TodoTask<T>]) -> Option<&'a TodoTask<T>>
where
    T: Prioritizable,
{
    tasks.iter()
        .filter(|t| !t.is_completed())
        .max_by_key(|t| t.get_priority().priority_level())
        .copied()
}

// STRUCT con múltiples lifetimes
// -------------------------------
struct TaskReference<'a, 'b, T> {
    task: &'a TodoTask<T>,
    manager_name: &'b str,
}

impl<'a, 'b, T: fmt::Display> TaskReference<'a, 'b, T> {
    fn new(task: &'a TodoTask<T>, manager_name: &'b str) -> Self {
        TaskReference {
            task,
            manager_name,
        }
    }
    
    fn display(&self) {
        println!("Task from '{}': {}", self.manager_name, self.task);
    }
}

// FUNCIÓN MAIN - DEMOSTRACIÓN
// ============================
fn main() {
    println!("=== SISTEMA DE GESTIÓN DE TAREAS ===\n");
    
    // Crear tareas con diferentes prioridades
    let mut task1 = TodoTask::new(
        String::from("Aprender Rust"),
        String::from("Completar el capítulo de Generics"),
        Priority::High,
    );
    
    let mut task2 = TodoTask::new(
        String::from("Hacer ejercicio"),
        String::from("Correr 5km"),
        Priority::Medium,
    );
    
    let mut task3 = TodoTask::new(
        String::from("Bug crítico"),
        String::from("Arreglar el sistema de auth"),
        Priority::Critical,
    );
    
    let mut task4 = TodoTask::new(
        String::from("Leer documentación"),
        String::from("Revisar docs de lifetimes"),
        Priority::Low,
    );
    
    // Crear un manager
    let manager_name = "Mi Task Manager";
    let mut manager = TaskManager::new(manager_name);
    
    // Agregar tareas al manager
    manager.add_task(&mut task1);
    manager.add_task(&mut task2);
    manager.add_task(&mut task3);
    manager.add_task(&mut task4);
    
    // Mostrar todas las tareas
    manager.display_all();
    
    // Completar algunas tareas
    println!("\n--- Completando tareas ---");
    task1.mark_completed();
    task2.mark_completed();
    
    manager.display_all();
    
    // Encontrar la tarea de mayor prioridad pendiente
    println!("\n--- Tarea de mayor prioridad pendiente ---");
    let tasks_refs: Vec<&TodoTask<Priority>> = vec![&task1, &task2, &task3, &task4];
    
    if let Some(highest) = find_highest_priority(&tasks_refs) {
        println!("Deberías trabajar en: {}", highest);
    }
    
    // Demostración de TaskReference con múltiples lifetimes
    println!("\n--- Referencias a tareas ---");
    let task_ref = TaskReference::new(&task3, manager_name);
    task_ref.display();
    
    // Tareas con prioridad numérica (otro tipo genérico)
    println!("\n--- Tareas con prioridad numérica ---");
    let mut numeric_task = TodoTask::new(
        String::from("Tarea numérica"),
        String::from("Ejemplo con prioridad 1-10"),
        8,
    );
    
    println!("{}", numeric_task);
    
    // Ejemplo con String como prioridad
    let mut string_task = TodoTask::new(
        String::from("Tarea flexible"),
        String::from("Puede usar cualquier tipo"),
        String::from("Urgente"),
    );
    
    println!("{}", string_task);
    
    println!("\n=== FIN DEL DEMO ===");
}