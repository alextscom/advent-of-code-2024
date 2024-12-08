pub fn run() -> String {
    let input = include_str!("../../inputs/day8.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len() as usize;
    let cols = grid[0].len() as usize;

    let mut antinodes = vec![vec!['.'; grid[0].len()]; grid.len()];

    let mut antennas = vec![];

    // collect all antennas
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas.push((cell, i, j));
            }
        }
    }

    // check for antinodes
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (ant1, x1, y1) = antennas[i];
            let (ant2, x2, y2) = antennas[j];

            if ant1 == ant2 {
                let xa1 = x1 as isize - x2 as isize;
                let ya1 = y1 as isize - y2 as isize;
                
                let antinode1x = (x1 as isize + xa1 as isize) as usize;
                let antinode1y = (y1 as isize + ya1 as isize) as usize;

                let xa2 = x2 as isize - x1 as isize;
                let ya2 = y2 as isize - y1 as isize;

                let antinode2x = (x2 as isize + xa2 as isize) as usize;
                let antinode2y = (y2 as isize + ya2 as isize) as usize;
                
                // place antinode
                if antinode1x<rows && antinode1y < cols {
                    antinodes[antinode1x as usize][antinode1y as usize] = '#';
                }
                if antinode2x<rows && antinode2y < cols {
                    antinodes[antinode2x as usize][antinode2y as usize] = '#';
                }
            }
        }
    }

    let mut count_antinodes = 0;

    // count antinodes
    for row in antinodes {
        count_antinodes += row.iter().filter(|&&c| c == '#').count();
    }


    count_antinodes.to_string()
}
