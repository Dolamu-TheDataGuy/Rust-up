
// WHY AM I LEARNING RUST?
// -FASTEST LANGUAGE AFTER C
// RICH TYPE SYSTEM
// NO GARBAGE COLLECTOR (FASTER RUNTIME)
// USEFUL COMPILER OUTPUT
// MEMORY SAFETY
// MOST BELOVED PROGRAMMING LANGUAGE SINCE 2016 ACCORDING TO STACK OVERFLOW
// FAST ADOPTION IN VARIOUS BRANCHES


// IMPORTANT POINTS IN VARIABLES
// - EVERY VARIABLE IN RUST IS INTRINSICALLY IMMUTABLE
// - VARIABLES CAN ONLY BE USED WHEN IT HAS BEEN INITIALIZED
// - SHADOWING ALLOWS A VARIABLE TO BE RE-DECLARED IN THE SAME SCOPE AT THE SAME TIME.


// Variables are assigned using `let` keyword.
// Print to standard output by print!() or println!().
// Scope of a variable defined by the block of code in which it is declared.
// Function is a named block of code that is reusable.
// Shadowing allows a variable to be re-declared in the same scope with the same name.

// Binding and mutability
// 1. A variable can be used only if it has been initialized.
// fix error below with the least amount of modification to the code
// fn main() {
//     let x: i32;
//     let _y: i32; // used _ to suppress warning

//     assert_eq!(x, 5);
//     println!("Success!");
// }


// 2 . Use `mut` to mark a variable as mutable.
// fill the blanks in the code to make it compile
// fn main() {
//     let mut x = 1;
//     x += 2; // 3

//     assert_eq!(x , 3);
//     println!("Success!");
// }

// 3. Scope is the range within thr program for which the item is valid.
fn main() {
    // global scope
    let x: i32 = 10;
    let y: i32 = 11; 

    // local scope
    {
        let y: i32 = 5;
        println!("The value of x is {} an the value of y is {}", x, y);
    }

    println!("The value of x is {} and the value of y is {}", x, y)
}

// 4. Fix the error with the use of define_x
// fn main() {
//     define_x();
// }

// fn define_x() {
//     let x: &str = "hello";

//     println!("{}, world", x);
// }

// SHADOWING
// 5 . Only modify 'assert_eq!' to make the `println!` work(print 42 in terminal)
// fn main() {
//     let x: i32 = 5;

//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);

//     let x = 42;
//     println!("{}", x); // Prints "42".
// }


// 6. Remove one line to make code void of errors
// fn main() {
    // let mut x: i32 = 1;
    // x = 7;
    //Shadowing and re-binding
//     let mut x = 7; //7
//     x = x + 3;

//     let _y = 4;
//     let _y: &str = "I can also be bound to text!";

//     println!("The value of x is {}", x);
// }


// 7. // unused variables
// fn main() {
//     let _x = 1;
// }
// // OR

// #[allow(unused_variables)]
// fn main() {
//     let _x = 1;
// }



