pub fn run() -> String {
    let input = include_str!("../../inputs/day7.txt");
    solve(input)
}

fn check_combinations(target: u64, numbers: &[u64]) -> bool {
    fn recursive_combination_checker(target: u64, nums: &[u64], current_value: u64) -> bool {
        if nums.is_empty() {
            return current_value == target;
        }

        let (first, rest) = nums.split_first().unwrap();

        // try adding next number
        if recursive_combination_checker(target, rest, current_value + first) {
            return true;
        }

        // try multiplying next number
        if recursive_combination_checker(target, rest, current_value * first) {
            return true;
        }

        // try writing next number after each other -- comment this out for part 1
        if recursive_combination_checker(target, rest, format!("{}{}", current_value, first).parse().expect("Failed to parse combined number")){
            return true;
        }

        false
    }

    if numbers.is_empty() {
        return false;
    }

    // start with first number in vector
    let (first, rest) = numbers.split_first().unwrap();
    recursive_combination_checker(target, rest, *first)
}

fn solve(input: &str) -> String {
    let tasks = input.split_terminator("\n");
    let parsed_tasks: Vec<(u64, Vec<u64>)> = tasks.map(|task| {
        let mut parts = task.split(':');
        let first = parts.next().unwrap_or("");
        let second = parts.next().unwrap_or("");

        let first_number = first.parse::<u64>().expect("not a number");

        let second_numbers: Vec<u64> = second.split_whitespace()
            .map(|num| num.parse::<u64>().expect("not a number"))
            .collect();

        (first_number, second_numbers)
    }).collect();

    let mut sum_of_valid_tasks= 0;

    // filter for debug only
    let _filtered_tasks: Vec<(u64, Vec<u64>)> = parsed_tasks.into_iter()
        .filter(|(num, nums)| {
            if check_combinations(*num, &nums) {
                sum_of_valid_tasks += num;
                return true
            } else {
                return false
            }
        }).collect();
    
    // print!("{:?}", filtered_tasks);

    sum_of_valid_tasks.to_string()
}
