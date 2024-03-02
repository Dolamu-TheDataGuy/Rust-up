#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move {x: i32, y: i32}, // Struct
    Write(String),
    ChangeColor(i32, i32, i32), // Tuple
}

fn main() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x: 1, y: 1},
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
    println!("Success!");
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}