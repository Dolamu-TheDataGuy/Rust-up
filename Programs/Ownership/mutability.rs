// Mutability can be changed when ownership is transferred

// fn main() {
//      let s: String = String::from("hello, ");

//      // Modify this line only!
//      let mut s1 = s;

//      s1.push_str("world");

//      print!("Success!\n")
// }


// fn main() {
//     let x: Box<i32> = Box::new(5); // similar to malloc in C, allocated memory in heap and assign base address to x.

//     let mut y: Box<i32> = Box::new(1);  // Implement this line, don't change other lines!

//     *y = 4; // dereferencing and changing the value that the pointer y points to.
    
//     // x and y are both pointers.
//     assert_eq!(*x, 5);
//     assert_eq!(*y, 4);

//     print!("Success!\n");
// }


// Partial Move
// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person: Person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // name is moved out of person, but age is referenced
//     let Person {name, ref age} = person;

//     print!("The person's age is {}\n", age);

//     print!("The person's name is {}\n", name);

// }


// 8
// fn main() {
//     let t: (String, String) = (String::from("hello"), String::from("world"));

//     let _s: String = t.0; // t is no longer the owner of t[0], so we cant access t[0] again.

//     // Modify this line only, don't use `_s`
//     println!("{:?}", t.1);
// }


fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let (s1, s2) = __;

    // Modify this line only, don't use `_s`
    println!("{:?}, {:?}, {:?}", s1, s2, t);  // -> "hello", "world", ("hello", "world")
}
