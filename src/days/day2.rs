fn is_sorted_by(report: &Vec<u8>, compare: fn(u8, u8) -> bool) -> bool {
    // iterate over the windows of size 2 in the report vector
    for window in report.windows(2) {                
        // dynamically check if sorted
        if !compare(window[0], window[1]) {
            return false;
        }
    }
    true
}

fn is_increasing(report: &Vec<u8>) -> bool {
    is_sorted_by(report, |x, y| x < y)
}
fn is_decreasing(report: &Vec<u8>) -> bool {
    is_sorted_by(report, |x, y| x > y)
}

fn is_in_range(report: &Vec<u8>) -> bool {
    for window in report.windows(2) {      
        // get absolute diff
        let diff = window[0].abs_diff(window[1]);
        
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    // if all diff are safe
    true
}

fn is_safe(report: &Vec<u8>) -> bool {
    (is_increasing(report) || is_decreasing(report)) && is_in_range(report)
}

// + '_ is a lifetime annotation -> the returned iterator is tied to the lifetime of the input reference
fn generate_subreports(report: &Vec<u8>) -> impl Iterator<Item = Vec<u8>> + '_ {
    // basically bruteforce the solution by creating subreports where one number is sliced out
    (0..report.len()).map(|index| {
        let mut subreport = Vec::new();
        
        for (report_index, &value) in report.iter().enumerate() {
            if report_index != index {
                subreport.push(value);
            }
        }
        
        subreport
    })
}


pub fn run() -> String {
    let input = include_str!("../../inputs/day2.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let reports = input.split_terminator('\n'); // return iterator which gets rid of last empty newline directly

    // directly map over iterator
    let parsed_numbers_reports: Vec<Vec<u8>> = reports
        .map(|numbers_string| {
            numbers_string
                .split_whitespace()
                .map(|level| level.parse().expect("not a number"))
                .collect()
        })
        .collect();

    let num_safe_reports = parsed_numbers_reports
        .iter()
        .filter(|report| {
            // check if any of these subreports is safe
            is_safe(report) || generate_subreports(report).any(|subreport| is_safe(&subreport))
        })
        .count();

    num_safe_reports.to_string()
}
