#[allow(unused_variables)]
// Fix the errors
enum Number {
    _Zero, // 0
    One, // 1
    _Two, // 2
}

enum Number1 {
    _Zero = 0,
    One,
    _Two,
}

// C-like enum
enum Number2 {
    _Zero = 0, // You can't use a float/discriminator value
    One = 1, // 6
    _Two = 2, // 7 
}

fn main() {
    // An enum variant can be converted to an integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    print!("{}\n", Number::One as u8);

    println!("Success")
}