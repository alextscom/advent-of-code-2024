pub fn run() -> String {
    let input = include_str!("../../inputs/day1.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let mut split_vector: Vec<&str> = input.split('\n').collect(); // split by new line
    split_vector.pop(); // get rid of last empty line

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    // parse input into two vectors of left and right numbers -- surely all of this can be done in one step but step by step helps learning i guess
    split_vector.iter().for_each(|numbers| {
        let split_numbers: Vec<&str> = numbers.split("   ").collect();
        if split_numbers.len() >= 2 {
            let left_number: i32 = split_numbers[0].parse().expect("Not a valid number");
            let right_number: i32 = split_numbers[1].parse().expect("Not a valid number");
            left_numbers.push(left_number);
            right_numbers.push(right_number);
        }
    });
    
    left_numbers.sort();
    right_numbers.sort();

    let mut total_distance = 0;
    left_numbers.iter().enumerate().for_each(|(index, value)| {
        total_distance += (right_numbers[index] - value).abs(); // get the absolute value of the substraction to get the actual distance
    });

    
    total_distance.to_string()
}
