fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}


fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    let _s = s.as_bytes();
    s
}