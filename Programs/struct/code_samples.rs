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

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age: age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}



