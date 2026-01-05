fn a1() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn a2() {
    // se define una constante en mayusculas por convencion
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");
}

fn a3(){
    let x = 5;
    // shadowing
    let x = x + 1;
    {
        // shadowing en un bloque separado, todo dentro de este bloque funciona solo para este bloque
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn main(){
    let mut entrada = String::new();
    println!("Input 1 for a1, 2 for a2, 3 for a3:");
    std::io::stdin().read_line(&mut entrada).expect("Failed to read line");
    let entrada = entrada.trim();
    if entrada == "1" {
        a1();
    } else if entrada == "2" {
        a2();
    } else if entrada == "3" {
        a3();
    }
    else {
        println!("No function found, please input 1, 2 or 3");
    }
}