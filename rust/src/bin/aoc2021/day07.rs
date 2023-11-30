// https://adventofcode.com/2021/day/7
// both parts can be calculated for each iteration, making it a simple "linear" problem.
// learned fold construct

use std::str::FromStr;
use log::info;
use adventofcode::Solver;

pub struct Solution {
    crabs: Vec<u32>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            crabs: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.crabs = line.split(",").map(|v| u32::from_str(v).unwrap()).collect();
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let min = self.crabs.iter().min().unwrap();
        let max = self.crabs.iter().max().unwrap();

        let mut min_fuel1 = u32::MAX;
        let mut min_pos1 = 0;
        let mut min_fuel2 = u32::MAX;
        let mut min_pos2 = 0;
        for pos in *min..max + 1 {
            let (fuel1, fuel2) = self.crabs.iter()
                .map(|v| {
                    // part 1 - straight diff
                    // part 2 - arithmetic progression
                    let delta = pos.abs_diff(*v);
                    (delta, delta * (delta + 1) / 2)
                })
                .fold((0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1));
            if fuel1 < min_fuel1 {
                min_fuel1 = fuel1;
                min_pos1 = pos;
            }
            if fuel2 < min_fuel2 {
                min_fuel2 = fuel2;
                min_pos2 = pos;
            }
        }
        info!("[1] Min fuel at position {min_pos1}: {min_fuel1}");
        info!("[2] Min fuel at position {min_pos2}: {min_fuel2}");
        Some((min_fuel1.to_string(), min_fuel2.to_string()))
    }
}
