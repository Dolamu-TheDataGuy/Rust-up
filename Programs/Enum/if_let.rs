#![allow(warnings)]
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire: Direction = Direction::South;

//     match dire {
//     Direction::East => println!("East"),
//     Direction::South | Direction::North => {
//             println!("South or North")
//         },
//         _ => println!("West")
//     };
// }


fn main() {
    let boolean: bool = true;

    let binary:u8 =  match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    print!("Success!\n")
}