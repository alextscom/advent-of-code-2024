use std::collections::HashSet;

pub fn run() -> String {
    let input = include_str!("../../inputs/day6.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.trim().lines().map(|line| line.chars().collect()).collect();

    // find starting position
    let mut start_position = None;
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            start_position = Some((i as isize, j as isize));
            break;
        }
    }

    let start_position = start_position.expect("starting position not found");
    let mut position = start_position;
    // start walking upwards
    let mut direction = (-1, 0);

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    // add starting position
    visited_positions.insert(position);

    while position.0 >= 0 && position.0 < rows && position.1 >= 0 && position.1 < cols {
        let (x, y) = position;
        let next_position = (x + direction.0, y + direction.1);

        if next_position.0 < 0 || next_position.0 >= rows || next_position.1 < 0 || next_position.1 >= cols {
            break;
        }

        if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
            // hit '#' -> turn right
            direction = (direction.1, -direction.0);
        } else {
            // not hit '#' -> move further
            position = next_position;
            visited_positions.insert(position);
        }
    }

    visited_positions.len().to_string()
}
