use std::collections::{HashSet, VecDeque};

pub fn run() -> String {
    let input = include_str!("../../inputs/day18.txt");
    solve(input)
}

fn simulate_falling_bytes_and_find_path(grid_size: usize, bytes: &[(usize, usize)], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut grid = vec![vec![false; grid_size]; grid_size]; // false means uncorrupted
    let mut corrupted_positions = HashSet::new();

    // println!("bytes: {:?}", bytes);

    for &(x, y) in bytes {
        corrupted_positions.insert((x, y));
        grid[y][x] = true; // mark position as corrupted
    }

    // println!("grid after corrupted positions added: {:?}", grid);

    // BFS for shortest path
    let mut queue = VecDeque::new();
    queue.push_back((start, 0)); // (current position, steps taken)
    let mut visited = HashSet::new();
    visited.insert(start);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(((current_x, current_y), steps)) = queue.pop_front() {
        if (current_x, current_y) == end {
            return Some(steps);
        }

        for &(direction_x, direction_y) in &directions {
            let new_x = current_x as isize + direction_x;
            let new_y = current_y as isize + direction_y;

            if new_x >= 0 && new_y >= 0 && (new_x as usize) < grid_size && (new_y as usize) < grid_size {
                let next_x = new_x as usize;
                let next_y = new_y as usize;

                if !grid[next_y][next_x] && !visited.contains(&(next_x, next_y)) {
                    visited.insert((next_x, next_y));
                    queue.push_back(((next_x, next_y), steps + 1));
                }
            }
        }
    }

    None
}

fn solve(input: &str) -> String {
    let grid_size = 71;
    let mut bytes : Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                bytes.push((x, y));
            }
        }
    }


    let start = (0, 0);
    let end = (70, 70);

    let x_bits = 1024;
    let first_x_elements: Vec<(usize, usize)> = bytes.iter().take(x_bits).cloned().collect();

    match simulate_falling_bytes_and_find_path(grid_size, &first_x_elements, start, end) {
        Some(steps) => steps.to_string(),
        None => "no path to the exit found!".to_string()
    }
}
