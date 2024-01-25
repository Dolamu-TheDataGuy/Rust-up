#[warn(unused_mut)]
fn main() {
    let s: String = String::from("hello");

    let r1: &String = &s; // make reference immutable for code to work
    let r2: &String = &s;

    println!("{}, {}", r1, r2);

    println!("s: {}", s);

    println!("Success!");
}