// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// Enum for IP address. An IP address can either be of V4 format or V6 format
// Each variant in the enum holds a String value.
// const home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));

// const loopback: IpAddr = IpAddr::V6(String::from("::1"));

// Enums are Enumerated basically
// Fix the errors
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }

// // C-like enum
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 2,
// }


// fn main() {
//     // An enum variant can be converted to a integer by 'as
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as u8, Number2::One as u8);

//     print!("{}", Number::One as u8);

//     print!("Success!");

// }

// // 2. Enum variant can hold its own data.
// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg1: Message = Message::Move{ x: 1, y: 2}; // instantiating with x=1, y=2
//     let msg2: Message = Message::Write(String::from("hello, world!")); //Instantiating with 'hello, world!'

//     print!("Success!");
// }

// // 3. We can get the data which an enum variant is holding by pattern match.

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg: Message = Message::Move{x: 1, y: 1};

//     if let Message::Move{x: a, y: b} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("NEVER LET THIS RUN! ");
//     }
    
//     println!("Success!");
// }

// 4. Fill in the blank and fix the errors.
// #![allow(warnings)]
// #[derive(Debug)]
// enum Message {
//     Quit, 
//     Move {x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }


// fn main() {
//     let msgs: [Message; 4] = [
//         Message::Quit,
//         Message::Move{x: 1, y: 3},
//         Message::Write(String::from("hello!")),
//         Message::ChangeColor(255, 255, 0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// }

// fn show_mes sage(msg: Message) {
//     print!("{:?}\n", msg);
// }

// 5. Since there is no null in Rust, we have to use enum Option<T> to deal
// with the cases when the value is absent.
// Fill in the blank to make the println work.
// Also add some code to prevent the `panic` from running.

#![allow(warnings)]
fn main() {
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five); // Some(6)
    let none: Option<i32> = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    } else {
        panic!("Never let this run!");
    }    
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}