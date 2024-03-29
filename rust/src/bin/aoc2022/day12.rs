// What did I learn?
// HashMap implementing Indexed (but only for reading), refactored GridPos already, iter().min_by_key,
// -> interesting it's slower than python by far and large (220sec vs 150sec) -> something wrong.
// (only due to debug target - release build took 10s - all good)

use std::collections::HashMap;
use log::{debug, info};
use adventofcode::Solver;
use adventofcode::grid::{GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};

pub(crate) struct Solution {
    width: usize,
    height: usize,
    start: Option<GridPos>,
    end: Option<GridPos>,
    map: HashMap<GridPos, u8>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            width: 0,
            height: 0,
            start: None,
            end: None,
            map: HashMap::new(),
        }
    }

    fn walk(&self, start: &GridPos) -> HashMap<GridPos, GridPos> {
        let end = self.end.as_ref().unwrap();
        let max_cost = self.width * self.height + 1;
        let mut costs = HashMap::new();
        (0..self.width).for_each(|x| {
            (0..self.height).for_each(|y| {
                let pos = GridPos::of(x as i64, y as i64);
                let v = if &pos == start { 0 } else { max_cost };
                costs.insert(pos, v);
            })
        });
        debug!("=== Finding path {start} => {end} [costs: {}]", costs.len());
        let mut parents = HashMap::new();
        let mut next_node = start.clone();
        while next_node != *end {
            self.neighbours(&next_node, &costs).into_iter().for_each(|n| {
                if costs[&next_node] + 1 < costs[&n] {
                    costs.insert(n.clone(), costs[&next_node] + 1);
                    parents.insert(n.clone(), next_node.clone());
                } else {
                    assert!(parents.contains_key(&n),
                            "No parent for {} ({next_node}) // {} // {}", n, costs[&next_node], costs[&n]);
                }
            });
            costs.remove(&next_node);
            // find the next to visit
            let to_visit = costs.iter().min_by_key(|e| e.1).unwrap();
            if to_visit.1 == &max_cost {
                parents.clear();
                break;
            }
            next_node = to_visit.0.clone();
        }
        parents
    }

    fn walk_back(&self, parents: HashMap<GridPos, GridPos>, start: &GridPos) -> Vec<GridPos> {
        let mut path = Vec::new();
        let end = self.end.as_ref().unwrap();
        if !parents.contains_key(end) {
            return path;
        }
        let mut n = end;
        while n != start {
            path.push(n.clone());
            n = &parents[&n];
        }
        // not needed, but just to be clear
        path.reverse();
        path
    }

    fn neighbours(&self, node: &GridPos, costs: &HashMap<GridPos, usize>) -> Vec<GridPos> {
        let max_height = self.map[node] + 1;
        [node.add(&MOVE_U), node.add(&MOVE_D), node.add(&MOVE_L), node.add(&MOVE_R)]
            .into_iter()
            .filter(|pos| {
                // not out of bounds && not visited && acceptable height (less than current + 1)
                self.map.contains_key(pos) && costs.contains_key(pos) && self.map[pos] <= max_height
            })
            .collect()
    }
}

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;
const LOWEST: u8 = 'a' as u8;
const HIGHEST: u8 = 'z' as u8;

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(self.width, line.len(), "Invalid line length");
        }

        let bytes = line.as_bytes();
        (0..self.width).for_each(|i| {
            let mut letter = bytes.get(i).unwrap();
            if *letter == START {
                self.start = Some(GridPos::of(i as i64, self.height as i64));
                letter = &LOWEST;
            } else if *letter == END {
                self.end = Some(GridPos::of(i as i64, self.height as i64));
                letter = &HIGHEST;
            }
            self.map.insert(GridPos::of(i as i64, self.height as i64), *letter);
        });
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        assert!(self.start.is_some() && self.end.is_some(), "Start or end not found?");

        let parents = self.walk(&self.start.as_ref().unwrap());
        let path = self.walk_back(parents, &self.start.as_ref().unwrap());
        let part1_min_length = path.len();
        info!("[1] Min length found: {part1_min_length}");

        // finding shortest
        let mut min_start = self.start.as_ref().unwrap();
        let mut part2_min_length = self.map.len() + 1;
        let mut visited = 0;
        self.map.iter()
            .filter(|e| e.1 == &LOWEST)
            .map(|e| e.0)
            .for_each(|start| {
                visited += 1;
                if visited % 100 == 0 {
                    println!("{visited} visited ...")
                }
                let path = self.walk_back(self.walk(start), start);
                if path.len() > 0 && path.len() < part2_min_length {
                    min_start = start;
                    part2_min_length = path.len();
                }
            });
        info!("[2] Shortest path from {min_start}: {part2_min_length}");
        Some((part1_min_length.to_string(), part2_min_length.to_string()))
    }
}
