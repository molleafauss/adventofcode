// What did I learn?
// Vec basic APIs and a bit of functional/iterator constructs.

use std::str::FromStr;
use log::info;
use crate::solutions::Solver;

struct Elf {
    calories: u32,
}

pub struct Solution {
    elf: u32,
    elf_calories: u32,
    calories: Vec<Elf>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            elf: 0,
            elf_calories: 0,
            calories: Vec::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            self.calories.push(Elf{calories: self.elf_calories});
            self.elf += 1;
            self.elf_calories = 0;
        } else {
            self.elf_calories += u32::from_str(line).unwrap();
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("solve day01");
        self.calories.push(Elf{calories: self.elf_calories});
        self.calories.sort_by_key(|elf| elf.calories);
        self.calories.reverse();
        info!("[1] Saw {} elves: maximum: {}", self.calories.len(), self.calories[0].calories);
        let top3: u32 = self.calories.iter().take(3).map(|elf| elf.calories).sum();
        info!("[2] First 3 elves: {top3}");
        Some((self.calories[0].calories.to_string(), top3.to_string()))
    }
}