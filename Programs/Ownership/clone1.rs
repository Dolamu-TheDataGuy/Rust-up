fn main() {
    let s1: String = String::from("hello, world");
    let s2: String = takes_ownership(s1);

    println!("{}", s2);
}

//only modify the code below!
fn takes_ownership(s:String) -> String { 
    println!("{}", s);
    s
}