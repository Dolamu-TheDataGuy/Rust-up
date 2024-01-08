// Shadowing allows you to re-initialize a variable with another value.

fn main() {
    let x: i32 = 5;

    {
         let x = 12;
         assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42; // shadowing variable x
    println!("{}", x) //Prints 42
}