//  Arrays are fixed and are stored in a contiguous block of memory


// You cant initialize an array like this, because the compiler has no idea of the 
// size of the array
// fn init_arr(n: i32) {
//     let arr = [1; n];
// }

//1 .
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     assert!(arr.len() == 5);

//     println!("Success!");
// }

// 2
// fn main() {
//     let _arr0:[i32;3] = [1,2,3];
//     let arr: [char;3] = ['a', 'b', 'c'];

//     assert!(std::mem::size_of_val(&arr) == 12);

//     println!("Success\n");

// }

//All arrays can be initialized with the same value at once.
// fn main() {
//     let list: [i32; 100] =  [1; 100];

//     assert!(list[0] == 1);
//     assert!(list.len() == 100);

//     println!("Success!");
// }

// All elements  in an array must be of the same type
// fn main() {
//     // Fix the error
//     let_arr: [i32, 3] = [1, 2, 4];

//     println!("Success!");
// }

// Indexing starts at zero (0)
// fn main() {
//     let arr: [char; 3] = ['a', 'b', 'c'];

//     let ele = arr[0];
    
//     assert!(ele == 'a');

//     println!("Success!");
// }


// 6
// Out of bounds indexing causes panic
// fn main() {
//     let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()]

//     // Get returns an Option<T>, it's safe to use
//     let name0 = names.get(0).unwrap();

//     let _name1 = &names[2];

//     println!("Success!");
    
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let slice = &a[1..3];

//     assert_eq!(slice, &[2,3]);
// }

// Fix the errors, Dont add a new line
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     let s1: &[i32] =  &arr[0..2]; // &[1. 2]
//     assert_eq!(s1, [1,2]);
//     let s2: &str = "hello, world"; //string literal
//     assert_eq!(s2, "hello, world");

//     println!("Success!");
// }

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3,4, 5];
//     // Fill in the blanks to make the code work
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);

//     println!("Success!");
// }


// fn main() {
//      let s: String = String::from("hello");

//      let slice1: &str = &s[0..2];

//      let slice2: &str = &s[..2];

//      assert_eq!(slice1, slice2);

//      println!("Success!")

// }


// &String can be implicitly converted to &str.
fn main() {
    let mut s: String = String::from("hello wold");

    let word: &str = first_word(&s); // &str
    println!("the first word is {}", word);
    
    s.clear();  //error
}


fn first_word(s: &str) -> &str {
    &s[..1]
}