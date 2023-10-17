// Options enums can be used for error handling

// enums syntax

enum TrafficLight {
    Red,
    Green,
    Yellow,  
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64)
}

fn main() {
    let red = TrafficLight::Red;
    let error = Result::Err("Something went wrong");
    let circle = Shape::Circle(5.0);
}