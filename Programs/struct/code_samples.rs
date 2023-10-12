// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }

// fn main() {
//     let age: u8 = 30;
//     let p: Person = Person {
//         name: String::from("sunface"),
//         age: age,
//         hobby: String::from("coding"), 
//     };
//     println!("Success!")
// }

// Tuples struct
// struct _Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let v: Point = Point(0, 127,255);
//     check_color(v);

//     println!("Success!");
// }

// fn check_color(p: Point) {
//     let Point(x, _, z) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(z, 255);
// }

// 4 You can make a whole struct mutable when instantiating it, but Rust doesnt
// allow to mark only certain fields as mutable.

// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
//     let age = 18;
//     let mut p = Person {
//         name: String::from("sunface"),
//         age: age,
//     };

//     // How can you believe sunface is only 18?
//     p.age = 30;

//     // Fill the blank
//     p.name = String::from("sunfei");

//     println!("Success!");
// }


// 5. Using field init shorthand syntax to reduce repetitions.

// struct Person {
//     name: String,
//     age: u8;
// }

// fn main() {
//     println!("Success!");
// }

// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         name: name,
//     }
// }



// 6. You can create instance from other instance with struct update syntax
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // Instance of struct User
//     let u1 = User {
//         active: true,
//         username: String::from("Dolamu"),
//         email: String::from("oludaredolamu@gmail.com"),
//         sign_in_count: 1,
//     };

//     let u2: User = set_email(u1);

//     println!("Success!");
// }

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }

//7. Print the structs, we can use #[derive(Debug)] to make structs printable.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale: u32 = 2;
//     let rect1: Rectangle = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1); //Print debug info to stderr

//     println!("{:?}", rect1); // Print debug info to stdout
// }


// 8. Fix errors to make it work.
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust by Practise".to_string()
    };

    let _name: String = f.name.clone();

    // ONLY modify this line
    println!("{}, {}, {:?}", f.name, f.data, f)
}


