pub mod grid;

/// A solver for a AOC puzzle
pub trait Solver {
    /// parse the given line
    fn parse(&mut self, line: &str);
    /// solve the puzzle
    fn solve(&mut self) -> Option<(String, String)>;
}
