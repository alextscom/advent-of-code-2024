use std::collections::{HashMap, HashSet};

pub fn run() -> String { 
    let input = include_str!("../../inputs/day12.txt"); 
    solve(input) 
}

fn solve(input: &str) -> String { 
    let grid: Vec<Vec<char>> = input.lines() 
        .map(|line| line.chars().collect::<Vec<char>>()) 
        .collect::<Vec<Vec<char>>>(); 

    let mut plot_map = HashMap::new(); 
    // map coordinates to character at those coordinates
    for (position_y, line) in grid.iter().enumerate() { 
        for (position_x, &plot) in line.iter().enumerate() { 
            plot_map.insert((position_x as i32, position_y as i32), plot); 
        }
    }

    let mut visited = HashSet::new();
    // sum for part1
    let mut sum_perimeter_times_area_size = 0; 
    // sum for part2
    let mut sum_sides_times_area_size = 0; 

    // iterate over each coordinate and its corresponding character
    for (&(position_x, position_y), &plot) in plot_map.iter() { 
        // skip already processed coordinates (coordinate was in already processed region)
        if !visited.contains(&(position_x, position_y)) {
            // store positions for current region
            let mut region = HashSet::new();
            // start stack-based traversal from current coordinate
            let mut stack = vec![(position_x, position_y)];

            // take coordinate from stack --> while loop adds all matching neighbors (same char) into region
            while let Some((x_position_from_stack, y_position_from_stack)) = stack.pop() { 
                if visited.contains(&(x_position_from_stack, y_position_from_stack)) { 
                    // skips coordinates already processed in this region
                    continue;
                }
                // mark coordinate as visited
                visited.insert((x_position_from_stack, y_position_from_stack));
                // include coordinate in the current region
                region.insert((x_position_from_stack, y_position_from_stack)); 

                let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; 
                for (direction_horizontal, direction_vertical) in directions.iter() { 
                    let neighor_horizontal_position = x_position_from_stack + direction_horizontal;
                    let neighor_vertical_position = y_position_from_stack + direction_vertical;
                    if !visited.contains(&(neighor_horizontal_position, neighor_vertical_position)) && plot_map.get(&(neighor_horizontal_position, neighor_vertical_position)) == Some(&plot) { 
                        // push matching neighbor into stack
                        stack.push((neighor_horizontal_position, neighor_vertical_position));
                    }
                }
            }

            // saves edges of the region with direction from which it was reached
            let mut perimeter = HashSet::new(); 
            // iterate through coordinates of region
            for &(region_x, region_y) in &region { 
                // check all four directions for neighbors
                for (direction_x, direction_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)].iter() { 
                    let region_neighbor_x = region_x + direction_x; 
                    let region_neighbor_y = region_y + direction_y; 
                    
                    // neighbor not part of region? -> boundary
                    if !region.contains(&(region_neighbor_x, region_neighbor_y)) { 
                        perimeter.insert((region_neighbor_x, region_neighbor_y, *direction_x, *direction_y)); 
                    }
                }
            }

            sum_perimeter_times_area_size += region.len() * perimeter.len();

            // count sides of perimeter
            let mut sides = 0;
            while !perimeter.is_empty() { 
                // each iteration -> trace new side
                sides += 1; 
                // picks a starting boundary segment.
                let &(perimeter_x_pos, perimeter_y_pos, direction_previous_x, direction_previous_y) = perimeter.iter().next().unwrap(); 
                // remove current position + direction came from
                perimeter.remove(&(perimeter_x_pos, perimeter_y_pos, direction_previous_x, direction_previous_y)); 
                // check both sides to trace the shape
                for (direction_x, direction_y) in [(direction_previous_y, direction_previous_x), (-direction_previous_y, -direction_previous_x)].iter() { 
                    // traversal x
                    let mut traversing_x = perimeter_x_pos + direction_x; 
                    // traversal y
                    let mut traversing_y = perimeter_y_pos + direction_y; 
                    // loop while tracing same direction -> loop breaks whenever direction changes -> new side
                    while perimeter.contains(&(traversing_x, traversing_y, direction_previous_x, direction_previous_y)) { 
                        // removes traced perimeter
                        perimeter.remove(&(traversing_x, traversing_y, direction_previous_x, direction_previous_y)); 
                        // move further in currently checking direction
                        traversing_x += direction_x; 
                        traversing_y += direction_y; 
                    }
                }
            }
            sum_sides_times_area_size += region.len() * sides;
        }
    }

    // print part 1
    println!("part 1: {}", sum_perimeter_times_area_size);

    sum_sides_times_area_size.to_string()
}
