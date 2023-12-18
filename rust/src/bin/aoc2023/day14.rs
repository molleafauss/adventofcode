// https://adventofcode.com/2023/day/14

use log::debug;
use adventofcode::Solver;

pub struct Solution {
    rocks: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            rocks: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn move_rocks_north(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                if self.rocks[row][col] != 'O' {
                    continue;
                }
                let mut r = row;
                while r > 0 && self.rocks[r - 1][col] == '.' {
                    self.rocks[r - 1][col] = 'O';
                    self.rocks[r][col] = '.';
                    r -= 1;
                }
            }
        }
    }

    fn calculate_rock_weight(&self) -> usize {
        (0..self.height).map(
            |row| {
                let rocks = (0..self.width)
                    .filter(|col| self.rocks[row][*col] == 'O')
                    .count();
                let weight = (self.height - row);
                debug!("Found {rocks} at row {row} * {weight} => {}", rocks * weight);
                rocks * weight
            }).sum()
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(self.width, line.len());
        }
        self.height += 1;
        self.rocks.push(line.chars().collect());
    }

    fn solve(&mut self) -> Option<(String, String)> {
        debug!("Rocks area: {}x{}", self.width, self.height);

        self.move_rocks_north();
        let part1 = self.calculate_rock_weight();
        debug!("[1] Weight of rocks: {}", part1);

        Some((part1.to_string(), "".to_string()))
    }
}
