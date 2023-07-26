use std::str::FromStr;
use crate::solutions::Solver;

struct Elf {
    elf_id: i32,
    calories: i32,
}

pub struct Solution {
    elf: i32,
    elf_calories: i32,
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
            self.calories.push(Elf{elf_id: self.elf, calories: self.elf_calories});
            self.elf += 1;
            self.elf_calories = 0;
        } else {
            self.elf_calories += i32::from_str(line).unwrap();
        }
    }

    fn solve(&mut self) {
        println!("solve day01");
        self.calories.push(Elf{elf_id: self.elf, calories: self.elf_calories});
        self.calories.sort_by_key(|elf| elf.calories);
        self.calories.reverse();
        println!("[1] Saw {} elves: maximum: {}", self.calories.len(), self.calories[0].calories);
        let top3: i32 = self.calories.iter().take(3).map(|elf| elf.calories).sum();
        println!("[2] First 3 elves: {top3}");
    }
}