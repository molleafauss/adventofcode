// https://adventofcode.com/2021/day/6

use std::str::FromStr;
use std::usize;
use log::info;
use adventofcode::Solver;

pub struct Solution {
    timers: [u64; 9]
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            timers: [0; 9],
        }
    }
}

const DAYS1: u32 = 80;
const DAYS2: u32 = 256;

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        line.split(",").for_each(|v| self.timers[usize::from_str(v).unwrap()] += 1);
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let mut part1 = 0;
        for day in 0..DAYS2 {
            if day == DAYS1 {
                part1 = self.timers.iter().sum();
            }
            // get new born
            let born = self.timers[0];
            // shift down all
            for i in 0..self.timers.len() - 1 {
                self.timers[i] = self.timers[i + 1];
            }
            // 9 and 7 because we calculate the day before
            self.timers[8] = born;
            self.timers[6] += born;
        }

        info!("[1] After {} days: {} lanternfish", DAYS1, part1);
        let part2: u64 = self.timers.iter().sum();
        info!("[2] After {} days: {} lanternfish", DAYS2, part2);
        Some((part1.to_string(), part2.to_string()))
    }
}
