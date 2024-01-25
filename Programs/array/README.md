# Array

* Fixed-size collection of elements of the __same data type__ stored as contiguous block in __stack memory__.

* Signature of array is `[T, Length]` which indicated that the __length is fixed__ at compile time.

* Arrays can neither __grow__ nor __shrink__, they must __retain their size__.

* All elements in an array must be of the same type.

The type of array is `[T; Length]`, as you can see, array's length is part of their type signature. So their length must be known at compile time.

For example, you cant initialize an array like below:

```
fn init_arr(n: i32) {
    let arr = [1; n];
}
```

This will cause an error, because the compiler has no idea of the exact size of the array at compile time.
