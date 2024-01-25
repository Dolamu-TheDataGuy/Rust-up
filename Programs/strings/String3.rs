fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); // convert String to &str
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}