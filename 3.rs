fn main() {
    let a = 10.0;
    let b = 5.0;
    let operator = '+';

    let result = match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}

