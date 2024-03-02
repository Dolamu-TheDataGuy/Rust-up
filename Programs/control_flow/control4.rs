fn main() {
    let a: [i32; 4] = [4, 3, 2, 1];

    // iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println!("The {}the element is {}", i+1, v);
    }
} 