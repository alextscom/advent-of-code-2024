pub fn run() -> String {
    let input = include_str!("../../inputs/day5.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let rules_and_input: Vec<&str> = input.split_terminator("\n\n").collect();

    let rules: Vec<(i32, i32)> = rules_and_input[0]
        .split("\n")
        .map(|numbers_str| {
            let mut nums = numbers_str.split("|").map(|number_str| number_str.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let input_vectors: Vec<Vec<i32>> = rules_and_input[1]
        .split_terminator("\n")
        .map(|numbers_str| 
            numbers_str.split(",")
                .map(|number_str| number_str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        )
        .collect();

    let valid_vectors: Vec<Vec<i32>> = input_vectors.into_iter()
        .filter(|vec| {
            rules.iter().all(|&(first_rule_number, second_rule_number)| {
                let position_of_first_rule_number = vec.iter().position(|&x| x == first_rule_number);
                let position_of_second_rule_number = vec.iter().position(|&x| x == second_rule_number);
                match (position_of_first_rule_number, position_of_second_rule_number) {
                    (Some(first_position), Some(second_position)) => first_position < second_position,
                    // default return true because if rules aren't in input, its considered valid
                    _ => true,
                }
            })
        })
        .collect();


    let mut middle_vector_result = 0;
    for vec in valid_vectors {
        let middle_index = vec.len() / 2;
        let middle_element = &vec[middle_index];
        middle_vector_result += middle_element;
    }

    middle_vector_result.to_string()
}
