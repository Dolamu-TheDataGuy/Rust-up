// STRUCT
// * compound type allowing to group together values of different types into
// a named data structure.

// Similar to tuples, but each value has a name so values can be accessed
// through his name.

// Have to be instanciated with data, think of it like the struct is the
// template for the instances you create from it.

// Template

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Instance of struct User
    let user1 = User {
        active: true,
        username: String::from("Dolamu"),
        email: String::from("oludaredolamu@gmail.com"),
        sign_in_count: 1,
    };

    // Wrong Struct Update syntax
    let user2_wrong = User {
        active: user1.active,
        username: String::from("Dolamu"),
        email: String::from("dolamuoludare@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Correct syntax update
    let user2_correct = User {
        email: String::from("dolamuoludare@gmail.com"),
        ..user1
    };
    println!("User 2 (Wrong Update Syntax): {:?}", user2_wrong);
    println!("User 2 (Correct Update Syntax): {:?}", user2_correct);
}

// // Function returning structs
// fn build_user(email: String, username: String) -> User {
//     // absence of the semicolon after declaring the struct means we are returning it
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

