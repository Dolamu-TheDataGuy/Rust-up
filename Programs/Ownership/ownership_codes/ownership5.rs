// Dont use clone use copy instead

fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello"); // Most of the types in the tuple are in the stack memory (Their size is known)
    let y: (i32, i32, (), &str)  = x;  // &str==> string literal: hardcoded into the binary itself, it is immutable.
    println!("{:?}, {:?}", x, y)
} 