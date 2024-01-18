fn main() {
    let x: Box<i32> = Box::new(5); // Box allows direct allocation on the heap, x holds address of x not the value.

    let y: Box<i32> = Box::new(1);

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}