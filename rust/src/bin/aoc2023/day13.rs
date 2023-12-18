// https://adventofcode.com/2023/day/13

use std::cmp::min;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    map: Vec<Vec<char>>,
    part1: u32,
    width: usize,
    height: usize,
    pattern: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            map: Vec::new(),
            part1: 0,
            width: 0,
            height: 0,
            pattern: 1,
        }
    }

    fn find_reflection(&self) -> u32 {
        // first try column reflection
        for mid in 1..self.width {
            // find the width to analyze
            let w = min(mid, self.width - mid);
            if (0..w).all(|i| self.is_vertical_reflection(mid, i)) {
                debug!("[pattern {}] Found vertical reflection at column: {}", self.pattern, mid);
                return mid as u32;
            }
        }

        // now check wor reflection
        for mid in 1..self.height {
            // find the width to analyze
            let w = min(mid, self.height - mid);
            if (0..w).all(|i| self.is_horizontal_reflection(mid, i)) {
                debug!("[patterns {}] Found horizontal reflection at row: {}", self.pattern, mid);
                return (100 * mid) as u32;
            }
        }

        0
    }

    fn is_vertical_reflection(&self, mid: usize, i: usize) -> bool {
        (0..self.height).all(|row| self.map[row][mid - i - 1] == self.map[row][mid + i])
    }

    fn is_horizontal_reflection(&self, mid: usize, i: usize) -> bool {
        (0..self.width).all(|col| self.map[mid - i - 1][col] == self.map[mid + i][col])
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            self.part1 += self.find_reflection();
            self.map.clear();
            self.width = 0;
            self.height = 0;
            self.pattern += 1;
        } else {
            self.width = line.len();
            self.map.push(line.chars().collect());
            self.height += 1;
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        // find last reflection
        self.part1 += self.find_reflection();
        info!("[1] Reflection total: {}", self.part1);

        Some((self.part1.to_string(), "".to_string()))
    }
}
