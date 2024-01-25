#[derive(Debug)]
#[allow(dead_code)]

struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // println!("The rect1 is {:?}", rect1); //use a copy of the struct since it is on the heap and not the stack.
    
}
