# Traits

* Some of __methods__ that can be implemented for __multiple types__ in order to provide __common functionality__ and behaviour between them.

* Traits consist of __method signatures__ only, which then have to be implemented by the target type.

* Similar to "classes" in other languages, not quite the same though.

* Defines __shared behaviour__ in an abstract way.


```
trait Animal {
    fn sound(&self) -> String;
}

struct Sheep;
struct Cow;

impl Animal for Sheep {
    fn sound(&self) -> String {
        String::from("Meeh")
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Mooh")
    }
}
```

Trait Animal has one method, sound(). In the trait we define only the signature. Then we implement this trait for __every type__ we want. Here we copy the signature and write the actual method.

## Derivable Traits

* Traits that can be __automatically implemented__ for a struct or an enum by the Rust compiler.

* Called "derivable" because they can be derived automatically.

* Most common derivable traits:
- Debug: Allowing  the output content via "{:?}"
- Clone: Enables type to be duplicated with "clone()" method
- Copy: Enables type to be copied implicitly, without               requiring explicit"clone()" method.
- PartialEq: Enables comparison.


## Traits as Parameters

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

```

Traits can be used as __parameters__ for functions.

The function `notify()` takes as argument any type that has __implemented__ the Summary trait.

## Trait Bounds

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

```

Similar to example using "impl Summary" but more verbose. Trait bounds are declared like generics, after name of the function. Use __trait bounds__ if you have lots of parameters to avoid this:

```
pub fn notify(item: &impl Summary, item2: &impl Summary) {
}
```

```pub fn notify<T: Summary>(item: &T, item2: &T) {

}
```
## Where Clauses

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

if you have a function that makes __heavy use__ of trait bounds, we can use a where clause to make the code cleaner:

```
fn some_function<T, U>(t: &T, u: &U) -> i32 {
    where
        T: Display + Clone,
        U: Clone + Debug,
}
```

## Return Types that Implement Trait

```
trait Animal {}

struct Dog;
struct Cat;

impl Animal for Dog {}
impl Animal for Cat {}

fn return_dog() -> impl Animal {
    Dog {}
}

fn return_cat() -> impl Animal {
    Cat {}
}

fn main() {
    return_dog();
    return_cat();
}
```

Here we have a __trait__ Animal which is implemented for two structs, Dog and Cat.

The two functions return a struct, either Dog or Cat, that __implements__ the Animal trait.
