fn main() {
    let age: Option<i32> = Some(32);

    if let Some(age) = age { // We are shadowing age, create a new variable with the same name as previous 
        assert_eq!(age, 32);
    } // The new variable `age` goes out of scope here

match age {
    // Match can also introduce a new shadowed variable
    Some(age) => println!("age is a new variable, it's value is {}", age),
    _ => ()

}
}