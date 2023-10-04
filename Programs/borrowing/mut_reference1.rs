// Doesn't work because it violates the first rule of borrowing,
// which says that we can eith
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