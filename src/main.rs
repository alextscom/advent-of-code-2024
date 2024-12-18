mod days;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];
    let save_output = &args[2]; // 0 for print, 1 for save output file

    let output = match day.as_str() {
        "0" => days::day0::run(),
        "1" => days::day1::run(),
        "2" => days::day2::run(),
        "3" => days::day3::run(),
        "4" => days::day4::run(),
        "5" => days::day5::run(),
        "6" => days::day6::run(),
        "7" => days::day7::run(),
        "8" => days::day8::run(),
        "9" => days::day9::run(),
        "10" => days::day10::run(),
        "11" => days::day11::run(),
        "12" => days::day12::run(),
        "13" => days::day13::run(),
        "14" => days::day14::run(),
        "15" => days::day15::run(),
        "16" => days::day16::run(),
        "17" => days::day17::run(),
        "18" => days::day18::run(),
        _ => {
            eprintln!("Day {} not implemented yet!", day);
            return;
        }
    };

    match save_output.as_str() {
        "0" => {
            println!("{}", output);
        }
        "1" => {
            let output_path = format!("outputs/day{}.txt", day);
            if let Err(e) = fs::write(&output_path, &output) {
                eprintln!("Failed to write to {}: {}", output_path, e);
            } else {
                println!("Output saved to {}", output_path);
            }
        }
        _ => {
            eprintln!("Invalid argument for saving output: {}", save_output);
            std::process::exit(1);
        }
    }
}
