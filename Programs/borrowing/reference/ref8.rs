fn main() {
    let c: char = 'b';

    let r1: &char = &c; // r1 holds the address of variable c

    // Fill the blank, dont change other code
    let ref r2 = c; // ref is similar to the & sign, only that the ref keyword is used before the pointer variable.

    assert_eq!(*r1, *r2);  // They should both hold the same value.

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2)); // They should both hold the same address

    println!("Success!");
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)  //format! macro is the same as the println macro, it returns a String.
}