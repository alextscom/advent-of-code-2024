pub fn run() -> String {
    let input = include_str!("../../inputs/day15.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut instructions = String::new();

    // parse grid
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect());
    }

    // parse instructions
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        instructions.push_str(&line);
    }

    println!("{:?}", instructions);

    // convert instructions
    let directions: Vec<(i32, i32)> = instructions
        .chars()
        .map(|c| match c {
            '<' => (-1, 0), // Left
            '^' => (0, -1), // Up
            '>' => (1, 0),  // Right
            'v' => (0, 1),  // Down
            _ => (0, 0),    // Default case
        })
        .collect();

    // find robot in grid
    let mut robot_pos = (0, 0);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '@' {
                robot_pos = (x as i32, y as i32);
                break 'outer;
            }
        }
    }

    for dir in directions {
        let new_x = robot_pos.0 + dir.0;
        let new_y = robot_pos.1 + dir.1;

        if can_move(&grid, robot_pos, (new_x, new_y)) {
            // Attempt to move the robot
            if grid[new_y as usize][new_x as usize] == 'O' {
                // Try to push presents
                if push_presents(&mut grid, (new_x, new_y), dir) {
                    move_robot(&mut grid, &mut robot_pos, (new_x, new_y));
                }
            } else if grid[new_y as usize][new_x as usize] == '.' {
                // Move to an empty space
                move_robot(&mut grid, &mut robot_pos, (new_x, new_y));
            }
        }
    }

    // final grid
    for row in grid.clone() {
        println!("{}", row.iter().collect::<String>());
    }

    let mut total_value = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                total_value += 100 * y + x;
            }
        }
    }

    total_value.to_string()
}

// check if robot can move to new position
fn can_move(grid: &Vec<Vec<char>>, current: (i32, i32), new_pos: (i32, i32)) -> bool {
    let (x, y) = new_pos;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    if x < 0 || y < 0 || x >= cols || y >= rows {
        return false; // out of bounds
    }

    let cell = grid[y as usize][x as usize];
    match cell {
        '#' => false, // wall -> can't move
        _ => true,    // empty space
    }
}

fn push_presents(grid: &mut Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let mut presents: Vec<(i32, i32)> = vec![];
    let mut current_pos = pos;

    // traverse to collect all presents in the direction
    while grid[current_pos.1 as usize][current_pos.0 as usize] == 'O' {
        presents.push(current_pos);
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);

        // check if out of bounds
        if current_pos.0 < 0
            || current_pos.1 < 0
            || current_pos.0 >= grid[0].len() as i32
            || current_pos.1 >= grid.len() as i32
        {
            return false; // Can't push out of bounds
        }
    }

    // check if next position empty
    if grid[current_pos.1 as usize][current_pos.0 as usize] != '.' {
        return false; // Can't push if no empty space behind presents
    }

    // move presents
    for &p in presents.iter().rev() {
        grid[p.1 as usize][p.0 as usize] = '.';
        grid[(p.1 + dir.1) as usize][(p.0 + dir.0) as usize] = 'O';
    }

    true
}

// Helper function to move the robot
fn move_robot(grid: &mut Vec<Vec<char>>, robot_pos: &mut (i32, i32), new_pos: (i32, i32)) {
    grid[robot_pos.1 as usize][robot_pos.0 as usize] = '.';
    grid[new_pos.1 as usize][new_pos.0 as usize] = '@';
    *robot_pos = new_pos;
}
