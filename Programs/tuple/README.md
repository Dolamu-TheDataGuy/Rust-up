# Tuple in Rust

* Way to stored related pieces of information in a single variable.

* Collection of values of __different types__ grouped together as a single __compound value__ (type composed of other types).

* Stored as a fixed-size contiguous block of memory on the stack.

* Signature is (T, T, T...).

Elements in a tuple can have different types. Tuples type signature is (T1, T2, ...), where T1, T2 are the types of tuple's members.

```
fn main() {
    let _t0: (u8, i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"))
    println!("Success!");
}
```
