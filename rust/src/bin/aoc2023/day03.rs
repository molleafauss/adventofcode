// https://adventofcode.com/2023/day/3
// not the easiest, but Rust iterators and functional constructs are a pleasure to write

use std::collections::{HashMap, HashSet};
use log::{debug, info};
use adventofcode::grid::{ALL_SURROUNDING, GridPos};
use adventofcode::Solver;
use adventofcode::utils::ZERO;

pub struct Solution {
    numbers: Vec<Number>,
    numbers_pos: HashMap<GridPos, usize>,
    symbols: Vec<Symbol>,
    width: usize,
    height: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            symbols: Vec::new(),
            numbers: Vec::new(),
            numbers_pos: HashMap::new(),
            width: 0,
            height: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(line.len(), self.width);
        }
        let mut number = None;
        line.chars().enumerate().for_each(|(idx, ch)| {
            if ch == '.' {
                number = None;
                return;
            }
            if !ch.is_numeric() {
                self.symbols.push(Symbol { glyph: ch, pos: GridPos::of(idx as i64, self.height as i64) });
                number = None;
                return;
            }
            // this is a number
            if number.is_none() {
                let nidx = self.numbers.len();
                number = Some(nidx);
                self.numbers.push(Number {
                    value: ch as u32 - ZERO as u32,
                    start: GridPos::of(idx as i64, self.height as i64),
                    end: GridPos::of(idx as i64, self.height as i64),
                });
                self.numbers_pos.insert(GridPos::of(idx as i64, self.height as i64), nidx);
            } else {
                let nidx = self.numbers.len() - 1;
                let cur_number = self.numbers.get_mut(nidx).unwrap();
                cur_number.value = cur_number.value * 10 + (ch as u32 - ZERO as u32);
                cur_number.end.col = idx as i64;
                self.numbers_pos.insert(GridPos::of(idx as i64, self.height as i64), nidx);
            }
        });
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        debug!("Map is {}x{}", self.width, self.height);

        let part1: u32 = self.symbols.iter()
            .map(|symbol| {
                // overkill way to extract only uniques
                let numbers: HashSet<&usize> = ALL_SURROUNDING.iter()
                    .filter_map(|dir| self.numbers_pos.get(&symbol.pos.add(dir)))
                    .collect();
                numbers.into_iter().map(|idx| self.numbers[*idx].value).sum::<u32>()
            })
            .sum();
        info!("[1] part numbers sum: {}", part1);

        let part2: u32 = self.symbols.iter()
            .filter(|symbol| symbol.glyph == '*')
            .map(|symbol| {
                ALL_SURROUNDING.iter()
                    .filter_map(|dir| self.numbers_pos.get(&symbol.pos.add(dir)))
                    .collect::<HashSet<&usize>>()
            })
            .filter(|maybe_gears| maybe_gears.len() == 2)
            .map(|numbers| numbers.into_iter().map(|idx| self.numbers[*idx].value).product::<u32>())
            .sum();
        info!("[2] gear score sum: {}", part2);

        Some((part1.to_string(), part2.to_string()))
    }
}

struct Number {
    value: u32,
    start: GridPos,
    end: GridPos,
}

#[derive(Debug)]
struct Symbol {
    glyph: char,
    pos: GridPos,
}
