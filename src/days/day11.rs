pub fn run() -> String {
    let input = include_str!("../../inputs/day11.txt");
    solve(input)
}

fn process_stones(stones: &mut Vec<u64>) {
    for _ in 0..25 {
        let mut new_stones = Vec::new();

        for &stone in stones.iter() {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let stone_str = stone.to_string();
                let len = stone_str.len();

                if len % 2 == 0 {
                    let mid = len / 2;
                    let left_half: u64 = stone_str[..mid].parse().unwrap();
                    let right_half: u64 = stone_str[mid..].parse().unwrap();
                    new_stones.push(left_half);
                    new_stones.push(right_half);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }

        *stones = new_stones;
    }
}

fn solve(input: &str) -> String {
    let mut parsed_numbers: Vec<u64> = input.split_whitespace().map(|number_str| number_str.parse::<u64>().expect("not a valid number")).collect();

    process_stones(&mut parsed_numbers);
    
    parsed_numbers.len().to_string()
}
