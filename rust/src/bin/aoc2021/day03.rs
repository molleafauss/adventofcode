// https://adventofcode.com/2021/day/3
// nice way to learn how to use function pointers/lambdas as parameters

use log::info;
use adventofcode::Solver;
use adventofcode::utils::{ONE, ZERO};

pub struct Solution {
    digits: Vec<Digits>,
    signals: Vec<String>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            digits: Vec::new(),
            signals: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        while self.digits.len() < line.len() {
            self.digits.push(Digits(0, 0));
        }

        let bytes = line.as_bytes();
        for i in 0..bytes.len() {
            match bytes[i] {
                ZERO => self.digits[i].0 += 1,
                ONE => self.digits[i].1 += 1,
                _ => panic!("Invalid char: {}", bytes[i]),
            }
        }

        self.signals.push(String::from(line));
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..self.digits.len() {
            epsilon <<= 1;
            gamma <<= 1;
            let digit = &self.digits[i];
            if digit.0 > digit.1 {
                epsilon += 1;
            } else {
                gamma += 1;
            }
        }
        info!("[1] gamma {}, epsilon {} => result {}", gamma, epsilon, gamma * epsilon);

        // part2
        let oxygen = reduce(&self.signals, |digits| if digits.0 > digits.1 { ZERO } else { ONE });
        let co2 = reduce(&self.signals, |digits| if digits.0 <= digits.1 { ZERO } else { ONE });
        info!("[1] oxygen {}, co2 {} => result {}", oxygen, co2, oxygen * co2);

        Some(((gamma * epsilon).to_string(), (oxygen * co2).to_string()))
    }
}

// zero, one
struct Digits(u32, u32);

fn reduce(inputs: &Vec<String>, reductor: fn(Digits) -> u8) -> u32 {
    let mut signals : Vec<&String> = inputs.iter().collect();
    let mut idx = 0;
    while signals.len() > 1 {
        let mut digits = Digits(0, 0);
        // count digits at position idx
        signals.iter().for_each(|val| {
            if val.as_bytes()[idx] == ZERO {
                digits.0 += 1;
            } else {
                digits.1 += 1;
            }
        });
        let filter = reductor(digits);
        signals = signals.into_iter().filter(|val| val.as_bytes()[idx] == filter).collect();
        idx += 1;
    }
    // binary parse
    u32::from_str_radix(signals.get(0).unwrap(), 2).unwrap()
}
