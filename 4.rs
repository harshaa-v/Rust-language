fn main() {
    let number = Some(7);

    if let Some(value) = number {
        println!("Matched! The value is: {}", value);
    } else {
        println!("No match");
    }
}

