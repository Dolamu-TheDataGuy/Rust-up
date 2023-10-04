fn main () {
    let x: Box<i32> = Box::new(5);  //return address of the variable on the heap memory to x

    let mut y: Box<i32> = Box::new(1);  // returns address of the variable on the heap memory to y, x and y are both pointer variables.

    *y = 4; // just like pointers in C we can use the * symbol to access the value of the variabe

    assert_eq!(*x, 5);

    println!("Success!");
}
