fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());  // s gets out of scope here unless it is deep copied.

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s);
}

