fn main() {
    let x: i32 = 5;
    // Fill the blank
    let p: &i32 = &x;

    println!("The memory address of x is {:p}", p); // one possibe output -> 0x7fffcfd6033c.
}

