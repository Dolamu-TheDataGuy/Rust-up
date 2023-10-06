// // for modifying a string reference we have to make it mutable by using the `mut` keyword.
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// Doesn't work because it violates the first rule of borrowing,
// which says that we can only have ONE mutable reference to the same data at a time!
// fn main() {

//     let mut s = String::from("hello");

//     let r1 = &s; //no problem
//     let r2 = &s; //no problem
//     let r3 = &mut s; //BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);
// }


// This works
fn main () {

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);
    //variables and r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}


// This doesnt work because it violates the first rule of borrowing,
// which says that we can only have ONE mutable reference to the same data at
// a time.

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{}, {}", r1, r2)
// }


// This works we define the 2 mutable reference in different scopes
// fn main() {
//     let mut s = String::from("hello");
    

//     {
//         let r2 = &mut s;
//         println!("{}", r2)
//     }
//     let r1 = &mut s;
//     println!("{}", r1)
// }