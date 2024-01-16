# Ownership in Rust

The concept of ownership is peculiar to Rust and it is very logical and easy to understand.

* Rust ownership system is unique and sets it apart from other programming languages.

* Set of rule that govern memory management.

* Rules are enforced at compile time.

* If any of the rules are violated, the program won't compile.

## Three Rules of Ownership in Rust

1. Each value in Rust has an owner.

2. There can only be one owner at a time.

3. When the owner goes out of scope, the value will be dropped.

__Owner__: ```The owner of a value is the variable or data structure that holds it and is responsible for allocating and freeing the memory used to store that data.```

## Scope

Scope within a program for which an item is valid.

* __Global Scope__:
The Global scope is accessible through out the entire program.

* __Local scope__:
The local scope is accessible only within particular function or block of code an not accessible outside of that function or block.

### Code Block Explanation

```{   // s is not valid here because it is not yet declared
    let s = "hello"; // s is valid from this point forward 

    // do stuff with s

} // this scope is now over, and s is no longer valid
```

* When s comes into scope, it is valid.
* it remains valid until it goes out of scope.
* __General rule__: Scope ends where block of code ends (curly brackets)

## Memory

* Component in a computer to store data and instructions for the processor to execute.

* `Random Access Memory (RAM)` is temporary and volatile storage, when power is turned off all contents are lost.

* Two types of regions in `RAM` used by the program at runtime: `Stack` memory and `Heap` memory.

### Stack Memory
The stack memory uses the Last In First Out (`LIFO`) route to process data, meaning that the last object that was put into the data structure would be the first to be withdrawn.

* All data stored on the stack memory must have a fixed size (e.g. `integers`, `floats`, `bool`, `char`, etc...).

* Pushing  to the stack is faster than allocating to the heap, because the location for new data is always at the top of the stack.

* Type of unknown size will get allocated to the heap and a pointer to the value is pushed to stack, because a pointer is fixed in size `(usize)`.

#### Code Example

```
fn main() {
    let x = 42;
    let y = 10;
    let z = add_numbers(x, y);

    println!("The result is {}", z);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}
```
<img src="./images/stack_memory.png"/>


### Heap Memory

<img src="./images/heap_memory.png"/>

* Data of no known, fixed size belongs on the heap.

* Allocating data on the heap will return a pointer (an address to location where data has been allocated)

* Allocating on the heap is slower than pushing to stack.

* Accessing data on the heap is also slower, as it has to be accessed using a pointer which points to an address.

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

## Ownership and Functions

```
fn main() {

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s)  // s's value moves into the function...
                        // ... and s is no longer valid here.

    
    let x = 5;           // x comes into scope

    makes_copy(x);   // x would move into the function 
                     // but i32 is Copy, so it's okay to still..
                     // use x afterward

}  // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens

fn take_ownership(some_string: String) { // some_string comes into scope

    println!("{}", some_string);
}  // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.


fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out to scope. Nothing special happens.

```

### Preventing Issues.

* Ownership prevents memory safety issues:

- Dangling pointers.

- Double-free

    * Trying to free memory that has already been freed.

- Memory leaks

    * Not freeing memory that should have been freed.