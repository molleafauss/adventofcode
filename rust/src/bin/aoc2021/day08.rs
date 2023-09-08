// https://adventofcode.com/2021/day/8

use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    uniques: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            uniques: 0
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        // simple part - just count the patterns for the indicated numbers.
        let (patterns, output) = line.split_once(" | ").unwrap();
        for digit in output.split(" ") {
            let dlen = digit.len();
            // if it's a 1, 4, 7, 8
            if dlen == 2 || dlen == 4 || dlen == 3 || dlen == 7 {
                self.uniques += 1;
            }
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Found {} unique patterns", self.uniques);

        Some((self.uniques.to_string(), String::new()))
    }
}
