extern crate regex;

use regex::Regex;

pub fn run() -> String {
    let input = include_str!("../../inputs/day3.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    println!("{}", input);
    let reg_ex = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();

    let mut numbers_to_multiply = Vec::new();
    
    for cap in reg_ex.captures_iter(input) {
        if let Some(matched) = cap.get(1) {
            numbers_to_multiply.push(matched.as_str());
        }
    }
    
    println!("{:?}", numbers_to_multiply);

    let mut added_multiplications = 0;

    numbers_to_multiply.iter().for_each(|numbers_str| {
        println!("{}", numbers_str);
        let split_numbers: Vec<&str> = numbers_str.split(",").collect();
        let left_number: i32 = split_numbers[0].parse().expect("Not a valid number");
        let right_number: i32 = split_numbers[1].parse().expect("Not a valid number");
        println!("left number: {}, right number: {}", left_number, right_number);
        added_multiplications += left_number * right_number;

    });



    added_multiplications.to_string()
}
