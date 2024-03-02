// We can also implement methods for enums.

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,

}


// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Green => "green"
        }
    }

}

fn main() {
    let c: TrafficLightColor = TrafficLightColor::Yellow;
    let b: TrafficLightColor = TrafficLightColor::Red;
    let a: TrafficLightColor = TrafficLightColor::Green;
    assert_eq!(c.color(), "yellow");
    assert_eq!(b.color(), "red");
    assert_eq!(a.color(), "green");


    print!("{:?}\n", c);


}