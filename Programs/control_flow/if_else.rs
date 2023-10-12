#![allow(warnings)]
// 1.
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

// fn main() {
//     let n: i32 = 5;

//     let big_n = 
//         if n < 10 && n > -10 {
//             println!(", and is a small number, increase by ten fold");
//             10 * n
//         } else {
//             println!(", and is a big number, halve the number");

//             n / 2.0 as i32
//         };
//     print!("{} -> {}\n", n, big_n);
// }

// 3. The for in construct can be used to iterate through an iterator, e.g a range
// 

// fn main() {
// for n in 1..100 {// modify this line to make the code work
//     if n == 100 {
//         panic!("NEVER LET THIS RUN")
//     }
// }
//     print!("Success!");
// }


// 4 . 
// Fix the errors without adding or removing lines
//   

// 5. 
fn main() {
    let a: [i32; 4] = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        if i + 1 == 1 {
            println!("The {}st element is {}", i+1, v);
        };
        if i + 1 == 2 {
            println!("The {}nd element is {}", i+1, v);   
        };
        if i + 1 == 3 {
            println!("The {}rd element is {}", i+1, v);   
        };
        if i + 1 > 3 {
            println!("The {}th element is {}", i+1, v);
        };   
    }
}