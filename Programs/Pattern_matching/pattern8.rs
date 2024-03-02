fn main() {

    let o: Option<i32> = Some(7);

    // Remove the whole `match` block, using `if let` instead
    // match o {
    //     Some(i) => {
    //         println!("This is a really long string and `{:?}", i);

    //         print!("Success!");
    //     }

    //     _ => {}
    // };

    if let Some(i) = o {
        println!("This is a really long string and {:?}", i);

        print!("Success!\n");
     }
}