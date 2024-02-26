use std::io;

fn main() {
    let first_num = read_num_input("Enter the first number: ");
    let operation = read_operation_input("Enter an operation to perform (+, -, *, /): ");
    let second_num = read_num_input("Enter the second number: ");

    let operation_enum = match operation {
        '+' => Operation::Add,
        '-' => Operation::Subtract,
        '*' => Operation::Multiply,
        '/' => Operation::Divide,
        _ => {
            println!("Error: Invalid operation.");
            std::process::exit(1);
        }
    };

    match calculate(operation_enum, first_num, second_num) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operation: Operation, x: f64, y: f64) -> Result<f64, String> {
    match operation {
        Operation::Add => Ok(x + y),
        Operation::Subtract => Ok(x - y),
        Operation::Multiply => Ok(x * y),
        Operation::Divide => {
            if y != 0.0 {
                Ok(x / y)
            } else {
                Err("Division by zero is not allowed.".to_string())
            }
        }
    }
}

fn read_num_input(message: &str) -> f64 {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn read_operation_input(message: &str) -> char {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.len() == 1 && "+-*/".contains(input) {
            return input.chars().next().unwrap();
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}