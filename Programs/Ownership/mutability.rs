fn main() {
    let s: String = String::from("hello, ");

    //Modify the line of code only!
    let mut s1 = s;

    s1.push_str("world");
    println!("Success!");
}