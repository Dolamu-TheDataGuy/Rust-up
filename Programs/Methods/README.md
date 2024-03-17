# Methods in Rust
Methods are similar to functions: Declared with `fn`, they have parameters and return value. Unlike functions, methods are defined within the context of a struct(or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

* Function that is associated with a __particular type__ or __struct__

* Takes a __parameters__ and __returns a value__, but is defined as a __member__ of a struct or enum.

* Called using __dot notation__ like accessing members of a struct.

* Implemented through an `impl` block.

## Example of a Struct
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

```

Inside the `impl` block for the type `Rectangle` we define the method `area()` that doesn't take any arguments and __returns the product__ of width and height of an instance as an `u32` integer.

```
let rect1: Rectangle = Rectangle {
    width: 30,
    height: 50,
}

println!("The area of the rectangle is {} square pixels.", rect1.area());

```

Here, we create an instance of `Rectangle` with values for `width` and `height`. Then we can call the method using dot notation on the instance we've created.

__self__ refers to the instance the method is called upon, in this case `rect1`. `self` will take ownership of current struct instance, however, `&self` will  borrow a reference from the instance. `self` refers to the instantiated object while `Self` refers to the `Class`.

## Associated Functions

* Function that is associated with a struct or an enum, but __doesn't take an instance__ as its first parameter.

* Called using the name of the type, not an instance of it

* Often used as __constructors__ for a struct or enum.

## Example of Associated Function

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}

```
`new()` is an __associated function__ because it doesnt have `self` as its first parameter.

So to call the `new()` function, we don't need an instance of the struct, rather we call it on the class or type..


```
let rect1: Rectangle = Rectangle::new(5, 10);

println!("Rectangle: {:?}", rec1);
```

We can then call the associated function by using the name of the struct and the method name sperated by `::`.
