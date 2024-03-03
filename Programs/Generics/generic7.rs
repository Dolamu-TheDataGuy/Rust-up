fn main() {
    let number_list: Vec<i32> = vec![34, 50, 24, 67, 100];

    let largest: i32 = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list: Vec<char> = vec!['y', 'b', 'c', 'm'];

    let largest: char = get_largest_char(char_list);

    println!("The largest number is {}", largest);

}

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest: i32 = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest

}

fn get_largest_char(number_list: Vec<char>) -> char {
    let mut largest: char = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest

}