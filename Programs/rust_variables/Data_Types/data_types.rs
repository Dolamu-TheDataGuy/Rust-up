// Numbers - Integer types
// - Signed integer - can rep both positive and negative integers
// - Unsigned integer - always positive integers

// Length (8-bit) , Signed(i8), Unsigned(u8)
// Length (16-bit) , Signed(i16), Unsigned(u16)
// Length (32-bit) , Signed(i32), Unsigned(u32)
// Length (64-bit) , Signed(i64), Unsigned(u64)
// Length (128-bit) , Signed(i128), Unsigned(u128)
// Length (arch) , Signed(isize), Unsigned(usize)

// Default types in rust
// - integers: i32
// - floats: f64


// Numbers - Range of 8-bit integers
// SMALLEST POSSIBLE 8-BIT INTEGER (UNSIGNED): 0
// LARGEST  POSSIBLE 8-BIT INTEGER (UNSIGNED): 255
// SMALLEST POSSIBLE 16-BIT INTEGER (UNSIGNED): 0
// LARGEST POSSIBLE 16-BIT INTEGER (UNSIGNED): 65535

// NUMBERS - INTEGER RANGES
// DATA TYPE        MIN                       MAX
// i8             -128                        127
// i16            -32768                      32767
// i32            -2147483648                 2147483647
// i64            - 9223372036854775808       92772036854775807


//u8               0                         255
//u16              0                         65535
//u32              0                         4294967295
//64               0                         18446744073709551615
//128              0                         340282366920938463463374607431768211455


// 1.  Remove something to make it work
// #[allow(unused_variables)]
// fn main() {
//     let x: i32 = 5;
//     let mut y = 5; //i32 by default

//     y = x;

//     let z: i32 = 10; // i32 by default
//     println!("Success!");
// }


// 2. Fill the blank
// fn main() {
//     let v: u16 = 38_u8 as u16;

//     println!{"The value of v is {}", v}
//     println!("Worked!")
// }

// 3. Modify `assert_eq!` to make it work
// fn main() {
//     let x: u32 = 5;

//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>()) //"i32"
// }


// 4 . Fill the blank to make it work
// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);
//     println!("The max u8 value is {}", u8::MAX);
//     println!("The min u8 value is {}", u8::MIN);
//     println!("Success!")
// }

// 5. Fix the errors and panics to make to make it work.
// fn main() {
//     let v1: u16 = 251_u16 + 8;
//     let v2: i16 = i16::checked_add(251, 8).unwrap();
//     println!("{}, {}", v1, v2);

// } 

// 6 . Modify `assert` to make it work
// fn main() {
//     let v: u16 = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
//     print!("{}\n", v);
//     assert!(v == 1597);

//     println!("Success!");
// }

// FLOATING POINT
// 7 . Fill the blank to make it work
// fn main() {
//     let x: f64 = 1_000.000_1; //
//     let _y: f32 = 0.12; // f32
//     let _z: f64 = 0.01_f64;

//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("Success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// 8 . make it work in two distinct ways
// fn main() {
//     assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // floating point precision 0.1 + 0.2 = 0.30000000000000000
//     assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
//     println("Success!");
// }

// 9. Range in Rust
// Two goals: 1. Modify assert! to make it work 2. Make println! output: 97-122
// fn main() {
//     let mut sum: i32 = 0;
//     for i in -3..2 {  // 2 is excluded
//         sum += i;
//     }
    
//     assert!(sum==-5);

//     for c in 'a'..='z' {
//         println!("{}", c as u8); // ASCII Charts
//     }
// }

// 10 . Fill the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{start: 1, end: 5});
//     assert_eq!((1..=5), RangeInclusive::new(1,5));
//     println!("Success!");
// }

// 11. Computations (Fill the blanks)
// fn main() {
//     // Integer addition
//     assert!(1u32 + 2u32 ==3u32);

//     // Integer subtraction
//     assert!(1i32 - 2i32 == -1i32);
//     assert!(1i8 - 2i8 == -1i8);

//     assert!(30u16 * 50u16 == 1500u16); // u16

//     assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);
    
//     assert!(24 % 5 == 4); // i32

//     // Short-circuiting boolean logic  1 - True , 0 - False , AND, OR, NOT
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // Bitwise operations
//     print!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     print!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     print!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     print!("1 << 5 is {}", 1u32 << 5);
//     print!("0x80 >> 2 is 0x{:x}", 0x80u32 << 5);
    
//  }

// Char, Bool, and Unit.
//1. Character (Char)
// use std::mem::size_of_val;
// fn main() {
//     let c1: char = 'a'; // 4 bytes
//     assert_eq!(size_of_val(&c1),4);

//     print!("{}\n", size_of_val(&c1));

//     let c2: char = 'b';
//     assert_eq!(size_of_val(&c2), 4);

//     print!("Success!\n");
// }

// 1.1 Make it work
// fn main() {
//     let c1: char = 'a';
//     print_char(c1);
// }

// fn print_char(c: char) {
//     println!("{}", c);
// }

//2. Boolean
// fn main() {
//     let _f: bool = false;

//     let t: bool = true;
//     if t {
//         print!("Success!\n");
//     }
// }

// 2.1
// fn main() {
//     let f: bool = false;
//     let t: bool = true && false; // false

//     assert_eq!(t, f);

//     print!("Success!");
// }


// 3. Unit type
// Make it work, don't modify 'implicitly_ret_unit
// fn main() {
//     let _v: () = ();

//     let _x: (i32, i32) = (2, 3);
//     assert_eq!(_v, implicitly_ret_unit());

//     print!("Success!\n");
// }

// fn implicitly_ret_unit() {
//     print!("I will return a ()\n");
// }

// Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

// 3.1 What is the size of the unit type?
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success!");
// }

// RECAP
// Char: single character of size 4 bytes, represented with single quote.

// Bool: Boolean value of true or false of size 1byte.

// Unit: Empty tuple of size 0 bytes, used to return "nothing" in expressions or funtions.  

// Exercise

// Make it work with two ways

