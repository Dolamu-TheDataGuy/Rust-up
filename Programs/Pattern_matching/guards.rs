fn main() {
    let number: u8 = 34;

    match number {
        n if n < 10 => println!("Less than 10"),
        n if n > 10 => println!("Greater than 10"),
        _ => println!("it's 10"),
    }
}