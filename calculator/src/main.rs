use std::io;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// tolle Funktion
fn parse(input: &str) -> Option<Operator> {
    match input {
        "add" => Some(Operator::Add),
        "subtract" => Some(Operator::Subtract),
        "multiply" => Some(Operator::Multiply),
        "divide" => Some(Operator::Divide),
        _ => None,
    }
}

fn calculate(a: f64, b: f64, operator: Operator) -> Option<f64> {
    match operator {
        Operator::Divide => {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
        Operator::Add => Some(a + b),
        Operator::Subtract => Some(a - b),
        Operator::Multiply => Some(a * b),
    }
}

fn main() {
    println!("Please input a number:");

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line!");
    let a: f64 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    println!("Please input an operator: 'add', 'subtract', 'multiply' or 'divide' are allowed!");

    let operator = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        match parse(input.trim()) {
            Some(op) => break op,
            None => println!("Invalid input!"),
        }
    };

    println!("Please input a number:");

    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line!");
    let b: f64 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    let res = calculate(a, b, operator);
    match res {
        Some(result) => println!("The result is {}", result),
        None => println!("Cannot divide by zero!"),
    }
}
