// let s = String::from("hello world");

// let hello = &s[0..5];
// let world = &s[6..11]; 

//1 . The type of string literal "hello, world" is &str, e.g let s: str = "hello, world" .

// fn main() {
//     let s: &str = "hello, world";

//     println!("Success!");
// }

// 2. We can only use str by boxed it, & can be used to convert Box<str> to &str
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s:&str) {
//     println!("{}", s)
// }

// String type is defined in std and stored as a vector of bytes (Vec), but guaranteed
// to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.
// fn main() {
//     let mut s: mut String = String::from("hello");
//     s.push_str("hello, world");
//     s.push("!");

//     assert_eq!(s, "hello, world!");

//     println!("Success!");
// }


// fn main() {
//     let mut s: String = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";

//     println!("{}", s);
// }

// fn main() {
//     let mut s: String = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.replace("dogs", "cats");

//     assert_eq!(s1, "I like cats");

//     println!("Success!");
// }

// Case 6: You can only concat a String with a &str and String's ownership can
// be moved to another variable.
// fn main() {
//     let s1: String  = String::from("hello,");
//     let s2: String = String::from("world!");
//     let s3: String = s1 + s2.as_str(); // String -> &str
//     // let s3: String = s1 + &s2; // alternative.
//     assert_eq!(s3, "hello,world!")
//     println!("{}", s3); // s1 cannot be used, s3 is now in ownership of its data
// }


// //  &str can be converted to String in two ways using to_string or to_owned 
// fn main() {
//     let s = "hello, world";
//     greetings(String::from(s)); //&str -> String
// }

// fn greetings(s: String) {
//     println("{}", s)
// }

// // String escapes
// fn main() {
//     let byte_escape = "I am writing Ru\x73\x74!";
//     println!("What are you doing\x3F (// x3F is ? in ASCII) {}", byte_escape);
    
// }

// fn main() {
//     let raw_str = "Escapes don't work here: \x3F";
//     // Modify above line to make it work
//     assert_eq!(raw_str, "Escapes don't work here: ?");
// }

// String Index
fn main() {
    let s1 = String::from("hi!");
    let h: &str = &s1[0..1]; //reference to a string s1 is a string slice (&str)
    assert_eq!(h, "h");
}

