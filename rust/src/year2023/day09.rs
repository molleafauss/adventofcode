// https://adventofcode.com/2023/day/9

use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    part1: i32,
    part2: i32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            part1: 0,
            part2: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let vals = line.split(" ")
            .map(|x| i32::from_str(x).unwrap())
            .collect();
        let (first, last) = extend_sequence(&vals);
        debug!("Found side numbers for {:?} => ({} .. {})", vals, first, last);
        self.part1 += last;
        self.part2 += first;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Sum of new last numbers: {}", self.part1);
        info!("[2] Sum of new first numbers: {}", self.part2);

        Some((self.part1.to_string(), self.part2.to_string()))
    }
}

fn extend_sequence(initial: &Vec<i32>) -> (i32, i32) {
    let mut deltas = Vec::new();
    let mut diffs = sequence_diffs(initial);
    while !diffs.iter().all(|val| *val == 0) {
        deltas.push(diffs);
        diffs = sequence_diffs(deltas.last().unwrap());
    }
    // readd
    let mut first = 0;
    let mut last = 0;
    while !deltas.is_empty() {
        let seq = deltas.pop().unwrap();
        first = seq.first().unwrap() - first;
        last  = seq.last().unwrap() + last;
    }

    (initial.first().unwrap() - first, initial.last().unwrap() + last)
}

fn sequence_diffs(data: &Vec<i32>) -> Vec<i32> {
    let mut diffs = Vec::new();
    for i in 0..data.len() - 1 {
        diffs.push(data[i + 1] - data[i]);
    }
    diffs
}
