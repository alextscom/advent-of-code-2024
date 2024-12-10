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

fn find_routes(grid: Vec<Vec<u8>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_routes = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // check if valid as anonymous function (defined with let so the anonymous function can read variables in sorrounding scope)
    let is_valid = |x: usize, y: usize| x < rows && y < cols;

    // get all starting points
    let mut start_points = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                start_points.push((i, j));
            }
        }
    }

    // breitensuche :D
    for start in start_points {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // (x, y, current_value)
        queue.push_back((start.0, start.1, 0));

        while let Some((x, y, current_value)) = queue.pop_front() {
            // add to route count if 9 is reached
            if grid[x][y] == 9 {
                total_routes += 1;
                continue;
            }

            // add to visited to not visit multiple times
            visited.insert((x, y, current_value));

            // check neighbors
            for dir in &directions {
                let new_x = (x as isize + dir.0) as usize;
                let new_y = (y as isize + dir.1) as usize;

                if is_valid(new_x, new_y) {
                    let next_value = grid[new_x][new_y];
                    if next_value == current_value + 1 && !visited.contains(&(new_x, new_y, next_value)) {
                        queue.push_back((new_x, new_y, next_value));
                    }
                }
            }
        }
    }

    total_routes
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<u8>> = input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8)) 
            .collect::<Vec<u8>>() 
        )
        .collect::<Vec<Vec<u8>>>();


    let result: Vec<Vec<usize>> = reachable_nines(grid.clone());

    let mut reachable_9s: usize = 0;
    for (_i, row) in result.iter().enumerate() {
        let row_sum: usize = row.iter().sum();
        reachable_9s += row_sum;
        // debug rows
        // println!("row {}: {:?}, sum: {}", i, row, row_sum);
    }

    let result_distinct_routes = find_routes(grid);
    
    println!("distinct routes to 9: {}", result_distinct_routes);

    reachable_9s.to_string()
}
