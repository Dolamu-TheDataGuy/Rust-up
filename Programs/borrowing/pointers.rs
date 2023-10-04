// fn main() {
//     let x: i32 = 5;
//     let y: &i32 = &x;

//     //Modify this line only
//     assert_eq!(5, *y);

//     println!("The value of y is {}", y);
//     println!("Success!");
// }

// fn main() {
//     let x: i32 = 5;
//     // Fill the blank
//     let p : &i32 = &x;

//     println("The memory address of x is {:p}", p); // one possible output: ox16fa3ac

// }

// fn main() {
//     let x: i32 = 5;
//     let y: &i32 = &x; //pointer to variable x.   

//     assert_eq!(5, *y);

//     println!("Success!");
// }

// fn main() {
//     let mut s: String = String::from("hello");

//     let p: &mut String = &mut s; // s will remain the owner of the data.

//     p.push_str("world");

//     println!("Success!");
// }


