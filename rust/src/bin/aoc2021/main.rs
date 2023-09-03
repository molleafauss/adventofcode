mod day01;
mod day02;
mod day03;
mod day04;

use log::LevelFilter;
use adventofcode::Solver;

fn solver_for(day: &str) -> Box<dyn Solver> {
    match day {
        "day01" => Box::new(day01::Solution::new()),
        "day02" => Box::new(day02::Solution::new()),
        "day03" => Box::new(day03::Solution::new()),
        "day04" => Box::new(day04::Solution::new()),
        _ => panic!("Unsupported puzzle {day}"),
    }
}

fn main() {
    adventofcode::run("2021", solver_for, LevelFilter::Debug);
}