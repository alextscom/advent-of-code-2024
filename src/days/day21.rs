// shameless disclaimer: used a lot of AI questions for this since i couldn't really
// come up with a good solution for part 2
// at least i learned about memoize :D

use memoize::memoize;

fn num_keys_translation(key: char) -> (i32, i32) {
    match key {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!(),
    }
}
 
fn arrow_keys_translation(key: char) -> (i32, i32) {
    match key {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!(),
    }
}

#[memoize]
fn calculate_arrow_steps(delta_x: i32, delta_y: i32, steps: usize, horizontal_first: bool) -> usize {
    let absolute_delta_x = delta_x.unsigned_abs() as usize;
    let absolute_delta_y = delta_y.unsigned_abs() as usize;
    let mut chunk = Vec::new();
    if delta_x > 0 {
        for _ in 0..absolute_delta_x {
            chunk.push('^');
        }
    } else {
        for _ in 0..absolute_delta_x {
            chunk.push('v');
        }
    }
    if delta_y > 0 {
        for _ in 0..absolute_delta_y {
            chunk.push('<');
        }
    } else {
        for _ in 0..absolute_delta_y {
            chunk.push('>');
        }
    }

    if horizontal_first {
        chunk.reverse();
    }

    chunk.push('A');

    if steps == 0 {
        return chunk.len();
    } else {
        let mut arrow_key_coordinates = arrow_keys_translation('A');
        let mut total_steps = 0;

        for arrow_key in chunk {
            let translated_arrow_key = arrow_keys_translation(arrow_key);
            let p = arrow_key_coordinates;
            arrow_key_coordinates = translated_arrow_key;
            let delta_position = (p.0 - translated_arrow_key.0, p.1 - translated_arrow_key.1);
            if delta_position.0 == 0 || delta_position.1 == 0 {
                total_steps += calculate_arrow_steps(delta_position.0, delta_position.1, steps - 1, false);
            } else if translated_arrow_key == (1, 0) && p.0 == 0 {
                total_steps += calculate_arrow_steps(delta_position.0, delta_position.1, steps - 1, false);
            } else if p == (1, 0) && translated_arrow_key.0 == 0 {
                total_steps += calculate_arrow_steps(delta_position.0, delta_position.1, steps - 1, true);
            } else {
                total_steps += std::cmp::min(
                    calculate_arrow_steps(delta_position.0, delta_position.1, steps - 1, false),
                    calculate_arrow_steps(delta_position.0, delta_position.1, steps - 1, true),
                );
            }
        }

        return total_steps;
    }
}

fn calculate_key_steps(sequence: &str, steps: usize) -> usize {
    let mut key_translation = num_keys_translation('A');
    let mut total_steps = 0;

    for current_char in sequence.chars() {
        let num_key_current_char = num_keys_translation(current_char);
        let translated_key = key_translation;
        let translationd_ifference = (key_translation.0 - num_key_current_char.0, key_translation.1 - num_key_current_char.1);
        key_translation = num_key_current_char;
        if translated_key.0 == 3 && num_key_current_char.1 == 0 {
            total_steps += calculate_arrow_steps(translationd_ifference.0, translationd_ifference.1, steps, false);
        } else if translated_key.1 == 0 && num_key_current_char.0 == 3 {
            total_steps += calculate_arrow_steps(translationd_ifference.0, translationd_ifference.1, steps, true);
        } else {
            total_steps += std::cmp::min(
                calculate_arrow_steps(translationd_ifference.0, translationd_ifference.1, steps, true),
                calculate_arrow_steps(translationd_ifference.0, translationd_ifference.1, steps, false),
            );
        }
    }

    total_steps * sequence[0..3].parse::<usize>().unwrap()
}

pub fn run() -> String {
    let input = include_str!("../../inputs/day21.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    memoized_flush_calculate_arrow_steps();
    let part1 = Some(input.lines().map(|s| calculate_key_steps(s, 2)).sum::<usize>()).unwrap();
    println!("part1: {}", part1);
    let part2 = Some(input.lines().map(|s| calculate_key_steps(s, 25)).sum::<usize>()).unwrap();
    part2.to_string()
}
