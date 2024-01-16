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