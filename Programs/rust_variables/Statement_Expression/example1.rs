#![allow(warnings)]
#[allow(unused_variables)]
#[allow(unused_imports)]

fn main() {
    let x: u32 = 5;

    let y: u32 = {
        let x_squared = x * x;

        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z: () = {
        // The semicolon supresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}