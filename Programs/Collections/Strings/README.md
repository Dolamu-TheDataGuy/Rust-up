# String Collection in Rust

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. String literals, for example, are stored in the binary output of the program and are therefore string slices.

The `String` type, is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustacean's refer to "string" in Rust, they usually mean the `String` and the `string slice &str` types, not just one of those types. Both `String` and `string slice`  are [UTF-8 encoded](https://docs.google.com/document/d/1jiSe3Si385i8kuYnFVcUV8PrUGJKdlalEI-_SWxirqQ/edit).

## Creating a New String

Many of the same operation available with `Vec<T>` are available with `String` as well, starting with the `new` function to create a string, shown below:

```let mut s = String::new;
```

The line create a new, empty string called `s`, which we can then load data into. The data works on string `&str`. 

```
let data = "initial contents";

let s = data.to_string();

// the method also workd on a literal directly:
let s = "initial contents".to_string();
```

OR 

```
let s  = String::from("Initial content");
``` 
