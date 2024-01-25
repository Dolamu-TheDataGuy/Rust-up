# The String Type

The `String` type in Rust is mutable which means that its type can change (grow and shrink) at runtime. The String is stored on a stack with a pointer to the heap.

* String is mutable.

* String size can change at runtime.

* String stored on the stack with a pointer to the heap.

* Value of String is stored on the heap.

The String Type is initialized like so:

```
let s1: String = String::from("Hello);
```

<img src=./images/string_type.png>

* ptr: Pointer to the data stored on the heap.
* len: Data size in bytes.
* capacity: Total amount of memory received from the allocator.

```NB: size of variable s1 is known at runtime (pointer, len, capacity) and it is pushed to the stack memory but the content in s1 is not known and dynamically sized and it is stored on the heap.(We only need the memory address of the first content in the data.)```

## Copy Vs. Move in Rust

* Scalar values with fixed sizes (All types we covered at the beginning) will automatically get copied in the stack, copying here is cheap.

* Dynamically sized data would not get copied, but moved, copying is too expensive in this case.

### Example
```
let x = 5;
let y = x;
```
Here, the integer value of variable x will get copied into y and both variables are usable, because i32 value has been copied -> i32 is fixed size!.

```
let s1 = String::from("hello");
let s2 = s1;
```

As __s1__ is just a pointer to data on the heap just the pointer will get copied into __s2__, **NOT** the whole data on the heap!

<img src=./images/owner.png>

__s1__ and __s2__ point to the same location in heap memory.

This would **violate the second rule** of ownership which says that there can only be **ONE owner** at a time.

So, the first variable __s1__ will be **dropped** and cannot be used after assigning it to __s2__, to avoid dangling pointers.


## Deep Copy

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

<img src=./images/deep_copy.png>

This is actually expensive.


## Compound Types

### String vs. &str

A `String` is a compound data type because it is an array of character.

* A `String` is a __heap-allocated__ string type that owns its contents and is mutable. String is like the string in form of arrays in `C`.

* A `&str` is an __immutable__ sequence of UTF-8 bytes in memory, it does __not own__ the underlying data and is __immutable. `&str` is like `*str` in `C`, they are read-only.

* Think of `&str` as a __view__ on a sequence of characters (stored as UTF-8 bytes) in memory.

* Use `&str` if you just want to a view of a string.

* `&str` is more lightweight and efficient than `String`.

* Use `String` if you need to own the data and be able to mutate it.


### String Literal

* A string literal is a sequence of characters enclosed in double quote ('').

* Fixed size, compile-time known sequence of UTF-8 bytes

* The type is `& static str`, which indicates the data is stored in __static storage__, meaning it is __valid__ throughout the __entire lifetime__ of the program.

* The data is __hardcoded__ into the __executable__ and stored in __read-only memory__, meaning they are __immutable__.

#### Example of String Slice
```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

The string slice `world` points to a __sequence of characters__ stored on the heap.

<img src=./images/string_slice.png>

string slice only keep `pointer (ptr)` and `length (len)` information.

* You can only concatenate a `String` with `&str`, and `String's` ownership can be moved to another variable.
