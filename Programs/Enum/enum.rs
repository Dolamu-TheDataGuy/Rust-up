enum IpAddr {
    V4(String),
    V6(String),
}

// Enum for IP address. An IP address can either be of V4 format or V6 format
// Each variant in the enum holds a String value.
// const home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));

// const loopback: IpAddr = IpAddr::V6(String::from("::1"));

// Enums are Enumerated basically
// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn main() {
    // An enum variant can be converted to a integer by 'as
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    print!("{}", Number::One as u8);

    print!("Success!");

}

// 2. Enum variant can hold its own data.
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move{__}; // instantiating with x=1, y=2
    let msg2 = Message::Write(__); //Instantiating with 'hello, world!'

    print!("Success!");
}

