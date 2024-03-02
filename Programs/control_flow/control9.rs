fn main() {
    let mut counter = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // breaks and return counter
        }
    };

    assert_eq!(result, 20);

    print!("Success!");
}