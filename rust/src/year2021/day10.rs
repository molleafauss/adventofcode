// https://adventofcode.com/2021/day/10

use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    corrupted_score: u32,
    incomplete_scores: Vec<u64>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            corrupted_score: 0,
            incomplete_scores: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if let Some(score) = corrupted_score(line) {
            // line is corrupted
            self.corrupted_score += score;
        } else {
            // line is incomplete
            self.incomplete_scores.push(incomplete_score(line));
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Corrupted score: {}", self.corrupted_score);

        assert_eq!(self.incomplete_scores.len() % 2, 1);
        self.incomplete_scores.sort();
        let mid = self.incomplete_scores.len() / 2;
        let incomplete = self.incomplete_scores[mid];

        info!("[2] Incomplete (avg) score: {incomplete} (incomplete {} / mid point {})", self.incomplete_scores.len(), mid);
        Some((self.corrupted_score.to_string(), incomplete.to_string()))
    }
}

fn corrupted_score(line: &str) -> Option<u32> {
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

fn score(ch: char) -> u32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid matchign character? {ch}"),
    }
}

fn incomplete_score(line: &str) -> u64 {
    let mut stack = Vec::new();
    line.chars().for_each(|ch| {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => check_match(ch, stack.pop().unwrap()),
            _ => panic!("Invalid character: {ch}"),
        }
    });

    let mut score = 0;
    while !stack.is_empty() {
        let ch = stack.pop().unwrap();
        score *= 5;
        score += match ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Not a valid start character? {ch}"),
        };
    }
    debug!("Found score {score} for {line}");
    score
}

fn check_match(start: char, end: char) {
    if (start == '(' && end != ')')
        || (start == '[' && end != ']')
        || (start == '{' && end != '}')
        || (start == '<' && end != '>') {
        panic!("Not matching start/end {start} => {end}")
    }
}
