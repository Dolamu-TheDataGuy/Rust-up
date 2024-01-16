#![allow(warnings)]
#[allow(unused_variables)]
#[allow(unused_imports)]


fn main() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}