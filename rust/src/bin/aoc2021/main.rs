mod day01;
mod day02;

use log::LevelFilter;
use adventofcode::Solver;

fn solver_for(day: &str) -> Box<dyn Solver> {
    match day {
        "day01" => Box::new(day01::Solution::new()),
        "day02" => Box::new(day02::Solution::new()),
        _ => panic!("Unsupported puzzle {day}"),
    }
}

fn main() {
    adventofcode::advent_of_code("2021", solver_for, LevelFilter::Info);
}