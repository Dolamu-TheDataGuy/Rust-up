fn main() {
    //use as many approaches as you can to make it work
    let x: String = String::from("hello, world");
    let y: String = x.clone();
    println!("{}, {}", x, y); //trying to access x after it has been assigned to y, x has been damaged after assigning it to y
} // we have to deepcopy x by using the clone method