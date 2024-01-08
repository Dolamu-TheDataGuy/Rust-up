// Scope is a range within the program for which the item is valid.

fn main(){
    // Global scope
    let x: i32 = 10;

    {
        let y:i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }

    println!("The value of x is {} and the value of y is {}", x, y);
}