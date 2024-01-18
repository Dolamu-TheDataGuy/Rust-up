fn main() {
    let (x,y) = (1,2);

    let s: i32 = sum(x, y); //  caller

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y:i32) -> i32 { // callee
    x + y
}