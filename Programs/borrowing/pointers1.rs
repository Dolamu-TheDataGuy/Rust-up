// fn main() {
//     let mut s: String = String::from("hello, ");

//     borrow_object(&s);

//     println!("Success!");
// }

// fn borrow_object(s: &String) {}

// fn main() {
//     let mut s: String = String::from("hello, ");

//     push_str(&mut s);

//     println!("Success!");
// }

// fn push_str(s: &mut String) {
//     s.push_str("world!")
// }

// fn main() {
//     let mut s: String = String::from("hello, ");

//     //Fill the blank to make it work
//     let p: &mut String = &mut  s;

//     p.push_str("world");

//     println!("Success!");
// }

//Reference using ref

fn main() {
    let c: char = 'a';

    let r1: &char = &c;

    let ref r2 = c;

    assert_eq!(*r1, *r2);

    //Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

// Funtion to get a memory address of string
fn get_addr(r: &char) -> String {
        format!("{:p}", r) // analogous to return statements in other languages
}







