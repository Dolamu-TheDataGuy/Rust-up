// Tuple Structs
// Like normal structs but using tuple like syntax for defining the fields.

// Basically a named tuple.

// Instantiated by parenthesis instead of curly braces.

// Accessed through point notation

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

// Unit-Like Structs
// Struct without any field
// Used when working with traits 
// Doesn't store any data