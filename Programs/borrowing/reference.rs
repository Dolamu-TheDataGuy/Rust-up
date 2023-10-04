fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // we provided a reference to s1 and not s1 itself so s1 still remains valid after this scope

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s:&String) -> usize {
    s.len()
}