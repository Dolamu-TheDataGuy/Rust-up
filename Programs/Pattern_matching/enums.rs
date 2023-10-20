#![allow(warnings)]
enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let color = Color::Red;

    match color {
        Color::Red => print!("it's red\n"),
        Color::Green => print!("It's green\n"),
        Color::Blue => print!("it's blue\n"),
    }
}