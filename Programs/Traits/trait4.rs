// Trait for a data type
trait Describable {
    fn describe(&self) -> String;
}

// a struct person
struct Person {
    name: String,
    age: u8,
}


// Implementing a trait on the struct
impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}

fn main() {
    let ade = Person {
        name: String::from("ade"),
        age: 25,
    };

    println!("{}", ade.describe());
}