use std::collections::{HashSet, VecDeque};

pub fn run() -> String {
    let input = include_str!("../../inputs/day10.txt");
    solve(input)
}

fn reachable_nines(grid: Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = vec![vec![0; cols]; rows];

    // directions for up down left right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for row in 0..rows {
        for column in 0..cols {
            if grid[row][column] == 0 {
                let mut visited = HashSet::new();
                let mut queue = VecDeque::new();
                queue.push_back((row, column, 0)); // (row, column, current_value)

                while let Some((current_row, current_column, current_value)) = queue.pop_front() {
                    // skip if already visited
                    if !visited.insert((current_row, current_column)) {
                        continue;
                    }

                    // count reached 9
                    if grid[current_row][current_column] == 9 {
                        result[row][column] += 1;
                        continue;
                    }

                    // go to next possible position
                    for (direction_row, direction_column) in directions {
                        let new_row = current_row as isize + direction_row;
                        let new_column = current_column as isize + direction_column;

                        if new_row >= 0 && new_column >= 0
                            && (new_row as usize) < rows
                            && (new_column as usize) < cols
                            && grid[new_row as usize][new_column as usize] == current_value + 1
                        {
                            queue.push_back((new_row as usize, new_column as usize, current_value + 1));
                        }
                    }
                }
            }
        }
    }

    result
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<u8>> = input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8)) 
            .collect::<Vec<u8>>() 
        )
        .collect::<Vec<Vec<u8>>>();


    let result: Vec<Vec<usize>> = reachable_nines(grid);

    let mut reachable_9s: usize = 0;
    for (_i, row) in result.iter().enumerate() {
        let row_sum: usize = row.iter().sum();
        reachable_9s += row_sum;
        // debug rows
        // println!("row {}: {:?}, sum: {}", i, row, row_sum);
    }

    reachable_9s.to_string()
}
