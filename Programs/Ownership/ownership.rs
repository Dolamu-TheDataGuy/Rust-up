fn main() {
    let s = String::from("hello");   // s comes into scope here

    takes_ownership(s);         // s's value moves into the function...
                                // ... and so is no  longer valid here

    let x = 5;                  // x comes into scope

    makes_copy(x);              // x would move into the function but 
                                // i32 is copy, so its okay to still use x afterward
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}