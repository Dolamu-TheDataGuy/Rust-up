#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// method attached to the Rectangle class as in Python.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}