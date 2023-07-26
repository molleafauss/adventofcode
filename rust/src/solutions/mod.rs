mod day01;

/// A solver for a AOC puzzle
pub(crate) trait Solver {
    /// parse the given line
    fn parse(&mut self, line: &str);
    /// solve the puzzle
    fn solve(&mut self);
}

pub(crate) fn solver_for(day: &str) -> impl Solver {
    match day {
        "day01" => day01::Solution::new(),
        _ => panic!("Unsupported puzzle {day}"),
    }
}
