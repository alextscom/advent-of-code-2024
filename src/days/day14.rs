#[derive(Debug)]
struct RobotConfig {
    position: (i32, i32),
    velocity: (i32, i32),
}

pub fn run() -> String {
    let input = include_str!("../../inputs/day14.txt");
    solve(input)
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

    let width = 101;
    let height = 103;
    let iterations = 100;

    for _ in 0..iterations {
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
        }
    }

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

    product_of_counts.to_string().to_string()
}
