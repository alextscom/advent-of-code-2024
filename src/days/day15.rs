pub fn run() -> String {
    let input = include_str!("../../inputs/day15.txt");
    solve(input)
}

fn can_be_moved_vertically(grid: &[char], width: usize, box_pos: (usize, usize), current_y: usize, delta_y: isize) -> bool {
    let new_y = (current_y as isize + delta_y) as usize;
    let grid_at = |x| grid[new_y * width + x]; // Helper to access grid positions

    if grid_at(box_pos.0) == '#' || grid_at(box_pos.1) == '#' {
        return false;
    }

    // function to check if given box char is blocked
    let is_large_box_blocked = |pos: usize, next_box_pos: (usize, usize), box_char: char| -> bool {
        grid_at(pos) == box_char && !can_be_moved_vertically(grid, width, next_box_pos, new_y, delta_y)
    };

    if is_large_box_blocked(box_pos.0, (box_pos.0, box_pos.0 + 1), '[') ||  // large box blocked
       is_large_box_blocked(box_pos.0, (box_pos.0 - 1, box_pos.0), ']') ||  // left side of box blocked
       is_large_box_blocked(box_pos.1, (box_pos.1, box_pos.1 + 1), '[') {   // right side of box blocked
        return false;
    }

    true
}

fn move_vertically(grid: &mut [char], width: usize, box_position: (usize, usize), current_y: usize, delta_y: isize) {
    let new_y = (current_y as isize + delta_y) as usize;

    // check and move large boxes if they are in the way
    match grid[new_y * width + box_position.0] {
        '[' => move_vertically(grid, width, (box_position.0, box_position.0 + 1), new_y, delta_y),
        ']' => move_vertically(grid, width, (box_position.0 - 1, box_position.0), new_y, delta_y),
        _ => {}
    }

    if grid[new_y * width + box_position.1] == '[' {
        move_vertically(grid, width, (box_position.1, box_position.1 + 1), new_y, delta_y);
    }

    // move current box
    grid[new_y * width + box_position.0] = grid[current_y * width + box_position.0];
    grid[new_y * width + box_position.1] = grid[current_y * width + box_position.1];

    // clear previous position
    grid[current_y * width + box_position.0] = '.';
    grid[current_y * width + box_position.1] = '.';
}

fn move_horizontally(grid: &mut [char], width: usize, position: (usize, usize), delta_x: isize) {
    let start_x = (position.0 as isize + delta_x) as usize;
    let mut current_x = start_x;

    // move horizontally while encountering large boxes
    while grid[position.1 * width + current_x] == '[' || grid[position.1 * width + current_x] == ']' {
        current_x = (current_x as isize + delta_x) as usize;
    }

    // swap if valid move is found
    if current_x != start_x && grid[position.1 * width + current_x] == '.' {
        while current_x != start_x {
            let move_x = (current_x as isize + (-(delta_x - 1) / 2)) as usize;
            let left_index = position.1 * width + move_x - 1;
            let right_index = position.1 * width + move_x;
            
            
            let temp = grid[left_index];
            grid[left_index] = grid[right_index];
            grid[right_index] = temp;

            current_x = (current_x as isize - delta_x) as usize;
        }
    }
}

fn run_instructions(
    mut robot_pos: (usize, usize),
    directions: Vec<char>,
    mut grid: Vec<char>,
    width: usize,
    height: usize,
) -> usize {
    for direction in directions {
        match direction {
            '^' | 'v' => {
                let (new_y, dy) = if direction == '^' {
                    (robot_pos.1 - 1, -1)
                } else {
                    (robot_pos.1 + 1, 1)
                };
                let present = match grid[new_y * width + robot_pos.0] {
                    '[' => Some((robot_pos.0, robot_pos.0 + 1)),
                    ']' => Some((robot_pos.0 - 1, robot_pos.0)),
                    _ => None,
                };
                if let Some(present) = present {
                    if can_be_moved_vertically(&grid, width, present, new_y, dy) {
                        move_vertically(&mut grid, width, present, new_y, dy);
                    }
                }
                if grid[new_y * width + robot_pos.0] == '.' {
                    robot_pos.1 = new_y;
                }
            }

            '>' => {
                move_horizontally(&mut grid, width, robot_pos, 1);
                if grid[robot_pos.1 * width + robot_pos.0 + 1] == '.' {
                    robot_pos.0 += 1;
                }
            }

            '<' => {
                move_horizontally(&mut grid, width, robot_pos, -1);
                if grid[robot_pos.1 * width + robot_pos.0 - 1] == '.' {
                    robot_pos.0 -= 1;
                }
            }

            _ => panic!("incorrect direction: {}", direction),
        }
    }

    let mut total = 0;
    for y_coord in 0..height {
        for x_coord in 0..width {
            let char = grid[y_coord * width + x_coord];
            // most left side of box is used for calculating
            if char == '[' {
                total += 100 * y_coord + x_coord;
            }
        }
    }

    total
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

    // keep instructions as char, makes match case easier
    /* // convert instructions
    let directions: Vec<(i32, i32)> = instructions
        .chars()
        .map(|c| match c {
            '<' => (-1, 0), // Left
            '^' => (0, -1), // Up
            '>' => (1, 0),  // Right
            'v' => (0, 1),  // Down
            _ => (0, 0),    // Default case
        })
        .collect(); */

    // use height and width instead of cloning/reading grid all the time
    let height = grid.len();
    let width = grid[0].len() * 2;
    let mut wider_grid = vec![vec!['.'; width]; height];

    // scale up warehouse size
    for (y, row) in grid.clone().into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            match c {
                '#' | '.' => {
                    wider_grid[y][x * 2] = c;
                    wider_grid[y][x * 2 + 1] = c;
                }
                'O' => {
                    wider_grid[y][x * 2] = '[';
                    wider_grid[y][x * 2 + 1] = ']';
                }
                '@' => {
                    wider_grid[y][x * 2] = '@';
                    wider_grid[y][x * 2 + 1] = '.';
                }
                _ => panic!("not a valid grid char: {}", c),
            }
        }
    }

    // find robot
    let mut robot_pos = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if wider_grid[y][x] == '@' {
                robot_pos = (x, y);
            }
        }
    }


    println!("pos robot in scaled up grid: {:?}", robot_pos);

    // overwrite robot position initially
    wider_grid[robot_pos.1][robot_pos.0] = '.';

    let directions: Vec<char> = instructions.chars().collect();
    let total = run_instructions(robot_pos, directions, wider_grid.concat(), width, height);

    total.to_string()
}
