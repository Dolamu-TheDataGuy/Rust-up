fn main() {
    let mut s: String = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!"; // you need to pass a &str, _to_string() ==> change from &str to String

    println!("{}", s);
}