use std::collections::HashSet;

pub fn run() -> String {
    let input = include_str!("../../inputs/day6.txt");
    solve(input)
}


fn solve(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.trim().lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    // find starting position
    let mut start_position = None;
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            start_position = Some((i as isize, j as isize));
            break;
        }
    }
    let start_position = start_position.expect("starting position not found");

    let mut valid_positions = 0;

    for i in 0..rows {
        for j in 0..cols {
            // only place '#' on '.' positions
            if grid[i as usize][j as usize] != '.' {
                continue;
            }

            // place new obstacle
            let mut modified_grid = grid.clone();
            modified_grid[i as usize][j as usize] = '#';

            let mut position = start_position;
            let mut direction = (-1, 0);
            // state = position and direction
            let mut visited_states: HashSet<((isize, isize), (isize, isize))> = HashSet::new();

            let mut is_loop = false;
            while position.0 >= 0 && position.0 < rows && position.1 >= 0 && position.1 < cols {
                let (x, y) = position;
                let next_position = (x + direction.0, y + direction.1);

                if next_position.0 < 0 || next_position.0 >= rows || next_position.1 < 0 || next_position.1 >= cols {
                    break;
                }

                if modified_grid[next_position.0 as usize][next_position.1 as usize] == '#' {
                    // hit '#' -> turn right
                    direction = (direction.1, -direction.0);
                } else {
                    // move further
                    position = next_position;
                }

                // check for a loop: revisit the same position and direction
                if !visited_states.insert((position, direction)) {
                    is_loop = true;
                    break;
                }
            }

            if is_loop {
                valid_positions += 1;
            }
        }
    }

    valid_positions.to_string()
}
