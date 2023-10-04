// What are statements?
// Statements are instructions that perform some actions but does not
// produce a value, function definitions are statements, as well as code
// that ends with `;`

// Example 1

fn main() {
    let x: u32 = 5;

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // This expression will be assigned to `y`
        x_cubed + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expressions and `()` is assigned to `z`
        2 * x;
    };

    print!("x is {:?}", x);
    print!("y is {:?}", y);
    print!("z is {:?}", z);

}