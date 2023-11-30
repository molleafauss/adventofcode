// https://adventofcode.com/2021/day/9
// learned use of reduce

use std::collections::{HashSet, VecDeque};
use log::{debug, info};
use adventofcode::grid::{GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};
use adventofcode::Solver;
use adventofcode::utils::ZERO;

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
            if self.map[pos] >= self.map[adj] {
                // debug!("[{}] {} => [{}] {} => not a low point: {} >= {}", pos, coord, dir, new_pos, self.map[pos], self.map[adj as usize]);
                return None;
            }
        }
        debug!("Pos [{}] {} is a low point", pos, coord);
        Some(self.map[pos] + 1)
    }

    fn find_basin_size(&self, start: usize) -> usize {
        let mut basin: HashSet<usize> = HashSet::new();
        debug!("Checking basin starting at pos {start}");
        let mut explore = VecDeque::from([start]);
        while explore.len() > 0 {
            let pos = explore.pop_front().unwrap();
            for dir in [MOVE_D, MOVE_R, MOVE_U, MOVE_L] {
                let new_pos = GridPos::from_linear(pos, self.width).add(&dir).to_linear(self.width, self.height);
                if let Some(val) = new_pos {
                    // just ignore the 9's?
                    if self.map[val] != 9  && !basin.contains(&val) {
                        explore.push_back(val);
                        basin.insert(val);
                    }
                }
            }
        }
        debug!("Basin size at {start}: {}", basin.len());
        basin.len()
    }
}

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
        let low_points: Vec<(usize, &u8)> = self.map.iter().enumerate()
            .filter(|pos| self.is_low_point(pos.0).is_some())
            .collect();
        let risk1: u32 = low_points.iter()
            .map(|val| (*val.1 + 1) as u32)
            .sum();
        info!("[1] Part 1 - risk level: {risk1}");

        // for ech low point expand to find the basin
        let mut result: Vec<(usize, usize)> = low_points.iter()
            .map(|(pos, _val)| (*pos, self.find_basin_size(*pos)))
            .collect::<Vec<(usize, usize)>>();
        result.sort_by_key(|val| val.1);
        let sizes = result.iter().rev().take(3).map(|val| val.1).reduce(|accum, val| accum * val).unwrap();
        info!("[2] Part 2 - basin sizes: {sizes}");

        Some((risk1.to_string(), sizes.to_string()))
    }
}
