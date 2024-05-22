// Vector -> They are stored on the heap memory, which means the amount of data does
// not need to known at compile time and can grow or shrink as the program runs.


#[derive(Debug)]

enum SpreadsheetCell {
    Int(i32),
    Float(f64),0
    Text(String),
}

fn main() {
    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Int(5),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Text(String::from("blue"))];

    println!("{:?}", row);
}
