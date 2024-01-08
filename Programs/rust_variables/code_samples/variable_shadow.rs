// Shadowing allows you to re-initialize a variable with another value.
#![allow(warnings)]
#[allow(unused_variables)]
// fn main() {
//     let x: i32 = 5;

//     {
//          let x = 12;
//          assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);

//     let x: i32 = 42; // shadowing variable x
//     println!("{}", x) //Prints 42
// }

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let mut x = x;
    x += 3;
    

    let y = 4;
    // Shadowing
    let y: &str = "I can also  be bound to a text";
    println!("{}\n {}", x, y);
}