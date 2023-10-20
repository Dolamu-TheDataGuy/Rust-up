fn main() {
    let age = 25;

    match age {
        0..=17 => println!("You are a minor"),
        18..=64 => println!("You are an adult"),
        _ => println!("You are an agba")
    }
}