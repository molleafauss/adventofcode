use std::collections::HashMap;
use crate::Solver;
use crate::grid::{GridPos, MOVE_D, MOVE_L, MOVE_R, MOVE_U};

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
                let pos = GridPos::of(x as i32, y as i32);
                let v = if &pos == start { 0 } else { max_cost };
                costs.insert(pos, v);
            })
        });
        println!("=== Finding path {start} => {end} [costs: {}]", costs.len());
        let mut parents = HashMap::new();
        let mut next_node = start.clone();
        let mut visited = 0;
        while next_node != *end {
            visited += 1;
            self.neighbours(&next_node, &costs).into_iter().for_each(|n| {
                if costs[&next_node] + 1 < costs[&n] {
                    costs.insert(n.clone(), costs[&next_node] + 1);
                    parents.insert(n.clone(), next_node.clone());
                } else {
                    assert!(parents.contains_key(&n));
                }
            });
            costs.remove(&next_node);
            // find the next to visit
            let to_visit = costs.iter().min_by_key(|e| e.1).unwrap();
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
                self.map.contains_key(pos) && costs.contains_key(pos) && self.map[pos] <= max_height
            })
            .collect()
    }
}

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;


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
                self.start = Some(GridPos::of(i as i32, self.height as i32));
                letter = &('a' as u8);
            } else if *letter == END {
                self.end = Some(GridPos::of(i as i32, self.height as i32));
                letter = &('z' as u8);
            }
            self.map.insert(GridPos::of(i as i32, self.height as i32), *letter);
        });
        self.height += 1;
    }

    fn solve(&mut self) {
        assert!(self.start.is_some() && self.end.is_some(), "Start or end not found?");

        let parents = self.walk(&self.start.as_ref().unwrap());
        let path = self.walk_back(parents, &self.start.as_ref().unwrap());
        println!("[1] Min length found: {}", path.len())
    }
}
