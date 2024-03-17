// Generics with Structs

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let _p1: Point<i32, i32> = Point{ x:5, y: 10 };
//     let _p2: Point<f64, f64> = Point{ x: 5.0, y: 10.0 };
//     let _p3: Point<i32, f64> = Point{ x: 5, y: 10.0 };
// }

struct goal<T> {
    x: T,
    y: T,
}

impl<U> goal<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl goal<f64> {
    fn y(&self) -> f64
    self.y
}

fn main() {
    let p: Point<i32> = Point { x:5, y: 10 };
    p.x()
    let p1: Point<i32> = Point { x:5.0, y: 1.0 };
    p1.y()
}