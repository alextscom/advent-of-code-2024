pub fn run() -> String {
    let input = include_str!("../../inputs/day8.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut antinodes = vec![vec!['.'; grid[0].len()]; grid.len()];

    let mut antennas = vec![];

    // collect all antennas
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas.push((cell, i as isize, j as isize));
            }
        }
    }

    // check for antinodes
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (ant1, x1, y1) = antennas[i];
            let (ant2, x2, y2) = antennas[j];

            if ant1 == ant2 {
                // if there are 2 antennas, these are in line -> place an antinodef or them
                antinodes[x1 as usize][y1 as usize] = '#';
                antinodes[x2 as usize][y2 as usize] = '#';

                // i guess following lines to calc the 1's and 2's (xa1, ya1, xa2...) could be done with abs() distance but i'm tired of grid coordinate logic for now :D
                let xa1 = x1 - x2;
                let ya1 = y1 - y2;
                
                let mut antinode1x = x1 + xa1;
                let mut antinode1y = y1 + ya1;

                let xa2 = x2 - x1;
                let ya2 = y2 - y1;

                let mut antinode2x = x2 + xa2;
                let mut antinode2y = y2 + ya2;
                
                // place antinode1
                while antinode1x >= 0 && antinode1x < rows && antinode1y >= 0 && antinode1y < cols {
                    antinodes[antinode1x as usize][antinode1y as usize] = '#';
                    antinode1x += xa1;
                    antinode1y += ya1;
                }
                // place antinode2
                while antinode2x >= 0 && antinode2x < rows && antinode2y >= 0 && antinode2y < cols {
                    antinodes[antinode2x as usize][antinode2y as usize] = '#';
                    antinode2x += xa2;
                    antinode2y += ya2;
                }
            }
        }
    }

    let mut count_antinodes = 0;

    // count antinodes
    for row in antinodes {
        count_antinodes += row.iter().filter(|&&c| c == '#').count();
        // print the placed antinodes
        // println!("{}", row.iter().collect::<String>());
    }

    count_antinodes.to_string()
}
