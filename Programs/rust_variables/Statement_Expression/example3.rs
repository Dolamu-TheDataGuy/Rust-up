
fn main() {
    let v: i32 = {

        let x = 3;
        x
    };

    assert_eq!(v , 3);

    println!("Success!");

}