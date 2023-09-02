mod solutions;

use adventofcode::Solver;

fn solver_for(day: &str) -> Box<dyn Solver> {
    match day {
        "day01" => Box::new(solutions::day01::Solution::new()),
        "day02" => Box::new(solutions::day02::Solution::new()),
        "day03" => Box::new(solutions::day03::Solution::new()),
        "day04" => Box::new(solutions::day04::Solution::new()),
        "day05" => Box::new(solutions::day05::Solution::new()),
        "day06" => Box::new(solutions::day06::Solution::new()),
        "day07" => Box::new(solutions::day07::Solution::new()),
        "day08" => Box::new(solutions::day08::Solution::new()),
        "day09" => Box::new(solutions::day09::Solution::new()),
        "day10" => Box::new(solutions::day10::Solution::new()),
        "day11" => Box::new(solutions::day11::Solution::new()),
        "day12" => Box::new(solutions::day12::Solution::new()),
        "day13" => Box::new(solutions::day13::Solution::new()),
        "day14" => Box::new(solutions::day14::Solution::new()),
        "day15" => Box::new(solutions::day15::Solution::new()),
        "day16" => Box::new(solutions::day16::Solution::new()),
        "day17" => Box::new(solutions::day17::Solution::new()),
        "day18" => Box::new(solutions::day18::Solution::new()),
        "day19" => Box::new(solutions::day19::Solution::new()),
        "day20" => Box::new(solutions::day20::Solution::new()),
        "day21" => Box::new(solutions::day21::Solution::new()),
        "day22" => Box::new(solutions::day22::Solution::new()),
        "day23" => Box::new(solutions::day23::Solution::new()),
        "day24" => Box::new(solutions::day24::Solution::new()),
        "day25" => Box::new(solutions::day25::Solution::new()),
        _ => panic!("Unsupported puzzle {day}"),
    }
}

fn main() {
    adventofcode::advent_of_code("2022", solver_for);
}