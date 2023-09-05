// https://adventofcode.com/2021/day/5

use std::collections::HashMap;
use std::str::FromStr;
use log::{debug, info};
use adventofcode::grid::{GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};
use adventofcode::Solver;

pub struct Solution {
    map: HashMap<GridPos, u32>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            map: HashMap::new(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let (start, end) = line.split_once(" -> ")
            .map(|(start, end)| (map_point(start), map_point(end)))
            .unwrap();
        // add all line points to the map - adding up for every point
        let delta = match (start.row - end.row, start.col - end.col) {
            (0, _) => if end.col > start.col { Some(MOVE_R) } else { Some(MOVE_L) }
            (_, 0) => if end.row > start.row { Some(MOVE_U) } else { Some(MOVE_D) }
            _ => None,
        };
        if delta.is_none() {
            debug!("Not a straight line: {:?} => {:?} - ignoring", start, end);
            return;
        }
        let mut pos = start.clone();
        let delta = delta.unwrap();
        loop {
            self.map.entry(pos.clone()).and_modify(|val| *val += 1).or_insert(1);
            if pos == end {
                break;
            }
            pos = pos.add(&delta);
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let part1 = self.map.iter().filter(|v| *v.1 > 1_u32).count();
        info!("[1] points with overlaps: {part1}");

        Some((part1.to_string(), String::new()))
    }
}

fn map_point(def: &str) -> GridPos {
    let (x, y) = def.split_once(",").unwrap();
    GridPos {
        row: i64::from_str(y).unwrap(),
        col: i64::from_str(x).unwrap(),
    }
}
