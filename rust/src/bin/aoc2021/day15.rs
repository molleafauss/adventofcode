// https://adventofcode.com/2021/day/15

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::str::FromStr;
use log::{debug, info};
use adventofcode::grid::{ALL_ORTHOGONAL, GridPos};
use adventofcode::Solver;

pub struct Solution {
    width: usize,
    height: usize,
    map: Vec<Vec<u32>>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            width: 0,
            height: 0,
            map: Vec::new(),
        }
    }

    fn make_part2_map(&self) -> (Vec<Vec<u32>>, usize, usize) {
        let width = self.width * 5;
        let height = self.height * 5;
        let mut map = vec![vec![0; width]; height];
        // copy over the existing map
        for r in 0..5 {
            for c in 0..5 {
                for row in 0..self.height {
                    for col in 0..self.width {
                        let nr = r * self.height + row;
                        let nc = c * self.width + col;
                        let mut val = self.map[row][col] + r as u32 + c as u32;
                        while val > 9 {
                            val -= 9;
                        }
                        map[nr][nc] = val;
                    }
                }
            }
        }
        (map, width, height)
    }
}

const ZERO: u8 = '0' as u8;

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(self.width, line.len());
        }
        self.map.push(line.bytes().into_iter().map(|ch| (ch - ZERO) as u32).collect());
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("Size of grid: {}x{}", self.width, self.height);
        let lowest_risk = find_path(&self.map, self.width, self.height);
        info!("[1] Lowest risk found: {}", lowest_risk);

        let (part2_map, width, height) = self.make_part2_map();
        debug!("Bigger map is {width}x{height}");
        let lowest_risk2 = find_path(&part2_map, width, height);
        info!("[2] Lowest risk found: {}", lowest_risk2);

        Some((lowest_risk.to_string(), lowest_risk2.to_string()))
    }
}

fn find_path(map: &Vec<Vec<u32>>, width: usize, height: usize) -> u32 {
    let end = GridPos::of((width - 1) as i64, (height - 1) as i64);
    let mut visited = vec![false; width * height];
    let mut queue = BinaryHeap::new();
    queue.push(Path(0, GridPos::of(0, 0)));

    loop {
        let Path(risk, pos) = queue.pop().unwrap();
        let idx = pos.to_linear(width, height).unwrap();
        if visited[idx] {
            continue;
        }
        if pos == end {
            return risk;
        }
        visited[idx] = true;
        for dir in &ALL_ORTHOGONAL {
            let next = pos.add(dir);
            if let Some(nidx) = next.to_linear(width, height) {
                if visited[nidx] {
                    continue;
                }
                queue.push(Path(risk + map[next.row as usize][next.col as usize], next));
            }
        }
    }
}

#[derive(Eq)]
struct Path(u32, GridPos);

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // need reverse to create a min-heap
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
