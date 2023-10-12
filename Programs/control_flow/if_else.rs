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
fn main() {
    let names: [String; 2] = [String::from("liming"), String::from("hanmeime")];
    for name in &names {
        print!("{}\n", name)
    }

    println!("{:?}", names); // violate ownership rule.

    let numbers: [i32; 3] = [1, 2, 3];
    for mut number in numbers {
        number = number * 2;
        print!("{}\n", number);
    }
    print!("{:?}\n", numbers);
}