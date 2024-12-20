use std::collections::{HashSet, HashMap};

pub fn run() -> String {
    let input = include_str!("../../inputs/day19.txt");
    solve(input)
}

fn count_possible_arrangements(word: &str, allowed_set: &HashSet<&str>, memo: &mut HashMap<String, usize>) -> usize {
    // check if current target is already in memoized set
    if let Some(&count) = memo.get(word) {
        return count;
    }
    if word.is_empty() {
        // emptry string always true
        return 1;
    }
    
    // just count, no check needed since 0 is a valid outcome
    let mut total_count = 0;
    
    for prefix in allowed_set {
        if word.starts_with(prefix) {
            let remaining = &word[prefix.len()..];
            total_count += count_possible_arrangements(remaining, allowed_set, memo);
        }
    }
    
    // store processed substring
    memo.insert(word.to_string(), total_count);
    total_count
}

fn count_all_solutions(allowed: Vec<&str>, targets: Vec<&str>) -> Vec<(String, usize)> {
    // allowed towels hashset
    let allowed_set: HashSet<&str> = allowed.into_iter().collect();
    // keep track of [arrangement, number of possible solutions]
    let mut results = Vec::new();
    
    for target in targets {
        let mut memo = HashMap::new(); // to store already processed substrings
        let count = count_possible_arrangements(target, &allowed_set, &mut memo);
        results.push((target.to_string(), count));
    }
    
    results
}

fn solve(input: &str) -> String {
    let (part1, part2) = input.split_once("\n\n").expect("failed to split input");
    let parsed_towel_designs: Vec<&str> = part1.split(",").into_iter().map(|char| char.trim()).collect();
    let parsed_target_towel_arrangment: Vec<&str> = part2.split_terminator("\n").into_iter().map(|char| char.trim()).collect();
    // println!("parsed_towls_designs: {:?}", parsed_towel_designs);
    // println!("parsed_target_towel_arrangment: {:?}", parsed_target_towel_arrangment);

    let results = count_all_solutions(parsed_towel_designs, parsed_target_towel_arrangment);
    
    let mut sum_possible_counts = 0;
    for (target, count) in results {
        sum_possible_counts += count;
        println!("{}: {}", target, count);
    }
    sum_possible_counts.to_string()
}
