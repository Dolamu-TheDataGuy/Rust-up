// Struct and impl
#[allow(dead_code)]
#[allow(unused_variables)]
// Implement struct point to make it work
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer: Point<i32> = Point { x: 5, y: 10};
    let float: Point<f64> = Point { x: 1.0, y: 4.0};

    println!("Success!")
}