fn extract_first_and_last_digit(input: &str) -> String {
   

    let digits: Vec<char> = input.chars()
        .filter(|c| c.is_digit(10))
        .collect();

    match digits.len() {
        0 => String::new(),
        1 => format!("{}{}", digits[0], digits[0]),
        _ => format!("{}{}", digits[0], digits[digits.len() - 1]),
    }
}

fn solve(input: &str) -> String {
    let words: Vec<&str> = input.split('\n').collect();
    let mut final_numbers: Vec<String> = Vec::new();
    for word in words {
        let numbers = extract_first_and_last_digit(word);
        final_numbers.push(numbers);
    }
    let sum: i32 = final_numbers.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum();

    return sum.to_string()
}

// day0 is 2023 day1 only first part for testing the rust setup
pub fn run() -> String {
    let input = include_str!("../../inputs/day0.txt");
    solve(input)
}
