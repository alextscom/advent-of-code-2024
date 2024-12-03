extern crate regex;

use regex::Regex;

fn split_and_multiply_numbers_str(numbers_str: &&str) -> i32 {
    let split_numbers: Vec<&str> = numbers_str.split(",").collect();
    let left_number: i32 = split_numbers[0].parse().expect("Not a valid number");
    let right_number: i32 = split_numbers[1].parse().expect("Not a valid number");
    left_number * right_number
}

pub fn run() -> String {
    let input = include_str!("../../inputs/day3.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let mult_reg_ex = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut numbers_to_multiply = Vec::new();
    let mut numbers_to_multiply_part_2 = Vec::new();

    for cap in mult_reg_ex.captures_iter(input) {
        if let Some(matched) = cap.get(1) {
            numbers_to_multiply.push(matched.as_str());
        }
    }

    let mut added_multiplications = 0;

    numbers_to_multiply.iter().for_each(|numbers_str| {
        added_multiplications += split_and_multiply_numbers_str(numbers_str)
    });

    // keep part 1 solution
    println!("part 1 solution {}", added_multiplications);

    // track do matches
    let mut last_was_do = true;

    let mut text_position = 0;

    // go through text step by step and track with do and don't regexes what positions in text are valid for the mult regex
    while text_position < input.len() {
        if let Some(mat) = do_re.find(&input[text_position..]) {
            if mat.start() == 0 {
                text_position += mat.end();
                last_was_do = true;
                continue;
            }
        }

        if let Some(mat) = dont_re.find(&input[text_position..]) {
            if mat.start() == 0 {
                text_position += mat.end();
                last_was_do = false;
                continue;
            }
        }

        // looks a bit complicated but first find the position for the mult regex to then capture only the numbers inside the matched string you got by using .find(), maybe there's a better solution
        if let Some(mat) = mult_reg_ex.find(&input[text_position..]) {
            if mat.start() == 0 {
                let matched_string = &input[text_position + mat.start()..text_position + mat.end()];
                if last_was_do {
                    if let Some(caps) = mult_reg_ex.captures(matched_string) {
                        if let Some(matched) = caps.get(1) {
                            numbers_to_multiply_part_2.push(matched.as_str());
                        }
                    }
                }
                text_position += mat.end();
                continue;
            }
        }

        text_position += 1;
    }

    let mut added_multiplications_part_2 = 0;

    numbers_to_multiply_part_2.iter().for_each(|numbers_str| {
        added_multiplications_part_2 += split_and_multiply_numbers_str(numbers_str)
    });

    added_multiplications_part_2.to_string()
}
