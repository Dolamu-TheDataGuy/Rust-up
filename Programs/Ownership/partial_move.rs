fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    }

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;
 

}