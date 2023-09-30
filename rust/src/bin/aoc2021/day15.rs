// https://adventofcode.com/2021/day/15

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::str::FromStr;
use log::{debug, info};
use adventofcode::grid::{ALL_ORTHOGONAL, GridPos, MOVE_R, MOVE_U};
use adventofcode::Solver;

pub struct Solution {
    width: usize,
    height: usize,
    map: Vec<u32>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            width: 0,
            height: 0,
            map: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(self.width, line.len());
        }
        for i in 0..line.len() {
            self.map.push(u32::from_str(&line[i..i+1]).unwrap());
        }
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Size of grid: {}x{}", self.width, self.height);
        let end = self.height * self.width - 1;
        let mut visited = vec![false; self.map.len()];
        let mut lowest_risk = u32::MAX;

        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, 0)));

        loop {
            let Reverse((risk, pos)) = queue.pop().unwrap();
            if visited[pos] {
                continue;
            }
            if pos == end {
                lowest_risk = risk;
                break;
            }
            visited[pos] = true;
            for dir in &ALL_ORTHOGONAL {
                if let Some(next) = GridPos::from_linear(pos, self.width).add(dir).to_linear(self.width, self.height) {
                    if visited[next] {
                        continue;
                    }
                    queue.push(Reverse((risk + self.map[next], next)));
                }
            }
        }

        info!("[1] Lowest risk found: {}", lowest_risk);

        Some((lowest_risk.to_string(), String::new()))
    }
}
