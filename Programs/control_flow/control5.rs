// While --> Used to run a loop when the condition is true.

fn main() {
    // A counter variable
    let mut n: i32 = 1;

    // Loop while the condition is true
    while n <= 10 {

        if n % 15 == 0 {
            println!("Fizzbuzz ");
        }
        else if n % 3 == 0 {
            println!("Fizz ");
        }
        else if n % 5 == 0 {
            print!("Buzz ");
        }
        else {
            print!("{} ", n);
        }

        n += 1;
    }

    print!("\nn reached {}, so loop is over\n", n);
}