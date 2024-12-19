use std::collections::{HashSet, HashMap};

pub fn run() -> String {
    let input = include_str!("../../inputs/day19.txt");
    solve(input)
}

fn can_arrange(target: &str, allowed_set: &HashSet<&str>, memo: &mut HashMap<String, bool>) -> bool {
    // check if current target is already in memoized set
    if let Some(&result) = memo.get(target) {
        return result; 
    }
    if target.is_empty() {
        return true; // emptry string always true
    }
    
    for prefix in allowed_set {
        if target.starts_with(prefix) {
            let remaining = &target[prefix.len()..];
            if can_arrange(remaining, allowed_set, memo) {
                // store processed substring
                memo.insert(target.to_string(), true);
                return true;
            }
        }
    }
    
    memo.insert(target.to_string(), false);
    false
}

fn check_strings(allowed: Vec<&str>, targets: Vec<&str>) -> Vec<(String, bool)> {
    // allowed towels hashset
    let allowed_set: HashSet<&str> = allowed.into_iter().collect();
    // keep track of [arrangement, possble]
    let mut results = Vec::new();
    
    for target in targets {
        let mut memo = HashMap::new(); // to store already processed substrings
        let arrangement_possible = can_arrange(target, &allowed_set, &mut memo);
        results.push((target.to_string(), arrangement_possible));
    }
    
    results
}

fn solve(input: &str) -> String {
    let (part1, part2) = input.split_once("\n\n").expect("failed to split input");
    let parsed_towel_designs: Vec<&str> = part1.split(",").into_iter().map(|char| char.trim()).collect();
    let parsed_target_towel_arrangment: Vec<&str> = part2.split_terminator("\n").into_iter().map(|char| char.trim()).collect();
    // println!("parsed_towls_designs: {:?}", parsed_towel_designs);
    // println!("parsed_target_towel_arrangment: {:?}", parsed_target_towel_arrangment);

    let results = check_strings(parsed_towel_designs, parsed_target_towel_arrangment);
    
    let mut possible_count = 0;
    for (target, arrangement_possible) in results {
        if arrangement_possible {
            possible_count += 1
        }
        // println!("{}: {}", target, if can_create { "Yes" } else { "No" });
    }
    possible_count.to_string()
}
