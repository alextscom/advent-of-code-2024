use std::collections::HashSet;

pub fn run() -> String {
    let input = include_str!("../../inputs/day12.txt");
    solve(input)
}

fn traverse(grid: &Vec<Vec<char>>, start_position: (usize, usize), target_char: char, visited: &mut HashSet<(usize, usize)>, perimeter: &mut usize) {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (start_row, start_col) = start_position;

    if grid[start_row][start_col] != target_char {
        return;
    }


    // add position to visited
    visited.insert(start_position);

    // traverse directions
    for &(dr, dc) in &directions {
        let new_row = start_row as isize + dr;
        let new_col = start_col as isize + dc;

        // check if in grid
        if new_row >= 0 && new_row < grid.len() as isize && new_col >= 0 && new_col < grid[0].len() as isize {
            let new_pos = (new_row as usize, new_col as usize);
            if !visited.contains(&new_pos) && grid[new_row as usize][new_col as usize] == target_char {
                traverse(grid, new_pos, target_char, visited, perimeter);
            }
        }
    }
}

fn calc_area_size_and_perimeter(start_position: (usize, usize), target_char: char, grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut area_perimeter = 0usize;
    
    traverse(&grid, start_position, target_char, &mut visited, &mut area_perimeter);

    visited
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars()
            .collect::<Vec<char>>() 
        )
        .collect::<Vec<Vec<char>>>();

    let mut unique_chars = HashSet::new();
    for row in &grid {
        for &ch in row {
            unique_chars.insert(ch);
        }
    }

    
    let mut areas: HashSet<(usize, usize)> = HashSet::new();

    for unique_char in &unique_chars {
        println!("{}", unique_char);
    }

    let first_area: HashSet<(usize, usize)> = calc_area_size_and_perimeter((0,0), 'R', &grid);
    println!("first area: {:?}", first_area);

    println!("unique chars: {:?}", unique_chars);
    println!("grid: {:?}", grid);

    input.trim().to_string()
}
