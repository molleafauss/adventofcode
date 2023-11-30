// https://adventofcode.com/2021/day/18

use adventofcode::Solver;

pub struct Solution {
    numbers: Vec<SnailNumber>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            numbers: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.numbers.push(SnailNumber::parse(line));
    }

    fn solve(&mut self) -> Option<(String, String)> {
        todo!()
    }
}

enum SnailValue {
    List(Vec<SnailNumber>),
    Value(u32),
}

struct SnailNumber(SnailValue, SnailValue);

impl SnailNumber {
    fn parse(text: &str) -> SnailNumber {
        SnailNumber(SnailValue::Value(0), SnailValue::Value(0))
    }
}