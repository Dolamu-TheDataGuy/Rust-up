// fn main() {
//     // s1 and s2 point to the same location in heap memory,

//     // This would violate the second rule which says that there
//     // can only be one owner at a time
//     let s1 = String::from("hello");
//     let _s2 = s1;

//     // So, the first variable s1 will be dropped and cannot be used after assigning it\
//     // to s2. to avoid dangling pointers.
// }


// fn main() {
//     // Here, the integer value of variable x will get copies into y
//     // and both variables are usable, because i32 value has been copies.
//    let x = 5;
//    let y = x;
   
//    print!("x is {}\n y is {}\n", x, y);
// }