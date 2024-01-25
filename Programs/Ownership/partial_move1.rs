fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s: String = t.0; // value in index 0 already moved to _s

    println!("{:?}", t.1);
}