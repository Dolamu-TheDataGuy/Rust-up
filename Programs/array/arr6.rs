fn main() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()]

    // Get returns an Option<T>, it's safe to use
    let name0 = name.get(0).unwrap();


    // But indexing is not safe
    let _name1 = &name[2];

    println!("Success!");
}