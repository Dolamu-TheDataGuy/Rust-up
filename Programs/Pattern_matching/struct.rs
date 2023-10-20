struct Point {
    x: i32,
    y: i32,
}

fn main() {
let p = Point {x: 1, y: 2};

match p {
    Point {x, y} => println!("x is {}, y is {}", x, y),
}
}
