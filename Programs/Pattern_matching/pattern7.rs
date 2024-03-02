#[derive(PartialEq)]


enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let mut count = 0;

    let v: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo]; // vectors are similar to arrays, except that they are dynamic in size.

    for e in v {
        // if e == MyEnum::Foo {
        //     count += 1;
        // }

        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}