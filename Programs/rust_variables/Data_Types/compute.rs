#![allow(warnings)]
#[allow(unused_variables)]

fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8  == -1);


    assert!(3 * 50 == 150) // i32

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32 )

    assert!(24 % 5 == 4);
    
    //Short-circuiting boolean logic
    assert!(true && false == __); 
    assert!(true || false == __);
    assert!(!true == __);

    //Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);


}