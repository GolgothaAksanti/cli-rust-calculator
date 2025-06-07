use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧮 Rust CLI Calculator");

    let num1 = read_number("Enter the first number: ")?;
    let num2 = read_number("Enter the second number: ")?;
    let op = read_input("Enter an operator(+, -, *, / ) ")?;

    let result = calculate(num1, num2, &op)?;
    println!("Result: {}", result);

    Ok(())
}

fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_number(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
    loop {
        let input = read_input(prompt)?;

        if input.to_lowercase() == "q" {
            return Err("Exiting...".into());
        }

        match input.parse::<f64>() {
            Ok(num) => return Ok(num),
            Err(_) => {
                println!("❌ Invalid number. Enter a valid number or 'q' to quit.");
                continue;
            }
        }
    }
}

fn calculate(a: f64, b: f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" | "x" | "X" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("❌ Cannot divide by zero.".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("❌ Invalid operator: {}", op)),
    }
}
