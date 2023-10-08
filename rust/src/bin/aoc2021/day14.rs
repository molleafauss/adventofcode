// https://adventofcode.com/2021/day/14
// TODO

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
        info!("Folding polymer {:?}", self.polymer);

        let mut cache = HashMap::new();
        let mut counts = HashMap::new();
        for i in 0..self.polymer.len() - 1 {
            let key = (self.polymer[i], self.polymer[i + 1]);
            let folding = cache.entry(key)
                .or_insert_with(|| Folding::create(key, &mut self.mapping, 10));
            folding.merge_counts(&mut counts);
            counts.entry(key.0).and_modify(|val| *val -= 1);
        }

        debug!("Counts after 10 folds: {:?}", counts);
        let mut counts: Vec<(char, usize)> = counts.into_iter().collect();
        counts.sort_by_key(|val| val.1);

        let first = counts.first().unwrap();
        let last = counts.last().unwrap();
        let part1 = last.1 - first.1;
        info!("[1] Polymer: first {:?}, last {:?} => {}", first, last, part1);

        // now for the 40 part, fold all to 20, then, per every resulting final polymer segment,
        // merge only the counts
        let mut cache = HashMap::new();
        let mut counts = HashMap::new();
        for i in 0..self.polymer.len() - 1 {
            let key = (self.polymer[i], self.polymer[i + 1]);
            let folding = cache.entry(key)
                .or_insert_with(|| Folding::create(key, &mut self.mapping, 20));

            // explore previous polymer (fold and cache)
            let polymer = folding.polymer.clone();
            for i in 0..polymer.len() - 1 {
                let key = (polymer[i], polymer[i + 1]);
                let folding = cache.entry(key)
                    .or_insert_with(|| Folding::create(key, &mut self.mapping, 20));
                folding.merge_counts(&mut counts);
                counts.entry(key.0).and_modify(|val| *val -= 1);
            }
            debug!("part 2 [{}/{}] {:?} => folded to 40", i, self.polymer.len(), key);
        }
        debug!("Counts after 40 folds: {:?}", counts);
        let mut counts: Vec<(char, usize)> = counts.into_iter().collect();
        counts.sort_by_key(|val| val.1);

        let first = counts.first().unwrap();
        let last = counts.last().unwrap();
        let part2 = last.1 - first.1;
        info!("[2] Polymer: first {:?}, last {:?} => {}", first, last, part2);

        Some((part1.to_string(), part2.to_string()))
    }
}

struct Folding {
    counts: HashMap<char, usize>,
    polymer: Vec<char>,
}

impl Folding {
    fn create(key: (char, char), mappings: &HashMap<(char, char), char>, folds: usize) -> Folding {
        let mut counts = HashMap::new();
        [key.0, key.1].iter().for_each(|v| {
            counts.entry(v.clone()).and_modify(|val| *val += 1).or_insert(1);
        });
        let polymer = vec![key.0, key.1];
        let mut folding = Folding {
            counts,
            polymer,
        };
        folding.fold(mappings, folds);
        debug!("{:?} Created and folded {} times", key, folds);
        folding
    }

    fn fold(&mut self, mappings: &HashMap<(char, char), char>, folds: usize) {
        for _ in 0..folds {
            let sequence = self.polymer.clone();
            let mut next = Vec::with_capacity(2 * sequence.len());
            for i in 0..sequence.len() - 1 {
                let pair = (sequence[i], sequence[i+1]);
                let generated = mappings.get(&pair).unwrap();
                next.push(pair.0);
                next.push(*generated);
            }
            next.push(sequence[sequence.len() - 1]);
            self.polymer = next;
        }
        self.counts.clear();
        self.polymer.iter().for_each(|ch| {
            self.counts.entry(*ch).and_modify(|val| *val += 1).or_insert(1);
        });
    }

    fn merge_counts(&self, counts: &mut HashMap<char, usize>) {
        self.counts.iter().for_each(|(ch, count)|
            { counts.entry(*ch).and_modify(|val| *val += count).or_insert(*count); }
        );
    }
}