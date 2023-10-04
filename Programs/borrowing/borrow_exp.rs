//Rules of borrowing
// 1. At any given time, you can have either one mutable reference or any number
// of immutable references

// 2. The reference must always be valid.


//fn main() {
//     let mut s: String = String::from("hello");

//     let mut r1 = &s;
//     let mut r2 = &s;

//     println!("{}, {}", r1, r2);

//     println!("s: {}", s);

//     println!("Success!");
// }

// Since we are not modifying we can just remove the `mut` keyword.
// fn main() {
//     let s: String = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);

//     println!("s: {}", s);

//     println!("Success!");
// }



// Borrow an immutable object as mutable
// fn main() {
//     let s: String = String::from("hello, "); //imumutable string

//     borrow_object(&mut s); // cannot borrow an immutable string as mutable.

//     println!("Success!");
// }

// fn borrow_object(s: &mut String) {}



//SOLUTION
// fn main() {
//     let mut s: String = String::from("hello, "); //add mut keyword

//     borrow_object(&mut s); 
//     println!("Success!");
// }

// fn borrow_object(s: &mut String) {}



// Case 9 : Borrow a mutable object as immutable: Very ok and would still compile
// fn main() {
//     let mut s: String = String::from("hello, ");

//     borrow_object(&s);

//     s.push_str("world");

//     println!("Success!");
// }

// fn borrow_object(s: &String) {
//     println!("{}", s);
// }


// Case 10: We can only have one valid mutable reference at a time, the rust compiler is one of the best compilers out there
// fn main() {
//     let mut s: String = String::from("hello, ");

//     let r1: &mut String = &mut s; // cannot use r1 after this line, prog wont compile
//                                     // if we do not use them we cannot get an error.
//     r1.push_str("world");

//     let r2: &mut String = &mut s;
//     r2.push_str("!");

//     println!("{}", r1);
// }


// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String;
//         age: Box<u8>;
//     }

//     let person: Person = Person {
//         name: String::from("Africa");
//         age: Box::new(20);
//     };

//     let Person { name, ref age} = person;

//     println!("The person's age is {}", age);

//     println!("The person's name  is {}", name);

// }

// Exercise 8
// fn main() {
//     let t: (String, String) = (String::from("Hello"), String::from("Africa"));

//     let _s: String = t.0; // _s is now the owner of first element in tuple

//     //Modify this line only, don't use "_s"
//     println!("{:?}", t.1);
// }

// Exercise 9
// fn main() {
//     let t: (String, String) = (String::from("Hello"), String::from("Africa"));

//     let (s1, s2) = t.clone(); // Destructuring, if we dont put clone t would no longer be the owner of the data it hold and would give error in the print statement.

//     //Modify this line only, don't Destructuring
//     println!("{:?}, {:?}, {:?}", s1, s2, t);
// }