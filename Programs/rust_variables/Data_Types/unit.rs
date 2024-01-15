// Unit type in rust is like None in Python
#![allow(warnings)]
#[allow(unused_variables)]
#[allow(unused_imports)]


fn main() {
    let v: () = ();

    let x: (i32, i32) = (2,3);
    assert_eq!(v, implicitly_ret_unit()); 

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicitly_ret_unit() {
    println!("I will return a ()");
}