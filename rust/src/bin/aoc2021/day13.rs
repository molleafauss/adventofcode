// https://adventofcode.com/2021/day/1

use std::collections::HashSet;
use std::str::FromStr;

use log::info;

use adventofcode::grid::GridPos;
use adventofcode::Solver;

pub struct Solution {
    dots: HashSet<GridPos>,
    folds: bool,
    part1: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            dots: HashSet::new(),
            folds: false,
            part1: 0,
        }
    }

    fn fold_x(&mut self, x: i64) {
        let (mut not_move, mut to_move): (HashSet<_>, HashSet<_>) = self.dots.drain()
            .partition(|dot| dot.col < x);
        to_move.drain()
            .for_each(|mut dot| {
                dot.col = x - (dot.col - x);
                not_move.insert(dot);
            });
        self.dots = not_move;
    }

    fn fold_y(&mut self, y: i64) {
        let (mut not_move, mut to_move): (HashSet<_>, HashSet<_>) = self.dots.drain()
            .partition(|dot| dot.row < y);
        to_move.drain()
            .for_each(|mut dot| {
                dot.row = y - (dot.row - y);
                not_move.insert(dot);
            });
        self.dots = not_move;
    }

    fn print_dots(&self) {
        let width = self.dots.iter().map(|dot| dot.col).max().unwrap() + 1;
        let height = self.dots.iter().map(|dot| dot.row).max().unwrap() + 1;
        let mut row = 0;
        while row < height {
            let mut line = vec!['.'; width as usize];
            self.dots.iter()
                .filter(|dot| dot.row == row)
                .for_each(|dot| line[dot.col as usize] = '#');
            println!("{}", line.iter().collect::<String>());
            row += 1;
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            self.folds = true;
            return;
        }

        if !self.folds {
            let (col, row) = line.split_once(",").unwrap();
            self.dots.insert(GridPos::of(
                i64::from_str(col).unwrap(),
                i64::from_str(row).unwrap(),
            ));
            return;
        }

        if line.starts_with("fold along y=") {
            self.fold_y(i64::from_str(&line[13..]).unwrap());
        } else if line.starts_with("fold along x=") {
            self.fold_x(i64::from_str(&line[13..]).unwrap());
        } else {
            panic!("Unknown instruction: {line}");
        }
        if self.part1 == 0 {
            self.part1 = self.dots.len();
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Dots visible after first fold: {}", self.part1);

        self.print_dots();

        Some((self.part1.to_string(), String::new()))
    }
}
