fn main() {

    let _arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}