// Fix error with the use of define_x

fn main() {
    println!("{}, world", x);

}

fn define_x() {
    let x: &str = "hello";
}