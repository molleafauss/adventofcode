// https://adventofcode.com/2021/day/14

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

        let mut counts = HashMap::new();
        self.polymer.iter()
            .for_each(|ch| { counts.entry(ch.clone()).and_modify(|val| *val += 1).or_insert(1_usize); });
        debug!("Initial counts: {:?}", counts);
        for it in 0..10 {
            let mut i = 0;
            while i < self.polymer.len() - 1 {
                let pair = (self.polymer[i], self.polymer[i+1]);
                let generated = self.mapping.get(&pair).unwrap();
                counts.entry(*generated).and_modify(|val| *val += 1).or_insert(1);
                self.polymer.insert(i + 1, *generated);
                i += 2;
            }
            debug!("Iteration {} - counts: {:?}", it, counts);
        }
        debug!("Final polymer length: {} - counts: {:?}", self.polymer.len(), counts);
        let mut counts: Vec<(char, i32)> = counts.into_iter().collect();
        counts.sort_by_key(|val| val.1);

        let first = counts.first().unwrap();
        let last = counts.last().unwrap();
        let part1 = last.1 - first.1;
        info!("[1] Polymer: first {:?}, last {:?} => {}", first, last, part1);

        Some((part1.to_string(), String::new()))
    }
}
