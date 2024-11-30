pub fn run() -> String {
    let input = include_str!("../../inputs/day1.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    input.trim().to_string()
}
