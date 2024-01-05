# Data Types in Rust

In Rust, data types have well-defined sizes in bytes and are guaranteed by the language. We have the following data types in Rust:

## Integer Types

The integer types just like every other language are used to hold whole numbers and there are different flavors of integer data types in rust, there are the unsigned and signed integer types and they have different capacities and sizes.

### Signed integer (i)

* `i8`: Signed 8-bit integer (Positive and Negative values)
    * Size: 1 byte (8bits)
    * Potential values: -128 to 127.

* `i16`: Signed 16-bit integer.
    * Size: 2 bytes.
    * Potential values: -32,768 to 32,767.

*  `i32`: Signed 32-bit integer.
    * Size: 4 bytes.
    * Potential values: -32,768 to 32,767.

*  `i64`: Signed 64-bit integer.
    * Size: 8 bytes.
    * Potential values: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807.

*  `i128`: Signed 128-bit integer.
    * Size: 16 bytes.
    * Potential values: Extremely large range.

*  `isize`: Signed integer with a size matching the target architecture (32-bit or 64-bit).
    * Size: Varies (4 bytes on 32-bit, 8 bytes on 64-bit).
    * Potential values: Platform-dependent.

### Unsigned integer (u)

* `u8`: Unsigned 8-bit integer (Only positive).
    * Size: 1 byte (8bits).
    * Potential values: 0 to 255.

* `u16`: Unsigned 16-bit integer.
    * Size: 2 bytes.
    * Potential values: 0 to 65,535.

*  `u32`: Unsigned 32-bit integer.
    * Size: 4 bytes.
    * Potential values: 0 to 4,294,967,295.

*  `u64`: Unsigned 64-bit integer.
    * Size: 8 bytes.
    * Potential values: 0 to 18,446,744,073,709,551,615.

*  `u128`: Unsigned 128-bit integer.
    * Size: 16 bytes.
    * Potential values: Extremely large range.

*  `usize`: Unsigned integer with a size matching the target architecture (32-bit or 64-bit).
    * Size: Varies (4 bytes on 32-bit, 8 bytes on 64-bit).
    * Potential values: Platform-dependent.


## Floating-point Types
The floating-point types in the rust are used to hold decimal numbers. They are also varieties of the floating-point data types.

* `f32`: Single-precision floating-point number.
    * Size: 4 bytes. 
    * Potential values: Depends on IEEE-754 standard, approximately ±1.18 x 10^-38 to ±3.4 x 10^38.

* `f64`: Double-precision floating-point number(commonly used for floating-point values).
    * Size: 4 bytes. 
    * Potential values: Depends on IEEE-754 standard, approximately ±1.18 x 10^-38 to ±3.4 x 10^38.

