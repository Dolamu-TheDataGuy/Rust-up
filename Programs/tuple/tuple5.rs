// Destructure assignments

fn main() {
    let (x, y, z): (u8, u8, u8);

    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}