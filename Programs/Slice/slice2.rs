fn main() {

    let arr: [char; 7] = ['x', 'y', 'z', 'a', 'b', 'c', 'd'];

    let slice: &[char] = &arr[..7]; //&['x', 'y'] // The slice (holds the address of the first element in the pointer) is different from &slice (slice reference --> This is a two-word object)

    // assert!(std::mem::size_of_val(slice) == 16); // calculating the size of the content/data in the slice.
    // assert!(std::mem::size_of_val(&slice) == 20);
    println!("The size of slice is {}", std::mem::size_of_val(slice)); // calculates the size of the content in the slice.
    println!("The size of the reference to the slice is {}", std::mem::size_of_val(&slice)); // always return 16 byte for the pointer box(8 byte) (usize) and the length box (8bytes)(usize)
}