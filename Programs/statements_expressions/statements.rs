// What are statements?
// Statements are instructions that perform some actions but does not
// produce a value, function definitions are statements, as well as code
// that ends with `;`

// Example 1

// fn main() {
//     let x: u32 = 5;

//     let y: u32 = {
//         let x_squared = x * x;
//         let x_cubed = x_squared * x;

//         // This expression will be assigned to `y`
//         x_cubed + x_squared + x
//     };

//     let z: u32 = {
//         // The semicolon suppresses this expressions and `()` is assigned to `z`
//         2 * x
//     };

//     print!("x is {:?}\n", x);
//     print!("y is {:?}\n", y);
//     print!("z is {:?}\n", z);

// }


// fn main() {
//     let v: i32 = {
//         let mut x: i32 = 1;
//         x += 2; // 3 but this is a statement
//         x
//     };

//     assert_eq!(v, 3);

//     print!("Success!\n");
// }

fn main() {
    let s = sum(1, 2); // 3
    assert_eq!(s, 3);
    print!("Success!\n");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}