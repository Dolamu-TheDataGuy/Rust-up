#![allow(warnings)]

// fn main() {
//     let n: i32 = 5;

//     if n < 0 {
//         print!("{} is negative", n);
//     } else if n > 0 {
//         print!("{} is positive", n);
//     } else {
//         print!("{} is zero", n);
//     }
// }

// 2 . if/else expressions can be used in assignments.

fn main() {
    let n: i32 = 5;

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase by ten fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0 as i32
        };
    print!("{} -> {}\n", n, big_n);
}