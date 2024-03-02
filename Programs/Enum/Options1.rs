#[allow(unused_variables)]
fn main() {
    let five: Option<i32> = Some(5); // Variant of Option Enum
    let six: Option<i32> = plus_one(five); // Some(6)
    let none: Option<i32> = plus_one(None); // None

    if let Some(n) = six { // Destructure Some(6) assign 6 to n
        println!("{}", n);

        print!("Success!");
    }
    else {
        panic!("NEVER LET THIS RUN!");
    }
    
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
