use std::io;

fn main() {
    println!("Simple Rust Calculator");
    println!("Choose operation: +  -  *  /");

    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read operation");

    println!("Enter first number:");
    let a = read_number();

    println!("Enter second number:");
    let b = read_number();

    let result = match operation.trim() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                println!("Error: Division by zero");
                return;
            }
            a / b
        }
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("Result = {}", result);
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");
    input.trim().parse().expect("Please enter a valid number")
}
