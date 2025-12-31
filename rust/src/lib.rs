use std::collections::HashMap;

use once_cell::sync::Lazy;

pub mod grid;
pub mod utils;

/// A solver for a AOC puzzle
pub trait Solver {
    /// parse the given line
    fn parse(&mut self, line: &str);
    /// solve the puzzle
    fn solve(&mut self) -> Option<(String, String)>;
}

pub type Constructor = fn() -> Box<dyn Solver>;

use std::sync::Mutex;
pub static SOLVERS: Lazy<Mutex<HashMap<String, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_solver(year: &str, day: &str, constructor: Constructor) {
    SOLVERS.lock().unwrap().insert(format!("{year}/{day}"), constructor);
}

pub fn get_solver(year: &str, day: &str) -> Option<Box<dyn Solver>> {
        SOLVERS.lock().unwrap()
            .get(format!("{year}/{day}").as_str())
            .and_then(|constructor| Some(constructor()))
}
