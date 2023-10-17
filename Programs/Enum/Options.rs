// lets use the Options enum to handle errors in rust
// The options enum has 2 possible states --> The None and Some(value)

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b) 
    }
}

fn main() {
    let result = divide(8.0, 0.0);

    match result {
        Some(value) => print!("Result: {}\n", value),
        None => print!("Cannot divide by zero!\n"),
    }
}