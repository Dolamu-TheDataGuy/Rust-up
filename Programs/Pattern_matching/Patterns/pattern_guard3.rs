// using pattern &mut V to match a mutable reference needs you to be very careful, due to V being a value after matching.

fn main() {
    let mut v = String::from("hello,");
    let r: &mut String = &mut v;

    match r {
        // value => print!("The new string is {:?}\n",value), // matches the value and not the reference
        value => value.push_str("world!"),
    }  
}