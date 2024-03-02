# Flow Control in Rust

* Normal flow of a program: __Top to bottom, line by line__.

* Concept that refers to ability to control the order in which statements or instructions are executed in a program.

* Allows to specify which instructions should be executed under which conditions and in what order.

* Conditionals
    * if/else

    * match

* Loop (Special in Rust)

    * for/while/loop

    * continue/break.

The `for in` construct can be used to iterate through an iterator e.g range, a..c

##### Iterating through an array
To iterate and get the index of an iterable in Rust, we have to use the `iter()` to convert the data structure into an iterabe and  the `enumerate()` method to get the index of the array while iterating.
```
fn main() {
    let a: [i32; 4] = [4, 3, 2, 1];

    // iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println("The {}the element is {}" i+1, v);
    }
}
```

##### While loop
The while loop is used to run a loop so far a condition remains true.


```
fn main() {
    // A counter variable
    let mut n: i32 = 1;

    // Loop while the condition is true
    while n <= 10 {

        if n % 15 == 0 {
            println!("Fizzbuzz ");
        }
        else if n % 3 == 0 {
            println!("Fizz ");
        }
        else if n % 5 == 0 {
            print!("Buzz ");
        }
        else {
            print!("{} ", n);
        }

        n += 1;
    }

    print!("\nn reached {}, so loop is over\n", n);
}

```

##### Break statement

The `break` statement is used to break out of a loop.

```
fn main() {
    let mut n: i32 = 0;

    for _i in 0..100 {
        if n == 66 {
            break;
        }

        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
```


##### Continue statement

The `continue` statement will skip over the remainig code in current iteration and go to the current iteration. To show its functionality let's write a code to loop through a range of 0 to 100 and breaks out of the loop when the number is `66`.

```
fn main() {
    let mut n: i32 = 0;

    for _i in 0..100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
```

##### Loop
Loop is used together with `break` and `continue`. it is used to run an infinite loop. Let's consider the code below:

```
fn main() {
    let mut count: u32 = 0u32;

    print!("Let's count until infinity!\n");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            print!("three\n");

            // Skip the rest of this iteration, program will ignore the rest of the code and restart the loop.
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough\n");

            break; 

        }
    }

    assert_eq!(count, 5);

    print!("Success\n");
}
```
In the loop, we increase the `count` variable and  infinitely loop to check if the variable is equal to `3` or `5`, when it's equal to 3, we would skip the iteration and restart the loop with the `continue` statement and when the variable is `5`, we break out of the loop using  the `break` statement.

Loop is also an expression, so we can use it with `break` to return a value to a variable.

```
fn main() {
    let mut counter = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // breaks and return counter
        }
    };

    assert_eq!(result, 20);

    print!("Success!");
}

```
It is possible to break or continue outer loops when dealing with nested loop in these cases, the loops must be annotated with some __`label__ and the label must be passed to the breal/continue statement.

```
#[allow(unused_labels)]
fn main() {
    let mut count = 0;

    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // break also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);
    

    print!("Success!");
}
```
##### Code Explanation:

The inner loops do not run concurrently. The program enter the outer loop with a variable `count` having a value of 0, the program first runs the inner loop and until the value of count is `20`, when `count` is `20`, the program enters the if statement in the inner1 loop and then breaks out of the 'inner1 loop. Then increments the value of count by `5` to give `25`. The program then moves into the 'inner2 loop and check if the value of count is greater than or equals to `30`, which is False. The program continues the 'outer loop and then increases count by `5` to a value of `30`. The program skips the 'inner1 loop since we have broken out from it and enters into the 'inner2 loop and then check the if statement, which is True. This would break out of the 'outer loop and end the program.
