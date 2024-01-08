// All variables in rust are immutable by default. Use the `mut` keyword to mark a variable as mutable


fn main() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}