# Vector in Rust

A vector allows you to store a variable number of values next to each other or in a contiguous memory level.

## Storing List of Values with Vectors

Vectors are represented as `Vec<T>` in Rust. Vector can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

## Creating a New Vector

We create new vectors with `Vec::new()`

```let v: Vec<i32> = Vec::new();
```

Because we have given the initial i32 values, Rust can infer that the type of `v` is `Vec<i32>` and the type annotation isnt neccessary. Next, we'll look at how to modify a vector.

```let v = vec![1,2,3];
```

## Updating a vector

We use the `push` method to update a vector. As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword.

```let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

```

## Dropping a Vector Drops its Elements

A vector is freed when it goes out of scope, as annotated below.

```{
    let v = vec![1, 2, 3, 5];

    // do stuff with v

} // <- v goes out of scope and is freed here
```

## Reading Elements of Vectors

There are 2 ways to reference a value stored in a vector. Either with indexing syntax or using the `get` method.

```let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];

let third: Option<&i32> = v.get(2);

```

using the indexing syntax, we can get the element by using `&` and `[]`, which gives us a reference, or by using the `get` method with the index passed as an argument, which gives us an Option<&T>.

When the `get` method is passed an index that is outside the vector, it returns `None` without panicking. Your code will then have the logic to handle having either `Some(&element)` or `None`.

For example, the index could be coming from a person entering a number. If they accidentally enter a number that’s too large and the program gets a `None` value, you could tell the user how many items are in the current vector and give them
another chance to enter a valid value. That would be more user-friendly than crashing the program due to a typo!

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid. The rule states that you can't have a mutable and immutable reference in the same scope.

```let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0] //immutable borrow

v.push(6); //mutable borrow
```

You might ask that why should a reference to the first element care about what changes at the end of the vector? This error is due to the wat vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

## Iterating over the Values in a Vector

if we want to access each element in a vector in turn, we can iterate through all of the elements rather than use indexes to access one at a time. We use for loop to get immutable references to each element in a vector of `i32` values and print them.

``` let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The for loop will add 50 to each element.

``` let mut v = vec![100, 32, 57];

for i in &mut v {
    *1 += 50;
}
```

To change the value that the mutable reference refers to, we have to use the dereference operator `(*)` to get the value in i before we can use the `+=` operator.


