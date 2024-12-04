extern crate regex;
use regex::Regex;

pub fn run() -> String {
    let input = include_str!("../../inputs/day4.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    // lines
    let forward_lines: String = input.lines().collect();
    let backward_lines: String = input.chars().rev().collect();


    // columns
    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut transposed = vec![String::new(); max_len];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }

    let forward_columns: String = transposed.concat();
    let backward_columns: String = forward_columns.chars().rev().collect();


    // diagonals
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut diagonals = Vec::new();

    for col in 0..cols {
        let mut diagonal = String::new();
        let mut r = 0;
        let mut c = col;
        while r < rows && c < cols {
            if let Some(ch) = lines[r].chars().nth(c) {
                diagonal.push(ch);
            }
            r += 1;
            c += 1;
        }
        diagonals.push(diagonal);
    }

    for row in 1..rows {
        let mut diagonal = String::new();
        let mut r = row;
        let mut c = 0;
        while r < rows && c < cols {
            if let Some(ch) = lines[r].chars().nth(c) {
                diagonal.push(ch);
            }
            r += 1;
            c += 1;
        }
        diagonals.push(diagonal);
    }

    let forward_diagonals: String = diagonals.concat();
    let backwards_diagonals: String = forward_diagonals.chars().rev().collect();

    let re = Regex::new(r"(?=XMAS)").unwrap();
    println!("forward_lines: {}", forward_lines);
    let forward_lines_count = re.find_iter(&forward_lines).count();
    let backward_lines_count = re.find_iter(&backward_lines).count();
    let forward_columns_count = re.find_iter(&forward_columns).count();
    let backward_columns_count = re.find_iter(&backward_columns).count();
    let forward_diagonals_count = re.find_iter(&forward_diagonals).count();
    let backward_diagonals_count = re.find_iter(&backwards_diagonals).count();

    let all_xmas_count = forward_lines_count
                                + backward_lines_count 
                                + forward_columns_count 
                                + backward_columns_count 
                                + forward_diagonals_count 
                                + backward_diagonals_count;

    all_xmas_count.to_string()
}
