use std::io;
// use std::num::ParseIntError;

fn main() {
    println!("Enter a mathematical expression (e.g., 3 + 5 or 2 * 3 + 4):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    match evaluate_expression(input) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn evaluate_expression(input: &str) -> Result<i32, String> {
    let mut tokens = input.split_whitespace();

    let mut result = match tokens.next() {
        Some(token) => parse_operand(token)?,
        None => return Err("Invalid input".into()),
    };

    while let Some(op) = tokens.next() {
        let operand = match tokens.next() {
            Some(token) => parse_operand(token)?,
            None => return Err("Invalid expression".into()),
        };

        result = match op {
            "+" => add(result, operand),
            "-" => subtract(result, operand),
            "*" => multiply(result, operand),
            "/" => divide(result, operand)?,
            _ => return Err(format!("Invalid operation: {}", op)),
        };
    }

    Ok(result)
}

fn parse_operand(token: &str) -> Result<i32, String> {
    token
        .parse::<i32>()
        .map_err(|_| format!("Invalid number: {}", token))
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".into())
    } else {
        Ok(a / b)
    }
}
