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
// fn main() {
//     let a: [i32; 4] = [4, 3, 2, 1];

//     // Iterate the indexing and value in 'a'
//     for (i, v) in a.iter().enumerate() {
//         if i + 1 == 1 {
//             println!("The {}st element is {}", i+1, v);
//         };
//         if i + 1 == 2 {
//             println!("The {}nd element is {}", i+1, v);   
//         };
//         if i + 1 == 3 {
//             println!("The {}rd element is {}", i+1, v);   
//         };
//         if i + 1 > 3 {
//             println!("The {}th element is {}", i+1, v);
//         };   
//     }
// }

// 6 .  While Loop

// fn main() {
//     // A counter variable
//     let mut n: i32 = 1;

//     // Loop while the condition is true
//     while n < 10 {
//         if n % 15 == 0 {
//             print!("fizzbuzz!\n");
//         } else if n % 3 == 0 {
//             print!("fizz\n");
//         } else if n % 5 == 0 {
//             print!("buzz\n");
//         } else {
//             println!("{}", n);
//         }

//         n += 1;
//     }
//     print!("n reached {}, so loop is over\n", n);
    
// }

// Continue and break
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n == 66 {
//             break;
//         }
//         n += 1;
//     }

//     assert_eq!(n, 66);

//     print!("Success!");
// }

// Continue
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n != 66 {
//             n+=1;
//             continue;
//         }
//         break;
//     }

//     assert_eq!(n, 66);

//     print!("Success!");
// }


// fn main() {
//     let mut counter: i32 = 0;
    
//     let result: i32 =  loop {
//         counter += 1;

//         if  counter == 10 {
//             break counter * 2;
//         }
//     };

//     assert_eq!(result, 20);

//     print!("Success!"); 
// }


fn main() {
    let mut count = 0;

    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; //break also works
            }
            count += 2;
        }
        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }
            // This will continue the outer loop
            continue 'outer;
        }
    }
    assert!(count == 30);

    print!("Success!");
}