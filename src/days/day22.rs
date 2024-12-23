use std::collections::{HashMap, HashSet};

pub fn run() -> String {
    let input = include_str!("../../inputs/day22.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let mut total_sum = 0;
    let mut max_bananas: HashMap<u32, i64> = HashMap::new();

    for line in lines {
        let mut secret_number = line.parse::<i64>().unwrap();
        let mut seen_sequences = HashSet::new();
        let mut current_sequence = 0u32;
        let mut previous_price = secret_number % 10;

        for current_secret_number_index in 0..2000 {
            secret_number = (secret_number ^ (secret_number * 64))   % 16777216;
            secret_number = (secret_number ^ (secret_number / 32))   % 16777216;
            secret_number = (secret_number ^ (secret_number * 2048)) % 16777216;

            let current_price = secret_number % 10;
            let price_difference = current_price - previous_price;
            current_sequence = (current_sequence << 8) + (price_difference + 10) as u32;

            if current_secret_number_index >= 3 && !seen_sequences.contains(&current_sequence) {
                seen_sequences.insert(current_sequence);
                *max_bananas.entry(current_sequence).or_default() += current_price;
            }

            previous_price = current_price;
        }

        total_sum += secret_number;
    }

    let max_banana_value = max_bananas.values().max().unwrap();
    println!("max_banana_value: {}", max_banana_value);
    total_sum.to_string()
}
