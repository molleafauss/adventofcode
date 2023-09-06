// https://adventofcode.com/2021/day/5

use std::collections::HashMap;
use std::str::FromStr;
use log::{debug, info};
use adventofcode::grid::GridPos;
use adventofcode::Solver;

pub struct Solution {
    map: HashMap<GridPos, (u32, u32)>,
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
        let dx = if end.col > start.col { 1 } else if start.col > end.col { -1 } else { 0 };
        let dy = if end.row > start.row { 1 } else if start.row > end.row { -1 } else { 0 };
        let straight = dx == 0 || dy == 0;
        let mut pos = start.clone();
        let delta = GridPos::of(dx, dy);
        debug!("Moving {:?} => {:?} - delta {:?} - straight {}", start, end, delta, straight);
        loop {
            self.map.entry(pos.clone()).and_modify(|val|
                if straight {
                    val.0 += 1;
                } else {
                    val.1 += 1;
                }).or_insert(if straight { (1, 0) } else { (0, 1) });
            if pos == end {
                break;
            }
            pos = pos.add(&delta);
        }
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let part1 = self.map.iter().filter(|(_k, v)| v.0 > 1_u32).count();
        info!("[1] points with overlaps (only straight lines): {part1}");

        let part2 = self.map.iter().filter(|(_k, v)| (v.0 + v.1) > 1_u32).count();
        info!("[2] points with overlaps (including diagonal): {part2}");

        Some((part1.to_string(), part2.to_string()))
    }
}

fn map_point(def: &str) -> GridPos {
    let (x, y) = def.split_once(",").unwrap();
    GridPos {
        row: i64::from_str(y).unwrap(),
        col: i64::from_str(x).unwrap(),
    }
}
