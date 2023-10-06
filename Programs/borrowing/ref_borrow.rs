// Exercises


// 1.
// fn main() {
//     let x: i32 = 5;
//     // Fill the blank
//     let p: &i32 = &x;

//     println!("The memory address of x is {:p}", p); // one possibe output -> 0x7fffcfd6033c.
//     println!("The value of x is {}", p);
// }

// 2.
// fn main() {
//     let x: i32 = 5;
//     let y: &i32 = &x;

//     assert_eq!(5, *y);

//     print!("Success!\n");
// }

// 3.
// fn main() {
//     let mut s: String = String::from("hello, ");

//     borrow_object(&mut s);

//     print!("Success!");
// }

// fn borrow_object(_s: &String) {

// }

// 4.
// fn main() {
//     let mut s: String = String::from("hello, ");

//     push_str(&mut s);

//     println!("Success!\n");
// }

// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// 5.
fn main() {
    let mut s: String = String::from("hello, ");

    // Fill the blank to make it work
    let p: &mut String = &mut s ;

    p.push_str("world");

    println!("Success!");
}

// 6. Ref: ref can be used to take references to a value, similar to &.

fn main() {
    let c: char = 'a';

    let r1: &char = &c;

    // Fill the blank, don't change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    print!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}