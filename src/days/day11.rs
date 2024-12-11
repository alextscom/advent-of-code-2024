use std::collections::HashMap;

pub fn run() -> String {
    let input = include_str!("../../inputs/day11.txt");
    solve(input)
}

fn process_stones(mut parsed_numbers: HashMap<i64, usize>) -> (usize, usize) {
    let mut part_1_solution = 0;
    for i in 0..75 {
        // extract part 1 solution
        if i == 25 {
            part_1_solution = parsed_numbers.values().sum();
        }
        parsed_numbers = {
            let mut new_stones = HashMap::new();
            for (&stone_number, &stone_value) in &parsed_numbers {
                if stone_number == 0 {
                    *new_stones.entry(1).or_default() += stone_value;
                    continue;
                }
            
                let stone_str = stone_number.to_string();
                let len = stone_str.len();
                if len % 2 == 0 {
                    let mid = len / 2;
                    let left_half: i64 = stone_str[..mid].parse().unwrap();
                    let right_half: i64 = stone_str[mid..].parse().unwrap();
                    *new_stones.entry(left_half).or_default() += stone_value;
                    *new_stones.entry(right_half).or_default() += stone_value;
                    continue;
                }
            
                *new_stones.entry(stone_number * 2024).or_default() += stone_value;
            }
            new_stones
        };
    }
    (part_1_solution, parsed_numbers.values().copied().sum::<usize>())
}

fn solve(input: &str) -> String {
    /* 
        most important consideration for part 2:
            order of stones doesn't matter so we use a hashmap to count only unique numbers
            -> we don't need to iterate over all of the numbers after each step but only over the unique ones
    */

    let parsed_numbers = input.split_whitespace()
        // puzzle input only has unique numbers so we add them to the hashmap with value 1
        .filter_map(|number_str: &str| number_str.parse::<i64>().ok().map(|number| (number, 1)))
        .collect::<HashMap<i64, usize>>();

    let (part_1_solution, part_2_solution) = process_stones(parsed_numbers);

    println!("part 1: {}", part_1_solution);
    part_2_solution.to_string()
}
