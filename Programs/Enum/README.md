# Enum in Rust

* Way of definig a type with only one of a possible set of values.

* We can only access one variant of an enum at a time.

* Can hold additional information using tuples.

* Especially useful when using in `match` statements.

## Example Enum

```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

```

Enum for IP addresses. An IP address can __either__ be of __V4 format__ or __V6 format__. Each variant in the enum holds a String value.

## The Option Enum

* Option is an enum that represents a value that __may or may not be present__.

* Known in other languages as `null` or `None`, refering to the absence of value.

* Used to handle cases where a function or method __might fail__ to __return__ a value.

Option enum as defined by the standard library.

```
enum Option<T> {
    None,
    Some(T),
}
```

```
fn main() {
    let five = Some(5); // Variant of Option Enum
    let six = plus_one(five);
    let none = plus_one(None);
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```
#### Code Explanation

`plus_one()` expects an argument of type `Option`, so we have to wrap an `i32` inside `Some()`.

x gets __matched__ and if there is a `Some()` value, it gets incremented by one.
