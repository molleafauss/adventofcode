// https://adventofcode.com/2023/day/13

use std::cmp::min;

use log::{debug, info};

use adventofcode::Solver;

pub struct Solution {
    maps: Vec<Pattern>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            maps: vec![Pattern::new(1)],
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            self.maps.push(Pattern::new(self.maps.len() + 1));
        } else {
            self.maps.last_mut().unwrap().add_line(line.chars().collect());
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        // find last reflection
        let scores: Vec<(usize, usize)> = self.maps.iter()
            .map(|m| m.find_reflections())
            .collect();
        let part1: usize = scores.iter().map(|(score, _)| *score).sum();
        info!("[1] Reflection total: {}", part1);
        let part2: usize = scores.iter().map(|(_, score)| *score).sum();
        info!("[2] Reflection total: {}", part2);

        Some((part1.to_string(), "".to_string()))
    }
}

struct Pattern {
    id: usize,
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn new(id: usize) -> Pattern {
        Pattern {
            id,
            map: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_line(&mut self, text: Vec<char>) {
        if self.width == 0 {
            self.width = text.len();
        } else {
            assert_eq!(self.width, text.len());
        }
        self.height += 1;
        self.map.push(text);
    }

    fn find_reflections(&self) -> (usize, usize) {
        // check similarities - will count all similar points and discard whenever it's > 1
        let mut all_diffs: Vec<(&str, usize, usize, usize)> = (1..self.width).map(|mid| {
            // find the width to analyze
            let w = min(mid, self.width - mid);
            let diffs: usize = (0..w)
                .map(|i| self.vertical_differences(mid, i))
                .sum();
            ("vertical", mid, mid, diffs)
        })
        .filter(|(_, _, _, diffs)| *diffs <= 1)
        .collect();

        (1..self.height).map(|mid| {
            // find the height to analyze
            let w = min(mid, self.height - mid);
            let diffs: usize = (0..w)
                .map(|i| self.horizontal_differences(mid, i))
                .sum();
            ("horizontal", mid, mid * 100, diffs)
        })
        .filter(|(_, _, _, diffs)| *diffs <= 1)
        .for_each(|v| all_diffs.push(v));

        all_diffs.sort_by_key(|val| val.3);
        for (direction, pos, score, diffs) in &all_diffs {
            debug!("[pattern {}] Found {direction} diff at position {pos} => {diffs} [score {score}]", self.id);
        }
        assert_eq!(all_diffs.len(), 2);
        assert_eq!(all_diffs[0].3, 0);
        assert_eq!(all_diffs[1].3, 1);

        (all_diffs[0].2, all_diffs[1].2)
    }

    fn vertical_differences(&self, mid: usize, i: usize) -> usize {
        (0..self.height)
            .filter(|row| self.map[*row][mid - i - 1] != self.map[*row][mid + i])
            .count()
    }

    fn horizontal_differences(&self, mid: usize, i: usize) -> usize {
        (0..self.width)
            .filter(|col| self.map[mid - i - 1][*col] != self.map[mid + i][*col])
            .count()
    }
}