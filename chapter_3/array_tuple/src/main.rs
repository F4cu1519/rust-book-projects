use std::io;
fn main() {
    println!("In this example, you will learn about arrays and tuples in Rust.");
    println!("Type a numbre to continue 1 to array and 2 to tuple");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read line");
    match op.trim() {
        "1" => array(),
        "2" => tuple(),
        _ => println!("Invalid option"),
    }

}
fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.5, 1);
    println!("Tell me which element you want: 0, 1, or 2?");
    let mut indice = String::new();
    io::stdin().read_line(&mut indice).expect("Failed to read line");

    // En caso de tupla no usar el match si tienen distintos tipos
    if indice.trim() == "0" {
        let element = tup.0;
        println!("The value of the element at index {indice} is: {element}");
    } else if indice.trim() == "1" {
        let element = tup.1;
        println!("The value of the element at index {indice} is: {element}");
    } else if indice.trim() == "2" {
        let element = tup.2;
        println!("The value of the element at index {indice} is: {element}");
    } else {
        panic!("Index out of bounds");
    }
}

fn array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    if index >= a.len() {
        panic!("Index out of bounds");
    }else {
        let element = a[index];
        println!("The value of the element at index {index} is: {element}");
    }
}
