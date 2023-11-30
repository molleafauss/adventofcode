// https://adventofcode.com/2021/day/8
// tricky one - mostly in how to codify solving the puzzle.

use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    uniques: u32,
    total: u32,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            uniques: 0,
            total: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        // simple part - just count the patterns for the indicated numbers.
        let (patterns, output) = line.split_once(" | ").unwrap();
        let digits = find_digits(patterns);
        let mut total = 0;
        for text in output.split(" ") {
            let val = decode_digit(&digits, text);
            // if it's a 1, 4, 7, 8
            if val == 1 || val == 4 || val == 7 || val == 8 {
                self.uniques += 1;
            }
            total *= 10;
            total += val;
        }
        debug!("Value of {}: {}", output, total);
        self.total += total;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Found {} unique patterns", self.uniques);
        info!("[2] Total outputs: {}", self.total);

        Some((self.uniques.to_string(), self.total.to_string()))
    }
}

fn sort_letters(pat: &str) -> String {
    let mut chars: Vec<char> = pat.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

fn contains(haystack: &str, needle: &str) -> bool {
    needle.chars().all(|ch| haystack.contains(ch))
}

fn find_digits(input: &str) -> Vec<String> {
    // very hand made solve procedure
    debug!("Finding digits in: {}", input);
    let mut result = vec![String::new(); 10];
    let mut patterns: Vec<String> = input.split(" ")
        .map(|pat| sort_letters(pat))
        .collect();
    patterns.sort_by_key(|val| val.len());
    // expect 1st being 1 - len 2
    let digit = patterns.remove(0);
    assert_eq!(digit.len(), 2);
    result[1] = digit;
    debug!("Found 1: {}", result[1]);

    // expect 2nd being 7 - len 3
    let digit = patterns.remove(0);
    assert_eq!(digit.len(), 3);
    result[7] = digit;
    debug!("Found 7: {}", result[7]);

    // expect 3rd being 4 - len 4
    let digit = patterns.remove(0);
    assert_eq!(digit.len(), 4);
    result[4] = digit;
    debug!("Found 4: {}", result[4]);

    // expect last being 8 - len 7
    let digit = patterns.pop().unwrap();
    assert_eq!(digit.len(), 7);
    result[8] = digit;
    debug!("Found 8: {}", result[8]);

    // here we should have only length 5 and 6
    assert_eq!(patterns.len(), 6);
    // find 9 - should contain 4 and 1
    let digit = 3 + patterns[3..].iter()
        .position(|pat| pat.len() == 6 && contains(pat, &result[1]) && contains(pat, &result[4]))
        .unwrap();
    result[9] = patterns.remove(digit);
    debug!("Found 9: {}", result[9]);

    // find 0 - should contain 1 but not 4
    let digit = 3 + patterns[3..].iter()
        .position(|pat| pat.len() == 6 && contains(pat, &result[1]) && !contains(pat, &result[4]))
        .unwrap();
    result[0] = patterns.remove(digit);
    debug!("Found 0: {}", result[0]);

    // remainder 6 len is 6
    result[6] = patterns.remove(3);
    assert_eq!(result[6].len(), 6);
    debug!("Found 6: {}", result[6]);

    // find 3 - should contain 1
    let digit = patterns.iter()
        .position(|pat| pat.len() == 5 && contains(pat, &result[1]))
        .unwrap();
    result[3] = patterns.remove(digit);
    debug!("Found 3: {}", result[3]);

    // find 5 - should be contained in 6
    let digit = patterns.iter()
        .position(|pat| pat.len() == 5 && contains(&result[6], pat))
        .unwrap();
    result[5] = patterns.remove(digit);
    debug!("Found 5: {}", result[5]);

    assert_eq!(patterns.len(), 1);
    result[2] = patterns.remove(0);
    debug!("Found 2: {}", result[2]);

    result
}

fn decode_digit(digits: &Vec<String>, text: &str) -> u32 {
    let pattern = sort_letters(text);
    digits.iter().position(|digit| *digit == pattern).unwrap() as u32
}
