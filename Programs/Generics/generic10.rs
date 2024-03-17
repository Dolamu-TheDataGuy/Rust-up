struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1: Point<i32, f64> = Point { x: 5, y: 10.4 };
    let p2: Point<&str, char> = Point { x: "Hello", y: 'c'};
    let p3: Point<i32, char> = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}