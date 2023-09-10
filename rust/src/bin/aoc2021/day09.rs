// https://adventofcode.com/2021/day/9

use log::{debug, info};
use adventofcode::grid::{GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};
use adventofcode::Solver;

pub struct Solution {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            map: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn is_low_point(&self, pos: usize) -> Option<u8> {
        let coord = GridPos::from_linear(pos, self.width);
        for dir in [MOVE_D, MOVE_R, MOVE_U, MOVE_L] {
            let new_pos = coord.add(&dir);
            let adj = new_pos.to_linear(self.width, self.height);
            if adj.is_none() {
                // outside bounds
                // debug!("[{}] {} => [{}] {} => outside bounds", pos, coord, dir, new_pos);
                continue;
            }
            let adj = adj.unwrap();
            // if this position is > than neighbour - skip it
            if self.map[pos] >= self.map[adj as usize] {
                // debug!("[{}] {} => [{}] {} => not a low point: {} >= {}", pos, coord, dir, new_pos, self.map[pos], self.map[adj as usize]);
                return None;
            }
        }
        debug!("Pos [{}] {} is a low point", pos, coord);
        Some(self.map[pos] + 1)
    }
}

const ZERO: u8 = '0' as u8;

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(self.width, line.len());
        }
        let row: Vec<u8> = line.as_bytes().iter().map(|ch| ch - ZERO).collect();
        self.map.extend_from_slice(&row);
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Grid is {}x{}", self.width, self.height);
        let mut risk1: u32 = 0;
        for pos in 0..self.map.len() {
            if let Some(risk) = self.is_low_point(pos) {
                risk1 += risk as u32;
            }
        }
        info!("[1] Part 1 - risk level: {risk1}");

        Some((risk1.to_string(), String::new()))
    }
}
