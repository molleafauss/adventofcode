// https://adventofcode.com/2021/day/2

use std::str::FromStr;
use log::info;
use adventofcode::grid::GridPos;
use adventofcode::Solver;

pub struct Solution {
    pos1: GridPos,
    pos2: GridPos,
    aim: i64,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            pos1: GridPos::of(0, 0),
            pos2: GridPos::of(0, 0),
            aim: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.starts_with("forward ") {
            let val = i64::from_str(&line[8..]).unwrap();
            self.pos1.col += val;
            self.pos2.col += val;
            self.pos2.row += self.aim * val;
        } else if line.starts_with("down ") {
            let val = i64::from_str(&line[5..]).unwrap();
            self.pos1.row += val;
            self.aim += val;
        } else if line.starts_with("up ") {
            let val = i64::from_str(&line[3..]).unwrap();
            self.pos1.row -= val;
            self.aim -= val;
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let part1 = self.pos1.col * self.pos1.row;
        info!("[1] final result: pos {}, depth {} => {}", self.pos1.col, self.pos1.row, part1);

        let part2 = self.pos2.col * self.pos2.row;
        info!("[2] final result: pos {}, depth {} => {}", self.pos2.col, self.pos2.row, part2);
        Some((part1.to_string(), part2.to_string()))
    }
}
