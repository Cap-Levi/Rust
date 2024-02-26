use std::io;

fn main() {
    let mut input_1 = String::new();
    
    println!("\nEnter the first number: ");
    io::stdin().read_line(&mut input_1).expect("\nNot a valid string");
    let num_1: f64 = input_1.trim().parse().expect("\nNot a valid number");

    let mut op = String::new();

    println!("\nSelect the operation: \n1. Add \n2. Subtract \n3. Multiply \n4. Divide\n");

    println!("Enter the choice: ");
    io::stdin().read_line(&mut op).expect("\nNot a valid string");
    let operation: u8 = op.trim().parse().expect("\nNot a valid number");

    let mut input_2 = String::new();
    
    println!("\nEnter the second number: ");
    io::stdin().read_line(&mut input_2).expect("\nNot a valid string");
    let num_2: f64 = input_2.trim().parse().expect("\nNot a valid number");

    let selected_operation;
    let result: f64;

    if operation == 1 {
        selected_operation = Operation::Add{ x: num_1, y: num_2};
        result = calculate(selected_operation);
        println!("\nCalculator Result: {num_1} + {num_2} = {result}\n");
    } else if operation == 2 {
        selected_operation = Operation::Subtract{ x: num_1, y: num_2};
        result = calculate(selected_operation);
        println!("\nCalculator Result: {num_1} - {num_2} = {result}\n");
    } else if operation == 3 {
        selected_operation = Operation::Multiply{ x: num_1, y: num_2};
        result = calculate(selected_operation);
        println!("\nCalculator Result: {num_1} * {num_2} = {result}\n");
    } else if operation == 4 {
        selected_operation = Operation::Divide{ x: num_1, y: num_2};
        result = calculate(selected_operation);
        println!("\nCalculator Result: {num_1} / {num_2} = {result}\n");
    }

}

enum Operation {
    Add { x: f64, y: f64 }, 
    Subtract{ x: f64, y: f64 }, 
    Multiply{ x: f64, y: f64 }, 
    Divide{ x: f64, y: f64 },
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add {x, y} => x + y,
        Operation::Subtract {x, y} => x - y,
        Operation::Multiply {x, y} => x * y,
        Operation::Divide {x, y} => x / y,
    }
}
