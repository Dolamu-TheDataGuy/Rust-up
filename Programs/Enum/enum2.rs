// Each enum variant can hold its own data.

enum Message {
    Quit,
    Move {x: i32, y: i32}, // Struct
    Write(String),
    ChangeColor(i32, i32, i32), // Tuple
}

fn main() {
    let msg1: Message =  Message::Move{x: 1, y: 2}; // Instantiating with x = 1, y = 2
    let msg2: Message = Message::Write(String::from("Dolamu")); // Instantiating with "hello, world!"

    println!("Success!");
}