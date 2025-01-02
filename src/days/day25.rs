pub fn run() -> String {
    let input = include_str!("../../inputs/day25.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let input = input.split("\n\n");
    let mut locks: Vec<[usize; 5]> = vec![];
    let mut keys: Vec<[usize; 5]> = vec![];

    for schematic in input {
        let counts = schematic.lines().fold([0, 0, 0, 0, 0], |mut acc, line| {
            line.chars().enumerate().for_each(|(i, ch)| {
                if ch == '#' {
                    acc[i] += 1;
                }
            });

            acc
        });

        if schematic.starts_with("#") {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    }

    let result = locks.iter().fold(0, |acc, lock| {
        acc + keys
            .iter()
            .filter(|key| {
                lock.iter()
                    .zip(key.iter())
                    .all(|(pin1, pin2)| pin1 + pin2 <= 7)
            })
            .count()
    });
    result.to_string()
}
