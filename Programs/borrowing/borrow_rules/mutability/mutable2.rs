fn main() {

    let mut s: String =  String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");

    let r2: &mut String = &mut s;
    r2.push_str("!");

    println!("{}", r2); // works if we are not using or printing r1 , Also works because the only valid reference is r2 since we are not using r1.
}