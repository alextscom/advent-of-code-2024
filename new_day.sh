#!/bin/bash

day=$1
if [ -z "$day" ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

mkdir -p src/days
mkdir -p inputs
mkdir -p outputs

cat <<EOL > src/days/day${day}.rs
pub fn run() -> String {
    let input = include_str!("../../inputs/day${day}.txt");
    solve(input)
}

fn solve(input: &str) -> String {
    input.trim().to_string()
}
EOL

echo "pub mod day${day};" >> src/days/mod.rs
touch inputs/day${day}.txt
touch outputs/day${day}.txt

echo "Day ${day} skeleton created!"
echo "To run and print: cargo run -- ${day} 0"
echo "To run and save:  cargo run -- ${day} 1"
