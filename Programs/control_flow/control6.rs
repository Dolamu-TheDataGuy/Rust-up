// Break statement is used to break out of a loop.

fn main() {
    let mut n: i32 = 0;

    for _i in 0..100 {
        if n == 66 {
            break;
        }

        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}