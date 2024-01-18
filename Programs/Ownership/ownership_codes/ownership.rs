// fn main() {
//     let s1 = gives_ownership();  // gives_ownership moves its return
//                                 // value into s1

//     let s2 = String::from("hello");  // s2 comes into scope

//     let s3 = takes_and_give_back(s2);  // s2 is moved into 
//                                         // takes_and_give_back, which also
//                                         // moves its return value into s3


// }  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//     // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String { // gives_ownership will move its
//                                  // return value into the function
//                                 // that call it


//     let some_string = String::from("yours");  // some_string comes into scope

//     some_string                              // some_string is returned and moves out to the calling function
// }

// fn takes_and_give_back(a_string: String) -> String { // a_string comes into scope

//     a_string // a_string is returned and moves out to the calling function
// }


// Ownership prevents memory safety issues:
// - Dangling pointers
// - Double-free
//   * Trying to free memory that has already been freed.
// - Memory leaks
//   * Not freeing memory that should have been freed.

// Exercise on ownership
// 1.

// fn main() {
//     let x: String = String::from("hello, world!");
//     let y: String = x.clone();

//     print!("{}, {}\n", x, y);
// }

// 2. 

// fn main() {
//     let s1: String = String::from("hello, world");
//     let s2 = takes_ownership(s1);

//     print!("{}\n", s2);
// }

// fn takes_ownership(s: String) -> String {
//     print!("{}", s);
//     // s
// }

// 3.
// fn main() {
//     let s: String = gives_ownership();
//     print!("{}", s);
// }

// fn gives_ownership() -> String {
//     let s: String = String::from("hello, world");
//     // Convert String to Vec
//     let _s = s.as_bytes();
//     s
// }

//4 . 
// fn main() {

//     let s = String::from("hello, world");

//     print_str(s.clone());

//     println!("{}", s); // s would be out of scope and inaccessible.
// }

// fn print_str(s: String) {
//     print!("{}\n", s);
// }

// 5.  A type that lives in stack memory is of fixed size and gets copied implicity.
fn main() { // copy happens to a data type when the size of the data type is known.
    let x: (i32, i32, (), &str) = (1, 2, (), "hello"); // since &str is of known fixed size at compile time, the next line of code will copy.
    let y: (i32, i32, (), &str) = x;
    print!("{:?}, {:?}\n", x, y);
}

