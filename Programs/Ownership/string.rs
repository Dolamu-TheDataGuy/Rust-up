fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s: String = t.0;  //_s becomes the owner of the tuple data in the first index

    //Modify this line only, don't use '_s'
    println!("{:?}", t.1);
}



