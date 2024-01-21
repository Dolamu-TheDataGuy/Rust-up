

fn main() {
    let mut _s: String = String::from("hello, ");

    borrow_objects(&_s);

    println!("Success!");
}

fn borrow_objects(_s: &String) {

} 