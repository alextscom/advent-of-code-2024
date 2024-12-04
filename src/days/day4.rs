extern crate regex;

pub fn run() -> String {
    let input = include_str!("../../inputs/day4.txt");
    solve(input)
}

fn solve(input: &str) -> String {

    let grid: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let word = "XMAS";
    // directions to search for word
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1), 
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let rows = grid.len();
    let columns = grid[0].len();
    let mut count = 0;

    for row_index in 0..rows {
        for column_index in 0..columns {
            for &(horizontal_direction, vertical_direction) in &directions {
                if find_word(&grid, word, row_index as isize, column_index as isize, horizontal_direction, vertical_direction) {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

fn find_word(grid: &Vec<String>, word: &str, start_x: isize, start_y: isize, horizontal_direction: isize, vertical_direction: isize) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let chars: Vec<char> = word.chars().collect();

    for (k, &ch) in chars.iter().enumerate() {
        let x = start_x + k as isize * horizontal_direction;
        let y = start_y + k as isize * vertical_direction;

        if x < 0 || x >= rows || y < 0 || y >= cols || grid[x as usize].chars().nth(y as usize) != Some(ch) {
            return false;
        }
    }

    true
}
