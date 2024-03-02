#[allow(dead_code)]
#[allow(unused_variables)]

struct Person {
    name: String,
    age: u8,
    hobby: String
}

fn main() {
    let age: u8 = 30;
    // Instance of Person struct
    let _p: Person = Person {
        name: String::from("sunface"),
        age: age,
        hobby: String::from("coding"), 
    };
    println!("Success!")
}
