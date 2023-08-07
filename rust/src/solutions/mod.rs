mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

/// A solver for a AOC puzzle
pub(crate) trait Solver {
    /// parse the given line
    fn parse(&mut self, line: &str);
    /// solve the puzzle
    fn solve(&mut self);
}

pub(crate) fn solver_for(day: &str) -> Box<dyn Solver> {
    match day {
        "day01" => Box::new(day01::Solution::new()),
        "day02" => Box::new(day02::Solution::new()),
        "day03" => Box::new(day03::Solution::new()),
        "day04" => Box::new(day04::Solution::new()),
        "day05" => Box::new(day05::Solution::new()),
        "day06" => Box::new(day06::Solution::new()),
        "day07" => Box::new(day07::Solution::new()),
        "day08" => Box::new(day08::Solution::new()),
        "day09" => Box::new(day09::Solution::new()),
        "day10" => Box::new(day10::Solution::new()),
        "day11" => Box::new(day11::Solution::new()),
        _ => panic!("Unsupported puzzle {day}"),
    }
}
