
use std::io;

fn main() {
    let (first_input, second_input) = get_user_inputs();
    let selected_operation = get_operation();

    // Converted the operation to a CalcOperations enum variant
    let operation = match selected_operation.trim().to_lowercase().as_str() {
        "add" => Operation::Add(first_input, second_input),
        "subtract" => Operation::Subtract(first_input, second_input),
        "multiply" => Operation::Multiply(first_input, second_input),
        "divide" => Operation::Divide(first_input, second_input),
        _ => panic!("Invalid operation"), // Handle invalid operation input
    };

    let answer = calculate(operation);

    println!("{}", answer)
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add (a, b) =>  a + b,
        Operation::Subtract (a, b) =>  a - b,
        Operation::Multiply (a, b) =>  a * b,
        Operation::Divide (a, b) =>  a / b,
    }
}

fn get_user_inputs() -> (f64, f64) {
    // Prompt the user to enter the first input
    println!("Enter the first number:");
    let first_input: f64 = get_input();

    // Prompt the user to enter the second input
    println!("Enter the second number:");
    let second_input: f64 = get_input();

    (first_input, second_input)
}

fn get_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number entered")
}

fn get_operation() -> String {
    // Prompt the user to enter the operation
    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    operation.trim().to_string()
}