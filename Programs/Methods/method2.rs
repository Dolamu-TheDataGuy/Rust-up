#[allow(dead_code)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(self: &mut Self) {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`,
    pub fn change_state(& mut self) {
        self.color = "green".to_string() // mutating string, so we take mutable reference.
    }
}

fn main() {
    println!("Success!");
}