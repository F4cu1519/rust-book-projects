use std::io;

struct Usuario {
    name : String,
    password : String,
    active : bool,
}

fn main() {
    println!("Ingrese su nombre de usuario: ");
    let mut input1  =  String::new();
    io::stdin().read_line(&mut input1).expect("Error al leer el contenido");
    println!("Ingrese su contraseña: ");
    let mut input2  =  String::new();
    io::stdin().read_line(&mut input2).expect("Error al leer el contenido");
    let usuario1 = Usuario{
        name: input1,
        password : input2,
        active : true,
    }; // No le debo poner el mut sino lo voy a cambiar por que salta warning
    println!("Usuario: {}", usuario1.name);
    println!("Activo: {}", usuario1.active);
    println!("Contraseña: {}", usuario1.password);
    // Si en lugar de usar esto uso #[derive(Debug)] me salta warning por que en el resto del codigo
    // si bien defino el usuario1 no lo estaba usando por lo que me sltaba eso ahora con el println no
}
