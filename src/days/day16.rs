use std::collections::BinaryHeap;
use std::collections::HashMap;
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

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i32 {
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
            return cost;
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

    -1 // no path (shouldn't happen in puzzle)
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

    let result = dijkstra(&grid, start, end);

    result.to_string()
}
