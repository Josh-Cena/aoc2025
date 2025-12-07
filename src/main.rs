mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let prob = &args[2];
    let input = if args.len() > 3 { &args[3] } else { "real" };
    let filename = format!("inputs/day{}/{}.txt", day, input);
    let contents = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();
    match (day.as_str(), prob.as_str()) {
        ("1", "1") => day1::solve1(contents),
        ("1", "2") => day1::solve2(contents),
        ("2", "1") => day2::solve1(contents),
        ("2", "2") => day2::solve2(contents),
        ("3", "1") => day3::solve1(contents),
        ("3", "2") => day3::solve2(contents),
        ("4", "1") => day4::solve1(contents),
        ("4", "2") => day4::solve2(contents),
        ("5", "1") => day5::solve1(contents),
        ("5", "2") => day5::solve2(contents),
        ("6", "1") => day6::solve1(contents),
        ("6", "2") => day6::solve2(contents),
        ("7", "1") => day7::solve1(contents),
        ("7", "2") => day7::solve2(contents),
        _ => println!("Day not implemented"),
    }
}
