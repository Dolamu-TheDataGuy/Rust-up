fn main() {
    let names: [String; 2] = [String::from("liming"), String::from("hanmeimei")];
    for name in &names { // for loop would take ownership of the provided value. so we have to loop through reference of array
          print!("{}\n", name);
    }

    println!("{:?}", names);

    let numbers: [i32; 3] = [1, 2, 3];

    for n in numbers {
        print!("twice of {} is {}\n", n, n*2);
    }
    print!("The array is {:?}\n", numbers); // data types in array number have a fixed size and are placed on the stack so you do not require reference or borrowing
}