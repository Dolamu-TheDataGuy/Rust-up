fn main() {
    let cordinates: (u8, u8) = (3, 4);

    match cordinates {
        (0,0) => println!("At the origin"),
        (x, 0) => println!("On the x-axis at {}", x),
        (0, y) => print!("On thr y-axis at {}", y),
        (x, y) => print!("At Cordinates ({}, {})", x, y),
    }
}