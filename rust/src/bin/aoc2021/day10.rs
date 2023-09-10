// https://adventofcode.com/2021/day/1

use log::info;
use adventofcode::Solver;

pub struct Solution {
    corrupted_score: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            corrupted_score: 0
        }
    }

    fn corrupted_score(&self, line: &str) -> Option<u32> {
        let mut stack = Vec::new();
        let unmatching = line.chars().find(|ch| {
            if *ch == ')' || *ch == '}' || *ch == ']' || *ch == '>' {
                if stack.is_empty() {
                    return true;
                }
                let pair = stack.pop().unwrap();
                return match (pair, ch) {
                    ('(', ')') => false,
                    ('(', _) => true,
                    ('[', ']') => false,
                    ('[', _) => true,
                    ('{', '}') => false,
                    ('{', _) => true,
                    ('<', '>') => false,
                    ('<', _) => true,
                    _ => panic!("unexpected matching pair {pair}/{ch}"),
                }
            }
            stack.push(*ch);
            return false;
        });
        unmatching.map(|val| score(val))
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if let Some(score) = self.corrupted_score(line) {
            self.corrupted_score += score;
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Corrupted score: {}", self.corrupted_score);
        Some((self.corrupted_score.to_string(), String::new()))
    }
}

fn score(ch: char) -> u32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid matchign character? {ch}"),
    }
}