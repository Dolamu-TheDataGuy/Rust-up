enum Message {
    Quit,
    Move {x: i32, y: i32}, // Struct
    Write(String),
    ChangeColor(i32, i32, i32), // Tuple
}

fn main() {
    let msg = Message::Move{x: 1, y: 1};

    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN! ")
    }

    println!("Success!");
}