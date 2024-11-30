// day0 is 2023 day1 for testing the rust setup
pub fn run() -> String {
    let input = include_str!("../../inputs/day0.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let words: Vec<&str> = input.split('\n').collect();
    for word in words {
        println!("{}", word);
    }
    input.trim().to_string()
}
