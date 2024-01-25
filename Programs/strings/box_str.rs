fn main() {
    let s: &str = "hello, world";  // returns the address

    greetings(s)
}

fn greetings(s: &str) {
    println!("{}", s);
}

