fn main() {
    let mut s: String = String::from("hello world");

    // Here, &s is `&String` type, but `first_word` need a `&str` type.
    // It works because `&String` can be implicitly converted to `&str`. If you want
    let word: &str = first_word(&s); // &String -> &str implicitly by compiler, immutable borrow
    println!("the first word is: {}", word);  // immutable borrow
    
    s.clear(); // mutable borrow.

    
}

fn first_word(s: &str) -> &str {
    &s[..1]
}