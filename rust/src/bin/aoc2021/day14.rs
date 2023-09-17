// https://adventofcode.com/2021/day/1

use std::collections::HashMap;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    polymer: Vec<char>,
    mapping: HashMap<(char, char), char>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            polymer: Vec::new(),
            mapping: HashMap::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if self.polymer.is_empty() {
            self.polymer = line.chars().collect();
        } else {
            let (from, to) = line.split_once(" -> ").unwrap();
            let mut from = from.chars();
            let seq = (from.next().unwrap(), from.next().unwrap());
            self.mapping.insert(seq, to.chars().next().unwrap());
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("transforimg polymer {:?}", self.polymer);

        for it in 0..10 {
            let mut i = 0;
            while i < self.polymer.len() - 1 {
                let pair = (self.polymer[i], self.polymer[i+1]);
                let generated = self.mapping.get(&pair).unwrap();
                self.polymer.insert(i, *generated);
                i += 2;
            }
        }
        debug!("Final polymer length: {}", self.polymer.len());

        None
    }
}
