// The compiler is capable of providing basic implementations for some traits via
// the #[derive] attribute.

// Centimeters, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)] // PartialEq for comparing, PartialOrd is for ordering
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// ADD some attributes to make the code work!
// DON'T modify other code!
struct Seconds(i32);

fn main() {
    let _one
}