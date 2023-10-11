// STRUCT
// * compound type allowing to group together values of different types into
// a named data structure.

// Similar to tuples, but each value has a name so values can be accessed
// through his name.

// Have to be instanciated with data, think of it like the struct is the
// template for the instances you create from it.

// Template
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Instance of struct User
    let mut user1 = User {
        active: true,
        username: String::from("Dolamu"),
        email: String::from("oludaredolamu@gmail.com"),
        sign_in_count: 1,
    };
}