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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

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
        "day12" => Box::new(day12::Solution::new()),
        "day13" => Box::new(day13::Solution::new()),
        "day14" => Box::new(day14::Solution::new()),
        "day15" => Box::new(day15::Solution::new()),
        "day16" => Box::new(day16::Solution::new()),
        "day17" => Box::new(day17::Solution::new()),
        "day18" => Box::new(day18::Solution::new()),
        "day19" => Box::new(day19::Solution::new()),
        "day20" => Box::new(day20::Solution::new()),
        "day21" => Box::new(day21::Solution::new()),
        "day22" => Box::new(day21::Solution::new()),
        _ => panic!("Unsupported puzzle {day}"),
    }
}
