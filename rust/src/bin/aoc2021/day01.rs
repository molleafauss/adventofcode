use std::str::FromStr;
use log::info;
use adventofcode::Solver;

pub struct Solution {
    prev: Option<u32>,
    increases: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            prev: None,
            increases: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        let val = u32::from_str(line).unwrap();
        if let Some(pval) = self.prev {
            if val > pval {
                self.increases += 1;
            }
        }
        self.prev = Some(val);
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] increases found: {}", self.increases);
        Some((self.increases.to_string(), String::new()))
    }
}