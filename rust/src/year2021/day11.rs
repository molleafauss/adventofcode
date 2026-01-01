// https://adventofcode.com/2021/day/11

use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use log::{debug, info};
use adventofcode::grid::{ALL_SURROUNDING, GridPos};
use adventofcode::Solver;

pub struct Solution {
    octopus: Vec<u8>,
    width: usize,
    height: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            octopus: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

const ENERGY_LEVEL: u8 = 10;

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(line.len(), self.width, "Invalid length line? {} <=> {}", line.len(), self.width);
        }
        for i in 0..line.len() {
            self.octopus.push(u8::from_str(&line[i..i+1]).unwrap())
        }
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let mut flashes = 0;
        let mut step = 0;
        let mut sync = None;
        while step < 100 || sync.is_none() {
            step += 1;
            // add 1 energy to everyone
            let mut to_flash: VecDeque<usize> = VecDeque::new();
            let mut flashed = HashSet::new();
            self.octopus.iter_mut().enumerate().for_each(|(pos, oct)| {
                *oct += 1;
                if *oct >= ENERGY_LEVEL {
                    to_flash.push_back(pos);
                }
            });

            while !to_flash.is_empty() {
                let pos = to_flash.pop_front().unwrap();
                if flashed.contains(&pos) {
                    continue;
                }
                flashed.insert(pos);
                // flash - add 1 energy to all surroundings
                for dir in &ALL_SURROUNDING {
                    if let Some(next) = GridPos::from_linear(pos, self.width).add(dir).to_linear(self.width, self.height) {
                        self.octopus[next] += 1;
                        if self.octopus[next] >= ENERGY_LEVEL {
                            to_flash.push_back(next);
                        }
                    }
                }
            }
            if step <= 100 {
                flashes += flashed.len();
                debug!("Step {} => {} total flashes", step + 1, flashes);
            }
            if flashed.len() == self.width * self.height {
                sync = Some(step)
            }
            // reset all flashed to 0
            flashed.drain().for_each(|pos| {self.octopus[pos] = 0});
        }
        info!("[1] Found {flashes} flashes at step 100");
        info!("[2] All sync at round {}", sync.unwrap());
        Some((flashes.to_string(), sync.unwrap().to_string()))
    }
}
