fn main() {
    let reference_to_nothing = dangle();
    println!("The value is {}", reference_to_nothing)
}


// Doesnt work because it violates the second rule which states the references must ALWAYS be valid
// fn dangle() ->  &String {
//     let s = String::from("hello");
//     &s // returns reference of string s to the reference_to_nothing variable but s gets dropped and out of scope after this function 
        // meaning that the reference_to_nothing variable will point to a something that has been dropped and invalid.
// }

fn dangle() ->  String {
    let s = String::from("hello");
    s
}