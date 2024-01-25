# Slice

* __Reference__ to contiguous sequence of elements in a __collection__.

* Provide a way to __borrow part__ of a collection without taking __ownership__ of the entire collection.

* Can be created from arrays, vectors, Strings, and other collections implementing the Deref trait.

## Example Slice

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2,3]);

```

slice has the type `&i32` in the example.

Work like string slices do, by storing a __reference to the first element and a length__.

Slices are similar to arrays, but their length is not known at compile time, so you can't use them directly.

#### A Slice is different from a Slice Reference
The Slice is a fat pointer that consists of a pointer to the first element and the length of the slice, while the Slice reference is a two-word object, for simplicity reasons, from now on we will use slice instead of `slice reference`. The first word is a poiter to the data, and the second word is the length of the slice. The word size is the same as `usize`, determined by the processor architecture, e.g 64 bits on an x86-64. Slices can be used to borrow a section of an array, and have the type signature &[T].
