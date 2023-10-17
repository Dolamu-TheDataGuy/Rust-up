fn find_even(numbers: Vec<i32>, target: i32) -> Option<i32> {
    for number in numbers {
        if number % 2 == 0 && number == target {
            return Some(number);
        }
    }
    None
}

fn main() {
    let numbers = vec![1, 3, 6, 8, 9];
    let target = 6;

    // Pattern matching
    match find_even(numbers, target) {
        Some(result) => println!("Found an even number that matches the target: {}", result),
        None => println!("No matching even number found.")
    }


}