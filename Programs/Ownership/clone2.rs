fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!

fn give_ownership() -> String {
    let s: String = String::from("hello, world");
    //Convert String to Vec
    let _s = s.as_bytes();
    s
}