use adventofcode::register_solver;

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

pub fn register_solvers() {
    let year = "2023";
    register_solver(year, "day01", || Box::new(day01::Solution::new()));
    register_solver(year, "day02", || Box::new(day02::Solution::new()));
    register_solver(year, "day03", || Box::new(day03::Solution::new()));
    register_solver(year, "day04", || Box::new(day04::Solution::new()));
    register_solver(year, "day05", || Box::new(day05::Solution::new()));
    register_solver(year, "day06", || Box::new(day06::Solution::new()));
    register_solver(year, "day07", || Box::new(day07::Solution::new()));
    register_solver(year, "day08", || Box::new(day08::Solution::new()));
    register_solver(year, "day09", || Box::new(day09::Solution::new()));
    register_solver(year, "day10", || Box::new(day10::Solution::new()));
    register_solver(year, "day11", || Box::new(day11::Solution::new()));
    register_solver(year, "day12", || Box::new(day12::Solution::new()));
    register_solver(year, "day13", || Box::new(day13::Solution::new()));
    register_solver(year, "day14", || Box::new(day14::Solution::new()));
    register_solver(year, "day15", || Box::new(day15::Solution::new()));
    register_solver(year, "day16", || Box::new(day16::Solution::new()));
}