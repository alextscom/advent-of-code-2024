use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

pub fn run() -> String {
    let input = include_str!("../../inputs/day16.txt");
    solve(input)
}

const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
];

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    cost: i32, // total cost to reach this state
    x: usize,
    y: usize,
    direction: usize, // facing direction: 0 -> east, 1 -> south, 2 -> west, 3 -> north
}

// ordering priority queue: reverse order because binaryheap is max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse ordering (smallest cost first)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> (i32, HashMap<(usize, usize, usize), i32>) {
    let rows = grid.len();
    let cols = grid[0].len();

    // priority queue to explore states based on cost
    let mut heap = BinaryHeap::new();

    // track the minimum cost to reach each (x, y, direction)
    let mut costs: HashMap<(usize, usize, usize), i32> = HashMap::new();

    // push starting point with direction east
    heap.push(State { cost: 0, x: start.0, y: start.1, direction: 0 });
    costs.insert((start.0, start.1, 0), 0);

    while let Some(State { cost, x, y, direction }) = heap.pop() {
        // check if we reached the end
        if (x, y) == end {
            // return costs for part 2
            return (cost, costs);
        }

        // 1. move forward in current direction
        let (dx, dy) = DIRECTIONS[direction];
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if grid[new_x][new_y] != '#' {
                // moving forward = 1 cost
                let new_cost = cost + 1;
                let state_key = (new_x, new_y, direction);

                if new_cost < *costs.get(&state_key).unwrap_or(&i32::MAX) {
                    costs.insert(state_key, new_cost);
                    heap.push(State { cost: new_cost, x: new_x, y: new_y, direction });
                }
            }
        }

        // 2. rotate clockwise
        let new_direction_cw = (direction + 1) % 4;
        // rotation = 1000 points
        let cw_cost = cost + 1000;
        let cw_key = (x, y, new_direction_cw);
        if cw_cost < *costs.get(&cw_key).unwrap_or(&i32::MAX) {
            costs.insert(cw_key, cw_cost);
            heap.push(State { cost: cw_cost, x, y, direction: new_direction_cw });
        }

        // 3. rotate counterclockwise
        let new_direction_ccw = (direction + 3) % 4;
        // rotation = 1000 points
        let ccw_cost = cost + 1000;
        let ccw_key = (x, y, new_direction_ccw);
        if ccw_cost < *costs.get(&ccw_key).unwrap_or(&i32::MAX) {
            costs.insert(ccw_key, ccw_cost);
            heap.push(State { cost: ccw_cost, x, y, direction: new_direction_ccw });
        }
    }

    (-1, costs) // no path (shouldn't happen in puzzle)
}

fn backtrack_shortest_paths(grid: &Vec<Vec<char>>, costs: &HashMap<(usize, usize, usize), i32>, end: (usize, usize)) -> HashSet<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let min_end_cost = (0..4)
        .filter_map(|direction| costs.get(&(end.0, end.1, direction)))
        .min()
        .cloned()
        .unwrap_or(i32::MAX);

    let mut on_shortest_path = HashSet::new();
    let mut shortest_path_queue = VecDeque::new();

    for direction_index in 0..4 {
        let endpoint_state = (end.0, end.1, direction_index);
        if let Some(&cost) = costs.get(&endpoint_state) {
            if cost == min_end_cost {
                on_shortest_path.insert(endpoint_state);
                shortest_path_queue.push_back(endpoint_state);
            }
        }
    }

    while let Some((current_x, current_y, current_direction)) = shortest_path_queue.pop_front() {
        let current_cost = costs[&(current_x, current_y, current_direction)];

        // backwards for forward moves
        let (delta_x, delta_y) = DIRECTIONS[current_direction];
        let previous_x = current_x as i32 - delta_x;
        let previous_y = current_y as i32 - delta_y;
        if previous_x >= 0 && previous_x < rows as i32 && previous_y >= 0 && previous_y < cols as i32 {
            let previous_x_index = previous_x as usize;
            let previous_y_index = previous_y as usize;
            if grid[previous_x_index][previous_y_index] != '#' {
                let prev_cost = current_cost - 1;
                if prev_cost >= 0 {
                    let prev_state = (previous_x_index, previous_y_index, current_direction);
                    if let Some(&cost) = costs.get(&prev_state) {
                        if cost == prev_cost && !on_shortest_path.contains(&prev_state) {
                            on_shortest_path.insert(prev_state);
                            shortest_path_queue.push_back(prev_state);
                        }
                    }
                }
            }
        }

        // backward for turns
        let turn_cost = current_cost - 1000;
        if turn_cost >= 0 {
            // check both possible directions
            for &previous_direction in &[(current_direction + 3) % 4, (current_direction + 1) % 4] {
                let prev_state = (current_x, current_y, previous_direction);
                if let Some(&cost) = costs.get(&prev_state) {
                    if cost == turn_cost && !on_shortest_path.contains(&prev_state) {
                        on_shortest_path.insert(prev_state);
                        shortest_path_queue.push_back(prev_state);
                    }
                }
            }
        }
    }

    on_shortest_path.into_iter().map(|(x, y, _)| (x, y)).collect()
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    // find reindeer and endpoint in maze
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (i, j);
            } else if cell == 'E' {
                end = (i, j);
            }
        }
    }

    let (result, costs) = dijkstra(&grid, start, end);
    // println!("costs: {:?}", costs);
    let shortest_path_tiles = backtrack_shortest_paths(&grid, &costs, end);

    format!("part1: {}, part2: {}", result, shortest_path_tiles.len())
}
