// Mutability can be changed when ownership is transferred

// fn main() {
//      let s: String = String::from("hello, ");

//      // Modify this line only!
//      let mut s1 = s;

//      s1.push_str("world");

//      print!("Success!\n")
// }


// fn main() {
//     let x: Box<i32> = Box::new(5); // similar to malloc in C, allocated memory in heap and assign base address to x.

//     let mut y: Box<i32> = Box::new(1);  // Implement this line, don't change other lines!

//     *y = 4; // dereferencing and changing the value that the pointer y points to.
    
//     // x and y are both pointers.
//     assert_eq!(*x, 5);
//     assert_eq!(*y, 4);

//     print!("Success!\n");
// }