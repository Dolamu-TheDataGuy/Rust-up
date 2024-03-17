enum Message {
    Hello { id: i32},
}

fn main() {
    let msg: Message = Message::Hello {id: 30};


    match msg {
        Message::Hello {
            id : id @ 3..=7
    } => println!("Found an id in range [3, 7] : {}", id),
    Message::Hello { id: newid @ (10 | 11 | 12)} => {println!("Found an id in another range [10, 12]: {}", newid) // id gets destructured into newid
    }
    Message::Hello { id } => print!("Found some other id: {}/n", id),
}
}