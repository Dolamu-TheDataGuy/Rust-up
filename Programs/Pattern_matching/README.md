# Pattern Match

*Powerful construct that allows to __compare a value__ against a set of __patterns__, then __execute__ different code based on which pattern matches

* Patterns can be made up of literal values, variable names, wildcards etc.

* In match, __all possible cases__ must be handled, enforce by the compiler.


## Example Match

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin::Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
Here, we have an enum Coin which hold __different denominations__ of US coins.

The function `value_in_cents()` takes as argument a Coin (which can only __hold one field__ of the enum) and checks, which field in the enum has been __matched__. Then return the right amount as `u8`.

## if let

```
let config_max = Some(3u8);

match config_max {
    Some(max) => {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
```

In a match statement, every case has to be handled. This sometimes leads to annoying boilerplate code that is not neccessary. Instead we can use if let to unwrap a value of an Option type.

```
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

Another example
```
#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire: Direction = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        },
        _ => println!("West"),
    }
}
```

`Match` is an expression, so we can use it in assignments.

```
fn main() {
    let boolean: bool = true;

    let binary: u8 = match boolean {
        true => 1,
        false => 0,
    }

    assert_eq!(binary, 1);

    println!("Success!);
}
```

## matches!
`matches!` looks like `match`, but can do something different.

```
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
    }

    println!("Success!");
}
```