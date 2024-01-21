// Mutable reference

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn chane(some_string: &mut String) {
    some_string.push_str(", world");
}