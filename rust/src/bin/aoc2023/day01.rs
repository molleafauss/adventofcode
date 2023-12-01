// https://adventofcode.com/2023/day/1

use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    part1: u32,
    part2: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            part1: 0,
            part2: 0,
        }
    }
}

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let first_num_pos = line.find(char::is_numeric).unwrap();
        let last_num_pos = line.rfind(char::is_numeric).unwrap();
        let mut first_num_val = u32::from_str(&line[first_num_pos..first_num_pos + 1]).unwrap();
        let mut last_num_val = u32::from_str(&line[last_num_pos..last_num_pos + 1]).unwrap();

        self.part1 += first_num_val * 10 + last_num_val;

        let first_digit = DIGITS.iter().enumerate()
            .filter_map(|(idx, digit)| line.find(digit).map(|pos| (pos, idx + 1)))
            .min();
        if first_digit.is_some() && first_digit.unwrap().0 < first_num_pos {
            first_num_val = first_digit.unwrap().1 as u32;
        }
        let last_digit = DIGITS.iter().enumerate()
            .filter_map(|(idx, digit)| line.rfind(digit).map(|pos| (pos, idx + 1)))
            .max();
        if last_digit.is_some() && last_digit.unwrap().0 > last_num_pos {
            last_num_val = last_digit.unwrap().1 as u32;
        }

        self.part2 += first_num_val * 10 + last_num_val;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Overall calibration: {}", self.part1);
        info!("[2] Overall calibration: {}", self.part2);

        Some((self.part1.to_string(), self.part2.to_string()))
    }
}
