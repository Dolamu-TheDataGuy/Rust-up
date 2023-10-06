// Borrowing
// 1. Way of temporarily accessing data without taking ownership of it.

// 2. When borrowing, you're taking a reference(pointer) to the data,
// not the data itself.

// Prevention of dangling pointers and data races.

// Data can be borrowed immutably and mutably.

// There are certain rules when borrowing which we have to comply with,
// otherwise the program won't compile.


// Referencing (Rules of References)
// 1. At any given time, you can have either have one mutable reference or any number
// of immutable references.

//2. References must always be valid.

fn main() {
    let s1 = String::from("hello");

    let len: usize = calculate_length(&s1); // we provided a reference to s1 and not s1 itself so s1 still remains valid after this scope

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s:&String) -> usize {
    s.len()
}