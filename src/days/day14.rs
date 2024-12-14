#[derive(Debug)]
struct RobotConfig {
    position: (i32, i32),
    velocity: (i32, i32),
}

pub fn run() -> String {
    let input = include_str!("../../inputs/day14.txt");
    solve(input)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn has_gathering(grid: &Vec<Vec<char>>, pattern_width: usize, pattern_height: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    // iterate over grid where pattern height and width would fit
    for y in 0..=height - pattern_height {
        for x in 0..=width - pattern_width {
            let mut matches = true;
            // check if for the given pattern every entry is 1
            for dy in 0..pattern_height {
                for dx in 0..pattern_width {
                    if grid[y + dy][x + dx] != '1' {
                        matches = false;
                        break;
                    }
                }
                if !matches {
                    break;
                }
            }
            if matches {
                return true;
            }
        }
    }
    false
}

fn solve(input: &str) -> String {

    let mut robots: Vec<RobotConfig> = input.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let p_part = parts[0];
            let v_part = parts[1];

            let p_coords: Vec<i32> = p_part[2..].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            let v_coords: Vec<i32> = v_part[2..].split(',').map(|s| s.parse::<i32>().unwrap()).collect();

            RobotConfig {
                position: (p_coords[0], p_coords[1]),
                velocity: (v_coords[0], v_coords[1]),
            }
        })
        .collect();

    let width: i32 = 101;
    let height: i32 = 103;
    // increased iterations for part 2
    let iterations: i32 = 10000;

    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for t in 0..iterations {
        // clear the grid
        for row in &mut grid {
            for cell in row.iter_mut() {
                *cell = '.';
            }
        }

        // iterate over every robot
        for robot in robots.iter_mut() {
            // update the positions and (with modulo) make sure they teleport to other side of grid when walking out of edges
            robot.position.0 = (robot.position.0 + robot.velocity.0) % width;
            robot.position.1 = (robot.position.1 + robot.velocity.1) % height;

            // modulo can update to negative positions -> add width/height to stay within the grid
            if robot.position.0 < 0 {
                robot.position.0 += width;
            }
            if robot.position.1 < 0 {
                robot.position.1 += height;
            }
            // set grid position to 1 if a robot 
            grid[robot.position.1 as usize][robot.position.0 as usize] = '1';
        }

        // check if grid has a gathering of 1's (MAYBE THIS IS A CHRISTMAS TREE?!?!?!)
        if has_gathering(&grid, 5, 3) {
            // print current grid and iteration
            println!("iteration: {}", t+1);
            print_grid(&grid);
            break;
        }
        
    }

    // part 1 -> need to change iterations back to 100
/* 
    // get center x and y of grid
    let center_x = width / 2;
    let center_y = height / 2;

    let mut quadrant_counts = vec![0,0,0,0];

    // count robots in quadrants
    for robot in &robots {
        let (x, y) = robot.position;

        // skip if in center
        if x == center_x || y == center_y {
            continue;
        }
        // check which quadrant the robot is in and increment it
        if x < center_x && y < center_y {
            quadrant_counts[0] += 1;
        } else if x > center_x && y < center_y {
            quadrant_counts[1] += 1;
        } else if x < center_x && y > center_y {
            quadrant_counts[2] += 1;
        } else if x > center_x && y > center_y {
            quadrant_counts[3] += 1;
        }
    }

    let product_of_counts = quadrant_counts.iter().product::<i32>();

    product_of_counts.to_string().to_string() */
    "input".to_string()
}
