#![allow(warnings)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Quarter(UsState), // Variant with associated data
}

// enum UsState {
//     Alabama,
//     Alaska,
//     // ... other states
// }

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter(state) => {
        //     println!("State quarter from {}", state);
        //     25
        // }
    }
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of the coin is {} cents", value);
}
