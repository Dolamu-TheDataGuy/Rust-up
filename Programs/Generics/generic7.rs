fn main() {
    let number_list: Vec<i32> = vec![34, 50, 24, 67, 100];

    let mut largest: i32 = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list: Vec<i32> = vec![102, 34, 60000, 89, 54, 2, 8, 91];

    let mut largest: i32 = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

}