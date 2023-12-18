// https://adventofcode.com/2023/day/13

use std::cmp::min;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    maps: Vec<Pattern>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            maps: vec![Pattern::new(1)],
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            self.maps.push(Pattern::new(self.maps.len() + 1));
        } else {
            self.maps.last_mut().unwrap().add_line(line.chars().collect());
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        // find last reflection
        let part1: u32 = self.maps.iter()
            .map(|m| m.find_reflection())
            .sum();
        info!("[1] Reflection total: {}", part1);

        Some((part1.to_string(), "".to_string()))
    }
}

struct Pattern {
    id: usize,
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn new(id: usize) -> Pattern {
        Pattern {
            id,
            map: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_line(&mut self, text: Vec<char>) {
        if self.width == 0 {
            self.width = text.len();
        } else {
            assert_eq!(self.width, text.len());
        }
        self.height += 1;
        self.map.push(text);
    }

    fn find_reflection(&self) -> u32 {
        // first try column reflection
        for mid in 1..self.width {
            // find the width to analyze
            let w = min(mid, self.width - mid);
            if (0..w).all(|i| self.is_vertical_reflection(mid, i)) {
                debug!("[pattern {}] Found vertical reflection at column: {}", self.id, mid);
                return mid as u32;
            }
        }

        // now check wor reflection
        for mid in 1..self.height {
            // find the width to analyze
            let w = min(mid, self.height - mid);
            if (0..w).all(|i| self.is_horizontal_reflection(mid, i)) {
                debug!("[patterns {}] Found horizontal reflection at row: {}", self.id, mid);
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