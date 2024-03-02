# Generics

* __Placeholders__ for __concrete types__.

* Enables writing more __reusable__ and __flexible__ code

* Avoids having __duplicate__ code for different types.

* __Zero cost abstraction__, Rust compiler will at compile time fill  out generics with concrete types.


## Const Generics

* Type parameter that represents a compile-time __constant value__.

* Allows to write generic code that operates on values that are known at compile time.

* Used for array sizes, bit widths and other constant.
