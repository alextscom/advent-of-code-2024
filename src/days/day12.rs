use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() -> String {
    let input = include_str!("../../inputs/day12.txt");
    solve(input)
}

fn flood_fill(grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>, start: (usize, usize)) -> (usize, usize) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;
    let char = grid[start.0][start.1];
    
    queue.push_back(start);
    visited.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        area += 1;
        let mut edge_count = 0;

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.len() && (ny as usize) < grid[0].len() {
                let neighbor = (nx as usize, ny as usize);
                if grid[neighbor.0][neighbor.1] == char {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                        visited.insert(neighbor);
                    }
                } else {
                    edge_count += 1;
                }
            } else {
                edge_count += 1;
            }
        }
        perimeter += edge_count;
    }

    (area, perimeter)
}

fn calc_area_size_and_perimeter(grid: Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut visited = HashSet::new();
    let mut results = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited.contains(&(i, j)) {
                let char = grid[i][j];
                let (area, perimeter) = flood_fill(&grid, &mut visited, (i, j));
                results.entry(char).or_insert(Vec::new()).push((area, perimeter));
            }
        }
    }

    results
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars()
            .collect::<Vec<char>>() 
        )
        .collect::<Vec<Vec<char>>>();

    let results = calc_area_size_and_perimeter(grid);

    let mut fence_cost = 0usize;
    for (char, regions) in results {
        for (area, perimeter) in regions {
            println!("char: {}, area: {}, perimeter: {}", char, area, perimeter);
            fence_cost += area * perimeter;
        }
    }


    fence_cost.to_string()
}
