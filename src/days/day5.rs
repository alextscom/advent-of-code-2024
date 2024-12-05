use std::collections::{HashMap, HashSet};

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

        let (valid_vectors, invalid_vectors): (Vec<Vec<i32>>, Vec<Vec<i32>>) = input_vectors.into_iter()
            .partition(|vec| {
                rules.iter().all(|&(first_rule_number, second_rule_number)| {
                    let position_of_first_rule_number = vec.iter().position(|&x| x == first_rule_number);
                    let position_of_second_rule_number = vec.iter().position(|&x| x == second_rule_number);
                    match (position_of_first_rule_number, position_of_second_rule_number) {
                        (Some(first_position), Some(second_position)) => first_position < second_position,
                        // default return true because if rules aren't in input, its considered valid
                        _ => true,
                    }
                })
            });


    let mut middle_vector_result = 0;
    for vec in valid_vectors {
        let middle_index = vec.len() / 2;
        let middle_element = &vec[middle_index];
        middle_vector_result += middle_element;
    }

    println!("part 1 result: {}", middle_vector_result);

    let reordered_vectors = reorder_vector(invalid_vectors, rules);

    // println!("reorderd vectors: {:?}", reordered_vectors);

    let mut middle_vector_result_part2 = 0;
    for vec in reordered_vectors {
        let middle_index = vec.len() / 2;
        let middle_element = &vec[middle_index];
        middle_vector_result_part2 += middle_element;
    }


    middle_vector_result_part2.to_string()
}

fn reorder_vector(input_vectors: Vec<Vec<i32>>, rules: Vec<(i32, i32)>) -> Vec<Vec<i32>> {
    // lookup table for better performance
    let rule_set: HashSet<(i32, i32)> = rules.into_iter().collect();
    
    // check if input numbers follow the rules
    let follows_rule = |first_number: i32, second_number: i32| rule_set.contains(&(first_number, second_number));

    input_vectors
        .into_iter()
        .map(|vec| {
            let mut fixed_vector = vec.clone();
            let len = vec.len();

            // check all pairs in the vector for violations
            for i in 0..len {
                for j in (i + 1)..len {
                    // swap numbers if they don't follow the rule
                    if !follows_rule(fixed_vector[i], fixed_vector[j]) {
                        fixed_vector.swap(i, j);
                    }
                }
            }

            fixed_vector
        })
        .collect()
}
