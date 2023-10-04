fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    //Destructuring (`name` is moved out of person, but `age` is referenced)
    let Person { name, ref age} = person;

    println!("The person's age is {}", age);

    println!("The person's age is {}", name);

    println!("The person's age from the struct is {}", person.age);
}