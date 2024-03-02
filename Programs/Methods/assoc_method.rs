#[allow(dead_code)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Method use `Self`, DONT use `TrafficLight` in fn signatures or body

    pub fn new() -> Self {
        Self {
            color: String::from("red"),
        }
    }
    

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light: TrafficLight = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}