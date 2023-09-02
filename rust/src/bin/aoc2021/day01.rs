use std::collections::VecDeque;
use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    prev: Option<u32>,
    window: VecDeque<u32>,
    increases1: u32,
    increases2: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            prev: None,
            increases1: 0,
            increases2: 0,
            window: VecDeque::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        let val = u32::from_str(line).unwrap();
        if let Some(pval) = self.prev {
            if val > pval {
                self.increases1 += 1;
            }
        }
        self.prev = Some(val);

        // part 2 solution
        if self.window.len() == 3 {
            let head = self.window.pop_front().unwrap();
            let w1 = self.window.iter().sum::<u32>() + head;
            let w2 = w1 - head + val;
            debug!("window: {} - head {head} - w1 {w1} - w2 {w2} - val {val}", self.window.len() + 1);
            if w2 > w1 {
                self.increases2 += 1;
            }
        }
        self.window.push_back(val);
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] increases found: {}", self.increases1);
        info!("[2] increases found: {}", self.increases2);
        Some((self.increases1.to_string(), self.increases1.to_string()))
    }
}