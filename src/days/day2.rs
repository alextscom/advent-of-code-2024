pub fn run() -> String {
    let input = include_str!("../../inputs/day2.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let mut reports: Vec<&str> = input.split('\n').collect(); // split by new line
    reports.pop(); // get rid of last empty line

    let mut parsed_numbers_reports: Vec<Vec<i32>> = reports.iter()
        .map(|numbers_string: &&str| {
            numbers_string.split_whitespace()
                .filter_map(|number_str| number_str.parse::<i32>().ok())
                .collect()
        })
        .collect();

    parsed_numbers_reports.retain(|levels| {
        // windows = An iterator over overlapping subslices of length -- official doc
        // check all windows if the following number is increasing by 1 to 3 levels (same level is also not safe)
        let is_increasing = levels.windows(2).all(|window| {
            let diff = window[1] as i32 - window[0] as i32;
            diff >= 1 && diff <= 3
        });
        
        // same here but decreasing
        let is_decreasing = levels.windows(2).all(|window| {
            let diff = window[0] as i32 - window[1] as i32;
            diff >= 1 && diff <= 3
        });

        is_increasing ||is_decreasing
    });

    let number_of_safe_reports = parsed_numbers_reports.iter().count();
    

    number_of_safe_reports.to_string()
}
