# PRIVACY AND PUBLICITY RULES FOR MODULES
The default state of all code in  Rust is private. If you do not use a private function within your program, because your program is the only code allowed to use that function. Making a function public lets Rust know that the function will be used by code outside of your program.

## Privacy Rules

Overall, these are the rules for item visibility:

* If an item is public, it can be accessed through any of its parent modules.

* If an item is private, it can be accessed only by its immediate parent module and any of the parent's child modules.

## Privacy Examples

```
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn_try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

## The `use` keyword

We use the `use` keyword in bringing names into scope. For example the following code

```
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

use a::series::of

fn main() {
    of::nested_modules();
}

```

Another example using enums:

```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red,
    let yellow = Yellow,
    let green = TrafficLight::Green;
}
```

## Bringing All names into scope with a Glob
To bring all the items in a namespace into scope at once, we can use the `*` syntax just like in Python, which is called the `glob` operator. This example brings all the variants of an enum into scope without having to list each specifically:

```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

## Using super to Access a Parent Module
