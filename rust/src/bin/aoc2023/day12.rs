// https://adventofcode.com/2023/day/12

use std::str::FromStr;
use log::info;
use adventofcode::Solver;

pub struct Solution {
    part1: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            part1: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let (springs, counts) = line.split_once(" ").unwrap();

        self.part1 += find_arrangements(
            springs.chars().collect(),
            counts.split(",").map(|num| u32::from_str(num).unwrap()).collect()
        );
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Found {} total arrangements", self.part1);

        Some((self.part1.to_string(), "".to_string()))
    }
}

fn find_arrangements(springs: Vec<char>, counts: Vec<u32>) -> u32 {
    info!("Looking to find arrangements of {:?} for counts {:?}", springs, counts);

    0
}
