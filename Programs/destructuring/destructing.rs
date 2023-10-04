// Destructuring - We can use a pattern with let to destructure a tuple to seperate variables.

// Fix the error below with least amount of modification
fn main() {
    let (mut x , y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    print!("Success!\n")
}



// Destructuring assignment
// fn main() {
//     let (x, y);  // same as let x and let y on different lines
    
//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];

//     assert_eq!([x,y], [3, 2])
//     println!("Worked!");
// }


// Tuples in Rust
// fn main() {

//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }


