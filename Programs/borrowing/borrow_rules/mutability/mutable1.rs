#![allow(warnings)]
#[allow(unused_variables)]


fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s); // Immutable references

    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {

}