struct Rectangle {
    width: u32;
}
height: u32,


fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1)); //use a copy of the struct since it is on the heap and not the stack.
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

