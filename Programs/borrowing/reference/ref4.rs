fn main() {
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, *y); // * is the dereference operator (which means go to a address and get the data in the address)

    println!("Success!");
}