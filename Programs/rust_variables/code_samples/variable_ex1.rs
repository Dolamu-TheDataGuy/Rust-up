// Binding and immutability

// A variable can be used only if it has been utilized.
#![allow(warnings)]

fn main() {
    let x: i32 = 5;
    let y:i32; // uninitialized but unused, only a Warning!

    assert_eq!(x, 5);
    println!("Success!");
}