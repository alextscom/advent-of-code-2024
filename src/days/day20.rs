use std::collections::VecDeque;

pub fn run() -> String {
    // Read input file
    let input = include_str!("../../inputs/day20.txt");
    let result = solve(input);
    result
}

fn solve(input: &str) -> String {
    let (map, width, best_distance) = build_distance_map(input);

    // part 1 calculations
    let mut part_1_result = 0;
    for y in 1..(map.len() / width - 1) {
        for x in 1..(width - 1) {
            let pos = y * width + x;
            if map[pos].is_wall {
                let surrounding = [
                    &map[pos - 1],
                    &map[pos + 1],
                    &map[pos - width],
                    &map[pos + width],
                ];

                let mut count = 0;
                for a in &surrounding {
                    for b in &surrounding {
                        if let (
                            Some(dist_from_start),
                            Some(dist_from_end)
                        ) = (a.distances[0], b.distances[1]) {
                            if dist_from_start + dist_from_end + 2 <= best_distance - SAVE_DISTANCE {
                                count += 1;
                            }
                        }
                    }
                }
                part_1_result += count;
            }
        }
    }
    println!("part 1 result: {}", part_1_result);

    // part 2 calculations
    let height = map.len() / width;
    let mut part_2_result = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let pos = y * width + x;
            if let Some(dist_from_start) = map[pos].distances[0] {
                let x_start = if x > CHEAT_DIST { x - CHEAT_DIST } else { 1 };
                let x_end = (x + CHEAT_DIST).min(width - 2);
                for x2 in x_start..=x_end {
                    let x_dist = x.abs_diff(x2);
                    let max_y_dist = CHEAT_DIST - x_dist;
                    let y_start = if y > max_y_dist { y - max_y_dist } else { 1 };
                    let y_end = (y + max_y_dist).min(height - 2);
                    for y2 in y_start..=y_end {
                        let pos2 = y2 * width + x2;
                        if let Some(dist_from_end) = map[pos2].distances[1] {
                            if dist_from_start + dist_from_end + x_dist as usize + y.abs_diff(y2) as usize <= best_distance - SAVE_DISTANCE {
                                part_2_result += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("part 2 result: {}", part_2_result);
    ".".to_string()
}

const CHEAT_DIST: usize = 20;

struct MapTile {
    distances: [Option<usize>; 2],
    is_wall: bool,
}

fn fill_distance_map(map: &mut Vec<MapTile>, width: usize, index: usize, start: usize, end: usize) {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (pos, dist) = queue.pop_front().unwrap();
        if !map[pos].is_wall && map[pos].distances[index].is_none() {
            map[pos].distances[index] = Some(dist);
            queue.push_back((pos + 1, dist + 1));
            queue.push_back((pos - 1, dist + 1));
            queue.push_back((pos + width, dist + 1));
            queue.push_back((pos - width, dist + 1));
        }

        if pos == end {
            break;
        }
    }
}

fn build_distance_map(input: &str) -> (Vec<MapTile>, usize, usize) {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let mut start = 0;
    let mut end = 0;
    let mut map: Vec<MapTile> = Vec::new();

    for (n, c) in input.chars().filter(|&c| c != '\n').enumerate() {
        match c {
            '.' => map.push(MapTile { distances: [None; 2], is_wall: false }),
            '#' => map.push(MapTile { distances: [None; 2], is_wall: true }),
            'S' => {
                start = n;
                map.push(MapTile { distances: [None; 2], is_wall: false });
            }
            'E' => {
                end = n;
                map.push(MapTile { distances: [None; 2], is_wall: false });
            }
            _ => panic!("wrong character: {}", c),
        }
    }

    fill_distance_map(&mut map, width, 0, start, end);
    fill_distance_map(&mut map, width, 1, end, start);

    let best_distance = map[end].distances[0].expect("no path found");

    (map, width, best_distance)
}

const SAVE_DISTANCE: usize = 100;
