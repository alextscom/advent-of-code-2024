use regex::Regex;

pub fn run() -> String {
    let input = include_str!("../../inputs/day13.txt");
    solve(input)
}
#[derive(Debug)]
struct ButtonData {
    a: Coordinates,
    b: Coordinates,
    prize: Prize,
}

#[derive(Debug)]
struct Coordinates {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Prize {
    x: i64,
    y: i64,
}
fn solve(input: &str) -> String {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut machine_descriptions = Vec::new();

    for cap in re.captures_iter(input) {
        let a_x: i64 = cap[1].parse().unwrap();
        let a_y: i64 = cap[2].parse().unwrap();
        let b_x: i64 = cap[3].parse().unwrap();
        let b_y: i64 = cap[4].parse().unwrap();
        let prize_x: i64 = cap[5].parse().unwrap();
        let prize_y: i64 = cap[6].parse().unwrap();

        machine_descriptions.push(ButtonData {
            a: Coordinates { x: a_x, y: a_y },
            b: Coordinates { x: b_x, y: b_y },
            prize: Prize {
                // remove 10000000000000 additions for part 1
                x: 10000000000000 + prize_x,
                y: 10000000000000 + prize_y,
            },
        });
    }

    let mut sum_cost_opt: i64 = 0;
    for description in machine_descriptions {
        if let Some((x, y)) = find_min_instructions(description.prize.x, description.prize.y, description.a.x, description.a.y, description.b.x, description.b.y) {
            sum_cost_opt += x * 3 + y;
        }
    }
    

   sum_cost_opt.to_string()
}

fn find_min_instructions(x_target: i64, y_target: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> Option<(i64, i64)> {
    // german math guy coming for the rescue again: https://www.youtube.com/watch?v=OKFLT8KPFHo
    let det = x1 * y2 - y1 * x2;
    if det == 0 {
        return None; // no unique solution
    }

    let x_num = x_target * y2 - y_target * x2;
    let y_num = x1 * y_target - y1 * x_target;

    if x_num % det != 0 || y_num % det != 0 {
        return None; // no whole number solution
    }

    let x = x_num / det;
    let y = y_num / det;
    Some((x, y))
}
