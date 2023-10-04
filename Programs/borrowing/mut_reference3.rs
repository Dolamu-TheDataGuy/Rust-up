
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
fn main() {
    let mut s = String::from("hello");
    

    {
        let r2 = &mut s;
        println!("{}", r2)
    }
    let r1 = &mut s;
    println!("{}", r1)
}
