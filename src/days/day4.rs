extern crate regex;

pub fn run() -> String {
    let input = include_str!("../../inputs/day4.txt");
    solve(input)
}

fn solve(input: &str) -> String {

    let grid: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let word = "XMAS";
    // directions to search for xmas
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

    let mut count_part1 = 0;

    for row_index in 0..rows {
        for column_index in 0..columns {
            for &(horizontal_direction, vertical_direction) in &directions {
                if find_word(&grid, word, row_index as isize, column_index as isize, horizontal_direction, vertical_direction) {
                    count_part1 += 1;
                }
            }
        }
    }

    println!("part 1: {}", count_part1);

    let mut count_part2 = 0;

    for row_index in 0..rows {
        for column_index in 0..columns {
            if is_cross(&grid, row_index, column_index) {
                count_part2 += 1;
            }
        }
    }

    count_part2.to_string()

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

fn is_cross(grid: &Vec<String>, center_x: usize, center_y: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    if center_x > 0 && center_x + 1 < rows && center_y > 0 && center_y + 1 < cols {
        let top_left_to_bottom_right = (
            grid[center_x - 1].chars().nth(center_y - 1),
            grid[center_x].chars().nth(center_y),
            grid[center_x + 1].chars().nth(center_y + 1),
        );

        let top_right_to_bottom_left = (
            grid[center_x - 1].chars().nth(center_y + 1),
            grid[center_x].chars().nth(center_y),
            grid[center_x + 1].chars().nth(center_y - 1),
        );

        let is_mas_cross = matches_mas_or_sam(top_left_to_bottom_right)
            && matches_mas_or_sam(top_right_to_bottom_left);

        if is_mas_cross {
            return true;
        }
    }

    false
}

fn matches_mas_or_sam(diagonal: (Option<char>, Option<char>, Option<char>)) -> bool {
    matches!(
        diagonal,
        (Some('M'), Some('A'), Some('S')) | (Some('S'), Some('A'), Some('M'))
    )
}
