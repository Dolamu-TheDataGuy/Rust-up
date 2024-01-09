#![allow(warnings)]
#[allow(unused_variables)]
#[allow(unused_imports)]
use std::mem::size_of_val;

// fn main() {
//     let c1: char = 'a'; //4 bytes
//     assert_eq!(size_of_val(&c1), 4);

//     let c2: char = 'z';
//     assert_eq!(size_of_val(&c2), 4);

//     println!("Success!");
// }

fn main() {
    let c1: char = 'a'; //4 bytes
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}