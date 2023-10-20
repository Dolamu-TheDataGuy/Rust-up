// Pattern matching is a powerful features in rust used for control
// it is likened to the switch statement in C.

fn main() {

let number: u8 = 5;

match number {
    1 => print!("it's one\n"),
    2 => print!("it's two\n"),
    _ => print!("it's something else\n"),
}
}
