fn main() {
    let mut s1 = String::from("hello, ");
    let s2 = "world!";
    
    // Modify s1 in place
    s1.push_str(s2);
    
    // Create a new string by concatenation
    // let s3 = s1 + s2;

    println!("Modified string: {}", s1);
    // println!("New string: {}", s3);
}
