fn main() {
    let mut count: u32 = 0u32;

    print!("Let's count until infinity!\n");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            print!("three\n");

            // Skip the rest of this iteration, program will ignore the rest of the code and restart the loop.
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough\n");

            break; 

        }
    }

    assert_eq!(count, 5);

    print!("Success\n");
}