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

    // Wrong Struct Update syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("dolamuoludare@gmail.com");
        sign_in_count: user1.sign_in_count,
    };

    // Correct syntax update
    let user2 = User {
        email: String::from("dolamuoludare@gmail.com"),
        ..user1
    }
}

// Function returning structs
fn build_user(email: String, username: String) -> User {
    // absence of the semicolon after declaring the struct means we are returning it
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

