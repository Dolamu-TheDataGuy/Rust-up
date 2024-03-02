#[derive(Debug)]

struct File {
    name: String,
    data: String,
}

fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    // let name = &f.name; // You have to use a reference to the name string or else this would take ownership of the string and f.name would become invalid.
    let  ref name: String = f.name; // a reference to f.name, f.name is still valid.

    println!("{}, {}, {}, {:?}", name, f.name, f.data, f);
}

