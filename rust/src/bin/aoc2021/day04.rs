// https://adventofcode.com/2021/day/4

use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    draws: Vec<u32>,
    boards: Vec<Board>,
    draws_parsed: bool,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            draws: Vec::new(),
            boards: Vec::new(),
            draws_parsed: false,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if !self.draws_parsed  {
            self.draws = line.split(",").map(|x| u32::from_str(x).unwrap()).collect();
            self.draws_parsed = true;
            return;
        }
        if line.is_empty() && self.draws_parsed {
            self.boards.push(Board::new(self.boards.len() + 1));
        } else {
            self.boards.last_mut().unwrap().add_line(line);
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Found {} draws and {} boards", self.draws.len(), self.boards.len());
        let mut part1 = None;
        let mut win = 0;
        let mut last_num = 0;
        let mut last_id = 0;
        for num in &self.draws {
            if win == self.boards.len() - 1 {
                // find last that will win
                last_id = self.boards.iter().find(|b| !b.win).unwrap().id;
            }
            if win == self.boards.len() {
                // all have won
                break;
            }
            last_num = *num;
            for b in &mut self.boards {
                if let Some(score) = b.draw(*num) {
                    win += 1;
                    if part1.is_none() {
                        part1 = Some(score);
                    }
                }
            }
        }
        info!("[1] Part 1 result: {}", part1.unwrap());

        // find the board which has not yet win and calculate the score
        let last_board = self.boards.iter().find(|b| b.id == last_id).unwrap();
        let last_score = last_board.score();
        let part2 = last_score * last_num;
        info!("[2] Part 2 result: last score {}, last_num {} => {}", last_score, last_num, part2);

        Some((part1.unwrap().to_string(), part2.to_string()))
    }
}

struct Board {
    win: bool,
    id: usize,
    table: [[(u32, bool); 5]; 5],
    line: usize,
}

impl Board {
    fn new(id: usize) -> Board {
        Board {
            id,
            win: false,
            table: [[(0, false); 5]; 5],
            line: 0,
        }
    }

    fn add_line(&mut self, text: &str) {
        let numbers: Vec<u32> = text.split_whitespace().map(|x| u32::from_str(x).unwrap()).collect();
        assert_eq!(numbers.len(), 5);
        assert!(self.line < 5);
        for i in 0..5 {
            self.table[self.line][i].0 = numbers[i];
        }
        self.line += 1;
    }

    fn draw(&mut self, num: u32) -> Option<u32> {
        if self.win {
            return None;
        }
        // check if the number is in the table
        let mut pos = None;
        for row in 0..5 {
            if pos.is_some() {
                break;
            }
            for col in 0..5 {
                if self.table[row][col].0 == num {
                    self.table[row][col].1 = true;
                    pos = Some((row, col));
                    break;
                }
            }
        }

        if pos.is_none() {
            return None;
        }

        // evaluate if row or col is full
        let (row, col) = pos.unwrap();
        debug!("Found drawn number {num} on board {} at ({row}, {col})", self.id);
        let full_row = (0..5).into_iter().all(|col| self.table[row][col].1);
        let full_col = (0..5).into_iter().all(|row| self.table[row][col].1);

        if full_row || full_col {
            let sum = self.score();
            debug!("Board {}: winner on {num} - {full_row}/{full_col} ({row}, {col}) => unmarked {sum}", self.id);
            self.win = true;
            return Some(sum * num);
        }

        None
    }

    fn score(&self) -> u32 {
        // sum all unmarked numbers
        (0..5).into_iter().map(|row| {
            (0..5).into_iter()
                .filter(|col| self.table[row][*col].1 == false)
                .map(|col| self.table[row][col].0)
                .sum::<u32>()
        }).sum::<u32>()
    }
}