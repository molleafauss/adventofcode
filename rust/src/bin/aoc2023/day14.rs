// https://adventofcode.com/2023/day/14

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use log::{debug, info};

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

    fn move_rocks_south(&mut self) {
        for row in (0..self.height - 1).rev() {
            for col in 0..self.width {
                if self.rocks[row][col] != 'O' {
                    continue;
                }
                let mut r = row;
                while r < self.height - 1 && self.rocks[r + 1][col] == '.' {
                    self.rocks[r + 1][col] = 'O';
                    self.rocks[r][col] = '.';
                    r += 1;
                }
            }
        }
    }

    fn move_rocks_west(&mut self) {
        for col in 1..self.width {
            for row in 0..self.height {
                if self.rocks[row][col] != 'O' {
                    continue;
                }
                let mut c = col;
                while c > 0 && self.rocks[row][c - 1] == '.' {
                    self.rocks[row][c - 1] = 'O';
                    self.rocks[row][c] = '.';
                    c -= 1;
                }
            }
        }
    }

    fn move_rocks_east(&mut self) {
        for col in (0..self.width - 1).rev() {
            for row in 0..self.height {
                if self.rocks[row][col] != 'O' {
                    continue;
                }
                let mut c = col;
                while c < self.width - 1 && self.rocks[row][c + 1] == '.' {
                    self.rocks[row][c + 1] = 'O';
                    self.rocks[row][c] = '.';
                    c += 1;
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
                let weight = self.height - row;
                rocks * weight
            }).sum()
    }

    fn calculate_rock_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        for row in 0..self.height {
            for col in 0..self.width {
                if self.rocks[row][col] == 'O' {
                    (row, col).hash(&mut s);
                }
            }
        }
        s.finish()
    }

    fn print_rocks(&self) {
        if self.width > 10 {
            return;
        }
        for row in 0..self.height {
            let text: String = self.rocks[row].iter().collect();
            println!("{text}");
        }
    }
}

static MAX_CYCLES: usize = 1000000000;

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

        let mut part1 = 0;
        let mut part2 = 0;
        let mut cycle = 1;
        // cache cycle -> weight
        // idea is when I find again the same weight means I can keep cycle
        let mut cache: HashMap<u64, (usize, usize)> = HashMap::new();
        while cycle < MAX_CYCLES && part2 == 0 {
            self.move_rocks_north();
            if part1 == 0 {
                part1 = self.calculate_rock_weight();
            }
            self.move_rocks_west();
            self.move_rocks_south();
            self.move_rocks_east();
            let weight = self.calculate_rock_weight();
            let cache_key = self.calculate_rock_hash();
            debug!("[cycle {cycle}] => weight {weight}, cache_key {cache_key}");
            if !cache.contains_key(&cache_key) {
                cache.insert(cache_key, (cycle, weight));
                cycle += 1;
                continue;
            }
            // calculate part 2 by finding the previous time we were at this weight, and extrapolate
            // to MAX_CYCLES
            let (previous_cycle, _) = cache.get(&cache_key).unwrap();
            let repeat_cycles = cycle - previous_cycle;
            let cycles_to_max = (MAX_CYCLES - cycle) / repeat_cycles;
            debug!("Found cycle on {cycle} [weight {weight}] => cycle length {repeat_cycles}");
            // map back from previous cycle the cycle that would overlap with MAX_CYCLES
            let next_cycle = cycle + cycles_to_max * repeat_cycles;
            let final_diff = MAX_CYCLES - next_cycle;
            let cycle_with_final_weight = previous_cycle + final_diff;
            // find the final cycle in the cache and pick the weight from there
            let (_, final_val) = cache.iter()
                .find(|(_, (c, _))| *c == cycle_with_final_weight)
                .unwrap();
            part2 = final_val.1;
        }
        if part2 == 0 {
            part2 = self.calculate_rock_weight();
        }
        info!("[1] Weight of rocks: {}", part1);
        info!("[2] Weight of rocks: {}", part2);

        Some((part1.to_string(), part2.to_string()))
    }
}
